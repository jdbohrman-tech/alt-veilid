import type { VeilidWASMConfig } from 'veilid-wasm';
import { veilidClient } from 'veilid-wasm';

export const veilidCoreInitConfig: VeilidWASMConfig = {
  logging: {
    api: {
      enabled: true,
      level: 'Debug',
      ignore_log_targets: [],
    },
    performance: {
      enabled: false,
      level: 'Info',
      logs_in_timings: false,
      logs_in_console: false,
      ignore_log_targets: [],
    },
  },
};

export var veilidCoreStartupConfig = (() => {
  var defaultConfig = veilidClient.defaultConfig();
  defaultConfig.program_name = 'veilid-wasm-test';
  // Ensure we are starting from scratch
  defaultConfig.table_store.delete = true;
  defaultConfig.protected_store.delete = true;
  defaultConfig.block_store.delete = true;
  return defaultConfig;
})(); 
