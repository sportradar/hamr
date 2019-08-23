![hamr](https://raw.githubusercontent.com/sportradar/hamr/master/hamr-logo.png)
## The CLI tool that lets you create, share and import secrets with your Git repositories easily via LastPass.
[![GitHub license](https://img.shields.io/badge/license-Apache%20License%202.0-blue.svg?style=flat)](http://www.apache.org/licenses/LICENSE-2.0)


## Installation
The tool is in its early stages, so it has not been released to any official package managers yet.
If you want to try it out follow the instructions below. 

The LastPass CLI needs to be installed. 
See its [GitHub page](https://github.com/lastpass/lastpass-cli) for details on installing it.

Cargo needs to be installed. 
See how to install Rust and Cargo on the language's [offical site](https://www.rust-lang.org/tools/install).

After the LastPass CLI and Cargo is installed, you can install `hamr`:
1. Clone/download the project.
2. Change directory to the project's root folder `cd hamr`
3. Run `cargo install --path . --force`.

## Usage

### Example scenarios

#### IntelliJ run configurations
IntelliJ lets you share run configurations via version control.
If your run configuration has secrets in it like environment variables with database passwords, you probably don't want to check those in to version control.
`hamr` can be used for that instead. 
 
 1. Add `.idea/runConfigurations/*` to your `.gitignore`, or replace _*_ with the name of the specific run configuration you want to not be checked into version control.
 2. Click on your run configuration in the IntelliJ UI, then press `Edit Configurations...` and enable `Share through VCS` for your run configuration. 
 This will create the run configuration as an XML file in `.idea/runConfigurations/<name>.xml`. 
 3. Run `hamr save -f .idea/runConfigurations/<name>.xml`. After completing these steps the file should now be available in your LastPass vault.
 4. Share the secret for the project with other users you want to have access to the file.
 5. Other users can now clone the project with `hamr clone <repository>`. 
 If they have the correct rights the git repository will be cloned, and the files shared with them will be downloaded. 

### Commands
Below is an overview of the commands available.

#### Help
Running `hamr`, `hamr -h` or `hamr --help` outputs the help command text:

```
hamr 0.1.0
Emil Orvik Kollstrøm <e.kollstrom@gmail.com>, Ruben Svanåsbakken Sevaldson <r.sevaldson@gmail.com>, Gisle Martin Aune
<dev@gisle.me>
The build tool that let's you create, share and import secrets with your Git repositories easily via LastPass

USAGE:
    hamr <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    clone    Clones repo and adds files shared with you
    help     Prints this message or the help of the given subcommand(s)
    load     Load files and environment variables from LastPass
    login    Logs in to LastPass
    save     Saves files and environment variables to LastPass
    time     MC Hammer
```

Running `hamr <SUBCOMMAND> -h` or `hamr <SUBCOMMAND> --help` outputs the specific help text for that sub-command.

#### Version
Running `hamr -V` or `hamr --version` outputs the installed version: `hamr 0.1.0`.

#### Clone
Running `hamr clone <repository> [folder]` clones a git repository, and downloads files shared with you related to that repository.
The contents are stored in the specified folder.

#### Load
Running `hamr load` fetches the files shared with you. 
You need to be in the root folder. 

#### Save
Running `hamr save [OPTIONS]` lets you store files and environment variables:

```
OPTIONS:
    -e, --env <env-variables>...    Add the environment variables to be uploaded
    -f, --files <files>...          Add the files to be uploaded
```
