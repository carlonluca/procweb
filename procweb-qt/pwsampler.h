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

#ifndef PWSAMPLER_H
#define PWSAMPLER_H

#include <QObject>
#include <QTimer>
#include <QRegularExpression>

#include <lqtutils_prop.h>

#include "pwdata.h"

struct PWIoValues
{
    quint64 read;
    quint64 written;
};

class PWSampler : public QObject
{
    Q_OBJECT
    L_RW_PROP_AS(qint64, sampleInterval, 1000)
public:
    explicit PWSampler(int pid, QObject* parent = nullptr);

    QList<PWSampleRef> samples() const { return m_samples; }
    void clearSamples();

private slots:
    void acquireSample();

private:
    std::optional<quint64> readTotalMem();
    std::optional<quint64> readSysUptimeMillis();
    bool readIoValues(PWIoValues& disk, PWIoValues& all);

private:
    static const QRegularExpression REGEX_RCHAR;
    static const QRegularExpression REGEX_WCHAR;
    static const QRegularExpression REGEX_RBYTES;
    static const QRegularExpression REGEX_WBYTES;
    QList<PWSampleRef> m_samples;
    QTimer* m_samplerTimer;
    int m_pid;
    qint64 m_lastProcCpuTime;
    qint64 m_lastCpuTime;
};

#endif // PWSAMPLER_H
