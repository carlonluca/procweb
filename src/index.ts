import yargs = require("yargs")
import helpers = require("yargs/helpers")
import express = require('express')
import fs = require("fs")
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

const app = express()
const port = 3000

app.get('/api/samples', (req, res) => {
    res.send(SampleStorage.SAMPLES)
})

app.listen(port, () => {
    console.log(`Example app listening on port ${port}`)
})

let pid = argv._[0]
let lastCpuTime: number = -1
let lastProcCpuTime: number = -1

setInterval(() => {
    fs.readFile("/proc/" + pid + "/stat", "utf8", (err, data) => {
        if (err) {
            console.warn("Failed to read prc stat. Process not running?")
            return
        }

        const values: string[] = data.split(" ")
        const procUtime = +values[13]
        const procStime = +values[14]
        const procStartTime = +values[21]
        const procUsageTicks = procUtime + procStime
        fs.readFile("/proc/stat", "utf8", (err, cpuData) => {
            const cpuValues: string[] = cpuData.split("\n")[0].split(" ")
            let cpuTime = 0
            for (let i = 1; i < cpuValues.length; i++) {
                cpuTime += +cpuValues[i]
            }

            if (lastCpuTime < 0 || lastProcCpuTime < 0) {
                lastCpuTime = cpuTime
                lastProcCpuTime = procUsageTicks
                return
            }

            let cpu = (cpuTime - lastCpuTime === 0) ? 0 : (procUsageTicks - lastProcCpuTime)/(cpuTime - lastCpuTime)
            let cpuSample = Sample.takeSample(cpu)
            SampleStorage.SAMPLES.push(cpuSample)

            console.log("Sample:", cpuSample)
            console.log("---")

            lastProcCpuTime = procUsageTicks
            lastCpuTime = cpuTime
        })
    })
}, 1000)
