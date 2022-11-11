import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { interval, Observable, Observer, shareReplay } from 'rxjs';

export interface Sample {
    ts: number,
    cpu: number,
    rssSize: number,
    ramSize: number
}

export interface Setup {
    sampleInterval: number
}

export class TimeUom {
    constructor(public display: string, public value: string, public secs: number) {}
}

@Injectable({
    providedIn: 'root'
})
export class SamplesService {
    samples: Observable<Sample[]>

    constructor(private http: HttpClient) {
        this.samples = new Observable<Sample[]>((observer) => {
            shareReplay(1)
            this.refreshSamples(observer)
        })
    }

    getSetup() {
        return this.http.get<Setup>("/api/setup")
    }

    private getSamples() {
        return this.http.get<Sample[]>("/api/samples")
    }

    private refreshSamples(observer: Observer<Sample[]>) {
        interval(1000).subscribe((_) => {
            this.getSamples().subscribe((data: Sample[]) => {
                observer.next(data)
            })
        })
    }
}
