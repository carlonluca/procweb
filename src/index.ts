import pidusage = require("pidusage")
import yargs = require("yargs")
import helpers = require('yargs/helpers')

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
        pidusage.clear()
    })
}, 1000)
