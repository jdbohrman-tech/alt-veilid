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
import { waitForMs } from './utils/wait-utils';

describe('VeilidRoutingContext', () => {
  before('veilid startup', async () => {
    veilidClient.initializeCore(veilidCoreInitConfig);
    await veilidClient.startupCore((_update) => {
      // if (_update.kind === 'Log') {
      //   console.log(_update.message);
      // }
    }, JSON.stringify(veilidCoreStartupConfig));
    await veilidClient.attach();
    await waitForMs(2000);
  });

  after('veilid shutdown', async () => {
    await veilidClient.detach();
    await veilidClient.shutdownCore();
  });

  describe('constructors', () => {
    it('should create using .create()', async () => {
      const routingContext = VeilidRoutingContext.create();
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);

      routingContext.free();
    });

    it('should create using new', async () => {
      const routingContext = new VeilidRoutingContext();
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);

      routingContext.free();
    });

    it('should create with privacy', async () => {
      const routingContext = VeilidRoutingContext.create().withPrivacy();
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);

      routingContext.free();
    });

    it('should create with custom privacy', async () => {
      const routingContext = VeilidRoutingContext.create().withCustomPrivacy({
        Safe: {
          hop_count: 2,
          sequencing: 'EnsureOrdered',
          stability: 'Reliable',
        },
      });
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);

      routingContext.free();
    });

    it('should create with sequencing', async () => {
      const routingContext =
        VeilidRoutingContext.create().withSequencing('EnsureOrdered');
      expect(routingContext instanceof VeilidRoutingContext).toBe(true);

      routingContext.free();
    });
  });

  describe('operations', () => {
    let routingContext: VeilidRoutingContext;

    before('create routing context', () => {
      routingContext = VeilidRoutingContext.create()
        .withPrivacy()
        .withSequencing('EnsureOrdered');
    });

    after('free routing context', () => {
      routingContext.free();
    });

    describe('DHT kitchen sink', async () => {
      let dhtRecord: DHTRecordDescriptor;
      const data = '🚀 This example DHT data with unicode a Ā 𐀀 文 🚀';

      before('create dht record', async () => {
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

      after('free dht record', async () => {
        await routingContext.closeDhtRecord(dhtRecord.key);
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
    });
  });
});
