import { Component } from '@angular/core'
import { EChartsOption } from 'echarts'
import { interval, Observable } from 'rxjs'
import { PWMeasure, PWMeasureCpu, PWMwasureRss } from './measure'
import { HumanizeDurationLanguage, HumanizeDuration } from 'humanize-duration-ts';
import { Setup, Sample, SamplesService, TimeUom } from './samples.service'
import * as csv from 'csv-writer/web'
import prettyBytes from 'pretty-bytes'

class DisplayRow {
    constructor(
        public description: string,
        public value: string,
        public icon: string) {}
}

export class Measure {
    constructor(public label: string, public key: string) {}
}

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss']
})
export class AppComponent {
    title = 'procweb-webui'
    echartData: number[][] = []
    theme = 'dark'
    leftColor: string = "orange"
    rightColor: string = "red"
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

    // Measures
    measures: PWMeasure[] = [
        new PWMeasureCpu(),
        new PWMwasureRss(),
        new PWMeasure("Virtual memory size", "vmSize")
    ]
    measureLeft: PWMeasure = this.measures[0]
    measureRight: PWMeasure = this.measures[1]

    leftMin: number = 0
    leftSelectedMin: number = 10
    leftMax: number = 100
    leftSelectedMax: number = 80
    leftEnabled: boolean = false
    leftFullSelection: boolean = true
    rightMin: number = 0
    rightSelectedMin: number = 0
    rightMax: number = 50
    rightSelectedMax: number = 5E3
    rightEnabled: boolean = false
    rightFullSelection: boolean = true

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

            let leftData: number[][] = []
            let rightData: number[][] = []
            data.forEach((sample: Sample) => {
                leftData.push([sample.ts, ((sample as any)[this.measureLeft.key] as number) * this.measureLeft.displayFactor()])
                rightData.push([sample.ts, ((sample as any)[this.measureRight.key] as number) * this.measureRight.displayFactor()])
            })

            this.sampleLast = data[data.length - 1]
            this.computeSamplingTime(data)
            this.computeSampleTable(data[data.length - 1])

            this.rightMin = this.measureRight.minValue(data)
            this.rightMax = this.measureRight.maxValue(data)
            this.rightEnabled = true
            if (this.rightFullSelection) {
                this.rightSelectedMin = 0
                this.rightSelectedMax = 1
            }

            this.leftMin = this.measureLeft.minValue(data)
            this.leftMax = this.measureLeft.maxValue(data)
            this.leftEnabled = true
            if (this.leftFullSelection) {
                this.leftSelectedMin = 0
                this.leftSelectedMax = 1
            }

            this.dynamicData = {
                series: [{
                        type: "line",
                        data: leftData,
                        showSymbol: false,
                        yAxisIndex: 0,
                        color: this.leftColor
                    }, {
                        type: "line",
                        data: rightData,
                        showSymbol: false,
                        yAxisIndex: 1,
                        color: this.rightColor
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
                    min: this.leftSelectedMin*(this.leftMax - this.leftMin) - this.leftMin,
                    max: this.leftSelectedMax*(this.leftMax - this.leftMin) - this.leftMin,
                    axisLabel: {
                        formatter: (value: number, index: number): string => {
                            return this.measureLeft.displayValue(value)
                        },
                        color: this.leftColor
                    }
                }, {
                    min: this.rightSelectedMin*(this.rightMax - this.rightMin) - this.rightMin,
                    max: this.rightSelectedMax*(this.rightMax - this.rightMin) - this.rightMin,
                    axisLabel: {
                        formatter: (value: number, index: number): string => {
                            return this.measureRight.displayValue(value)
                        },
                        color: this.rightColor
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

    arrayMinValue(arr: Sample[], key: string): number {
        return (arr.reduce((p, v): Sample => {
            return (p as any)[key] < (v as any).cpu ? p : v
        }) as any)[key]
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
        rows.push(new DisplayRow("Total read from disk", prettyBytes(sample.readDisk), ""))
        rows.push(new DisplayRow("Total written to disk", prettyBytes(sample.writeDisk), ""))
        rows.push(new DisplayRow("Total read", prettyBytes(sample.readAll), ""))
        rows.push(new DisplayRow("Total written", prettyBytes(sample.writeAll), ""))
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

    selectionChanged() {
        this.leftFullSelection = false
        this.rightFullSelection = false
        this.dynamicData = {
            yAxis: [{
                min: this.leftSelectedMin*(this.leftMax - this.leftMin) - this.leftMin,
                max: this.leftSelectedMax*(this.leftMax - this.leftMin) - this.leftMin
            }, {
                min: this.rightSelectedMin*(this.rightMax - this.rightMin) - this.rightMin,
                max: this.rightSelectedMax*(this.rightMax - this.rightMin) - this.rightMin
            }]
        }
    }

    createCsv() {
        let writer = csv.createObjectCsvStringifier({
            header: [
                { id: "param", title: "Parameter" },
                { id: "value", title: "Value" }
            ]
        })

        const records = [
            { param: "p1", value: "v1" }
        ]

        console.log(writer.getHeaderString());
        // => 'NAME,LANGUAGE\n'
        
        console.log(writer.stringifyRecords(records));
        // => 'Bob,"French, English"\nMary,English\n'
    }
}
