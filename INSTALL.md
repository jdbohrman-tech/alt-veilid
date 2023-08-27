# Install a Veilid Node


## Server Grade Headless Nodes


These network support nodes are heavier than the node a user would establish on their phone in the form of a chat or social media application. A cloud based virtual private server (VPS), such as Digital Ocean Droplets or AWS EC2, with high bandwidth, processing resources, and uptime availability is crucial for building the fast, secure, and private routing that Veilid is built to provide.


### Add the repo to a Debian based system and install a Veilid node
This is a multi-step process.


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
*Explanation*: Using the command in **Step 2** you will need to run **one** of the following:

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
*Explanation*: This tells the `apt` package manager to rebuild the list of available software using the files in `/etc/apt/sources.list.d/` directory. This is invoked with "sudo" to grant superuser permission to make the changes.
```shell
sudo apt update
```

**Step 5**: Install Veilid.<br />
*Explanation*: With the package manager updated, it is now possible to install Veilid! This is invoked with "sudo" to grant superuser permission to make the changes.
```shell
sudo apt install veilid-server veilid-cli
```

### Add the repo to a Fedora based system and install a Veilid node
**Step 1**: Add Veilid to your list of available software.<br />
*Explanation*: With the package manager updated, it is now possible to install Veilid!
```shell
yum-config-manager --add-repo https://packages.veilid.net/rpm/veilid-rpm-repo.repo
```
**Step 2**: Install Veilid.<br />
*Explanation*: With the package manager updated, it is now possible to install Veilid!
```shell
dnf install veilid-server veilid-cli
```
