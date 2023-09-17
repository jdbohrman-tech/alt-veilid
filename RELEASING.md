# Veilid Release Process

## Introduction

This guide outlines the process for releasing a new version of Veilid. The end result is an update of the package repositories and Pypi.

## Create a Gitlab Release

Releases happen via a CI/CD pipeline. The release process flows as follows:

1. Complete outstanding merge requests (MR):

    1.1 Evaluate the MR's adherence to the published requirements and if automatic tests passed.

    1.2 (Optional) Perform the merge in a local dev environment if testing is required beyond the standard Earthly tests.

    1.3 If everything checks out, MR meets the published requirements, and tests passed, execute the merge functions in the Gitlab UI.

2. Maintainer performs version bump:

    2.1 Update your local copy of `main` to mirror the newly merged upstream `main`

    2.2 Ensure the [CHANGELOG](./CHANGELOG.md) is updated

    2.3 Activate your bumpversion Python venv (see bumpversion setup section for details)

    2.4 Execute version_bump.sh with the appropriate parameter (patch, minor, or major). This results in all version entries being updated and a matching git tag created locally.

    2.5 Add all changes `git add *`

    2.6 Git commit the changes with the following message: `Version update: v{current_version} → v{new_version}`

    2.7 Create the Git tag `git tag v{new_version}`

    2.8 Push your local 'main' to the upstream origin 'main' `git push`

    2.9 Push the new tag to the upstream origin `git push origin {tag name made in step 2.7}` i.e. `git push origin v0.1.5`

    2.10 Ensure the package/release/distribute pipeline autostarted in the Gitlab UI

Git tags serve as a historical record of what repo versions were successfully released at which version numbers.

## Publish to crates.io

1. Configure the crates.io credentials, if not already accomplished.
2. Execute `cargo publish -p veilid-tools --dry-run`
3. Execute `cargo publish -p veilid-tools`
4. Execute `cargo publish -p veilid-core --dry-run`
5. Execute `cargo publish -p veilid-core`

## Publish to Pypi

1. Change directory to veilid-python
2. Install Poetry and configure the Pypi credentials, if not already accomplished.
3. Execute `poetry build`
4. Execute `poetry publish`

## Reverting Releases

Occasionally a release will happen that needs to be reverted. This is done manually on `crates.io` or the APT repository, or wherever the artifacts end up. Tags are not removed.

## Released Artifacts

### Rust Crates

- [x] __veilid-tools__ [__Tag__: `veilid-tools-v0.0.0`]
  > An assortment of useful components used by the other Veilid crates.  
  > Released to crates.io when its version number is changed in `Cargo.toml`
- [x] __veilid-core__  [__Tag__: `veilid-core-v0.0.0`]
  > The base rust crate for Veilid's logic  
  > Released to crates.io when its version number is changed in `Cargo.toml`
- [ ] __veilid-server__
  > The Veilid headless node end-user application  
  > Not released to crates.io as it is an application binary that is either built by hand or installed using a package manager.  
  > This application does not currently support `cargo install`
- [ ] __veilid-cli__ A text user interface to talk to veilid-server and operate it manually
  > Not released to crates.io as it is an application binary that is either built by hand or installed using a package manager.  
  > This application does not currently support `cargo install`
- [ ] __veilid-wasm__
  > Not released to crates.io as it is not a library that can be linked by other Rust applications
- [ ] __veilid-flutter__
  > The Dart-FFI native interface to the Veilid API  
  > This is currently built by the Flutter plugin `veilid-flutter` and not released.

### Python Packages

- [x] __veilid-python__ [__Tag__: `veilid-python-v0.0.0`]
  > The Veilid API bindings for Python  
  > Released to PyPi when the version number is changed in `pyproject.toml`
  
### Flutter Plugins

- [ ] __veilid-flutter__
  > The Flutter plugin for the Veilid API.  
  > Because this requires a build of a native Rust crate, this is not yet released via <https://pub.dev>  
  > TODO: Eventually the rust crate should be bound to

### Operating System Packages

- [x] __veilid-server__ DEB package [__Tag__: `veilid-server-deb-v0.0.0`]
  > The Veilid headless node binary in the following formats:  
  >
  > - Standalone Debian/Ubuntu DEB file as a 'release file' on the `veilid` GitLab repository
  > - Pushed to APT repository at <https://packages.veilid.net>
  >
- [x] __veilid-server__ RPM package [__Tag__: `veilid-server-rpm-v0.0.0`]
  > The Veilid headless node binary in the following formats:  
  >
  > - Standalone RedHat/CentOS RPM file as a 'release file' on the `veilid` GitLab repository
  > - Pushed to Yum repository at <https://packages.veilid.net>
  >
- [x] __veilid-cli__ DEB package [__Tag__: `veilid-cli-deb-v0.0.0`]
  > The Veilid headless node administrator control binary in the following formats:  
  >
  > - Standalone Debian/Ubuntu DEB file as a 'release file' on the `veilid` GitLab repository
  > - Pushed to APT repository at <https://packages.veilid.net>
  >
- [x] __veilid-cli__ RPM package [__Tag__: `veilid-cli-rpm-v0.0.0`]
  > The Veilid headless node administrator control binary in the following formats:  
  >
  > - Standalone RedHat/CentOS RPM file as a 'release file' on the `veilid` GitLab repository
  > - Pushed to Yum repository at <https://packages.veilid.net>

### Version Numbering

All versions of Veilid Rust crates as well as `veilid-python` and `veilid-flutter` packages are versioned using Semver. Versions can differ per crate and package, and it is important for the Semver rules to be followed (<https://semver.org/>):

- MAJOR version when you make incompatible API changes
- MINOR version when you add functionality in a backward compatible manner
- PATCH version when you make backward compatible bug fixes

The `version_bump.sh` script should be run on every release to stable. All of the Rust crates are versioned together and should have the same version, as well as the `veilid-python` Python package and `veilid-flutter` Flutter plugin.

## Bumpversion Setup and Usage

### Install Bumpversion

1. Create a Python venv for bumpversion.py. Mine is in my home dir so it persists when I update my local Veilid `main`.

    `python3 -m venv ~/bumpversion-venv`
2. Activate the venv. `source ~/bumpversion-venv/bin/activate`
3. Install bumpversion. `pip3 install bumpversion`

### Activate venv for version bumping step of the release process

1. Activate the venv. `source ~/bumpversion-venv/bin/activate`
2. Return to step 2.4 of _Create a Gitlab Release_
