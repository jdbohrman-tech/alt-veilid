# Dev Network Setup

## Purpose

There will be times when a contibutor wishes to dynamically test their work on live nodes. Doing so on the actual Veilid network would likely not yield productive test outcomes and so setting up an independent network for testing purposes is warranted.

This document outlines the process of using the steps found in [INSTALL.md](../INSTALL.md) and [BOOTSTRAP-SETUP.md](../BOOTSTRAP-SETUP.md) with some modifications which results in a reasonably isolated and independent network of Veilid development nodes which do not communicate with nodes on the actual Veilid network.

The minimum topology of a dev network is 1 bootstrap server and 4 nodes, all with public IP addresses with port 5150/TCP open. This allows enabling public address detection and private routing. The minimum specifications are 1 vCPU, 1GB RAM, and 25 GB storage.

## Quick Start

### The Network Key

This acts as a passphase to allow nodes to join the network. It is the mechanism that makes your dev network isolated and independent. Create a passphrase and protect/store it as you would any other a password.

### Dev Bootstrap Server

Follow the steps detailed in [BOOTSTRAP-SETUP.md](../BOOTSTRAP-SETUP.md) using the dev bootstrap example [config](../doc/config/veilid-dev-bootstrap-config.md) for the *Setup the config* section. Set your network key on line 28.

### Dev Nodes

1. Follow the steps detailed in [INSTALL.md](../INSTALL.md) *DO NOT START THE SYSTEMD SERVICE*
2. Replace the default veilid-server config using the dev node example [config](../doc/config/veilid-dev-server-config.md) as a template. Enter your information on lines 27 and 28 to match what was entered in the dev bootstrap server's config.
3. Start the node with fresh data

    ```shell
    sudo -u veilid veilid-server --delete-protected-store --delete-block-store --delete-table-store`
    ```

4. `ctrl-c` to stop the above process
5. Start the dev node service

    ```shell
    sudo systemctl start veilid-server.service
    ```

6. (Optionally) configure the service to start at boot

    ```shell
    sudo systemctl enable veilid-server.service
    ```
