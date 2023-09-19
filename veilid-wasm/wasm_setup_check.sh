#!/bin/bash
set -eo pipefail

SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
pushd "$SCRIPTDIR" &> /dev/null

OS="unknown"
if [ "$(uname)" == "Linux" ]; then
    if [ ! "$(grep -Ei 'debian|buntu|mint' /etc/*release)" ]; then
        echo "Not a supported Linux for this script"
        exit 1
    fi
    OS="linux"
elif [ "$(uname)" == "Darwin" ]; then
    OS="macos"
fi
if [ "$OS" == "unknown" ]; then
    echo "Not a supported operating system for this script"
    exit 1
fi

if command -v node &> /dev/null; then 
    echo '[X] NodeJS is available in the path'
else
    echo -e 'NodeJS is not available in the path.
  Install NodeJS from here: https://nodejs.org/en/download
  Or from a package manager: https://nodejs.org/en/download/package-manager'
    exit 1
fi


if command -v npm &> /dev/null; then 
    echo '[X] npm is available in the path'
else
    echo -e 'npm is not available in the path. It should have been installed with NodeJS.
  Install NodeJS from here: https://nodejs.org/en/download
  Or from a package manager: https://nodejs.org/en/download/package-manager'
    exit 1
fi

popd &> /dev/null