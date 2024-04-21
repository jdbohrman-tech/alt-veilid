#!/bin/bash

# Clean and reset the workspace
echo "Setting up the workspace"
rm -rf /home/gitlab-runner/srv
mkdir -p /home/gitlab-runner/srv/{gpg,rpm,apt/{dists/stable/main/{binary-amd64,binary-arm64},pool/main}}

# Setup crypto
export GNUPGHOME="$(mktemp -d ~/pgpkeys-XXXXXX)"
cat ~/package-signing-key.private | gpg --import
gpg --armor --export admin@veilid.org > ~/srv/gpg/veilid-packages-key.public

# Copy .deb files into the workspace and generate repo files
echo "Starting deb process"
cd ~
tar -xf amd64-debs.tar
tar -xf arm64-debs.tar
cp *.deb ~/srv/apt/pool/main
cd ~/srv/apt
echo "Creating Packages file"
dpkg-scanpackages --arch amd64 pool/ > dists/stable/main/binary-amd64/Packages
dpkg-scanpackages --arch arm64 pool/ > dists/stable/main/binary-arm64/Packages
cat dists/stable/main/binary-amd64/Packages | gzip -9 > dists/stable/main/binary-amd64/Packages.gz
cat dists/stable/main/binary-arm64/Packages | gzip -9 > dists/stable/main/binary-arm64/Packages.gz
echo "Creating Release file"
cd ~/srv/apt/dists/stable
~/generate-release.sh > Release
echo "Signing Release file and creating InRelease"
cat ~/srv/apt/dists/stable/Release | gpg --default-key admin@veilid.org -abs > ~/srv/apt/dists/stable/Release.gpg
cat ~/srv/apt/dists/stable/Release | gpg --default-key admin@veilid.org -abs --clearsign > ~/srv/apt/dists/stable/InRelease

# Copy .rpm files into the workspace and generate repo files
echo "Starting rpm process"
cd ~
tar -xf amd64-rpms.tar
echo "Copying rpms to container workspace"
cp *.rpm /home/gitlab-runner/rpm-build-container/mount/repo
echo "Copying signing material to container workspace"
cp -R $GNUPGHOME /home/gitlab-runner/rpm-build-container/mount/keystore
echo "Executing container actions"
docker run --rm -d -it --name rpm-repo-builder --mount type=bind,source=/home/gitlab-runner/rpm-build-container/mount,target=/mount rpm-repo-builder-img:v8
sleep 2
cp -R /home/gitlab-runner/rpm-build-container/mount/repo/* ~/srv/rpm
cd ~/srv/rpm
echo "Signing the rpm repository"
gpg --default-key admin@veilid.org --detach-sign --armor ~/srv/rpm/repodata/repomd.xml

echo "[veilid-rpm-repo]
name=Veilid RPM Repo
baseurl=https://packages.veilid.net/rpm
enabled=1
gpgcheck=1
gpgkey=https://packages.veilid.net/gpg/veilid-packages-key.public" > /home/gitlab-runner/srv/rpm/veilid-rpm-repo.repo

# Tar the repo data and transfer to the repo server
echo "Moving the repo scaffold to the repo server"
cd ~
tar -cf /home/gitlab-runner/repo.tar srv
scp -i /home/gitlab-runner/.ssh/id_ed25519 /home/gitlab-runner/repo.tar gitlab-runner@10.116.0.3:~

# Cleanup
echo "Cleaning up the workspace"
rm -rf $GNUPGHOME
rm /home/gitlab-runner/repo.tar
rm /home/gitlab-runner/*.deb
rm /home/gitlab-runner/*.rpm
rm -rf /home/gitlab-runner/rpm-build-container/mount/keystore
rm rpm-build-container/mount/repo/*.rpm
rm -rf rpm-build-container/mount/repo/repodata/*
echo "Process complete"