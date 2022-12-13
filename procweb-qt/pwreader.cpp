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

QString PWReader::procIoDir(int pid)
{
    return lqt::path_combine({
        procDir(pid),
        QSL("io")
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
