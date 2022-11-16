import { Component } from '@angular/core'
import { EChartsOption } from 'echarts'
import { interval, Observable } from 'rxjs'
import prettyBytes from 'pretty-bytes'
import { HumanizeDurationLanguage, HumanizeDuration } from 'humanize-duration-ts';
import { Setup, Sample, SamplesService, TimeUom } from './samples.service'

class DisplayRow {
    constructor(
        public description: string,
        public value: string,
        public icon: string) {}
}

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.sass']
})
export class AppComponent {
    title = 'procweb-webui'
    echartData: number[][] = []
    theme = 'dark'
    chartOption: EChartsOption = {
        xAxis: {
            type: 'value',
            min: 0,
            max: 0,
            axisLabel: {
                rotate: 45,
                formatter: (value: number, index: number): string => {
                    const options: Intl.DateTimeFormatOptions = {
                        weekday: undefined,
                        year: undefined,
                        month: 'numeric',
                        day: 'numeric',
                        hour: "numeric",
                        minute: "numeric",
                        second: "numeric",
                        hour12: false
                    }
                    return new Date(value).toLocaleString(undefined, options)
                }
            }
        }
    }
    dynamicData: EChartsOption = {}

    // Time range
    timeUoms: TimeUom[] = [
        new TimeUom("minute(s)", "minutes", 60),
        new TimeUom("hour(s)", "hours", 60*60),
        new TimeUom("day(s)", "days", 24*60*60),
        new TimeUom("month(s)", "months", 30*24*60*60),
        new TimeUom("year(s)", "years", 12*30*24*60*60)
    ]
    selectedUom: TimeUom = this.timeUoms[1]
    selectedValue: number = 1
    procData: string = "-"
    sampledTime: string = "-"
    sampleTable: DisplayRow[] = []
    sampleTableTime: string = "-"
    sampleLast?: Sample = undefined
    displayedColumns: string[] = ['description', 'value']

    constructor(private sampleService: SamplesService) { }

    ngOnInit() {
        this.sampleService.setup.subscribe((data: Setup) => {
            if (!data)
                return
            this.procData = data.pid + " - " + data.cmdline
        })

        this.sampleService.samples.subscribe((data: Sample[]) => {
            if (!data || data.length <= 0)
                return

            let cpuData: number[][] = []
            let memData: number[][] = []
            data.forEach((sample: Sample) => {
                cpuData.push([sample.ts, sample.cpu * 100])
                memData.push([sample.ts, sample.rssSize])
            })

            this.sampleLast = data[data.length - 1]
            this.computeSamplingTime(data)
            this.computeSampleTable(data[data.length - 1])

            this.dynamicData = {
                series: [{
                        type: "line",
                        data: cpuData,
                        showSymbol: false,
                        yAxisIndex: 0
                    }, {
                        type: "line",
                        data: memData,
                        showSymbol: false,
                        yAxisIndex: 1
                    }
                ],
                xAxis: {
                    min: this.arrayMaxTimestamp(data) - this.selectedValue*this.selectedUom.secs*1000,
                    max: this.arrayMaxTimestamp(data),
                    axisLabel: {
                        color: "white"
                    }
                },
                yAxis: [{
                    min: 0,
                    max: 100,
                    axisLabel: {
                        formatter: (value: number, index: number): string => {
                            return value + "%"
                        },
                        color: "white"
                    }
                }, {
                    min: 0,
                    max: data[0].ramSize,
                    axisLabel: {
                        formatter: (value: number, index: number): string => {
                            return prettyBytes(value)
                        },
                        color: "white"
                    }
                }]
            }
        })
    }

    arrayMinTimestamp(arr: Sample[]): number {
        return arr.reduce((p: Sample, v: Sample) => {
            return p.ts < v.ts ? p : v
        }).ts
    }

    arrayMaxTimestamp(arr: Sample[]): number {
        return arr.reduce((p, v): Sample => {
            return p.ts > v.ts ? p : v
        }).ts
    }

    arrayMinValue(arr: Sample[]): number {
        return arr.reduce((p, v): Sample => {
            return p.cpu < v.cpu ? p : v
        }).cpu * 100
    }

    arrayMaxValue(arr: Sample[]): number {
        return arr.reduce((p, v): Sample => {
            return p.cpu > v.cpu ? p : v
        }).cpu * 100
    }

    computeSamplingTime(data: Sample[]) {
        let max = data.reduce((a: Sample, b: Sample) => a.ts > b.ts ? a : b)
        let min = data.reduce((a: Sample, b: Sample) => a.ts < b.ts ? a : b)
        let langService = new HumanizeDurationLanguage()
        let humanizer = new HumanizeDuration(langService)

        this.sampledTime = humanizer.humanize(max.ts - min.ts)
    }

    computeSampleTable(sample: Sample) {
        let langService = new HumanizeDurationLanguage()
        let humanizer = new HumanizeDuration(langService)

        // TODO: check null values
        let rows: DisplayRow[] = []
        rows.push(new DisplayRow("State", this.computeStateValue(sample.state), "fa-face-sleeping"))
        rows.push(new DisplayRow("CPU usage", (sample.cpu*100).toFixed(2) + "%", ""))
        rows.push(new DisplayRow("Resident set size", prettyBytes(sample.rssSize), ""))
        rows.push(new DisplayRow("Virtual memory size", prettyBytes(sample.vmSize), ""));
        rows.push(new DisplayRow("Total main memory", prettyBytes(sample.ramSize), ""))
        rows.push(new DisplayRow("Niceness", "" + sample.nice, ""));
        rows.push(new DisplayRow("Number of threads", "" + sample.numThreads, ""))
        rows.push(new DisplayRow("Uptime", humanizer.humanize(sample.uptime), ""))
        rows.push(new DisplayRow("Start time", new Date(sample.startTime).toString(), ""))

        this.sampleTable = rows
        this.sampleTableTime = new Date(sample.ts).toString()
    }

    computeStateValue(state: string): string {
        switch (state) {
            case "S":
                return "Sleeping in an interruptible wait (S)"
            case "R":
                return "Running (R)"
            case "D":
                return "Waiting in uninterruptible disk sleep (D)"
            case "Z":
                return "Zombie (Z)"
        }

        return state
    }

    onChange() {}
}
