import { Component } from '@angular/core'
import { EChartsOption } from 'echarts'
import { interval, Observable } from 'rxjs'
import prettyBytes from 'pretty-bytes'
import { Sample, SamplesService, TimeUom } from './samples.service'

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss']
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
        new TimeUom("minute(s)", "minutes"),
        new TimeUom("hour(s)", "hours"),
        new TimeUom("day(s)", "days"),
        new TimeUom("month(s)", "months"),
        new TimeUom("year(s)", "years")
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
                    min: this.arrayMinTimestamp(data),
                    max: this.arrayMaxTimestamp(data)
                },
                yAxis: [{
                    min: 0,
                    max: 100,
                    axisLabel: {
                        formatter: (value: number, index: number): string => {
                            return value + "%"
                        }
                    }
                }, {
                    min: 0,
                    max: data[0].ramSize,
                    axisLabel: {
                        formatter: (value: number, index: number): string => {
                            return prettyBytes(value)
                        }
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

    }
}
