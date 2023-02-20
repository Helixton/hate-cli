use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

extern crate assert_cli;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_text_reverse() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("hate-cli")?;
        cmd.arg("text").arg("reverse").arg("helixton");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("notxileh"));
        Ok(())
    }
}
