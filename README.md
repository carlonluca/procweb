# Procweb
Procweb is a process that can be run to monitor another process. Procweb runs and samples the process periodically retrieving these values:

* State;
* CPU usage;
* Resident set size;
* Virtual memory size;
* Total main memory;
* Total read from disk;
* Total written to disk;
* Total read;
* Total written;
* Niceness;
* Number of threads;
* Uptime;
* Start time.

Some of these values can be used to draw a chart. Procweb provides a web interface to display data.

Procweb can be used to monitor a process for memory leaks or unexpected behavior during many hours or days of work.

## Details

Procweb is a C++ process that can be run in background. It depends on Qt >= 6.4. The process provides a web interface written in TypeScript with Angular which represents data in time. Procweb stores samples without expiration, so it is possible to inspect old data. The web interface can be accessed at http://<ip_addr>:3000.

## Installation

Procweb includes a AppImage that can be run, without having to install any dependency. At the moment it only works on x64.

## Demo

Here is an example of how procweb can monitor data and present it through the browser.

<p align="center">
<img width="70%" src="docs/screenshot.webp">
</p>

## Build

The project includes two modules: one is the angular webapp, the other is the Qt process running as a server and sampling the process. First, you'll need to build the angular application. The angular app is then embedded into the procweb executable:

```
cd procweb-webui
./build.sh
```

then you'll be able to build the Qt binary:

```
mkdir build
cd build
cmake ..
make
```