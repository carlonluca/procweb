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

#include <QFile>
#include <QDebug>
#include <QCoreApplication>

#include <lqtutils_string.h>

#include <unistd.h>
#include <sys/sysinfo.h>

#include "pwsampler.h"
#include "pwreader.h"

const QRegularExpression PWSampler::REGEX_RCHAR =
        QRegularExpression(QSL("^rchar:\\s+(\\d+)"));
const QRegularExpression PWSampler::REGEX_WCHAR =
        QRegularExpression(QSL("^wchar:\\s+(\\d+)"));
const QRegularExpression PWSampler::REGEX_RBYTES =
        QRegularExpression(QSL("^read_bytes:\\s+(\\d+)"));
const QRegularExpression PWSampler::REGEX_WBYTES =
        QRegularExpression(QSL("^write_bytes:\\s+(\\d+)"));

PWSampler::PWSampler(int pid, QObject* parent) :
    QObject{parent}
  , m_samplerTimer(new QTimer(this))
  , m_pid(pid)
  , m_lastProcCpuTime(0)
  , m_lastCpuTime(0)
{
    connect(this, &PWSampler::sampleIntervalChanged, this, [this] {
        m_samplerTimer->setInterval(m_sampleInterval);
    });

    connect(m_samplerTimer, &QTimer::timeout,
            this, &PWSampler::acquireSample);
    m_samplerTimer->setInterval(sampleInterval());
    m_samplerTimer->setSingleShot(false);

    if (qEnvironmentVariableIsSet("PROCWEB_APPIMAGE_BUILDER_TEST"))
        QTimer::singleShot(5000, this, [] { qApp->exit(0); });
    else
        m_samplerTimer->start();
}

void PWSampler::clearSamples()
{
    m_samples.clear();
}

void PWSampler::acquireSample()
{
    // CPU
    const QString procStatPath = PWReader::procStatDir(m_pid);
    QFile procStatFile(procStatPath);
    if (!procStatFile.exists()) {
        qCritical() << "Process cannot be found:" << m_pid;
        qApp->exit(-1);
        return;
    }

    if (!procStatFile.open(QIODevice::ReadOnly)) {
        qWarning() << "Could not open" << procStatPath << "for reading";
        return;
    }

    const QString procStatContent = QString::fromUtf8(procStatFile.readAll()).trimmed();
    if (procStatContent.isEmpty()) {
        qWarning() << "Failed to read process stats";
        return;
    }

    const QStringList procStatValues = procStatContent.split(' ', Qt::KeepEmptyParts);
    const quint64 procUtime = lqt::string_to_uint64(procStatValues[13], -1);
    const quint64 procStime = lqt::string_to_uint64(procStatValues[14], -1);
    const quint64 procStartTime = lqt::string_to_uint64(procStatValues[21], -1);
    if (procUtime == -1 || procStime == -1 || procStartTime == -1) {
        qWarning() << "Failed to parse proc stats";
        return;
    }

    const quint64 procUsageTicks = procUtime + procStime;

    const QString statPath = QSL("/proc/stat");
    QFile statFile(statPath);
    if (!statFile.exists()) {
        qWarning() << "Failed to read cpu stats";
        return;
    }

    if (!statFile.open(QIODevice::ReadOnly)) {
        qWarning() << "Failed to read stat file" << statPath;
        return;
    }

    const QString statContent = QString::fromUtf8(statFile.readAll()).trimmed();
    if (statContent.isEmpty()) {
        qWarning() << "Failed to read cpu stats";
        return;
    }

    const QStringList statLines = statContent.split('\n', Qt::SkipEmptyParts);
    if (statLines.isEmpty()) {
        qWarning() << "Failed to parse cpu stats file";
        return;
    }

    const QStringList statValues = statLines[0].split(' ', Qt::SkipEmptyParts);
    if (statValues.isEmpty()) {
        qWarning() << "Failed to parse cpu stats file";
        return;
    }

    quint64 cpuTime = 0;
    for (const QString& statValue : statValues)
        cpuTime += lqt::string_to_uint64(statValue, 0);

    if (m_lastCpuTime < 0 || m_lastProcCpuTime < 0) {
        m_lastCpuTime = cpuTime;
        m_lastProcCpuTime = procUsageTicks;
        return;
    }

    double cpu = (cpuTime - m_lastCpuTime == 0) ? 0 : (procUsageTicks - m_lastProcCpuTime)/static_cast<double>(cpuTime - m_lastCpuTime);

    m_lastCpuTime = cpu;
    m_lastProcCpuTime = procUsageTicks;

    // RSS
    qint64 rss = 0;
    if (procStatValues.size() > 23) {
        const int pageSize = getpagesize();
        rss = procStatValues.at(23).toULongLong()*pageSize;
    }

    // Total mem
    std::optional<qint64> totalMem = readTotalMem();

    // Num threads
    qint64 numThreads = 0;
    if (procStatValues.size() > 19)
        numThreads = lqt::string_to_int64(procStatValues[19], 0);

    // Niceness
    qint64 niceness = 0;
    if (procStatValues.size() > 18)
        niceness = lqt::string_to_int64(procStatValues[18], 0);

    // State
    QString state;
    if (procStatValues.size() > 2)
        state = procStatValues[2];

    // Virtual size
    qint64 vsize = 0;
    if (procStatValues.size() > 22)
        vsize = lqt::string_to_int64(procStatValues[22], 0);

    // Start
    qint64 startTimeMs = 0;
    if (procStatValues.size() > 21)
        if (long int clockTick = sysconf(_SC_CLK_TCK))
            startTimeMs = qRound64(lqt::string_to_uint64(procStatValues[21], 0)/static_cast<double>(clockTick))*1000;

    std::optional<quint64> uptimeMs = readSysUptimeMillis();
    QDateTime startTimeProc;
    if (uptimeMs) {
        const quint64 procUptimeMs = *uptimeMs - startTimeMs;
        const quint64 nowMs = QDateTime::currentMSecsSinceEpoch();
        startTimeProc = QDateTime::fromMSecsSinceEpoch(nowMs - procUptimeMs);
    }

    // Proc io
    PWIoValues all { 0, 0 };
    PWIoValues disk { 0, 0 };

    PWSampleRef sample(new PWSample);
    sample->set_cpu(cpu);
    sample->set_ts(QDateTime::currentMSecsSinceEpoch());
    sample->set_rssSize(rss);
    sample->set_numThreads(numThreads);
    sample->set_nice(niceness);
    sample->set_state(state);
    sample->set_vmSize(vsize);
    if (uptimeMs)
        sample->set_uptime(*uptimeMs - startTimeMs);
    if (totalMem)
        sample->set_ramSize(*totalMem);
    if (!startTimeProc.isNull())
        sample->set_startTime(startTimeProc.toString(Qt::ISODateWithMs));
    if (readIoValues(disk, all)) {
        sample->set_readAll(all.read);
        sample->set_readDisk(disk.read);
        sample->set_writeAll(all.written);
        sample->set_writeDisk(disk.written);
    }
    m_samples.append(sample);

    m_lastCpuTime = cpuTime;
    m_lastProcCpuTime = procUsageTicks;
}

