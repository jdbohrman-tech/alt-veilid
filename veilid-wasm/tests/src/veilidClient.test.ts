import { expect } from '@wdio/globals';

import {
  veilidCoreInitConfig,
  veilidCoreStartupConfig,
} from './utils/veilid-config';

import { VeilidState, veilidClient } from 'veilid-wasm';
import { asyncCallWithTimeout, waitForPublicAttachment } from './utils/wait-utils';

describe('veilidClient', function () {
  before('veilid startup', async function () {
    veilidClient.initializeCore(veilidCoreInitConfig);
    await veilidClient.startupCore(function (_update) {
      // if (_update.kind === 'Log') {
      //   console.log(_update.message);
      // }
    }, JSON.stringify(veilidCoreStartupConfig));
  });

  after('veilid shutdown', async function () {
    await veilidClient.shutdownCore();
  });

  it('should print version', async function () {
    const version = veilidClient.versionString();
    expect(typeof version).toBe('string');
    expect(version.length).toBeGreaterThan(0);
  });

  it('should get config string', async function () {
    const defaultConfig = veilidClient.defaultConfig();
    expect(typeof defaultConfig).toBe('string');
    expect(defaultConfig.length).toBeGreaterThan(0);

    const cfgObject1 = JSON.parse(defaultConfig);
    const defaultConfigStr = JSON.stringify(cfgObject1);
    const cfgObject2 = JSON.parse(defaultConfigStr);
    const defaultConfigStr2 = JSON.stringify(cfgObject2);

    expect(defaultConfigStr).toEqual(defaultConfigStr2);
  });

  it('should attach and detach', async function () {
    await veilidClient.attach();
    await asyncCallWithTimeout(waitForPublicAttachment(), 10000);
    await veilidClient.detach();
  });

  describe('kitchen sink', function () {
    before('attach', async function () {
      await veilidClient.attach();
      await waitForPublicAttachment();

    });
    after('detach', async function () {
      await veilidClient.detach();
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
