#ifndef PWDATA_H
#define PWDATA_H

#include <QObject>
#include <QSharedPointer>
#include <QDebug>

#include <lqtutils_prop.h>

Q_NAMESPACE

L_BEGIN_CLASS(PWSetup)
L_RW_PROP_AS(qint64, sampleInterval, 1000)
L_RW_PROP_AS(int, pid, -1)
L_RW_PROP_AS(QString, cmdline)
L_END_CLASS

L_BEGIN_CLASS(PWSample)
L_RW_PROP_AS(qint64, ts, 0)
L_RW_PROP_AS(double, cpu, 0)
L_RW_PROP_AS(qint64, vmPeak, 0)
L_RW_PROP_AS(qint64, vmSize, 0)
L_RW_PROP_AS(qint64, rssPeak, 0)
L_RW_PROP_AS(qint64, rssSize, 0)
L_RW_PROP_AS(qint64, ramSize, 0)
L_RW_PROP_AS(qint64, numThreads, 0)
L_RW_PROP_AS(qint64, nice, 0)
L_RW_PROP_AS(qint64, uptime, 0)
L_RW_PROP_AS(qint64, readAll, 0)
L_RW_PROP_AS(qint64, writeAll, 0)
L_RW_PROP_AS(qint64, readDisk, 0)
L_RW_PROP_AS(qint64, writeDisk, 0)
L_RW_PROP_AS(QString, startTime)
L_RW_PROP_AS(QString, state)
L_END_CLASS
typedef QSharedPointer<PWSample> PWSampleRef;

inline QDebug& operator<<(QDebug& dbg, const PWSampleRef& sample)
{
    dbg << QString("{\n");
    dbg << QString("\t%1\n").arg(sample->ts());
    dbg << QString("\t%2\n").arg(sample->cpu());
    dbg << QString("\t%3\n").arg(sample->rssSize());
    return dbg << QString("}");
}

#endif // PWDATA_H
