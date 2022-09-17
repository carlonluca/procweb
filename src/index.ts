import pidusage = require("pidusage")
import yargs = require("yargs")
import helpers = require('yargs/helpers')
import { Sample, SampleStorage } from "./storage"

let argv = yargs(helpers.hideBin(process.argv))
    .command("<pid>", "Monitor a process", (yargs) => {
        yargs.positional("pid", {
            describe: "The PID to monitor",
            type: "number"
        })
    })
    .demandCommand(1)
    .version('v0.1.0')
    .help()
    .parseSync()

let pid = argv._[0]

setInterval(() => {
    pidusage(pid, { "usePs": true }, (err, stats: pidusage.Status) => {
        if (err) {
            console.warn("Error:", err.message)
            return
        }

        console.log("Stats:", stats.cpu, stats.memory)

        let cpuSample = Sample.takeSample(stats.cpu)
        let memSample = Sample.takeSample(stats.memory)
        SampleStorage.SAMPLES_CPU.push(cpuSample)
        SampleStorage.SAMPLES_MEM.push(memSample)

        pidusage.clear()
    })
}, 1000)
