import { expect } from '@wdio/globals';

import {
  veilidCoreInitConfig,
  veilidCoreStartupConfig,
} from './utils/veilid-config';

import { veilidClient, veilidCrypto } from 'veilid-wasm';

describe('veilidCrypto', () => {
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

  it('should list crypto kinds', () => {
    const kinds = veilidCrypto.validCryptoKinds();
    const bestKind = veilidCrypto.bestCryptoKind();

    expect(typeof bestKind).toBe('string');
    expect(kinds.includes(bestKind)).toBe(true);
  });

  it('should generate key pair', async () => {
    const bestKind = veilidCrypto.bestCryptoKind();
    const keypair = veilidCrypto.generateKeyPair(bestKind);
    expect(typeof keypair).toBe('string');
    // TODO: fix TypeScript return type of generateKeyPair to return string instead of KeyPair
  });
});
