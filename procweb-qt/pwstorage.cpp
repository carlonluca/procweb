#include "pwstorage.h"

void PWStorage::registerSample(PWSampleRef sample)
{
    QMutexLocker locker(&m_mutex);
    m_samples.append(sample);
}

QList<PWSampleRef> PWStorage::samples()
{
    QMutexLocker locker(&m_mutex);
    return m_samples;
}

PWStorage::PWStorage()
{

}
