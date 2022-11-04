#include <QFile>
#include <QDebug>
#include <QCoreApplication>

#include <lqtutils_string.h>

#include <unistd.h>
#include <sys/sysinfo.h>

#include "pwsampler.h"

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

void PWSampler::acquireSample()
{
    // CPU
    const QString procStatPath = QString("/proc/%1/stat").arg(m_pid);
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

    // RSS
    quint64 rss = 0;
    if (procStatValues.size() > 23) {
        const int pageSize = getpagesize();
        rss = procStatValues.at(23).toULongLong()*pageSize;
    }

    // Total mem
    std::optional<quint64> totalMem = readTotalMem();

    PWSampleRef sample(new PWSample);
    sample->set_cpu(cpu);
    sample->set_ts(QDateTime::currentMSecsSinceEpoch());
    sample->set_rssSize(rss);
    if (totalMem)
        sample->set_ramSize(*totalMem);
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
