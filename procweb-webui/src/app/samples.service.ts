import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { interval, Observable, Observer, shareReplay } from 'rxjs';

export interface Sample {
    ts: number,
    pid: number,
    cpu: number,
    rssSize: number,
    ramSize: number,
    numThreads: number,
    nice: number,
    state: string
}

export interface Setup {
    sampleInterval: number,
    pid: number,
    cmdline: string
}

export class TimeUom {
    constructor(public display: string, public value: string, public secs: number) {}
}

@Injectable({
    providedIn: 'root'
})
export class SamplesService {
    samples: Observable<Sample[]>
    setup: Observable<Setup>

    constructor(private http: HttpClient) {
        this.samples = new Observable<Sample[]>((observer) => {
            shareReplay(1)
            this.getSamples().subscribe((data: Sample[]) => observer.next(data))
            this.refreshSamples(observer)
        })

        this.setup = new Observable<Setup>((observer) => {
            shareReplay(1)
            this.getSetup().subscribe((data: Setup) => observer.next(data))
            this.refreshSetup(observer)
        })
    }

    private getSetup() {
        return this.http.get<Setup>("/api/setup")
    }

    private refreshSetup(observer: Observer<Setup>) {
        interval(10000).subscribe((_) => {
            this.getSetup().subscribe((data: Setup) => {
                observer.next(data)
            })
        })
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
