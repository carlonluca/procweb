import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

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
    constructor(public display: string, public value: string) {}
}

@Injectable({
    providedIn: 'root'
})
export class SamplesService {
    constructor(private http: HttpClient) { }

    getSamples() {
        return this.http.get<Sample[]>("/api/samples")
    }

    getSetup() {
        return this.http.get<Setup>("/api/setup")
    }
}
