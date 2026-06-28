# clap-binary-enum

[![Crates.io](https://img.shields.io/crates/v/clap-binary-enum.svg)](https://crates.io/crates/clap-binary-enum)
[![Docs.rs](https://docs.rs/clap-binary-enum/badge.svg)](https://docs.rs/clap-binary-enum)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/clap-binary-enum.svg)](#license)

A derive macro for using two-variant enums as optional clap CLI flags.

## Overview

Deriving `YesNoArg` on a two-variant enum generates a companion struct with
two mutually exclusive flags (e.g. `--network` / `--no-network`) and a `get()`
method returning `Option<YourEnum>`.

No more manual boolean fields, `--flag` / `--no-flag` pairs, or `match` boilerplate.

Flag names are derived from the enum name via kebab-case by default, and can
be customized with the `#[yesno(...)]` helper attribute.

## Quick example

```rust
use clap::Parser;
use clap_binary_enum::YesNoArg;

#[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
pub enum Network { Online, Offline }

#[derive(Parser, Debug)]
struct Cli {
    #[command(flatten)]
    network: NetworkArg,
}

// --network → Some(Online)
let cli = Cli::parse_from(["myapp", "--network"]);
assert_eq!(cli.network.get(), Some(Network::Online));

// --no-network → Some(Offline)
let cli = Cli::parse_from(["myapp", "--no-network"]);
assert_eq!(cli.network.get(), Some(Network::Offline));

// omitted → None
let cli = Cli::parse_from(["myapp"]);
assert_eq!(cli.network.get(), None);
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
clap-binary-enum = "0.1"
```

Make sure you also have `clap` with the `derive` feature enabled.

## Usage

### Multiple flags

Derive `YesNoArg` on any number of two-variant enums and flatten them into
your CLI struct:

```rust
#[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
pub enum Network { Online, Offline }

#[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
pub enum PrintDebug { Enabled, Disabled }

#[derive(Parser, Debug)]
struct Cli {
    #[command(flatten)]
    network: NetworkArg,

    #[command(flatten)]
    print_debug: PrintDebugArg,
}

let cli = Cli::parse_from(["myapp", "--network", "--no-print-debug"]);
assert_eq!(cli.network.get(), Some(Network::Online));
assert_eq!(cli.print_debug.get(), Some(PrintDebug::Disabled));
```

### Subcommands and shared args

Flatten the generated args into a shared struct, then compose it into
subcommand-specific args:

```rust
#[derive(Args, Debug)]
struct SharedArgs {
    #[command(flatten)]
    network: NetworkArg,

    #[command(flatten)]
    dry_run: DryRunArg,
}

#[derive(Parser, Debug)]
enum Commands {
    Deploy(DeployArgs),
    Migrate(MigrateArgs),
}
```

### Conversion into `Option`

The generated struct also implements `From<{Name}Arg> for Option<{Name}>`,
so you can use `.into()` or `Option::from()`:

```rust
let cli = Cli::parse_from(["myapp", "--network"]);
let opt: Option<Network> = cli.network.into();
assert_eq!(opt, Some(Network::Online));
```

### Mutually exclusive flags

Passing both `--network` and `--no-network` is automatically rejected by clap
as a conflict error:

```rust
let result = Cli::try_parse_from(["myapp", "--network", "--no-network"]);
assert!(result.is_err());
```

## Customization

### Enum-level: custom flag name stem

Override the flag name stem with `#[yesno(name = "...")]`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
#[yesno(name = "net")]
pub enum Network { Online, Offline }

// → --net / --no-net
```

### Per-variant: help text

Add help text to individual flags via per-variant `#[yesno(help = "...")]`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
pub enum Network {
    #[yesno(help = "Connect to the network")]
    Online,
    #[yesno(help = "Work without a network")]
    Offline,
}

// → --network with help "Connect to the network"
// → --no-network with help "Work without a network"
```

## How it works

`YesNoArg` is a procedural derive macro. Given an enum like:

```rust
#[derive(YesNoArg)]
enum Network { Online, Offline }
```

it generates:

```rust
#[derive(clap::Args, Debug)]
#[group(multiple = false)]
struct NetworkArg {
    #[arg(long)]
    network: bool,
    #[arg(long)]
    no_network: bool,
}

impl NetworkArg {
    fn get(&self) -> Option<Network> {
        match (self.network, self.no_network) {
            (true, _) => Some(Network::Online),
            (_, true) => Some(Network::Offline),
            _ => None,
        }
    }
}

impl From<NetworkArg> for Option<Network> {
    fn from(arg: NetworkArg) -> Self {
        arg.get()
    }
}
```

The enum name is converted from CamelCase to kebab-case for the flag names
(e.g. `DryRun` → `--dry-run` / `--no-dry-run`). Clap's `#[group(multiple = false)]`
enforces mutual exclusivity.

## License

Licensed under either of:

- [MIT license](https://opensource.org/licenses/MIT)
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)

at your option.
