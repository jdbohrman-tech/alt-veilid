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

- For **AMD64** based systems run this command:

  ```shell
  echo "deb [arch=amd64 signed-by=/usr/share/keyrings/veilid-packages-keyring.gpg] https://packages.veilid.net/apt stable main" | sudo tee /etc/apt/sources.list.d/veilid.list 1>/dev/null
  ```

- For **ARM64** based systems run this command:

  ```shell
  echo "deb [arch=arm64 signed-by=/usr/share/keyrings/veilid-packages-keyring.gpg] https://packages.veilid.net/apt stable main" | sudo tee /etc/apt/sources.list.d/veilid.list 1>/dev/null
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

### RPM-based

Follow the steps here to add the repo to
RPM-based systems (CentOS, Rocky Linux, AlmaLinux, Fedora, etc.)
and install Veilid.

**Step 1**: Add Veilid to your list of available software.

```shell
sudo yum-config-manager --add-repo https://packages.veilid.net/rpm/veilid-rpm-repo.repo
```
**Step 2**: Install Veilid.

```shell
sudo dnf install veilid-server veilid-cli
```

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
