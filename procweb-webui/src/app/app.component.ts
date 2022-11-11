import { Component } from '@angular/core'
import { EChartsOption } from 'echarts'
import { interval, Observable } from 'rxjs'
import prettyBytes from 'pretty-bytes'
import { HumanizeDurationLanguage, HumanizeDuration } from 'humanize-duration-ts';
import { Sample, SamplesService, TimeUom } from './samples.service'

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

    constructor(private sampleService: SamplesService) { }

    ngOnInit() {
        this.sampleService.samples.subscribe((data: Sample[]) => {
            let cpuData: number[][] = []
            let memData: number[][] = []
            data.forEach((sample: Sample) => {
                cpuData.push([sample.ts, sample.cpu * 100])
                memData.push([sample.ts, sample.rssSize])
            })

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

    onChange() {
        let langService = new HumanizeDurationLanguage()
        let humanizer = new HumanizeDuration(langService)
        console.log("Value:", humanizer.humanize(this.selectedValue*this.selectedUom.secs*1000))
    }
}
