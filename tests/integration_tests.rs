use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use ::hate;

// #[test]
// fn it_displays_info() {
//     let info = hate::info();
//     assert_eq!(info, "hate - v0.1 - get shit done efficiently!");
// }

extern crate assert_cli;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("hate-cli")?;
        cmd.arg("filesystem").arg("");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("No such file or directory"));

        Ok(())
    }
}
