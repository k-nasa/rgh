# rgh

## Overview

[![Actions Status](https://github.com/k-nasa/rgh/workflows/CI/badge.svg)](https://github.com/k-nasa/rgh/actions)
[![crate-name at crates.io](https://img.shields.io/crates/v/rgh.svg)](https://crates.io/crates/rgh)

Creates GitHub release and upload asset files

## Demo

![Demo](https://user-images.githubusercontent.com/23740172/68768822-9ab1dd00-0666-11ea-9bc3-469333fde310.gif)

## Installation

### Pre-compiled executables

Get them [here](https://github.com/k-nasa/rgh/releases)

### using homebrew

```
brew install k-nasa/tap/rgh
```

### using cargo

Currently it cannot be built with the stable version.

```
cargo +beta install rgh
```

##### Installation of cargo itself.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage

```
rgh 0.1.0
Creates GitHub release and upload asset files

USAGE:
    rgh [OPTIONS] <tag> <packages>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --commit <target-commitish>    Specifies the commitish value that determines where the Git tag is created from.
                                       Can be any branch or commit SHA. Unused if the Git tag already exists. Default:
                                       the repository's default branch (usually master).
    -t, --token <token>                Set Github API Token (By default reads the GITHUB_TOKEN environment variable)
        --title <name>                 The title of the release
    -b, --body <body>                  Text describing the contents of the tag.
        --draft <draft>                 [possible values: true, false]
        --prerelease <prerelease>       [possible values: true, false]

ARGS:
    <tag>         tag
    <packages>    upload packages dir or file
```

## Contribution

1. Fork it ( http://github.com/k-nasa/rgh )
2. Create your feature branch (git checkout -b my-new-feature)
3. Commit your changes (git commit -am 'Add some feature')
4. Push to the branch (git push origin my-new-feature)
5. Create new Pull Request

## Licence

[MIT](https://github.com/k-nasa/rgh/blob/master/LICENCE)

## Author

[k-nasa](https://github.com/k-nasa)

[my website](https://k-nasa.me)
