

use ::clap::{Parser, Subcommand, Args};
use ::hate;

mod cli_hate;
use cli_hate::{*};


fn main() {
    let cli = CLI::parse();

    println!("{}", hate::info());

    match &cli.command {
        Some(Packages::Filesystem(info)) => {
            println!("fs commands with {:?}", info);
        },
        _ => {
            println!("error: unknown command");
        },
    }


    // match &cli.command {
    //     Some(Commands::Reverse(name)) => {
    //         match name.string {
    //             Some(ref _name) => {
    //                 let reverse = cli::commands::reverse(_name);
    //                 println!("{}", reverse);
    //             }
    //             None => {
    //                 println!("Please provide a string to reverse");
    //             }
    //         }
    //     }
    //     Some(Commands::Inspect(name)) => {
    //         match name.string {
    //             Some(ref _name) => {
    //                 let (res, kind) = cli::commands::inspect(_name, name.only_digits);

    //                 let mut plural_s = "s";
    //                 if res == 1 {
    //                     plural_s = "";
    //                 }

    //                 println!("{:?} has {} {}{}.", _name, res, kind, plural_s);
    //             }
    //             None => {
    //                 println!("Please provide a string to inspect");
    //             }
    //         }
    //     }
    //     None => {}
    // }
}