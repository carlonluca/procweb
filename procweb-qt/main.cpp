#include <QCoreApplication>
#include <QtHttpServer>
#include <QHttpServerResponse>

int main(int argc, char** argv)
{
    QCoreApplication a(argc, argv);

    QHttpServer httpServer;
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
