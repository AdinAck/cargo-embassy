use std::{fs, io::Write, process::Command};

use crate::cli::{Family, Target};

pub(crate) fn init_file(name: &str, content: &str) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(name)
        .expect(&format!(r#"Failed to create "{name}"."#));

    file.write_all(content.as_bytes())
        .expect(&format!(r#"Failed to write to "{name}"."#));

    println!("{}", name);
}

pub(crate) fn cargo_add(name: &str, features: Option<Vec<&str>>, optional: bool) {
    let features = features.unwrap_or(Vec::new()).join(",");
    let mut cmd = Command::new("cargo");

    cmd.arg("add")
        .args([name, &format!("--features={features}")]);

    if optional {
        cmd.arg("--optional");
    }

    cmd.output()
        .expect(&format!(r#"Failed to add "{name}" to manifest"#));

    println!("- {}", name);
}

pub(crate) fn get_family_and_target_from_chip(chip: &str) -> (Family, Target) {
    (
        Family::try_from(chip).expect("Invalid chip: Unknown family."),
        Target::try_from(chip).expect("Invalid chip: Unknown target."),
    )
}
