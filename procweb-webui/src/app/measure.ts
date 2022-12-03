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
        return PWMeasure.arrayMaxValue(samples, this.key)
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
    constructor() { super("Resident Set Size", "rssSize") }

    override maxValue(samples: Sample[]): number {
        return samples[0].ramSize
    }
}
