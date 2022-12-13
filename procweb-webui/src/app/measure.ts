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

import { Sample } from "./samples.service"
import prettyBytes from 'pretty-bytes'

/**
 * Represents a measure.
 */
export class PWMeasure {
    constructor(
        public label: string,
        public key: string
    ) {}

    minValue(_: Sample[]) {
        return 0
    }

    maxValue(samples: Sample[]): number {
        return PWMeasure.arrayMaxValue(samples, this.key)*1.1
    }

    displayValue(value: number): string {
        return prettyBytes(value)
    }

    displayFactor(): number {
        return 1
    }

    protected static arrayMaxValue(arr: Sample[], key: string): number {
        return (arr.reduce((p, v): Sample => {
            return (p as any).cpu > v.cpu ? p : v
        }) as any)[key]
    }
}

export class PWMeasureCpu extends PWMeasure {
    constructor() { super("CPU", "cpu") }

    override maxValue(samples: Sample[]): number {
        return 100
    }

    override displayValue(value: number): string {
        return Math.round(value) + "%"
    }

    override displayFactor(): number {
        return 100
    }
}

export class PWMwasureRss extends PWMeasure {
    constructor() { super("Resident set size", "rssSize") }

    override maxValue(samples: Sample[]): number {
        return samples[0].ramSize
    }
}
