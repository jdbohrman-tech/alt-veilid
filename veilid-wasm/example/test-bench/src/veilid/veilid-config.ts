import { veilidClient, VeilidWASMConfig } from 'veilid-wasm';
export interface VeilidConfigOptions {
    namespace: string;
    password: string;
}

export const veildCoreInitConfig: VeilidWASMConfig = {
    logging: {
        api: {
            enabled: true,
            level: 'Info',
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

export function getVeilidCoreStartupConfig(options: VeilidConfigOptions) {
    const defaultConfig = veilidClient.defaultConfig();

    defaultConfig.program_name = 'veilid-wasm-test-bench';
    defaultConfig.namespace = options.namespace;
    defaultConfig.protected_store.device_encryption_key_password = options.password;

    return defaultConfig;
}
