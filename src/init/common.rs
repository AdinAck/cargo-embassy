use crate::{
    cli::{Family, Target},
    init::{nrf::init_memory_x, utils::get_family_and_target_from_chip},
};
use probe_rs::config::{get_target_by_name, search_chips};
use std::{env::set_current_dir, fs, io::Write, process::Command};

use super::utils::{cargo_add, init_file};

fn init_config(target: &Target, chip: &str) {
    fs::create_dir_all(".cargo").expect("Failed to create \".cargo\".");

    init_file(
        ".cargo/config.toml",
        &format!(
            include_str!("../templates/config.toml.template"),
            target = target,
            chip = chip
        ),
    );
}

fn init_toolchain(target: &Target) {
    init_file(
        "rust-toolchain.toml",
        &format!(
            include_str!("../templates/rust-toolchain.toml.template"),
            target = target
        ),
    );
}

fn init_build(family: &Family) {
    match family {
        Family::STM32 => init_file(
            "build.rs",
            include_str!("../templates/build.rs.stm32.template"),
        ),
        Family::NRF => init_file(
            "build.rs",
            include_str!("../templates/build.rs.nrf.template"),
        ),
    }
}

fn init_embed(chip: &str) {
    init_file(
        "Embed.toml",
        &format!(
            include_str!("../templates/Embed.toml.template"),
            chip = chip
        ),
    );
}

fn init_fmt() {
    init_file("src/fmt.rs", include_str!("../templates/fmt.rs.template"));
}

fn init_main(family: &Family) {
    match family {
        Family::STM32 => init_file(
            "src/main.rs",
            include_str!("../templates/main.rs.stm32.template"),
        ),
        Family::NRF => init_file(
            "src/main.rs",
            include_str!("../templates/main.rs.nrf.template"),
        ),
    }
}

fn init_manifest(name: &str, chip: &str, commit: Option<String>) {
    let family = chip
        .find("stm32")
        .map(|_| Family::STM32)
        .or(chip.find("nrf52").map(|_| Family::NRF))
        .expect("Chip does not correspond to known family.");

    let source = if let Some(commit) = commit {
        format!("rev = \"{commit}\"")
    } else {
        "branch = \"main\"".into()
    };

    let features = match family {
        Family::STM32 => {
            format!(
                r#"["nightly", "memory-x", "{chip}", "time-driver-any", "exti", "unstable-pac"]"#
            )
        }
        Family::NRF => {
            format!(r#"["nightly", "{chip}", "gpiote", "time-driver-rtc1"]"#)
        }
    };

    init_file(
        "Cargo.toml",
        &format!(
            include_str!("../templates/Cargo.toml.template"),
            name = name,
            family = family,
            features = features,
            source = source
        ),
    );

    // NOTE: should be threaded proably
    cargo_add(
        "cortex-m",
        Some(vec!["inline-asm", "critical-section-single-core"]),
        false,
    );
    cargo_add("cortex-m-rt", None, false);
    cargo_add("defmt", None, true);
    cargo_add("defmt-rtt", None, true);
    cargo_add("panic-probe", None, true);
    cargo_add("panic-halt", None, false);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("Cargo.toml")
        .expect("Failed to open \"Cargo.toml\".");

    file.write_all(
        format!(
            include_str!("../templates/Cargo.toml.append"),
            family = family
        )
        .as_bytes(),
    )
    .expect("Failed to append to \"Cargo.toml\".");
}

pub fn init(name: String, chip: String, commit: Option<String>) {
    println!("Setting up Embassy project...");

    Command::new("cargo")
        .args(["new", &name])
        .output()
        .expect("Failed to create cargo project.");

    set_current_dir(&name).expect("Failed to change directory to cargo project.");

    println!("Cargo project created...");

    if let Ok(chips) = search_chips(&chip) {
        let probe_target = get_target_by_name(chips.first().unwrap()).unwrap();

        let (family, target) = get_family_and_target_from_chip(&chip);

        match family {
            Family::STM32 => {
                // nothing special to generate for stm32
            }
            Family::NRF => {
                init_memory_x();
            }
        }

        init_config(&target, &probe_target.name);
        init_toolchain(&target);
        init_embed(&probe_target.name);
        init_build(&family);
        init_manifest(&name, &chip, commit);
        init_fmt();
        init_main(&family);

        println!("Done! ✅");
    }
}
