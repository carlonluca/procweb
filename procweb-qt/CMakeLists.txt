#
# This file is part of procweb.
#
# Copyright (c) 2022 Luca Carlon
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, version 3.
#
# This program is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.
#
#
#
# Author:  Luca Carlon
# Date:    2022.12.13
# Company: -
#

cmake_minimum_required(VERSION 3.14)

project(procweb-qt LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(3rdparty/lqtutils/CMakeLists.txt)
include(3rdparty/lqobjectserializer/LQObjectSerializer/CMakeLists.txt)

find_package(QT NAMES Qt6 REQUIRED COMPONENTS Core HttpServer)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Core HttpServer)

add_executable(procweb-qt
    web.qrc
    main.cpp
    pwdata.h pwdata.cpp
    pwstorage.h pwstorage.cpp
    pwsampler.h pwsampler.cpp
    pwreader.h pwreader.cpp
)
target_link_libraries(procweb-qt
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::HttpServer
    lqtutils
    lqobjectserializer
)

install(TARGETS procweb-qt
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
