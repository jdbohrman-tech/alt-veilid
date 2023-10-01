import { expect } from '@wdio/globals';

import {
  veilidCoreInitConfig,
  veilidCoreStartupConfig,
} from './utils/veilid-config';

import { VeilidState, veilidClient } from 'veilid-wasm';
import { waitForMs } from './utils/wait-utils';

describe('veilidClient', () => {
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

  it('should print version', async () => {
    const version = veilidClient.versionString();
    expect(typeof version).toBe('string');
    expect(version.length).toBeGreaterThan(0);
  });

  it('should attach and detach', async () => {
    await veilidClient.attach();
    await waitForMs(2000);
    await veilidClient.detach();
  });

  describe('kitchen sink', () => {
    before('attach', async () => {
      await veilidClient.attach();
      await waitForMs(2000);
    });
    after('detach', () => veilidClient.detach());

    let state: VeilidState;

    it('should get state', async () => {
      state = await veilidClient.getState();
      expect(state.attachment).toBeDefined();
      expect(state.config.config).toBeDefined();
      expect(state.network).toBeDefined();
    });

    it('should call debug command', async () => {
      const response = await veilidClient.debug('txtrecord');
      expect(response).toBeDefined();
      expect(response.length).toBeGreaterThan(0);
    });
  });
});
