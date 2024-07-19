#!/bin/bash

echo "==========Log start $(date +%F_%T)==========" &>> /mount/logfile

echo "setting GNUPGHOME $(date +%F_%T)" &>> /mount/logfile
export GNUPGHOME=/mount/keystore

echo "Adding key to rpm utility $(date +%F_%T)" &>> /mount/logfile
echo "%_signature gpg
%_gpg_name 516C76D1E372C5C96EE54E22AE0E059BC64CD052" > /root/.rpmmacros

if [ "$IS_NIGHTLY" = "true" ]
then
    echo "Taking nightly actions branch $(date +%F_%T)" &>> /mount/logfile
    cd /mount/repo/nightly/x86_64
elif [ "$IS_NIGHTLY" = "false" ]
then
    echo "Taking stable branch actions $(date +%F_%T)" &>> /mount/logfile
    cd /mount/repo/stable/x86_64
else
    echo $IS_NIGHTLY "is not a valid state to determine if the build is STABLE or NIGHTLY (RPM RepoBuild)" &>> /mount/logfile
fi

echo "Signing RPMs $(date +%F_%T)" &>> /mount/logfile
rpm --addsign *.rpm &>> /mount/logfile

echo "Creating repo metadata $(date +%F_%T)" &>> /mount/logfile
createrepo . &>> /mount/logfile

echo "Setting file ownership $(date +%F_%T)" &>> /mount/logfile
chown -R 1000:1000 /mount

echo "==========RPM Packaging Process complete $(date +%F_%T)==========" &>> /mount/logfile