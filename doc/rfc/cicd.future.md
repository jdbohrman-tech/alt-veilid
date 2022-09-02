---
title: CI/CD Future Planning
keywords:
- dependencies
- cicd
- security
status: PROPOSAL
---

# CI/CD Future Planning

## Rationale for this document

In the coming year, it is the goal of this project to make a public announcment
of Veilid. When that occurs, not only will Veilid become available to users and
developers globally, it is also likely to become a high-value target for
nefarious actors. This means that, as a team, we must be concerned not only with
the functionality of the code, but the integrity of the code base and any
deployed assets that originate from the core Veilid project.

In this document I would like to propose some guidelines and processes that can
help to minimize the impact of malicious actors upon the core Veilid code base
by way of direct commits and/or to its dependencies.

Some of this work will be toil, but most ought to be automated.

## Forked Dependencies

There are a number of dependencies that have been forked to allow us to expand
on their capabilities. Some of these forks are hard forks, projects that have
diverged enough that the Veilid team will need to continue to maintain them.
There are other projects where Veilid changes have been minimal, and where we
will want to share our changes upstream.

There may be a very small number of cases where we will have to maintain patched
versions of active projects.

For the duration of the project, it will be important that we understand which
dependencies fall into which categories.

### Soft forks

**TODO** _Note which submodules are soft forks and changes can be contributed
upstream_

### Hard forks

**TODO** _Note which submodules are hard forks and will be maintained by us._
