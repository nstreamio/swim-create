# swim-create CLI tool

Install *nix:

```
curl -L https://github.com/nstreamio/swim-create/releases/latest/download/swim-create-x86_64-unknown-linux-gnu.tar.gz | sudo tar -xz -C /usr/local/bin
```
or download manually and run:
```
sudo tar -f swim-create-x86_64-unknown-linux-gnu.tar.gz -xz -C /usr/local/bin
```

## Project structure

The CLI tool creates a base Swim project that is runnable, allowing developers to get started quicker by just adding their business logic.

A brief description of the most important files and their function:

`settings.gradle` - Contains the project name.

`build.gradle` - Contains the name of the main Java class and the Swim version.

`server.recon` - Contains information about the Swim plane and the port for the Swim server.

`MainPlane.java` - Is the Swim plane definition and the entry point.