import * as veilid from 'veilid-wasm';
import loadVeilidWasm, {
    veilidClient,
    VeilidRoutingContext,
} from 'veilid-wasm'
import { getVeilidCoreStartupConfig, veildCoreInitConfig } from './veilid-config';
import { veilidEventEmitter } from './veilid-event-emitter';
import { v4 as uuidV4 } from 'uuid';

let loadVeilidWasmPromise: Promise<veilid.InitOutput>;
let isVeilidWasmLoaded = false;
export async function initVeilid() {
    if (isVeilidWasmLoaded) {
        return;
    }
    if (!loadVeilidWasmPromise) {
        console.log('loading veilid-wasm...');
        loadVeilidWasmPromise = loadVeilidWasm();
    }
    await loadVeilidWasmPromise;
    isVeilidWasmLoaded = true;
    console.log('veilid-wasm loaded!');
}

let IS_VEILID_RUNNING = false;

export function isVeilidRunning() {
    return IS_VEILID_RUNNING;
}

export async function startVeilid() {
    const config = getVeilidCoreStartupConfig({
        namespace: `example-bench-${uuidV4()}`,
        password: 'singleton',
    });

    console.log('starting veilid core...');
    await veilidClient.initializeCore(veildCoreInitConfig);

    veilidClient.startupCore(async (data) => {
        veilidEventEmitter.emit(data.kind, data);
    }, config);

    return veilidClient;
}

// Listen to updates, and `attach()` once startup is complete.
veilidEventEmitter.on('Log', (data) => {
    switch (data?.log_level) {
        case 'Warn':
            console.warn(data.message);
            break;
        case 'Info':
            console.info(data.message);
            break;
        case 'Debug':
            console.log(data.message);
            break;
        default:
            console.log(data.message);
            break;
    }

    // TODO: Wonder if there's a better way to detect startup complete.
    if (data.message?.includes('Veilid API startup complete')) {
        console.log('veilid core started!');
        IS_VEILID_RUNNING = true;
        console.log('Veilid Version', veilidClient.versionString());
        console.log('attaching to veilid network');
        veilidClient.attach();
    }
});

veilidEventEmitter.on('RouteChange', (routeChange) => {
    console.log('ROUTE CHANGE', routeChange);
});

veilidEventEmitter.on('ValueChange', (valueChange) => {
    console.log('VALUE CHANGE', valueChange);
});

export async function stopVeilid() {
    if (ROUTING_CONTEXT_SINGLETON) {
        ROUTING_CONTEXT_SINGLETON.free();
        ROUTING_CONTEXT_SINGLETON = undefined;
    }
    if (isVeilidRunning()) {
        IS_VEILID_RUNNING = false;
        await veilidClient.detach();
        await veilidClient.shutdownCore();
    }
}

let ROUTING_CONTEXT_SINGLETON: VeilidRoutingContext | undefined;
export function getRoutingContext() {
    if (!ROUTING_CONTEXT_SINGLETON) {
        ROUTING_CONTEXT_SINGLETON = VeilidRoutingContext.create();
    }
    return ROUTING_CONTEXT_SINGLETON;
}
