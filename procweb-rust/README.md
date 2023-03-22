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

Procweb is a Rust-written process that can be run in background. The process provides a web interface written in TypeScript with Angular which represents data in time. Procweb stores samples without expiration, so it is possible to inspect old data. The web interface can be accessed at http://<ip_addr>:3000.