import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { EChartsOption } from 'echarts';
import { XAXisComponentOption } from "echarts"

interface Sample {
  timestamp: number,
  value: number
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
  chartOption: EChartsOption = {
    xAxis: {
      type: 'value',
      min: 0,
      max: 0
    },
    yAxis: {
      type: 'value',
      min: 0,
      max: 100
    },
    series: [
      {
        type: "line",
        data: this.echartData
      }
    ]
  }

  ngOnInit() {
    this.getSamples().subscribe((data: Sample[]) => {
      data.forEach((sample: Sample) => {
        this.echartData.push([sample.timestamp, sample.value])
      })
      let xMin: number = this.arrayMinTimestamp(data);
      let xMax: number = this.arrayMaxTimestamp(data);
      let yMin: number = this.arrayMinValue(data);
      let yMax: number = this.arrayMaxValue(data);
      (this.chartOption.xAxis! as XAXisComponentOption).min = xMin;
      (this.chartOption.xAxis! as XAXisComponentOption).max = xMax;
      (this.chartOption.yAxis! as XAXisComponentOption).min = yMin;
      (this.chartOption.yAxis! as XAXisComponentOption).max = yMax;
    })
  }

  getSamples() {
    return this.http.get<Sample[]>("/api/samples/cpu")
  }

  arrayMinTimestamp(arr: Sample[]): number {
    return arr.reduce((p: Sample, v: Sample) => {
      return p.timestamp < v.timestamp ? p : v
    }).timestamp
  }

  arrayMaxTimestamp(arr: Sample[]): number {
    return arr.reduce((p, v): Sample => {
      return p.timestamp > v.timestamp ? p : v
    }).timestamp
  }

  arrayMinValue(arr: Sample[]): number {
    return arr.reduce((p, v): Sample => {
      return p.value < v.value ? p : v
    }).value
  }

  arrayMaxValue(arr: Sample[]): number {
    return arr.reduce((p, v): Sample => {
      return p.value > v.value ? p : v
    }).value
  }
}
