import { expect } from '@wdio/globals';

import {
  veilidCoreInitConfig,
  veilidCoreStartupConfig,
} from './utils/veilid-config';

import { VeilidState, veilidClient } from 'veilid-wasm';
import { asyncCallWithTimeout, waitForDetached, waitForPublicAttachment, waitForShutdown } from './utils/wait-utils';

describe('veilidClient', function () {
  before('veilid startup', async function () {
    veilidClient.initializeCore(veilidCoreInitConfig);
    await veilidClient.startupCore(function (_update) {
      // if (_update.kind === 'Log') {
      //   console.log(_update.message);
      // }
    }, veilidCoreStartupConfig);
  });

  after('veilid shutdown', async function () {
    await veilidClient.shutdownCore();
    await asyncCallWithTimeout(waitForShutdown(), 10000);
  });

  it('should print version', async function () {
    const version = veilidClient.versionString();
    expect(typeof version).toBe('string');
    expect(version.length).toBeGreaterThan(0);
  });

  it('should print features', async function () {
    const features = veilidClient.features();
    expect(Array.isArray(features)).toBe(true);
    expect(features.length).toBeGreaterThan(0);
  });

  it('should get config', async function () {
    const defaultConfig = veilidClient.defaultConfig();
    expect(typeof defaultConfig).toBe('object');

    expect(defaultConfig).toHaveProperty('program_name');
    expect(defaultConfig).toHaveProperty('namespace');
    expect(defaultConfig).toHaveProperty('capabilities');
    expect(defaultConfig).toHaveProperty('protected_store');
    expect(defaultConfig).toHaveProperty('table_store');
    expect(defaultConfig).toHaveProperty('block_store');
    expect(defaultConfig).toHaveProperty('network');
  });

  it('should attach and detach', async function () {
    await veilidClient.attach();
    await asyncCallWithTimeout(waitForPublicAttachment(), 10000);
    await veilidClient.detach();
    await asyncCallWithTimeout(waitForDetached(), 10000);
  });

  describe('kitchen sink', function () {
    before('attach', async function () {
      await veilidClient.attach();
      await asyncCallWithTimeout(waitForPublicAttachment(), 10000);
    });
    after('detach', async function () {
      await veilidClient.detach();
      await asyncCallWithTimeout(waitForDetached(), 10000);
    });

    let state: VeilidState;

    it('should get state', async function () {
      state = await veilidClient.getState();
      expect(state.attachment).toBeDefined();
      expect(state.config.config).toBeDefined();
      expect(state.network).toBeDefined();
    });

    it('should call debug command', async function () {
      const response = await veilidClient.debug('txtrecord');
      expect(response).toBeDefined();
      expect(response.length).toBeGreaterThan(0);
    });
  });
});
