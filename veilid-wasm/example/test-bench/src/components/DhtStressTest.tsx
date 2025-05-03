import { useState } from 'react';
import { getRoutingContext } from '../veilid/veilid-core';

async function dhtStressTest() {
    const routingContext = getRoutingContext();

    const recordCount = 30;
    const subkeyCount = 32;
    const inspectCount = 1;

    // Create a 32KB data buffer
    const dataSize = 32 * 1024; // 32KB in bytes
    const dataArray = new Uint8Array(dataSize);

    // Fill the array with some pattern (using values 0-255 repeating)
    for (let i = 0; i < dataSize; i++) {
        dataArray[i] = i % 256;
    }

    let a = Array();
    for (var r = 0; r < recordCount; r++) {
        let dhtRecord = await routingContext.createDhtRecord(
            {
                kind: 'DFLT',
                o_cnt: subkeyCount,
            },
        );

        // Set all subkeys
        for (var n = 0; n < subkeyCount; n++) {
            a.push((async () => {
                // const measureName = `${r}-setDhtValue-${n}`;

                // performance.mark(measureName + "-start")
                await routingContext.setDhtValue(
                    dhtRecord.key,
                    n,
                    dataArray,
                );

                // performance.measure(measureName, measureName + "-start")
            })());
        }

        // Inspect all records N times while sets are happening
        for (var n = 0; n < inspectCount; n++) {
            a.push((async () => {
                const measureName = `${r}-inspectDhtRecord-${n}`;

                performance.mark(measureName + "-start")
                await routingContext.inspectDhtRecord(
                    dhtRecord.key,
                    null,
                    "SyncSet",
                );

                performance.measure(measureName, measureName + "-start")
            })());
        }
    }

    // Wait for all results
    await Promise.all(a)
}


export function DhtStressTest() {
    const [isRunning, setIsRunning] = useState(false);

    return (
        <button onClick={() => {
            if (isRunning) {
                return;
            }
            setIsRunning(true);
            dhtStressTest().finally(() => {
                setIsRunning(false);
            });
        }} disabled={isRunning}>
            {isRunning ? 'Running...' : 'Run DHT Stress Test'}
        </button>
    )
}   