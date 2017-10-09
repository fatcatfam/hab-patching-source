# Patching Source in Habitat

In the previous modules you focus on packaging web applications, load balancers, and data-stores. Persistent services that have a lifecycle to manage. But what binaries, command-line tools?

This repository contains an exercise that explores the following concepts:

* Packaging a command-line application
* Packaging from existing archived source
* Patching source code

## Packaging a command-line application

Why would I want to package a command-line application?

A command-line application is a tool that could be used as a:

* development dependency of another package
* run-time dependency of another package
* tool used within a Habitat Studio

As development dependencies they are often build tools.

As run-time dependencies they are often tools that:

* Perform an optimized operation
* Communicate with a metadata service
* Manage credentials for services
* Query network state

As a Habitat Studio tool they could be to:

* Perform effect troubleshooting


## Packaging from existing archived source

The default behavior for packaging is to assume you are downloading a package. This is not always the case. Your packages may be delivered through different means and not available using default behavior for downloading the source archive; like an air-gapped environment or the origin of the source code has shutdown.

## Patching source code

When building applications from source it sometimes becomes necessary to apply fixes to the application's source code. The approach of patching is often the preferred manner of addressing deficiencies as it allows you to fix the issue with the source code without forking from future upstream changes in the application.

Patches represent a diff in the code that is expressed in a file. The `patch` application is able to apply this expressed diff to a source file as long as the original code exists.
