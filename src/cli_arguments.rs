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
    #[clap(alias = "tx")]
    Text(Text),
}

#[derive(Debug, Subcommand)]
pub enum TextCommands {
    Inspect(Inspect),
    Reverse(Reverse),
}

#[derive(Args, Debug)]
#[command(about = "Package for Text Operations")]
pub struct Text {
    #[command(subcommand)]
    pub command: Option<TextCommands>,
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
