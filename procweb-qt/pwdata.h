#ifndef PWDATA_H
#define PWDATA_H

#include <QObject>
#include <QSharedPointer>
#include <QDebug>

#include <lqtutils_prop.h>

Q_NAMESPACE

L_BEGIN_CLASS(PWSample)
L_RW_PROP_AS(qint64, ts, 0)
L_RW_PROP_AS(double, cpu, 0)
L_END_CLASS
typedef QSharedPointer<PWSample> PWSampleRef;

inline QDebug& operator<<(QDebug& dbg, const PWSampleRef& sample)
{ return dbg << QString("{ %1, %2 }").arg(sample->ts()).arg(sample->cpu()); }

#endif // PWDATA_H
