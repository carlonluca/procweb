# Procweb
Procweb is a process that can be run to monitor another process in Linux systems. Procweb runs and samples the process periodically retrieving these values:

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

Some of these values can be used to draw a chart through the web interface provided by procweb.

Procweb can be used to monitor a process for memory leaks or unexpected behavior during many hours or days of work.

## Details

Procweb is a Rust-written process that can be run in background. The process provides a web interface written in TypeScript with Angular which represents data in time. Procweb stores samples without expiration, so it is possible to inspect old data. The web interface can be accessed at http://<ip_addr>:3000.

## Usage

To use the crate, simply install it and run by passing the PID of a running process to monitor:

```
cargo install procweb-rust
procweb-rust 1234
```

Then open a browser and go to:

```
http://<ip-addr>:3000/
```

<img src="https://github.com/carlonluca/procweb/raw/master/docs/screenshot.webp" />

## Static builds

If you need to run procweb on a system that does not provide cargo, rustc etc... like embedded systems, or if you want to avoid the installation of many dependencies, you can simply download one of the static builds. In the release section you can download fully self-contained static binaries that can be run without any dependencies on x64, aarch64 or armv7 (only Linux is supported).

## Qt version

A version based on Qt is also available in [procweb-qt](procweb-qt). That version is, however, deprecated.