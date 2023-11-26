# Starting a Generic/Public Veilid Bootstrap Server

## Instance Recommended Setup

CPU: Single
RAM: 1GB
Storage: 25GB
IP: Static v4 & v6
Firewall: 5150/TCP/UDP inbound allow all

## Install Veilid

Follow instructions in [INSTALL.md](./INSTALL.md)

## Configure Veilid as Bootstrap

### Stop the Veilid service

```shell
sudo systemctl stop veilid-server.service
```

### Setup the config

In _/etc/veilid-server/veilid-server.conf`_ ensure _bootstrap: ['bootstrap.<your.domain>']_ in the _routing_table:_ section

If you came here from the [dev network setup](./dev-setup/dev-network-setup.md) guide, this is when you set the network key.

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
veilid-server --dump-txt-record
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

(This example assumes two bootstrap serves are being created)

| Record    | Value                       | Record Type |
|-----------|-----------------------------|-------------|
|bootstrap  | 1,2                         | TXT         |
|1.bootstrap| IPv4                        | A           |
|1.bootstrap| IPv6                        | AAAA        |
|1.bootstrap| output of --dump-txt-record | TXT         |
|2.bootstrap| IPv4                        | A           |
|2.bootstrap| IPv6                        | AAAA        |
|2.bootstrap| output of --dump-txt-record | TXT         |
