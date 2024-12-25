#!/bin/bash

### DISCLAIMER ###
#   This script originates from community contribution and has not been tested by
#   any member of the Veilid core team. Please review the source documentation at
#   https://github.com/unifi-utilities/unifios-utilities to make sure this script
#   is compatible with your Ubiquiti gear before blindly running a script that
#   contains a pipe to /bin/bash (line 17).

# install veilid via the debian package method
wget -O- https://packages.veilid.net/gpg/veilid-packages-key.public | sudo gpg --dearmor -o /usr/share/keyrings/veilid-packages-keyring.gpg
echo "deb [arch=arm64 signed-by=/usr/share/keyrings/veilid-packages-keyring.gpg] https://packages.veilid.net/apt stable main" | sudo tee /etc/apt/sources.list.d/veilid.list 1>/dev/null
apt update
apt install veilid-server veilid-cli

# install unifios-utilities (this allows us to enable "on boot" scripts)
curl -fsL "https://raw.githubusercontent.com/unifi-utilities/unifios-utilities/HEAD/on-boot-script/remote_install.sh" | /bin/bash

# create an on-boot script that will start veilid-server when this device is first booted, then start the server
cat > /data/on_boot.d/veilid.sh<< EOF
#!/bin/bash
sudo -u veilid veilid-server &
EOF
chmod +x  /data/on_boot.d/veilid.sh
./data/on_boot.d/veilid.sh
