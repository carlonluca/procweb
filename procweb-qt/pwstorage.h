#ifndef PWSTORAGE_H
#define PWSTORAGE_H

#include <QList>
#include <QMutex>

#include "pwdata.h"

class PWStorage
{
public:
    static PWStorage& instance() {
        static PWStorage _instance;
        return _instance;
    }

    void registerSample(PWSampleRef sample);
    QList<PWSampleRef> samples();

private:
    PWStorage();

private:
    QMutex m_mutex;
    QList<PWSampleRef> m_samples;
};

#endif // PWSTORAGE_H
