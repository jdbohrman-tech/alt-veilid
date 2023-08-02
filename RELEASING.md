# Veilid Release Process

## Introduction

Veilid is a monorepo consisting of several projects:  
(checked boxes are released as packages)

## Release Mechanism

Releases happen via a CI/CD pipeline. The release process flows as follows:
1. Maintainer responds to a merge request (MR):
    a. Evaluate the MR's adherence to the published requirements and if automatic tests passed.
    b. (Optional) Perform the merge in a local dev environment if testing is required beyond the standard Earthly tests.
    c. If everything checks out, MR meets the published requirements, and tests passed, execute the merge functions in the Gitlab UI.
2. Maintainer performs version bump:
    a. Update your local copy of `main` to mirror the newly merged version
    b. Execute the bumpversion.py process.
    c. Push your local 'main' to the upstream origin 'main' `git push`
    d. Push the new tag to the upstream origin `git push "tag name made in step 2d"`
    e. Ensure the package/release/distribute pipeline autostarted in the Gitlab UI

Tags serve as a historical record of what repo versions were successfully released at which version numbers.

## Reverting Releases

Occasionally a release will happen that needs to be reverted. This is done manually on `crates.io` or the APT repository, or wherever the artifacts end up. Tags are not removed.

## Released Artifacts

### Rust Crates:
- [x] __veilid-tools__ [**Tag**: `veilid-tools-v0.0.0`]
  > An assortment of useful components used by the other Veilid crates.  
  > Released to crates.io when its version number is changed in `Cargo.toml`
- [x] __veilid-core__  [**Tag**: `veilid-core-v0.0.0`]
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

### Python Packages:
- [x] __veilid-python__ [**Tag**: `veilid-python-v0.0.0`] 
  > The Veilid API bindings for Python  
  > Released to PyPi when the version number is changed in `pyproject.toml`
  
### Flutter Plugins:
- [ ] __veilid-flutter__ 
  > The Flutter plugin for the Veilid API.  
  > Because this requires a build of a native Rust crate, this is not yet released via https://pub.dev  
  > TODO: Eventually the rust crate should be bound to 

### Operating System Packages:
- [x] __veilid-server__ DEB package [**Tag**: `veilid-server-deb-v0.0.0`] 
  > The Veilid headless node binary in the following formats:  
  >  * Standalone Debian/Ubuntu DEB file as a 'release file' on the `veilid` GitLab repository
  >  * Pushed to APT repository at https://packages.veilid.net
- [x] __veilid-server__ RPM package [**Tag**: `veilid-server-rpm-v0.0.0`] 
  > The Veilid headless node binary in the following formats:  
  >  * Standalone RedHat/CentOS RPM file as a 'release file' on the `veilid` GitLab repository
  >  * Pushed to Yum repository at https://packages.veilid.net
- [x] __veilid-cli__ DEB package [**Tag**: `veilid-cli-deb-v0.0.0`] 
  > The Veilid headless node administrator control binary in the following formats:  
  >  * Standalone Debian/Ubuntu DEB file as a 'release file' on the `veilid` GitLab repository
  >  * Pushed to APT repository at https://packages.veilid.net
- [x] __veilid-cli__ RPM package [**Tag**: `veilid-cli-rpm-v0.0.0`] 
  > The Veilid headless node administrator control binary in the following formats:  
  >  * Standalone RedHat/CentOS RPM file as a 'release file' on the `veilid` GitLab repository
  >  * Pushed to Yum repository at https://packages.veilid.net

### Version Numbering:

All versions of Veilid Rust crates as well as `veilid-python` and `veilid-flutter` packages are versioned using Semver. Versions can differ per crate and package, and it is important for the Semver rules to be followed (https://semver.org/):

* MAJOR version when you make incompatible API changes
* MINOR version when you add functionality in a backward compatible manner
* PATCH version when you make backward compatible bug fixes

The `version_bump.sh` script should be run on every release to stable. All of the Rust crates are versioned together and should have the same version, as well as the `veilid-python` Python package and `veilid-flutter` Flutter plugin.

