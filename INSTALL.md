# Install and run a Veilid Node

## Server Grade Headless Nodes

These network support nodes are heavier than the node a user would establish on their phone in the form of a chat or social media application. A cloud based virtual private server (VPS), such as Digital Ocean Droplets or AWS EC2, with high bandwidth, processing resources, and uptime availability is crucial for building the fast, secure, and private routing that Veilid is built to provide.

## Install

### Debian

Follow the steps here to add the repo to a Debian based system and install Veilid.

**Step 1**: Add the GPG keys to your operating systems keyring.<br />
*Explanation*: The `wget` command downloads the public key, and the `sudo gpg` command adds the public key to the keyring.

```shell
wget -O- https://packages.veilid.net/gpg/veilid-packages-key.public | sudo gpg --dearmor -o /usr/share/keyrings/veilid-packages-keyring.gpg
```

**Step 2**: Identify your architecture<br />
*Explanation*: The following command will tell you what type of CPU your system is running

```shell
dpkg --print-architecture
```

**Step 3**: Add Veilid to your list of available software.<br />
*Explanation*: Use the result of your command in **Step 2** and run **one** of the following:

- For *STABLE* releases
  - **AMD64** based systems run this command:

  ```shell
  echo "deb [arch=amd64 signed-by=/usr/share/keyrings/veilid-packages-keyring.gpg] https://packages.veilid.net/apt stable main" | sudo tee /etc/apt/sources.list.d/veilid.list 1>/dev/null
  ```

  - **ARM64** based systems run this command:

  ```shell
  echo "deb [arch=arm64 signed-by=/usr/share/keyrings/veilid-packages-keyring.gpg] https://packages.veilid.net/apt stable main" | sudo tee /etc/apt/sources.list.d/veilid.list 1>/dev/null
  ```

- For *NIGHTLY* (bleeding edge) releases
  - **AMD64** based systems run this command:

  ```shell
  echo "deb [arch=amd64 signed-by=/usr/share/keyrings/veilid-packages-keyring.gpg] https://packages.veilid.net/apt nightly main" | sudo tee /etc/apt/sources.list.d/veilid.list 1>/dev/null
  ```

  - **ARM64** based systems run this command:

  ```shell
  echo "deb [arch=arm64 signed-by=/usr/share/keyrings/veilid-packages-keyring.gpg] https://packages.veilid.net/apt nightly main" | sudo tee /etc/apt/sources.list.d/veilid.list 1>/dev/null
  ```


*Explanation*:
Each of the above commands will create a new file called `veilid.list` in the `/etc/apt/sources.list.d/`. This file contains instructions that tell the operating system where to download Veilid.

**Step 4**: Refresh the package manager.<br />
*Explanation*: This tells the `apt` package manager to rebuild the list of available software using the files in `/etc/apt/sources.list.d/` directory.

```shell
sudo apt update
```

**Step 5**: Install Veilid.

```shell
sudo apt install veilid-server veilid-cli
```

**Step 6**: Start veilid-server.service

Go to [Start headless node](#start-headless-node)

**Step 7**: View Node Activity

Invoke the Veilid CLI utility.

Either add your user to the _veilid_ group and invoke the command
```shell
veilid-cli
```
Or use _sudo_ to invoke as the _veilid_ user
```shell
sudo -u veilid veilid-cli
```

### RPM-based

Follow the steps here to add the repo to
RPM-based systems (CentOS, Rocky Linux, AlmaLinux, Fedora, etc.)
and install Veilid.

**Step 1**: Add Veilid to your list of available software.

- For *STABLE* releases

```shell
sudo dnf config-manager --add-repo https://packages.veilid.net/rpm/stable/x86_64/veilid-stable-x86_64-rpm.repo
```

- For *NIGHTLY* (bleeding edge) releases

```shell
sudo dnf config-manager --add-repo https://packages.veilid.net/rpm/nightly/x86_64/veilid-nightly-x86_64-rpm.repo
```

**Step 2**: Install Veilid.

```shell
sudo dnf install veilid-server veilid-cli
```

**Step 3**: Start veilid-server.service

Go to [Start headless node](#start-headless-node)

**Step 4**: View Node Activity

Invoke the Veilid CLI utility.

Either add your user to the _veilid_ group and invoke the command
```shell
veilid-cli
```
Or use _sudo_ to invoke as the _veilid_ user
```shell
sudo -u veilid veilid-cli
```

### Setup Auto Updates

**Stable Releases**

We set the bootstrap nodes to check for updates every hour by using crontab

On a Debian based machine:

```shell
sudo crontab -e
```

In the editor that opens append:
```shell
0 * * * * apt -y update && apt -y upgrade --only-upgrade veilid-cli veilid-server > ~/auto_updates.log 2>&1
```

**Nightly Releases**

The nightly auto release triggers at 11PM US Central Time. The following crontab exmaple will
trigger at 6AM for your machine's local time which should give plenty of time for the auto release
to complete no matter where you are in the world.

```shell
sudo crontab -e
```

In the editor that opens append:
```shell
* 6 * * * apt -y update && apt -y upgrade veilid-cli veilid-server > ~/auto_updates.log 2>&1
```

**Fedora Based Machines**

The above steps should work, replace the apt commands with appropriate dnf commands.

### macOS
***Not maintained by the Veilid team. Seek assistance in the Discord community.***

Veilid is available [via Homebrew](https://formulae.brew.sh/formula/veilid).

```shell
brew install veilid
```

You can then run `veilid-server` and `veilid-cli` from the command line.

## Start headless node

### With systemd

To start a headless Veilid node, run:

```shell
sudo systemctl start veilid-server.service
```

*-OR-*

To have your headless Veilid node start at boot:

```shell
sudo systemctl enable --now veilid-server.service
```

### Without systemd

`veilid-server` must be run as the `veilid` user.

To start your headless Veilid node without systemd, run:

```shell
sudo -u veilid veilid-server
```

## Network Considerations

> **note:** if you're interested in using a veilid-server node for local development, you're better off reading the [Developer Book](https://veilid.gitlab.io/developer-book/), though the implementation to enable local development using a veilid-server node is still forthcoming.

Veilid nodes need to be internet facing or behind a firewall that allows inbound connections via port 5150 for both TCP and UDP. This will allow veilid-server to access other nodes in the wider network since 5150 is the port that the process uses by default. If the port is not available, veilid-server will wait for it to become available.

In the event the listening port is not opened in the firewall, an application may still operate, though in a fairly degraded mode that relies on another node to relay incoming RPC messages to them.
