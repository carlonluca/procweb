/**
 * Represents a sample.
 */
export class Sample {
    /**
     * Creates a new sample.
     * 
     * @param ts
     * @param value 
     */
    constructor(
        public ts: number,
        public cpu: number
    ) {}

    /**
     * Creates a sample for the current instant.
     * 
     * @param value 
     * @returns 
     */
    static takeSample(value: number): Sample {
        return new Sample(new Date().getTime(), value)
    }
}

/**
 * Handles the storage.
 */
export class SampleStorage {
    static SAMPLES: Sample[] = []
}
