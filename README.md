# Huffman Coding Visualization

This document aims to guide the project's users through the inital setup of this project.

> This projects assumes a Linux or Unix platform and makes no guarantees on running on Windows OS.

## Prerequisites

### Install Rust Toolchain

The complete guide on how to install the rust toolcahin can be found at this [link.]( https://www.rust-lang.org/tools/install)

After this, you should be able to get the version information of the specific compiler you've installed using the command below.

```bash
cargo --version
```

### Install NPM

NPM requires that there is Nodejs installed. This [guide](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) provides a complete walkthrough on how you can install Nodejs and NPM.

When the installations are complete, just to be on the same page, check whether the npm-cli and node compiler work on the terminal. The commands below should printout their version tags to standard output.

```bash
node -v
```

```bash
npm -v
```

### Install Make

The setup requires the make GNU tool for the builds. This command should show whether the make command is already present on your machine. If it'S not present, you can find out more about it [here](https://www.gnu.org/software/make/).

```bash
make -v
```

## How To Run

Having done away with the **prerequisites**, we can start setting up the project.

The simple make command should do it.

```bash
make
```

> The local tcp port 8080 is hardcoded for the server, this build fails if the port is taken by another program.

Once the project is setup, you can navigate to the designated port path. If tcp port **8000** is free, it's the default .

The terminal output will have information similar to this:
```
  App running at:
  - Local:   http://localhost:8080/ 
  - Network: http://192.168.0.184:8080/

  Note that the development build is not optimized.
  To create a production build, run npm run build.
```

