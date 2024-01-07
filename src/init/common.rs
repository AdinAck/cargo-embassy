use crate::{
    cli::{Family, PanicHandler, Target},
    init::{nrf::init_memory_x, utils::get_family_and_target_from_chip},
};
use indicatif::ProgressBar;
use probe_rs::config::{get_target_by_name, search_chips};
use std::{env::set_current_dir, fs, io::Write, process::Command};

use super::utils::{cargo_add, init_file};

#[derive(Debug)]
pub enum Error {
    CreateCargo,
    CreateFile(String),
    ChangeDir,
    InvalidChip,
    CargoAdd(String),
}

fn init_config(pb: &ProgressBar, target: &Target, chip: &str) -> Result<(), Error> {
    fs::create_dir_all(".cargo").map_err(|_| Error::CreateFile(".cargo/config.toml".into()))?;

    init_file(
        pb,
        ".cargo/config.toml",
        &format!(
            include_str!("../templates/config.toml.template"),
            target = target,
            chip = chip
        ),
    )
}

fn init_toolchain(pb: &ProgressBar, target: &Target) -> Result<(), Error> {
    init_file(
        pb,
        "rust-toolchain.toml",
        &format!(
            include_str!("../templates/rust-toolchain.toml.template"),
            target = target
        ),
    )
}

fn init_build(pb: &ProgressBar, family: &Family) -> Result<(), Error> {
    match family {
        Family::STM32 => init_file(
            pb,
            "build.rs",
            include_str!("../templates/build.rs.stm32.template"),
        ),
        Family::NRF => init_file(
            pb,
            "build.rs",
            include_str!("../templates/build.rs.nrf.template"),
        ),
    }
}

fn init_embed(pb: &ProgressBar, chip: &str) -> Result<(), Error> {
    init_file(
        pb,
        "Embed.toml",
        &format!(
            include_str!("../templates/Embed.toml.template"),
            chip = chip
        ),
    )
}

fn init_fmt(pb: &ProgressBar) -> Result<(), Error> {
    init_file(
        pb,
        "src/fmt.rs",
        include_str!("../templates/fmt.rs.template"),
    )
}

fn init_main(pb: &ProgressBar, family: &Family, panic_handler: &PanicHandler) -> Result<(), Error> {
    match family {
        Family::STM32 => init_file(
            pb,
            "src/main.rs",
            &format!(
                include_str!("../templates/main.rs.stm32.template"),
                panic_handler = inflector::cases::snakecase::to_snake_case(panic_handler.str())
            ),
        ),
        Family::NRF => init_file(
            pb,
            "src/main.rs",
            &format!(
                include_str!("../templates/main.rs.nrf.template"),
                panic_handler = inflector::cases::snakecase::to_snake_case(panic_handler.str())
            ),
        ),
    }
}

fn init_manifest(
    pb: &ProgressBar,
    name: &str,
    chip: &str,
    commit: Option<String>,
    panic_handler: &PanicHandler,
) -> Result<(), Error> {
    let family = Family::try_from(chip)?;

    let source = if let Some(commit) = commit {
        format!(r#"rev = "{commit}""#)
    } else {
        r#"branch = "main""#.into()
    };

    let features = match family {
        Family::STM32 => {
            format!(r#"["memory-x", "{chip}", "time-driver-any", "exti", "unstable-pac"]"#)
        }
        Family::NRF => {
            format!(r#"["{chip}", "gpiote", "time-driver-rtc1"]"#)
        }
    };

    init_file(
        pb,
        "Cargo.toml",
        &format!(
            include_str!("../templates/Cargo.toml.template"),
            name = name,
            family = family,
            features = features,
            source = source
        ),
    )?;

    // NOTE: should be threaded proably
    cargo_add(
        pb,
        "cortex-m",
        Some(vec!["inline-asm", "critical-section-single-core"]),
        false,
    )?;
    cargo_add(pb, "cortex-m-rt", None, false)?;
    cargo_add(pb, "defmt", None, true)?;
    cargo_add(pb, "defmt-rtt", None, true)?;
    cargo_add(pb, "panic-probe", Some(vec!["print-defmt"]), true)?;
    cargo_add(pb, panic_handler.str(), None, false)?;

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("Cargo.toml")
        .map_err(|_| Error::CreateFile("Cargo.toml".into()))?;

    file.write_all(
        format!(
            include_str!("../templates/Cargo.toml.append"),
            family = family
        )
        .as_bytes(),
    )
    .map_err(|_| Error::CreateFile("Cargo.toml".into()))?;

    Ok(())
}

pub(crate) fn init(
    pb: &ProgressBar,
    name: String,
    chip: String,
    commit: Option<String>,
    panic_handler: PanicHandler,
) -> Result<(), Error> {
    pb.set_message("Create cargo project");
    Command::new("cargo")
        .args(["new", &name])
        .output()
        .map_err(|_| Error::CreateCargo)?;

    set_current_dir(&name).map_err(|_| Error::ChangeDir)?;

    pb.set_message("Searching chips");
    if let Ok(chips) = search_chips(&chip) {
        let probe_target =
            get_target_by_name(chips.first().map_or(Err(Error::InvalidChip), |t| Ok(t))?).unwrap();

        let (family, target) = get_family_and_target_from_chip(&chip)?;

        match family {
            Family::STM32 => {
                // nothing special to generate for stm32
            }
            Family::NRF => {
                init_memory_x(&pb)?;
            }
        }

        init_config(&pb, &target, &probe_target.name)?;
        init_toolchain(&pb, &target)?;
        init_embed(&pb, &probe_target.name)?;
        init_build(&pb, &family)?;
        init_manifest(&pb, &name, &chip, commit, &panic_handler)?;
        init_fmt(&pb)?;
        init_main(&pb, &family, &panic_handler)?;

        Ok(())
    } else {
        Err(Error::InvalidChip)
    }
}
