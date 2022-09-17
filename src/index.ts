import pidusage = require("pidusage")

pidusage(1758, { "usePs": true }, (err, stats: pidusage.Status) => {
    if (err) {
        console.warn("Error:", err.message)
        return
    }

    console.log("Stats:", stats.cpu)
})
