# Install a Veilid Node

## Server Grade Headless Nodes

These network support nodes are heavier than the node a user would establish on their phone in the form of a chat or social media application. A cloud based virtual private server (VPS), such as Digital Ocean Droplets or AWS EC2, with high bandwidth, processing resources, and up time availability is cruicial for building the fast, secure, and private routing that Veilid is built to provide.

### Add the repo to a Debian based system and install a Veilid node
 ```shell 
wget -O- https://packages.veilid.net/gpg/veilid-packages-key.public | sudo gpg --dearmor -o /usr/share/keyrings/veilid-packages-keyring.gpg
```
For AMD64 based systems run the following:
```shell
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/veilid-packages-keyring.gpg] https://packages.veilid.net/apt stable main" | sudo tee /etc/apt/sources.list.d/veilid.list 1>/dev/null
```
For ARM64 based systems, run the following:
```shell
echo "deb [arch=arm64 signed-by=/usr/share/keyrings/veilid-packages-keyring.gpg] https://packages.veilid.net/apt stable main" | sudo tee /etc/apt/sources.list.d/veilid.list 1>/dev/null
```
For all, run:
```shell
apt update
```
```shell
apt install veilid-server veilid-cli
```
### Add the repo to a Fedora based system and install a Veilid node
```shell
yum-config-manager --add-repo https://packages.veilid.net/rpm/veilid-rpm-test-repo.repo
```
```shell
dnf install veilid-server veilid-cli
```
