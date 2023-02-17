

use ::clap::{Parser};
use ::hate;

mod cli_arguments;
use cli_arguments::{*};


fn main() {
    let cli: CLI = CLI::parse();

    match &cli.package {
        Some(Packages::Filesystem(package)) => {
            // println!("fs packages with {:?}", command.value);
            // println!("type: {}", hate::type_of(&command.value));

            match &package.command {
                Some(FSCommands::Inspect(command)) => {
                    match command.value {
                        Some(ref value) => {
                            let output: (i32, String) = hate::inspect(&value, false);
                            println!("{:?}", output.0);
                        },
                        _ => {}
                    }
                },
                Some(FSCommands::Reverse(command)) => {
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
        _ => {
            println!("{}", hate::info());
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