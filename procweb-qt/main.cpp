#include <QCoreApplication>
#include <QtHttpServer>
#include <QHttpServerResponse>
#include <QJsonObject>

#include <lserializer.h>

#include "pwsampler.h"
#include "pwdata.h"

int main(int argc, char** argv)
{
    QCoreApplication a(argc, argv);

    qint64 pid;
    if (qEnvironmentVariableIsSet("PROCWEB_APPIMAGE_BUILDER_TEST"))
        pid = 0;
    else if (qEnvironmentVariableIsSet("PROCWEB_SELF_PID"))
        pid = a.applicationPid();
    else
        pid = a.arguments()[1].toLongLong();

    PWSampler sampler(pid);
    QHttpServer httpServer;
    httpServer.route("/api/samples", [&sampler] (const QUrl& url) {
        QList<PWSampleRef> samples = sampler.samples();
        QJsonArray response;
        LSerializer s;
        for (const PWSampleRef& sample : samples)
            response.append(QJsonValue(s.serialize<PWSample>(sample.data())));

        return QHttpServerResponse(QByteArray("application/json"),
                                   QJsonDocument(response).toJson(QJsonDocument::Compact),
                                   QHttpServerResponse::StatusCode::Ok);
    });
    httpServer.route("/api/setup", [&sampler] (const QUrl& url) {
        PWSetup setup;
        setup.set_sampleInterval(sampler.sampleInterval());
        LSerializer s;
        QJsonObject json = s.serialize<PWSetup>(&setup);

        return QHttpServerResponse(QByteArray("application/json"),
                                   QJsonDocument(json).toJson(QJsonDocument::Compact),
                                   QHttpServerResponse::StatusCode::Ok);
    });
    httpServer.route("/<arg>", [] (const QUrl& url) {
        QString fileName = url.path().isEmpty() ? QStringLiteral("index.html") : url.path();
        qDebug() << "File name:" << fileName;

        QFile f(QString(":/%1").arg(fileName));
        if (!f.open(QIODevice::ReadOnly))
            qWarning("Could not open web resource");
        if (fileName.endsWith(QStringLiteral(".css")))
            return QHttpServerResponse(QByteArray("text/css"), f.readAll());
        if (fileName.endsWith(QStringLiteral(".js")))
            return QHttpServerResponse(QByteArray("text/javascript"), f.readAll());
        if (fileName.endsWith(QStringLiteral(".html")))
            return QHttpServerResponse(QByteArray("text/html"), f.readAll());
        return QHttpServerResponse(QHttpServerResponse::StatusCode::NotFound);
    });
    const auto port = httpServer.listen(QHostAddress::Any, 3000);
    if (!port) {
        qCritical("Webserver failed to start");
        return 0;
    }

    qDebug() << "Listening on port:" << port;


    return a.exec();
}