std::optional<quint64> PWSampler::readTotalMem()
{
    struct sysinfo info;
    if (sysinfo(&info) != 0) {
        qWarning() << "sysinfo returned error:" << strerror(errno);
        return std::nullopt;
    }

    quint32 memunit = info.mem_unit;
    quint64 total = info.totalram;
    return total*memunit;
}

std::optional<quint64> PWSampler::readSysUptimeMillis()
{
    QFile f(QSL("/proc/uptime"));
    if (!f.open(QIODevice::ReadOnly)) {
        qWarning() << "Could not open /proc/uptime";
        return std::nullopt;
    }

    QString s = f.readAll();
    QStringList tokens = s.split(' ');
    if (tokens.size() != 2) {
        qWarning() << "Cannot parse /proc/uptime content";
        return std::nullopt;
    }

    double uptimeSecs = lqt::string_to_float(tokens[0], -1);
    if (uptimeSecs < 0) {
        qWarning() << "Cannot parse /proc/uptime content";
        return std::nullopt;
    }

    return qRound64(uptimeSecs*1000);
}

bool PWSampler::readIoValues(PWIoValues& disk, PWIoValues& all)
{
    QFile procIoFile(PWReader::procIoDir(m_pid));
    if (!procIoFile.exists()) {
        qWarning() << "io stat file does not exist";
        return false;
    }

    if (!procIoFile.open(QIODevice::ReadOnly)) {
        qWarning() << "io stat could not be opened: permission issue?";
        return false;
    }

    auto readIfMatches = [] (const QRegularExpression& regex, const QString& pattern, quint64& v) {
        QRegularExpressionMatch match = regex.match(pattern);
        if (!match.hasMatch())
            return;
        v = lqt::string_to_uint64(match.captured(1), 0);
    };

    const QString content = procIoFile.readAll();
    const QStringList lines = content.split("\n", Qt::SkipEmptyParts);
    for (const QString& line : lines) {
        readIfMatches(REGEX_RBYTES, line, disk.read);
        readIfMatches(REGEX_WBYTES, line, disk.written);
        readIfMatches(REGEX_RCHAR, line, all.read);
        readIfMatches(REGEX_WCHAR, line, all.written);
    }

    return true;
}
