import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError, min, retry } from 'rxjs/operators';
import { EChartsOption } from 'echarts';

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
      min: (new Date()).getTime() - 1000000,
      max: (new Date()).getTime() + 100000
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
        this.echartData.push([ sample.timestamp, sample.value ])
      })

    })
  }

  getSamples() {
    return this.http.get<Sample[]>("/api/samples/cpu")
  }
}
