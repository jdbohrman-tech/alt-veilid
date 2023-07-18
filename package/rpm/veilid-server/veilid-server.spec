Summary: Install a server grade, headless Veilid node
Name: veilid-server
Version: $CARGO_VERSION
Release: 1
URL: https://veilid.com
Group: System
License: MPL 2.0
Packager: Veilid Foundation, Inc.
Requires: glibc-common >= 2.23
BuildRoot: /rpm-work-dir/veilid-server
BuildArch: $ARCH

%description
A server grade, headless Veilid node

%install
mkdir -p %{buildroot}/usr/bin/
cp /veilid/target/$CARGO_ARCH/release/veilid-server %{buildroot}/usr/bin/veilid-server

mkdir -p %{buildroot}/etc/systemd/system
cp /veilid/package/systemd/veilid-server.service %{buildroot}/etc/systemd/system/veilid-server.service

mkdir -p %{buildroot}/etc/veilid-server
cp /veilid/package/linux/veilid-server.conf %{buildroot}/etc/veilid-server/veilid-server.conf

%files
/usr/bin/veilid-server
/etc/systemd/system/veilid-server.service
/etc/veilid-server/veilid-server.conf

%post
adduser --system -U veilid &>/dev/null || true
mkdir -p /var/db/veilid-server/protected_store
mkdir -p /var/db/veilid-server/table_store
mkdir -p /var/db/veilid-server/block_store
chown -R veilid:veilid /var/db/veilid-server
chmod 0750 /var/db/veilid-server/protected_store
chmod 0750 /var/db/veilid-server/table_store
chmod 0750 /var/db/veilid-server/block_store 
chmod 0750 /var/db/veilid-server
chmod 755 /usr/bin/veilid-server

systemctl daemon-reload

echo "Congratulations! To start your Veilid node and set it to start at boot, run the command systemctl enable --now veilid-server"

%postun
systemctl daemon-reload

%posttrans
if systemctl is-active --quiet veilid-server.service; then
    systemctl restart veilid-server.service
else
    echo "Veilid-Server is installed but not currently running. Configure the service to start immediatly and at boot time by running the following command: systemctl enable --now veilid-server.service"
fi

%changelog
* Sun Jul 2 2023 TC <tc@veilid.org>
- experimental RPM building
