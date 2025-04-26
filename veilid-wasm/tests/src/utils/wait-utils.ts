import { veilidClient, VeilidRoutingContext, TypedKey } from 'veilid-wasm';

export const waitForMs = (milliseconds: number) => {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
};

export const asyncCallWithTimeout = async<T>(asyncPromise: Promise<T>, timeLimit: number) => {
  let timeoutHandle: ReturnType<typeof setTimeout>;

  const timeoutPromise = new Promise((_resolve, reject) => {
    timeoutHandle = setTimeout(
      () => reject(new Error('Async call timeout limit reached')),
      timeLimit
    );
  });

  return Promise.race([asyncPromise, timeoutPromise]).then(result => {
    clearTimeout(timeoutHandle);
    return result;
  })
}

export const waitForPublicAttachment = async () => {
  while (true) {
    let state = await veilidClient.getState();
    if (state.attachment.public_internet_ready) {
      var attached = false
      switch (state.attachment.state) {
        case "Detached":
        case "Detaching":
        case "Attaching":
          break;
        default:
          attached = true;
          break;
      }
      if (attached) {
        break;
      }
    }
    await waitForMs(1000);
  }
}

export const waitForDetached = async () => {
  while (true) {
    let state = await veilidClient.getState();
    var detached = false
    switch (state.attachment.state) {
      case "Detached":
        detached = true;
        break;
      default:
        break;
    }
    if (detached) {
      break;
    }
    await waitForMs(1000);
  }
}

export const waitForShutdown = async () => {
  while (true) {
    let isShutdown = veilidClient.isShutdown();
    if (isShutdown) {
      break;
    }
    await waitForMs(1000);
  }
}

export const waitForOfflineSubkeyWrite = async (routingContext: VeilidRoutingContext, key: TypedKey) => {
  while ((await routingContext.inspectDhtRecord(key)).offline_subkeys.length != 0) {
    await waitForMs(200);
  }
}
