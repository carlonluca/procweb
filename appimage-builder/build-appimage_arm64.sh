#!/bin/bash

docker run -v $PWD:/out -it --rm gitlab.pihome.lan:5050/opensource/procweb/builder:latest " \
cd && \
git clone https://github.com/carlonluca/procweb.git --depth=1 && \
cd procweb && \
git submodule update --init --recursive && \
cd procweb-webui  && \
./build.sh && \
cd .. && \
cd procweb-qt && \
mkdir build && \
cd build && \
/opt/Qt-arm64-6.4.1/bin/qt-cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr && \
make && \
make install DESTDIR=AppDir && \
mkdir -p AppDir/usr/share/icons && \
touch AppDir/usr/share/icons/test.png && \
appimage-builder --recipe ../../appimage-builder/AppImageBuilder_aarch64.yml && \
ls -l && \
mv procweb*.AppImage /out/
"