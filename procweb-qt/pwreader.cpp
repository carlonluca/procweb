#include <lqtutils_string.h>

#include "pwreader.h"

PWReader::PWReader() {}

QString PWReader::procDir(int pid)
{
    return QString("/proc/%1").arg(pid);
}

QString PWReader::procStatDir(int pid)
{
    return lqt::path_combine({
        procDir(pid),
        QSL("stat")
    });
}

QString PWReader::readCmdline(int pid)
{
    const QString filePath = lqt::path_combine({ procDir(pid), QSL("cmdline") });
    QFile file(filePath);
    if (!file.exists()) {
        qWarning() << "Could not find cmdline file:" << filePath;
        return QString();
    }

    if (!file.open(QIODevice::ReadOnly)) {
        qWarning() << "Could not open cmdline file:" << filePath;
        return QString();
    }

    return file.readAll();
}
