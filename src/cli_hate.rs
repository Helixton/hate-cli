use ::clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "hate-cli - provided by Helixton - get shit done efficiently by automation",
    long_about = "<longabout>"
)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<Packages>,
}

#[derive(Subcommand)]
pub enum Packages {
    Filesystem(Filesystem),
}

#[derive(Debug, Subcommand)]
pub enum FSCommands {
    Inspect(Inspect),
    Reverse(Reverse),
}

#[derive(Args, Debug)]
pub struct Filesystem {
    #[command(subcommand)]
    string: Option<FSCommands>,
}

#[derive(Debug, Args)]
pub struct Inspect {
    string: Option<String>,
    #[arg(short = 'd', long = "digits")]
    only_digits: bool,
}

#[derive(Debug, Args)]
pub struct Reverse {
    string: Option<String>,
}
