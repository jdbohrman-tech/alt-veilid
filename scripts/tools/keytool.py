import base64
import sys
import argparse

def urlsafe_b64encode_no_pad(b: bytes) -> str:
    """
    Removes any `=` used as padding from the encoded string.
    """
    return base64.urlsafe_b64encode(b).decode().rstrip("=")


def urlsafe_b64decode_no_pad(s: str) -> bytes:
    """
    Adds back in the required padding before decoding.
    """
    padding = 4 - (len(s) % 4)
    s = s + ("=" * padding)
    return base64.urlsafe_b64decode(s)


def do_value(args):

    key = urlsafe_b64decode_no_pad(args.key)

    print("key:", key.hex())


def dist(key1: bytes, key2: bytes) -> bytes:    
    distance = bytearray(len(key1))
    for n in range(len(key1)):
        distance[n] = key1[n] ^ key2[n]

    return bytes(distance)


def do_distance(args):
        
    key1 = urlsafe_b64decode_no_pad(args.key1)
    key2 = urlsafe_b64decode_no_pad(args.key2)

    print("key1:", key1.hex())
    print("key2:", key2.hex())

    distance = dist(key1, key2)
    print("dist:", distance.hex())

def keycmp(key1: bytes, key2: bytes) -> int:
    for n in range(len(key1)):
        if key1[n] < key2[n]:
            return -1
        if key1[n] > key2[n]:
            return 1
    return 0

def do_closer(args):
        
    key = urlsafe_b64decode_no_pad(args.key)
    near = urlsafe_b64decode_no_pad(args.near)
    far = urlsafe_b64decode_no_pad(args.far)

    print(" key:", key.hex())
    print("near:", near.hex())
    print(" far:", far.hex())

    distance_near = dist(key, near)
    distance_far = dist(key, far)
    
    print("  dn:", distance_near.hex())
    print("  df:", distance_far.hex())

    c = keycmp(distance_near, distance_far)
    print(" cmp:", c)

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(required=True)
    
    parser_value = subparsers.add_parser('value')
    parser_value.add_argument('key', type=str)
    parser_value.set_defaults(func=do_value)
    
    parser_value = subparsers.add_parser('distance')
    parser_value.add_argument('key1', type=str)
    parser_value.add_argument('key2', type=str)
    parser_value.set_defaults(func=do_distance)
    
    parser_value = subparsers.add_parser('closer')
    parser_value.add_argument('key', type=str)
    parser_value.add_argument('near', type=str)
    parser_value.add_argument('far', type=str)
    parser_value.set_defaults(func=do_closer)
    
    args = parser.parse_args()
    args.func(args)
    
    sys.exit(0)

if __name__ == "__main__":
    main()