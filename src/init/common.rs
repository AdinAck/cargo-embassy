use crate::{
    cli::{Family, PanicHandler, Target},
    init::{nrf::init_memory_x, utils::get_family_and_target_from_chip},
};
use probe_rs::config::{get_target_by_name, search_chips};
use std::{env::set_current_dir, fs, io::Write, process::Command};

use super::utils::{cargo_add, init_file};

fn init_config(target: &Target, chip: &str) {
    fs::create_dir_all(".cargo").expect(r#"Failed to create ".cargo"."#);

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

fn init_main(family: &Family, panic_handler: &PanicHandler) {
    match family {
        Family::STM32 => init_file(
            "src/main.rs",
            &format!(
                include_str!("../templates/main.rs.stm32.template"),
                panic_handler = inflector::cases::snakecase::to_snake_case(panic_handler.str())
            ),
        ),
        Family::NRF => init_file(
            "src/main.rs",
            &format!(
                include_str!("../templates/main.rs.nrf.template"),
                panic_handler = inflector::cases::snakecase::to_snake_case(panic_handler.str())
            ),
        ),
    }
}

fn init_manifest(name: &str, chip: &str, commit: Option<String>, panic_handler: &PanicHandler) {
    let family = Family::try_from(chip).expect("Chip does not correspond to known family.");

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
    cargo_add("panic-probe", Some(vec!["print-defmt"]), true);
    cargo_add(panic_handler.str(), None, false);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("Cargo.toml")
        .expect(r#"Failed to open "Cargo.toml"."#);

    file.write_all(
        format!(
            include_str!("../templates/Cargo.toml.append"),
            family = family
        )
        .as_bytes(),
    )
    .expect(r#"Failed to append to "Cargo.toml"."#);
}

pub fn init(name: String, chip: String, commit: Option<String>, panic_handler: PanicHandler) {
    println!("Setting up Embassy project...");

    Command::new("cargo")
        .args(["new", &name])
        .output()
        .expect("Failed to create cargo project.");

    set_current_dir(&name).expect("Failed to change directory to cargo project.");

    println!("Cargo project created...");

    if let Ok(chips) = search_chips(&chip) {
        let probe_target = get_target_by_name(
            chips
                .first()
                .expect("Selected chip is unknown to probe-rs."),
        )
        .unwrap();

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
        init_manifest(&name, &chip, commit, &panic_handler);
        init_fmt();
        init_main(&family, &panic_handler);

        println!("Done! ✅");
    }
}
