# Creating a Veilid Bootstrap Server

There are two versions of the Veilid bootstrap:

 * Version 0: Unsigned bootstrap - Any node can bootstrap with your server, but no guarantees are provided to those nodes about which network was connected, as DNS is subject to MITM attacks
 * Version 1: Signed bootstrap - Any node can bootstrap with your server, and if they have your bootstrap signing public key in their trusted keys config, they will get a verified bootstrap to a specific network

These instructions will cover both versions, however Version 1 is preferable, as Version 0 will eventually be deprecated.

## Instance Recommended Setup

CPU: Single<br>
RAM: 1GB<br>
Storage: 25GB<br>
IP: Static v4 & v6<br>
Firewall: 5150/TCP/UDP inbound allow all<br>

You will need to ensure your bootstrap server has PTR records for its hostname, as the TXT records generated for the bootstrap will rely on the hostname existing and being consistent for all of its IP addresses.

## Install Veilid

Follow instructions in [INSTALL.md](./INSTALL.md)

## Configure Veilid as Bootstrap

### Stop the Veilid service

```shell
sudo systemctl stop veilid-server.service
```

### Create a bootstrap signing key

You need a 'bootstrap signing key' to sign your bootstrap server records. A single signing key can be used to sign multiple bootstrap server records. If you don't have one yet, with `veilid-server` not already running:

```shell
sudo -u veilid veilid-server --generate-key-pair VLD0
```
which outputs a key in this form (example: `VLD0:NAPctwUP5NNynWdkX8rcUz_yk44v-cHuDM9ZzvsDXnQ:-ncghvgw2NFQK2RH2vCfvCJj3M3gTVOD-UM08-7n6kQ`):
```
VLD0:PUBLIC_KEY:SECRET_KEY
```

Copy down the generated keypair and store it in a secure location, preferably offline.
Remove the part after the second colon (the SECRET_KEY), and this is your 'Bootstrap Signing Public Key' (should look like: `VLD0:NAPctwUP5NNynWdkX8rcUz_yk44v-cHuDM9ZzvsDXnQ`)

### Setup the config

In `/etc/veilid-server/veilid-server.conf` ensure these keys are in the in the `routing_table:` section

- `bootstrap: ['bootstrap.<your.domain>']`

- V0: Use an empty bootstrap key list to enable unverified bootstrap
  - `bootstrap_keys: []`
- V1: Add your bootstrap signing public key to this list.
  - If your signing key is the only one:
    - `bootstrap_keys: ['VLD0:<your_bootstrap_signing_public_key>']`
  - You may also want to include any other signing keys for bootstraps you trust. If this is a bootstrap for the main Veilid network, include Veilid Foundation's signing keys here as well
    - `bootstrap_keys: ['VLD0:<your_bootstrap_signing_public_key>', 'VLD0:Vj0lKDdUQXmQ5Ol1SZdlvXkBHUccBcQvGLN9vbLSI7k', 'VLD0:QeQJorqbXtC7v3OlynCZ_W3m76wGNeB5NTF81ypqHAo','VLD0:QNdcl-0OiFfYVj9331XVR6IqZ49NG-E18d5P7lwi4TA']`

(If you came here from the [dev network setup](./dev-setup/dev-network-setup.md) guide, this is when you set the network key as well in the `network_key_password` field of the `network:` section)

**Switch to veilid user**

```shell
sudo -u veilid /bin/bash
```

### Generate a new keypair

Copy the output to secure storage such as a password manager. This information will be used in the next step and can be used for node recovery, moving to a different server, etc.

```shell
veilid-server --generate-key-pair VLD0
```

### Create new node ID and flush existing route table

Include the brackets [] when pasting the keys. Use the public key in the command. Secret key will be requested interactively and will not echo when pasted.

```shell
veilid-server --set-node-id [PUBLIC_KEY] --delete-table-store
```

### Generate the DNS TXT record

Copy the output to secure storage. This information will be use to setup DNS records.

```shell
veilid-server --dump-txt-record <FULL BOOTSTRAP SIGNING KEY PAIR>
```

(will look like this, but with your own key:)
```shell
veilid-server --dump-txt-record VLD0:NAPctwUP5NNynWdkX8rcUz_yk44v-cHuDM9ZzvsDXnQ:-ncghvgw2NFQK2RH2vCfvCJj3M3gTVOD-UM08-7n6kQ
```

### Start the Veilid service

Disconnect from the Veilid user and start veilid-server.service.

```shell
exit
```

```shell
sudo systemctl start veilid-server.service
```

Optionally configure the service to start at boot `sudo systemctl enable veilid-server.service`

_REPEAT FOR EACH BOOTSTRAP SERVER_

## Enter DNS Records

Create the following DNS Records for your domain:

(This example assumes two bootstrap servers are being created)

V1:
| Record       | Value                        | Record Type |
| ------------ | ---------------------------- | ----------- |
| bootstrap-v1 | IPv4 of bootstrap 1          | A           |
| bootstrap-v1 | IPv4 of bootstrap 2          | A           |
| bootstrap-v1 | IPv6 of bootstrap 1          | AAAA        |
| bootstrap-v1 | IPv6 of bootstrap 2          | AAAA        |
| bootstrap-v1 | TXTRecord v0 for bootstrap 1 | TXT         |
| bootstrap-v1 | TXTRecord v1 for bootstrap 1 | TXT         |
| bootstrap-v1 | TXTRecord v0 for bootstrap 2 | TXT         |
| bootstrap-v1 | TXTRecord v1 for bootstrap 2 | TXT         |

V0:
| Record      | Value                        | Record Type |
| ----------- | ---------------------------- | ----------- |
| bootstrap   | 1,2                          | TXT         |
| 1.bootstrap | IPv4                         | A           |
| 1.bootstrap | IPv6                         | AAAA        |
| 1.bootstrap | TXTRecord v0 for bootstrap 1 | TXT         |
| 2.bootstrap | IPv4                         | A           |
| 2.bootstrap | IPv6                         | AAAA        |
| 2.bootstrap | TXTRecord v0 for bootstrap 2 | TXT         |
