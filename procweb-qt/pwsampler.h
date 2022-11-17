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
