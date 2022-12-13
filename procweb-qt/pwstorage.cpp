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
