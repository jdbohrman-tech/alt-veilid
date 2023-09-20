import { expect } from '@wdio/globals';

import {
  veilidCoreInitConfig,
  veilidCoreStartupConfig,
} from './utils/veilid-config';

import { veilidClient, veilidCrypto } from 'veilid-wasm';
import { textEncoder, unmarshallBytes } from './utils/marshalling-utils';

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

  it('should generate key pair', () => {
    const bestKind = veilidCrypto.bestCryptoKind();
    const keypair = veilidCrypto.generateKeyPair(bestKind);
    expect(typeof keypair).toBe('string');

    const [publicKey, secretKey] = keypair.split(':');
    expect(unmarshallBytes(publicKey).length).toBe(32);
    expect(unmarshallBytes(secretKey).length).toBe(32);

    const isValid = veilidCrypto.validateKeyPair(
      bestKind,
      publicKey,
      secretKey
    );
    expect(isValid).toBe(true);
  });

  it('should generate random bytes', () => {
    const bestKind = veilidCrypto.bestCryptoKind();
    const bytes = veilidCrypto.randomBytes(bestKind, 64);
    expect(bytes instanceof Uint8Array).toBe(true);
    expect(bytes.length).toBe(64);
  });

  it('should hash data and validate hash', () => {
    const bestKind = veilidCrypto.bestCryptoKind();
    const data = textEncoder.encode('this is my data🚀');
    const hash = veilidCrypto.generateHash(bestKind, data);

    expect(hash).toBeDefined();
    expect(typeof hash).toBe('string');

    const isValid = veilidCrypto.validateHash(bestKind, data, hash);
    expect(isValid).toBe(true);
  });

  it('should hash and validate password', () => {
    const bestKind = veilidCrypto.bestCryptoKind();

    const password = textEncoder.encode('this is my data🚀');
    const saltLength = veilidCrypto.defaultSaltLength(bestKind);
    expect(saltLength).toBeGreaterThan(0);

    const salt = veilidCrypto.randomBytes(bestKind, saltLength);
    expect(salt instanceof Uint8Array).toBe(true);
    expect(salt.length).toBe(saltLength);

    const hash = veilidCrypto.hashPassword(bestKind, password, salt);
    expect(hash).toBeDefined();
    expect(typeof hash).toBe('string');

    const isValid = veilidCrypto.verifyPassword(bestKind, password, hash);
    expect(isValid).toBe(true);
  });

  it('should aead encrypt and decrypt', () => {
    const bestKind = veilidCrypto.bestCryptoKind();
    const body = textEncoder.encode(
      'This is an encoded body with my secret data in it🔥'
    );
    const ad = textEncoder.encode(
      'This is data associated with my secret data👋'
    );

    const nonce = veilidCrypto.randomNonce(bestKind);
    expect(typeof nonce).toBe('string');

    const sharedSecred = veilidCrypto.randomSharedSecret(bestKind);
    expect(typeof sharedSecred).toBe('string');

    const encBody = veilidCrypto.encryptAead(
      bestKind,
      body,
      nonce,
      sharedSecred,
      ad
    );
    expect(encBody instanceof Uint8Array).toBe(true);

    const overhead = veilidCrypto.aeadOverhead(bestKind);
    expect(encBody.length - body.length).toBe(overhead);

    const decBody = veilidCrypto.decryptAead(
      bestKind,
      encBody,
      nonce,
      sharedSecred,
      ad
    );
    expect(decBody instanceof Uint8Array).toBe(true);
    expect(body).toEqual(decBody);
  });

  it('should sign and verify', () => {
    const bestKind = veilidCrypto.bestCryptoKind();
    const keypair = veilidCrypto.generateKeyPair(bestKind);
    const data = textEncoder.encode(
      'This is some data I am signing with my key 🔑'
    );
    expect(typeof keypair).toBe('string');

    const [publicKey, secretKey] = keypair.split(':');

    const sig = veilidCrypto.sign(bestKind, publicKey, secretKey, data);
    expect(typeof sig).toBe('string');

    expect(() => {
      const res = veilidCrypto.verify(bestKind, publicKey, data, sig);
      expect(res).toBeUndefined();
    }).not.toThrow();
  });

});
