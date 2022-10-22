import { Component } from '@angular/core'
import { HttpClient } from '@angular/common/http'
import { EChartsOption } from 'echarts'
import { XAXisComponentOption } from "echarts"
import { interval, Observable } from 'rxjs';

interface Sample {
    ts: number,
    cpu: number
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
        },
        yAxis: {
            type: 'value',
            min: 0,
            max: 100
        },
        series: [
            {
                type: "line",
                data: this.echartData,
                showSymbol: false
            }
        ]
    }
    dynamicData: EChartsOption = {}

    ngOnInit() {
        this.getSamples().subscribe((data: Sample[]) => {
            data.forEach((sample: Sample) => {
                this.echartData.push([sample.ts, sample.cpu * 100])
            })
            let xMin: number = this.arrayMinTimestamp(data);
            let xMax: number = this.arrayMaxTimestamp(data);
            let yMin: number = 0;
            let yMax: number = 100;
            (this.chartOption.xAxis! as XAXisComponentOption).min = xMin;
            (this.chartOption.xAxis! as XAXisComponentOption).max = xMax;
            (this.chartOption.yAxis! as XAXisComponentOption).min = yMin;
            (this.chartOption.yAxis! as XAXisComponentOption).max = yMax;
        })
        interval(1000).subscribe((val) => {
            this.getSamples().subscribe((data: Sample[]) => {
                let echartData: number[][] = []
                data.forEach((sample: Sample) => {
                    echartData.push([sample.ts, sample.cpu * 100])
                })
                this.dynamicData = {
                    series: [
                        {
                            type: "line",
                            data: echartData,
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
