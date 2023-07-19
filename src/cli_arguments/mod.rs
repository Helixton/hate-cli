use ::clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "hate-cli - provided by Helixton - get shit done efficiently by automation")]
pub struct CLI {
    #[command(subcommand)]
    pub package: Option<Package>,
}

#[derive(Subcommand)]
pub enum Package {
    #[clap(alias = "tx")]
    Text(TextPackage),
    PDF(PDFPackage)
}

#[derive(Args, Debug)]
#[command(about = "Package for Text Operations")]
pub struct TextPackage {
    #[command(subcommand)]
    pub command: Option<TextCommands>,
}

#[derive(Args, Debug)]
#[command(about = "Package for PDF Operations")]
pub struct PDFPackage {
    #[command(subcommand)]
    pub command: Option<PDFCommands>,
}

#[derive(Debug, Subcommand)]
pub enum TextCommands {
    Inspect(Inspect),
    #[clap(alias = "rev")]
    Reverse(Reverse),
}

#[derive(Debug, Subcommand)]
pub enum PDFCommands {
    #[clap(alias = "xtr")]
    Extract(Extract),
}

#[derive(Debug, Args)]
pub struct Reverse {
    pub value: Option<String>,
}

#[derive(Debug, Args)]
pub struct Inspect {
    pub value: Option<String>,
    #[arg(short = 'd', long = "digits")]
    pub only_digits: bool,
}

#[derive(Debug, Args)]
pub struct Extract {
    pub value: Option<String>,
}


