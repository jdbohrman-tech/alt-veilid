import { expect } from '@wdio/globals';

import {
  veilidCoreInitConfig,
  veilidCoreStartupConfig,
} from './utils/veilid-config';

import { VeilidTableDB, veilidClient } from 'veilid-wasm';
import { marshall, unmarshall } from './utils/marshalling-utils';

const TABLE_NAME = 'some-table';
const TABLE_COLS = 1;

describe('VeilidTable', () => {
  before('veilid startup', async () => {
    veilidClient.initializeCore(veilidCoreInitConfig);
    await veilidClient.startupCore((_update) => {
      // if (_update.kind === 'Log') {
      //   console.log(_update.message);
      // }
    }, JSON.stringify(veilidCoreStartupConfig));
  });

  after('veilid shutdown', async () => {
    await veilidClient.shutdownCore();
  });

  it('should open and close a table', async () => {
    const table = new VeilidTableDB(TABLE_NAME, TABLE_COLS);
    await table.openTable();

    const keys = await table.getKeys(0);
    expect(keys.length).toBe(0);

    table.free();
  });

  describe('table operations', () => {
    let table: VeilidTableDB;

    before('create table', async () => {
      table = new VeilidTableDB(TABLE_NAME, TABLE_COLS);
      await table.openTable();
    });

    after('free table', async () => {
      table.free();
    });

    it('should have no keys', async () => {
      const keys = await table.getKeys(0);
      expect(keys.length).toBe(0);
    });

    describe('store/load', () => {
      const key = 'test-key with unicode 🚀';
      const value = 'test value with unicode 🚀';

      it('should store value', async () => {
        await table.store(0, marshall(key), marshall(value));
      });

      it('should load value', async () => {
        const storedValue = await table.load(0, marshall(key));
        expect(storedValue).toBeDefined();
        expect(unmarshall(storedValue!)).toBe(value);
      });

      it('should have key in list of keys', async () => {
        const keys = await table.getKeys(0);
        const decodedKeys = keys.map(unmarshall);
        expect(decodedKeys).toEqual([key]);
      });
    });

    describe('transactions', () => {
      it('should commit a transaction', async () => {
        let transaction = await table.createTransaction();

        const key = 'tranaction-key🔥';
        const first = 'first🅱';
        const second = 'second✔';
        const third = 'third📢';

        transaction.store(0, marshall(key), marshall(first));
        transaction.store(0, marshall(key), marshall(second));
        transaction.store(0, marshall(key), marshall(third));

        await transaction.commit();

        const storedValue = await table.load(0, marshall(key));
        expect(storedValue).toBeDefined();
        expect(unmarshall(storedValue!)).toBe(third);

        transaction.free();
      });
    });
  });
});
