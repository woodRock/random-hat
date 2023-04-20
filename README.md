# Doomerism

[![CI](https://github.com/woodRock/doomerism/actions/workflows/ci.yml/badge.svg)](https://github.com/woodRock/doomerism.github.io/actions/workflows/ci.yml)
[![gh pages](https://github.com/woodRock/doomerism/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/woodRock/doomerismactions/workflows/pages/pages-build-deployment)

Are you a doomer? Find out in this bare mininium 1980s-ish HTML website.

## Rust Frontend

Build a simple Rust frontend web application using Yew.

Tutorial https://www.youtube.com/watch?v=MddGbXgIt2E

## Organisation

This repository contains these directories.

```
.
├── docs
├── src
├── Cargo.toml
├── index.html
├── LICENSE
├── README.md
├── styles.css
└── Trunk.toml
```

- [**docs**](docs) contains the production build of the website.
- [**srcs**](src) contains the source files for the website.
- [**Cargo.toml**](Cargo.toml) contains the dependencies for the website.
- [**index.html**](index.html) loads the rust wasm file.
- [**LICENSE**](LICENSE) biolerplate MIT license.
- [**README.md**](README.md) this file.
- [**styles.css**](styles.css) contains the css for the website.
- [**Trunk.toml**](Trunk.toml) contains config for the trunk utility.

## Installation

Trunk is a crate that helps us build and package web applications.

```bash
$ cargo install trunk
```

Add web assembly as a compilation target to rust.

```bash
$ rustup target add wasm32-unknown-unknown
```

Create a package

```bash
$ cargo new my-app
```

Change directory

```bash
$ cd my-app
```

## Serve

Run `trunk serve` to run the application on your local machine.

```bash
$ trunk serve
```

Note: the `Trunk.toml` file contains the configuration for the trunk utility.

## Build

Build the application using `trunk build` and specify the `--release` tag. This generates the deployment ready files in the `docs` directory.

```bash
$ trunk build
```

## Github Pages - 404 Error

This trick handles the 404 error for using Browser Routing when deploying to Github Pages.

Create React App [docs](https://create-react-app.dev/docs/deployment/#github-pages):

You can use a trick to teach GitHub Pages to handle 404s by redirecting to your index.html page with a custom redirect parameter. You would need to add a 404.html file with the redirection code to the build folder before deploying your project, and you’ll need to add code handling the redirect parameter to index.html. You can find a detailed explanation of this technique in this [guide](https://github.com/rafrex/spa-github-pages).
