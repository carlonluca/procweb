#ifndef PWREADER_H
#define PWREADER_H

#include <QString>

class PWReader
{
public:
    PWReader();

    static QString procDir(int pid);
    static QString procStatDir(int pid);
    static QString procIoDir(int pid);
    static QString readCmdline(int pid);
};

#endif // PWREADER_H
