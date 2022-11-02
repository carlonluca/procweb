import { Component } from '@angular/core'
import { HttpClient } from '@angular/common/http'
import { EChartsOption } from 'echarts'
import { XAXisComponentOption } from "echarts"
import { interval, Observable } from 'rxjs';

interface Sample {
    ts: number,
    cpu: number,
    rssSize: number
}

interface Setup {
    sampleInterval: number
}

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss']
})
export class AppComponent {
    constructor(private http: HttpClient) { }
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

    ngOnInit() {
        this.refresh()
        interval(1000).subscribe((_) => {
            this.refresh()
        })
    }

    refresh() {
        this.getSamples().subscribe((data: Sample[]) => {
            let cpuData: number[][] = []
            let memData: number[][] = []
            data.forEach((sample: Sample) => {
                cpuData.push([sample.ts, sample.cpu * 100])
                memData.push([sample.ts, sample.rssSize])
            })

            this.dynamicData = {
                series: [
                    {
                        type: "line",
                        data: cpuData,
                        showSymbol: false
                    }
                ],
                xAxis: {
                    min: this.arrayMinTimestamp(data),
                    max: this.arrayMaxTimestamp(data)
                },
                yAxis: {
                    min: 0,
                    max: 100
                }
            }
        })
    }

    getSamples() {
        return this.http.get<Sample[]>("/api/samples")
    }

    getSetup() {
        return this.http.get<Setup>("/api/setup")
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
}
