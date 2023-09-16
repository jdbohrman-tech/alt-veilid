#!/bin/bash

# Fail out if any step has an error
set -e

if [ "$1" == "patch" ]; then
    echo Bumping patch version
    PART=patch
elif [ "$1" == "minor" ]; then
    echo Bumping minor version
    PART=minor
elif [ "$1" == "major" ]; then
    echo Bumping major version
    PART=major
else
    echo Unsupported part! Specify 'patch', 'minor', or 'major'
    exit 1
fi

# Change version of crates and packages everywhere
bumpversion $PART

# Get the new version we bumped to
NEW_VERSION=$(cat .bumpversion.cfg | grep current_version\ = | cut -d\  -f3)
echo NEW_VERSION=$NEW_VERSION

# Update crate dependencies for the crates we publish
cargo upgrade -p veilid-tools@$NEW_VERSION
cargo upgrade -p veilid-core@$NEW_VERSION

# Update lockfile
cargo update