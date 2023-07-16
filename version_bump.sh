#!/bin/sh
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

CARGO_TOMLS=$(find . -name Cargo.toml -not -path ./external/\* -not -path ./Cargo.toml | xargs)

bumpversion --dry-run $PART $CARGO_TOMLS ./veilid-python/pyproject.toml ./veilid-flutter/pubspec.yaml

