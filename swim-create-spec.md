# Swim-Create

## Purpose
The purpose of swim-create is to allow for easier and more beginner-friendly creation and deployment of swim applications.

<br><br>
Main benefits:

* Allow developers to initialise projects from templates.
* Allow for creating templates from existing projects.
* Add planes, agents or lanes to existing projects.
* Support for multiple languages (Java, Rust).
* Produce build scripts for deployment.
* Manage swim dependencies and allow for automatic updates when new versions are available.
* Allow for uploading projects directly from the CLI.

## Design ideas

### Project creation

The main goal of the swim-creator is to speed up the creation of new projects and generate all the common swim code that 
the developers will likely need. 
The simplest project creation will create a swim project skeleton, allowing developers to directly
start working on adding swim planes, agents and lanes. 
This would help new developers to get familiar with the structure of a swim application as well as it will provide a common
pattern to follow.

### Project templates

The creator tool will have support for more complex project creation, based on commonly used patterns,
such as creating swim apps that work with Apache Kafka.
It should also be possible to create projects based on custom templates and to create templates from existing projects.
The templates can be defined in a common language such as Recon and the creator tool would be able to use them in order to
produce project in different languages (Rust, Java), allowing for a seamless transition between them.

### Project modification

The swim-creator will also have an interactive mode, allowing developers to add new planes, agents or lanes
to their existing projects. This will have the benefit of quickly helping developers get started without the need to 
consult the documentation in order to create the most basic swim structures, such as lanes and agents.

### Deployment

In addition to the creation of projects, the tool should be able to produce build scripts compatible with the automatic
swim deployment system. 
It should also be possible to use the tool to deploy projects directly from the CLI and manage them (stop, restart, etc.).
Finally, the swim-creator should be able to update the project dependencies when new versions of the 
swim system are released.

## Features

Create a new swim project.
```
swim-create new                          
  -t  --template            Create a project from a template file.
  -l  --language            A language in which the project should be created.
```
<hr>
Create a swim project template from the current project. 

```
swim-create template   
```

The structure of the current project (planes, agents, lanes) will be extracted and converted into a Recon template file.
The file can then be used to create projects with the same structure in any of the supported languages.
The custom logic of the project (for example logic inside lane callbacks) will not be transferred.
<hr>
Add a plane to the project.

```
swim-create add plane NAME
```
<hr>
Add an agent to a plane in the project.

```
swim-create add agent PLANE NAME
```
<hr>
Add a lane to an agent in the project.

```
swim-create add lane PLANE AGENT NAME
```
<hr>
Updates the swim system version of the project.

```
swim-create update
```
<hr>
Deploy the project to the swim cloud.

```
swim-create deploy
```
This should allow the user to log into his swim account and deploy the project from the command line.
<hr>
