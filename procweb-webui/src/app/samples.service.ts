/*
 * This file is part of procweb.
 *
 * Copyright (c) 2022 Luca Carlon
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

/**
 * Author:  Luca Carlon
 * Date:    2022.12.13
 * Company: -
 */

import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { interval, Observable, Observer, shareReplay } from 'rxjs';

export interface Sample {
    ts: number,
    pid: number,
    cpu: number,
    rssSize: number,
    rssPeak: number,
    vmSize: number,
    ramSize: number,
    numThreads: number,
    nice: number,
    state: string,
    uptime: number,
    startTime: string,
    readAll: number,
    writeAll: number,
    readDisk: number,
    writeDisk: number
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

    requestClearSamples() {
        this.http.delete("/api/proc/samples").subscribe()
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
        return this.http.get<Sample[]>("/api/proc/samples")
    }

    private refreshSamples(observer: Observer<Sample[]>) {
        interval(1000).subscribe((_) => {
            this.getSamples().subscribe((data: Sample[]) => {
                observer.next(data)
            })
        })
    }
}
