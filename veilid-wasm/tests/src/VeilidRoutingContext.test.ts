import { expect } from '@wdio/globals';

import {
  veilidCoreInitConfig,
  veilidCoreStartupConfig,
} from './utils/veilid-config';

import {
  DHTRecordDescriptor,
  VeilidRoutingContext,
  veilidClient,
  veilidCrypto,
} from 'veilid-wasm';
import { textEncoder, textDecoder } from './utils/marshalling-utils';
import { asyncCallWithTimeout, waitForPublicAttachment } from './utils/wait-utils';

describe('VeilidRoutingContext', () => {
  before('veilid startup', async () => {
    veilidClient.initializeCore(veilidCoreInitConfig);
    await veilidClient.startupCore((_update) => {
      // if (_update.kind === 'Log') {
      //   console.log(_update.message);
      // }
    }, JSON.stringify(veilidCoreStartupConfig));
    await veilidClient.attach();
    await asyncCallWithTimeout(waitForPublicAttachment(), 10000);
    //console.log("---Started Up---");
  });

  after('veilid shutdown', async () => {
    //console.log("---Shutting Down---");
    await veilidClient.detach();
    await veilidClient.shutdownCore();
  });

  describe('constructors', () => {
    it('should create using .create()', async () => {
      const routingContext = VeilidRoutingContext.create();
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);
    });

    it('should create using new', async () => {
      const routingContext = new VeilidRoutingContext();
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);
    });

    it('should create with default safety', async () => {
      const routingContext = VeilidRoutingContext.create().withDefaultSafety();
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);
    });

    it('should create with safety', async () => {
      const routingContext = VeilidRoutingContext.create().withSafety({
        Safe: {
          hop_count: 2,
          sequencing: 'EnsureOrdered',
          stability: 'Reliable',
        },
      });
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);
    });

    it('should create with sequencing', async () => {
      const routingContext =
        VeilidRoutingContext.create().withSequencing('EnsureOrdered');
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);
    });
  });

  describe('operations', () => {
    let routingContext: VeilidRoutingContext;

    before('create routing context', () => {
      routingContext = VeilidRoutingContext.create().withSafety({
        Unsafe: 'EnsureOrdered',
      });
    });

    describe('DHT kitchen sink', () => {
      let dhtRecord: DHTRecordDescriptor;
      const data = '🚀 This example DHT data with unicode a Ā 𐀀 文 🚀';

      beforeEach('create dht record', async () => {
        const bestKind = veilidCrypto.bestCryptoKind();
        dhtRecord = await routingContext.createDhtRecord(
          {
            kind: 'DFLT',
            o_cnt: 1,
          },
          bestKind
        );

        expect(dhtRecord.key).toBeDefined();
        expect(dhtRecord.owner).toBeDefined();
        expect(dhtRecord.owner_secret).toBeDefined();
        expect(dhtRecord.schema).toEqual({ kind: 'DFLT', o_cnt: 1 });
      });

      afterEach('free dht record', async () => {
        await routingContext.deleteDhtRecord(dhtRecord.key);
      });

      it('should set value', async () => {
        const setValueRes = await routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(data)
        );
        expect(setValueRes).toBeUndefined();
      });

      it('should get value with force refresh', async () => {

        const setValueRes = await routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(data)
        );
        expect(setValueRes).toBeUndefined();

        const getValueRes = await routingContext.getDhtValue(
          dhtRecord.key,
          0,
          true
        );
        expect(getValueRes?.data).toBeDefined();
        expect(textDecoder.decode(getValueRes?.data)).toBe(data);

        expect(getValueRes?.writer).toBe(dhtRecord.owner);
        expect(getValueRes?.seq).toBe(0);
      });

      it('should open readonly record', async () => {
        await routingContext.closeDhtRecord(dhtRecord.key);

        const readonlyDhtRecord = await routingContext.openDhtRecord(
          dhtRecord.key
        );
        expect(readonlyDhtRecord).toBeDefined();

        const setValueRes = routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(data)
        );
        await expect(setValueRes).rejects.toEqual({
          kind: 'Generic',
          message: 'value is not writable',
        });
      });

      it('should open writable record', async () => {
        await routingContext.closeDhtRecord(dhtRecord.key);

        const writeableDhtRecord = await routingContext.openDhtRecord(
          dhtRecord.key,
          `${dhtRecord.owner}:${dhtRecord.owner_secret}`
        );
        expect(writeableDhtRecord).toBeDefined();
        const setValueRes = await routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(`${data}👋`)
        );
        expect(setValueRes).toBeUndefined();
      });

      it('should open readonly record and specify writer during the set', async () => {
        await routingContext.closeDhtRecord(dhtRecord.key);

        const writeableDhtRecord = await routingContext.openDhtRecord(
          dhtRecord.key,
        );
        expect(writeableDhtRecord).toBeDefined();
        const setValueResFail = routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(`${data}👋`),
        );
        await expect(setValueResFail).rejects.toEqual({
          kind: 'Generic',
          message: 'value is not writable',
        });
        const setValueRes = await routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(`${data}👋`),
          `${dhtRecord.owner}:${dhtRecord.owner_secret}`
        );
        expect(setValueRes).toBeUndefined();
      });

      it('should watch value and cancel watch', async () => {
        const setValueRes = await routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(data)
        );
        expect(setValueRes).toBeUndefined();

        // With typical values
        const watchValueRes = await routingContext.watchDhtValues(
          dhtRecord.key,
          [[0, 0]],
          "0",
          0xFFFFFFFF,
        );
        expect(watchValueRes).toBeDefined();
        expect(watchValueRes).not.toEqual("");
        expect(watchValueRes).not.toEqual("0");

        const cancelValueRes = await routingContext.cancelDhtWatch(
          dhtRecord.key,
          [],
        )

        expect(cancelValueRes).toEqual(false);

      });

      it('should watch value and cancel watch with default values', async () => {
        const setValueRes = await routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(data)
        );
        expect(setValueRes).toBeUndefined();

        // Again with default values
        const watchValueRes = await routingContext.watchDhtValues(
          dhtRecord.key,
        );
        expect(watchValueRes).toBeDefined();
        expect(watchValueRes).not.toEqual("");
        expect(watchValueRes).not.toEqual("0");

        const cancelValueRes = await routingContext.cancelDhtWatch(
          dhtRecord.key,
        )
        expect(cancelValueRes).toEqual(false);
      });

      it('should set a value and inspect it', async () => {
        const setValueRes = await routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(data)
        );
        expect(setValueRes).toBeUndefined();

        // Inspect locally
        const inspectRes = await routingContext.inspectDhtRecord(
          dhtRecord.key,
          [[0, 0]],
          "Local",
        );
        expect(inspectRes).toBeDefined();
        expect(inspectRes.subkeys).toEqual([[0, 0]]);
        expect(inspectRes.local_seqs).toEqual([0]);
        expect(inspectRes.network_seqs).toEqual([]);

        // Inspect network
        const inspectRes2 = await routingContext.inspectDhtRecord(
          dhtRecord.key,
          [[0, 0]],
          "SyncGet",
        );
        expect(inspectRes2).toBeDefined();
        expect(inspectRes2.subkeys).toEqual([[0, 0]]);
        expect(inspectRes2.local_seqs).toEqual([0]);
        expect(inspectRes2.network_seqs).toEqual([0]);
      });

      it('should set a value and inspect it with defaults', async () => {
        const setValueRes = await routingContext.setDhtValue(
          dhtRecord.key,
          0,
          textEncoder.encode(data)
        );
        expect(setValueRes).toBeUndefined();

        // Inspect locally
        const inspectRes = await routingContext.inspectDhtRecord(
          dhtRecord.key,
        );
        expect(inspectRes).toBeDefined();
        expect(inspectRes.subkeys).toEqual([[0, 0]]);
        expect(inspectRes.local_seqs).toEqual([0]);
        expect(inspectRes.network_seqs).toEqual([]);
      });
    });
  });
});
