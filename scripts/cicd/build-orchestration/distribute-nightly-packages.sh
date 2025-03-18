#!/bin/bash

# Clean and reset the workspaces
echo "Setting up the workspace"
# Rsync active repo to local workspace
rsync --archive gitlab-runner@10.116.0.3:/srv/ $HOME/srv/
# Delete previous versions of packages
rm -rf $HOME/srv/apt/pool/nightly/main/*.deb
rm -rf $HOME/srv/rpm/nightly/x86_64/*
rm -rf $HOME/rpm-build-container/mount/repo/nightly/x86_64/*

# Move build artifacts to workspaces
echo "Copying debs to $HOME/srv/apt/pool/nightly/main"
cp target/packages/*.deb $HOME/srv/apt/pool/nightly/main
echo "Copying rpms to $HOME/rpm-build-container/mount/repo/nightly/x86_64"
cp target/packages/*x86_64.rpm $HOME/rpm-build-container/mount/repo/nightly/x86_64

# Setup crypto
export GNUPGHOME="$(mktemp -d ~/pgpkeys-XXXXXX)"
cat veilid-packages-key.private | gpg --import
gpg --armor --export admin@veilid.org > $HOME/srv/gpg/veilid-packages-key.public

# Generate apt repo files
echo "Starting deb process"
cd $HOME/srv/apt
echo "Creating Packages file"
dpkg-scanpackages --arch amd64 pool/nightly > dists/nightly/main/binary-amd64/Packages
dpkg-scanpackages --arch arm64 pool/nightly > dists/nightly/main/binary-arm64/Packages
cat dists/nightly/main/binary-amd64/Packages | gzip -9 > dists/nightly/main/binary-amd64/Packages.gz
cat dists/nightly/main/binary-arm64/Packages | gzip -9 > dists/nightly/main/binary-arm64/Packages.gz
echo "Creating Release file"
cd $HOME/srv/apt/dists/nightly
bash $HOME/generate-nightly-release.sh > Release
echo "Signing Release file and creating InRelease"
cat $HOME/srv/apt/dists/nightly/Release | gpg --default-key admin@veilid.org -abs > /home/gitlab-runner/srv/apt/dists/nightly/Release.gpg
cat $HOME/srv/apt/dists/nightly/Release | gpg --default-key admin@veilid.org -abs --clearsign > /home/gitlab-runner/srv/apt/dists/nightly/InRelease

# Generate RPM repo files
echo "Starting rpm process"
cd $HOME
echo "Copying signing material to container workspace"
cp -R $GNUPGHOME/* $HOME/rpm-build-container/mount/keystore
echo "Executing container actions"
docker run --rm -d -it -e IS_NIGHTLY=$IS_NIGHTLY --name rpm-repo-builder --mount type=bind,source=$HOME/rpm-build-container/mount,target=/mount rpm-repo-builder-img:v12
sleep 2
cp -R $HOME/rpm-build-container/mount/repo/nightly/x86_64/* $HOME/srv/rpm/nightly/x86_64
cd $HOME/srv/rpm/nightly/x86_64
echo "Signing the rpm repository"
gpg --default-key admin@veilid.org --detach-sign --armor $HOME/srv/rpm/nightly/x86_64/repodata/repomd.xml

# Generate .repo file for stable x86_64 releases
echo "[veilid-nightly-x86_64-rpm-repo]
name=Veilid Nightly x86_64 RPM Repo
baseurl=https://packages.veilid.net/rpm/nightly/x86_64
enabled=1
gpgcheck=1
gpgkey=https://packages.veilid.net/gpg/veilid-packages-key.public" > $HOME/srv/rpm/nightly/x86_64/veilid-nightly-x86_64-rpm.repo

# Tar the repo data and transfer to the repo server
echo "Moving the repo scaffold to the repo server"
cd $HOME
rsync --archive --delete $HOME/srv/* gitlab-runner@10.116.0.3:/srv

# Cleanup
echo "Cleaning up the workspace"
rm -rf $GNUPGHOME
rm -rf $HOME/rpm-build-container/mount/keystore/*
echo "Nightly packages distribution process complete"