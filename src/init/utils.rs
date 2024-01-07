use std::{fs, io::Write, process::Command};

use indicatif::ProgressBar;

use crate::cli::{Family, Target};

use super::common::Error;

pub(crate) fn init_file(pb: &ProgressBar, name: &str, content: &str) -> Result<(), Error> {
    pb.set_message(format!("Create file: {name}"));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(name)
        .map_err(|_| Error::CreateFile(name.into()))?;

    file.write_all(content.as_bytes())
        .map_err(|_| Error::CreateFile(name.into()))?;

    Ok(())
}

pub(crate) fn cargo_add(
    pb: &ProgressBar,
    name: &str,
    features: Option<Vec<&str>>,
    optional: bool,
) -> Result<(), Error> {
    pb.set_message(format!("Cargo add: {name}"));

    let features = features.unwrap_or(Vec::new()).join(",");
    let mut cmd = Command::new("cargo");

    cmd.arg("add")
        .args([name, &format!("--features={features}")]);

    if optional {
        cmd.arg("--optional");
    }

    cmd.output().map_err(|_| Error::CargoAdd(name.into()))?;

    Ok(())
}

pub(crate) fn get_family_and_target_from_chip(chip: &str) -> Result<(Family, Target), Error> {
    Ok((Family::try_from(chip)?, Target::try_from(chip)?))
}
