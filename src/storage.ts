/**
 * Represents a sample.
 */
export class Sample {
    /**
     * Creates a new sample.
     * 
     * @param timestamp 
     * @param value 
     */
    constructor(
        public timestamp: number,
        public value: number
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
    static SAMPLES_CPU: Sample[] = []
    static SAMPLES_MEM: Sample[] = []
}
