# Welcome to Veilid

-   [From Orbit](#from-orbit)
-   [Run a Node](#run-a-node)
-   [Development](#development)

## From Orbit

The first matter to address is the question "What is Veilid?" The highest-level description is that Veilid is a peer-to-peer network for easily sharing various kinds of data.

Veilid is designed with a social dimension in mind, so that each user can have their personal content stored on the network, but also can share that content with other people of their choosing, or with the entire world if they want.

The primary purpose of the Veilid network is to provide the infrastructure for a specific kind of shared data: social media in various forms. That includes light-weight content such as Twitter's tweets or Mastodon's toots, medium-weight content like images and songs, and heavy-weight content like videos. Meta-content such as personal feeds, replies, private messages, and so forth are also intended to run atop Veilid.

## Run a Node
The easiest way to help grow the Veilid network is to run your own node. Every user of Veilid is a node, but some nodes help the netowrk more than others. These network support nodes are heavier than the node a user would establish on their phone in the form of a chat or social media application. A cloud based virtual private server (VPS), such as Digital Ocean Droplets or AWS EC2, with high bandwidth, processing resources, and up time availability is cruicial for building the fast, secure, and private routing that Veilid is built to provide.

To run such a node, establish a Debian or Fedora based VPS and install the veilid-server service. To make this process simple we are hosting package manager repositories for .deb and .rpm packages. See the [installing](./INSTALL.md) guide for more information.

## Development
If you're inclined to get involved in code and non-code development, please check out the [contributing](./CONTRIBUTING.md) guide. We're striving for this project to be developed in the open and by people for people. Specific areas in which we are looking for help include:

* Rust
* Flutter/Dart
* Python
* Gitlab DevOps and CI/CD
* Documentation
* Security reviews
* Linux packaging