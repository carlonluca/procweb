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
L_RW_PROP_AS(double, vmPeak, 0)
L_RW_PROP_AS(double, vmSize, 0)
L_RW_PROP_AS(double, rssPeak, 0)
L_RW_PROP_AS(double, rssSize, 0)
L_RW_PROP_AS(double, ramSize, 0)
L_RW_PROP_AS(long, numThreads, 0)
L_RW_PROP_AS(long, nice, 0)
L_RW_PROP_AS(QString, state)
L_END_CLASS
typedef QSharedPointer<PWSample> PWSampleRef;

inline QDebug& operator<<(QDebug& dbg, const PWSampleRef& sample)
{
    dbg << QString("{\n");
    dbg << QString("\t%1\n").arg(sample->ts());
    dbg << QString("\t%2\n").arg(sample->cpu());
    dbg << QString("\t%3\n").arg(QString::number(sample->rssSize(), 'f'));
    return dbg << QString("}");
}

#endif // PWDATA_H
