import * as veilid from 'veilid-wasm';
import EventEmitter from 'events';

export type VeilidUpdateKind = veilid.VeilidUpdate['kind'];
type VeilidUpdateKindMap = {
    [TKind in VeilidUpdateKind]: Extract<veilid.VeilidUpdate, { kind: TKind }>;
};

export type VeilidUpdateType<TKind extends VeilidUpdateKind> =
    VeilidUpdateKindMap[TKind];

/**
 * A typesafe event emitter for VeilidUpdate events.
 */
class VeilidEventEmitter extends EventEmitter {
    on<TKind extends VeilidUpdateKind>(
        event: TKind,
        listener: (veilidUpdate: VeilidUpdateType<TKind>) => void
    ): this {
        super.on(event, listener);
        return this;
    }

    emit<TKind extends VeilidUpdateKind>(
        event: TKind,
        veilidUpdate: VeilidUpdateType<TKind>
    ): boolean {
        return super.emit(event, veilidUpdate);
    }

    removeListener<TKind extends VeilidUpdateKind>(
        event: TKind,
        listener: (veilidUpdate: VeilidUpdateType<TKind>) => void
    ): this {
        super.removeListener(event, listener);
        return this;
    }
}

/**
 * a singleton instance of VeilidEventEmitter.
 */
export const veilidEventEmitter = new VeilidEventEmitter();
