# Veilid Automated Build and Distribution CICD

## Description

The release process for Veilid results in builds, packages, and libraries being distributed to various repositories. This is accomplished through Gitlab's Runner system interacting with droplets on Digital Ocean. Some of the droplets are always up while others are built at the time of release and deleted after the release job is accomplished.

The droplets are divided into three categories: build machines, build orchestration, and repo server. Build machines are ephemeral whereas build orchestration and repo server are online 24/7.

* Build Machines
  * Individual Debian high resource machine for each arch/OS combo being built
  * Enrolled with Gitlab Runner
  * Earthly installed
  * Droplet size: c2-8vcpu-16gb-intel
  * The amd64-deb build machine also builds and uploads the veilid-core and veilid-tools Rust crates and veilid-python module to crates.io and Pypi
  * SCPs compiled packages to the orchestration machine ovber private networking
* Build Orchestration
  * Single Debian machine with minimal resources
  * Creates and deletes build machines
  * Natively constructs and signs the .deb repository directory structure
  * Uses Docker with Rocky container to constuct and sign the .rpm directory structure
  * SCPs the repos to the repo server
* Repo Server
  * Single Debian machine with moderate resources
  * Hosts the .deb and .rpm package repositories for veilid-server and veilid-cli

## Process Flow

1. The release process is triggered by creating a new version number tag on Gitlab. The tag format must be `vX.X.X`.
2. Gitlab CICD builds a SaaS container in which Earthly tests are performed on the latest version of the Main branch. A test fail will exit the CICD process.
3. The Gitlab Runner registered to the build orchestration machine executes the build-machine-ctl.sh script to create the build machines.
4. The Gitlab Runners registered to the build machines execute their specified arch/OS build as defined in the Earthly execution command in .gitlab-ci.yml.
5. The build machine for amd64-deb additionally compiles veilid-core and veilid-tools Rust crates and the veilid-python module. These are uploaded to crates.io and Pypi as part of their respective build processes.
6. When a build completes, the Gitlab Runner then executes the scp-to-orchestrator.sh script when sends the .deb or .rpm packages to build orchestration.
7. Once the build jobs in CICD have completed, the Gitlab Runner registered to build orchestration executes the distribute-packages.sh script which results in signed .deb and .rpm repositories being sent to the repo server.
8. The build orchecstration machine sends droplet delete commands to Digital Ocean for each of the build machines.
9. The Gitlab Runner registered to the repo server executes the deploy-repo.sh script which updates the web server's file directory with the latest packages versions.
