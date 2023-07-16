#!/bin/sh
set -e

if [ "$1" == "patch" ]; then
    echo Bumping patch version
elif [ "$1" == "minor" ]; then
    echo Bumping minor version
elif [ "$1" == "major" ]; then
    echo Bumping major version
fi

cargo set-version --dry-run --bump $1

