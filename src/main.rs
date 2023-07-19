use ::clap::Parser;
use ::hate;


mod cli_arguments;
use cli_arguments::{*};

fn main() {
    let cli: CLI = CLI::parse();

    match &cli.package {
        Some(Package::Text(package)) => {
            match &package.command {
                Some(TextCommands::Inspect(command)) => {
                    match command.value {
                        Some(ref value) => {
                            let output: (i32, String) = hate::inspect(&value, false);
                            println!("{:?}", output.0);
                        },
                        _ => {}
                    }
                },
                Some(TextCommands::Reverse(command)) => {
                    match command.value {
                        Some(ref value) => {
                            let output: String = hate::reverse(&value);
                            println!("{}", output);
                        },
                        _ => {}
                    }
                },
                _ => {
                    println!("{}", hate::info());
                }
            }

        },
        Some(Package::PDF(package)) => {
            match &package.command {
                Some(PDFCommands::Extract(command)) => {
                    match command.value {
                        Some(ref value) => {
                            let output: String = hate::pdf_extract(&value);
                            println!("{}", output);
                        },
                        _ => {}
                    }
                }
                _ => {
                    println!("{}", hate::info());
                }
            }

        },
        _ => {
            println!("{}", hate::info());
        },
    }
}