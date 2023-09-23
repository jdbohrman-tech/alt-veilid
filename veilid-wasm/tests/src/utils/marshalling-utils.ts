export const textDecoder = new TextDecoder();
export const textEncoder = new TextEncoder();

//  TextEncoder/TextDecoder are used to solve for "The Unicode Problem" https://stackoverflow.com/a/30106551
export function marshallString(data: string) {
  return marshallBytes(textEncoder.encode(data));
}

export function unmarshallString(b64: string) {
  return textDecoder.decode(unmarshallBytes(b64));
}

export function marshallBytes(data: Uint8Array) {
  const byteString = bytesToString(data);
  return base64UrlEncode(byteString);
}

export function unmarshallBytes(b64: string) {
  const byteString = base64UrlDecode(b64);
  return stringToBytes(byteString);
}

function base64UrlEncode(data: string) {
  return removeBase64Padding(btoa(data));
}

function base64UrlDecode(b64: string) {
  return atob(addBase64Padding(b64));
}

function removeBase64Padding(b64: string) {
  // URL encode characters, and remove `=` padding.
  return b64.replace(/=/g, '').replace(/\+/g, '-').replace(/\//g, '_');
}

function addBase64Padding(b64: string) {
  // URL decode characters
  b64 = b64.replace(/-/g, '+').replace(/_/g, '/');
  // Add base64 padding characters (`=`)
  const rem = b64.length % 4;
  if (rem === 2) {
    return `${b64}==`;
  } else if (rem === 3) {
    return `${b64}=`;
  }
  return b64;
}

function stringToBytes(binString: string) {
  return Uint8Array.from(binString as any, (m) => (m as any).codePointAt(0));
}

function bytesToString(bytes: Uint8Array) {
  return Array.from(bytes, (x: number) => String.fromCodePoint(x)).join('');
}
