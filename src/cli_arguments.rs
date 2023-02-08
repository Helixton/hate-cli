use ::clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "hate-cli - provided by Helixton - get shit done efficiently by automation")]
pub struct CLI {
    #[command(subcommand)]
    pub package: Option<Packages>,
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
#[command(about = "Package for Operations on Filesystem")]
pub struct Filesystem {
    #[command(subcommand)]
    pub value: Option<FSCommands>,
}

#[derive(Debug, Args)]
pub struct Inspect {
    pub value: Option<String>,
    #[arg(short = 'd', long = "digits")]
    pub only_digits: bool,
}

#[derive(Debug, Args)]
pub struct Reverse {
    pub value: Option<String>,
}
