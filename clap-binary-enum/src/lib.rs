//! A derive macro for using two-variant enums as optional clap CLI flags.
//!
//! # Overview
//!
//! Deriving [`YesNoArg`] on a two-variant enum generates a companion struct
//! with two mutually exclusive flags (e.g. `--network` / `--no-network`) and
//! a `get()` method returning `Option<YourEnum>`.
//!
//! By default the flag names are derived from the enum name via kebab-case
//! (e.g. `DryRun` → `--dry-run` / `--no-dry-run`). You can customize the
//! flag names and help text with the `#[yesno(...)]` helper attribute.
//!
//! # Basic usage
//!
//! ```rust
//! use clap::Parser;
//! use clap_binary_enum::YesNoArg;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
//! pub enum Network { Online, Offline }
//!
//! #[derive(Parser, Debug)]
//! struct Cli {
//!     #[command(flatten)]
//!     network: NetworkArg,
//! }
//!
//! // --network → Some(Online)
//! let cli = Cli::parse_from(["myapp", "--network"]);
//! assert_eq!(cli.network.get(), Some(Network::Online));
//!
//! // --no-network → Some(Offline)
//! let cli = Cli::parse_from(["myapp", "--no-network"]);
//! assert_eq!(cli.network.get(), Some(Network::Offline));
//!
//! // omitted → None
//! let cli = Cli::parse_from(["myapp"]);
//! assert_eq!(cli.network.get(), None);
//! ```
//!
//! # Conversion into `Option`
//!
//! The generated struct also implements `From<{Name}Arg> for Option<{Name}>`,
//! so you can use `.into()` or `Option::from()`:
//!
//! ```rust
//! use clap::Parser;
//! use clap_binary_enum::YesNoArg;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
//! pub enum Network { Online, Offline }
//!
//! #[derive(Parser, Debug)]
//! struct Cli {
//!     #[command(flatten)]
//!     network: NetworkArg,
//! }
//!
//! let cli = Cli::parse_from(["myapp", "--network"]);
//! let opt: Option<Network> = cli.network.into();
//! assert_eq!(opt, Some(Network::Online));
//!
//! let cli = Cli::parse_from(["myapp"]);
//! let opt: Option<Network> = cli.network.into();
//! assert_eq!(opt, None);
//! ```
//!
//! # Custom flag names with `#[yesno(name = "...")]`
//!
//! Override the flag name stem at the enum level:
//!
//! ```rust
//! use clap::Parser;
//! use clap_binary_enum::YesNoArg;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
//! #[yesno(name = "net")]
//! pub enum Network { Online, Offline }
//!
//! #[derive(Parser, Debug)]
//! struct Cli {
//!     #[command(flatten)]
//!     network: NetworkArg,
//! }
//!
//! let cli = Cli::parse_from(["myapp", "--net"]);
//! assert_eq!(cli.network.get(), Some(Network::Online));
//!
//! let cli = Cli::parse_from(["myapp", "--no-net"]);
//! assert_eq!(cli.network.get(), Some(Network::Offline));
//! ```
//!
//! # Help text with `#[yesno(help = "...")]`
//!
//! Add help text to individual flags via per-variant attributes:
//!
//! ```rust
//! use clap::Parser;
//! use clap_binary_enum::YesNoArg;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
//! pub enum Network {
//!     #[yesno(help = "Connect to the network")]
//!     Online,
//!     #[yesno(help = "Work without a network")]
//!     Offline,
//! }
//!
//! #[derive(Parser, Debug)]
//! struct Cli {
//!     #[command(flatten)]
//!     network: NetworkArg,
//! }
//!
//! let cli = Cli::parse_from(["myapp", "--network"]);
//! assert_eq!(cli.network.get(), Some(Network::Online));
//! ```
//!
//! # Multiple flags
//!
//! ```rust
//! use clap::Parser;
//! use clap_binary_enum::YesNoArg;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
//! pub enum Network { Online, Offline }
//!
//! #[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
//! pub enum PrintDebug { Enabled, Disabled }
//!
//! #[derive(Parser, Debug)]
//! struct Cli {
//!     #[command(flatten)]
//!     network: NetworkArg,
//!
//!     #[command(flatten)]
//!     print_debug: PrintDebugArg,
//! }
//!
//! let cli = Cli::parse_from(["myapp", "--network", "--no-print-debug"]);
//! assert_eq!(cli.network.get(), Some(Network::Online));
//! assert_eq!(cli.print_debug.get(), Some(PrintDebug::Disabled));
//!
//! let cli = Cli::parse_from(["myapp"]);
//! assert_eq!(cli.network.get(), None);
//! assert_eq!(cli.print_debug.get(), None);
//! ```
//!
//! # Combined with subcommands and shared args
//!
//! ```rust
//! use clap::{Args, Parser, Subcommand};
//! use clap_binary_enum::YesNoArg;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
//! pub enum Network { Online, Offline }
//!
//! #[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
//! pub enum DryRun { Yes, No }
//!
//! #[derive(Args, Debug)]
//! struct SharedArgs {
//!     #[command(flatten)]
//!     network: NetworkArg,
//!
//!     #[command(flatten)]
//!     dry_run: DryRunArg,
//! }
//!
//! #[derive(Args, Debug)]
//! struct DeployArgs {
//!     #[command(flatten)]
//!     shared: SharedArgs,
//!
//!     #[arg(long)]
//!     region: String,
//! }
//!
//! #[derive(Args, Debug)]
//! struct MigrateArgs {
//!     #[command(flatten)]
//!     shared: SharedArgs,
//!
//!     #[arg(long)]
//!     target_version: String,
//! }
//!
//! #[derive(Subcommand, Debug)]
//! enum Commands {
//!     Deploy(DeployArgs),
//!     Migrate(MigrateArgs),
//! }
//!
//! #[derive(Parser, Debug)]
//! struct Cli {
//!     #[command(subcommand)]
//!     command: Commands,
//! }
//!
//! let cli = Cli::parse_from(["myapp", "deploy", "--region", "eu-west", "--network"]);
//! let Commands::Deploy(args) = cli.command else { panic!() };
//! assert_eq!(args.shared.network.get(), Some(Network::Online));
//! assert_eq!(args.shared.dry_run.get(), None);
//! assert_eq!(args.region, "eu-west");
//!
//! let cli = Cli::parse_from(["myapp", "migrate", "--target-version", "42", "--dry-run"]);
//! let Commands::Migrate(args) = cli.command else { panic!() };
//! assert_eq!(args.shared.dry_run.get(), Some(DryRun::Yes));
//! assert_eq!(args.shared.network.get(), None);
//! assert_eq!(args.target_version, "42");
//! ```
//!
//! # Mutually exclusive flags
//!
//! Passing both `--network` and `--no-network` is a clap error:
//!
//! ```rust
//! use clap::Parser;
//! use clap_binary_enum::YesNoArg;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, YesNoArg)]
//! pub enum Network { Online, Offline }
//!
//! #[derive(Parser, Debug)]
//! struct Cli {
//!     #[command(flatten)]
//!     network: NetworkArg,
//! }
//!
//! let result = Cli::try_parse_from(["myapp", "--network", "--no-network"]);
//! assert!(result.is_err());
//! ```

pub use clap_binary_enum_derive::YesNoArg;