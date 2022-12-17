/*
 * This file is part of procweb.
 *
 * Copyright (c) 2022 Luca Carlon
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

/**
 * Author:  Luca Carlon
 * Date:    2022.12.13
 * Company: -
 */

#include <QCoreApplication>
#include <QtHttpServer>
#include <QHttpServerResponse>
#include <QJsonObject>
#include <QCommandLineParser>
#include <QCommandLineOption>

#include <lserializer.h>
#include <lqtutils_string.h>

#include <unistd.h>

#include "pwsampler.h"
#include "pwdata.h"
#include "pwreader.h"

int main(int argc, char** argv)
{
    QCoreApplication a(argc, argv);
    QCoreApplication::setApplicationName("procweb");
    QCoreApplication::setApplicationVersion(APP_VERSION);

    QCommandLineOption daemonize(QStringList() << QSL("d") << QSL("daemonize"),
                                 QSL("Daemonize the process"));
    QCommandLineParser parser;
    parser.setApplicationDescription(QSL("Procweb samples a process and serves "
                                         "web pages to interface with stored data."));
    parser.addHelpOption();
    parser.addVersionOption();
    parser.addOption(daemonize);
    parser.addPositionalArgument(QSL("procid"), QSL("Process ID to sample"));
    if (!parser.parse(a.arguments())) {
        qFatal("Failed to parse arguments: %s", qPrintable(parser.errorText()));
        return 1;
    }

    qint64 pid;
    if (qEnvironmentVariableIsSet("PROCWEB_APPIMAGE_BUILDER_TEST"))
        pid = 0;
    else if (!qEnvironmentVariableIsSet("PROCWEB_SELF_PID")) {
        QStringList args = parser.positionalArguments();
        if (args.size() < 1)
            parser.showHelp(1);
        pid = args[0].toLongLong();
    }

    if (parser.isSet(daemonize)) {
        pid_t processId = fork();
        if (processId < 0)
            qFatal("Failed to fork process");
        if (processId > 0) {
            qInfo() << "PID of child is" << processId;
            return 0;
        }
    }

    if (qEnvironmentVariableIsSet("PROCWEB_SELF_PID"))
        pid = a.applicationPid();

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
    httpServer.route("/api/setup", [&sampler, &pid] (const QUrl& url) {
        PWSetup setup;
        setup.set_sampleInterval(sampler.sampleInterval());
        setup.set_pid(pid);
        setup.set_cmdline(PWReader::readCmdline(pid));
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
