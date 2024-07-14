Summary: Veilid Server Command Line Interface
Name: veilid-cli
Version: $RELEASE_VERSION
Release: 1
URL: https://veilid.com
Group: System
License: MPL 2.0
Packager: Veilid Foundation, Inc.
Requires: glibc-common >= 2.23
BuildRoot: /rpm-work-dir/veilid-cli
BuildArch: $ARCH

%description
Veilid Server Command Line Interface

%install
mkdir -p %{buildroot}/usr/bin/
cp /veilid/target/$CARGO_ARCH/release/veilid-cli %{buildroot}/usr/bin/veilid-cli

%files
/usr/bin/veilid-cli

%post
chmod 755 /usr/bin/veilid-cli

%changelog
* Sun Jul 2 2023 TC <tc@veilid.org>
- experimental RPM building
