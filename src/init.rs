use std::{env::set_current_dir, fs, io::Write, process::Command, time::Duration};

use indicatif::ProgressBar;
use probe_rs::config::{get_target_by_name, search_chips};

use crate::types::{Chip, Error, Family, InitArgs, InvalidChip, PanicHandler, Target};

pub struct Init {
    pb: ProgressBar,
}

impl Init {
    pub fn new() -> Self {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100));

        Self { pb }
    }

    pub fn run(&self, args: InitArgs) {
        match self.run_inner(args) {
            Ok(_) => self
                .pb
                .finish_with_message(format!("Finished in {}s", self.pb.elapsed().as_secs())),
            Err(e) => self
                .pb
                .abandon_with_message(format!("Failed with error: {:#?}.", e)),
        }
    }

    fn run_inner(&self, mut args: InitArgs) -> Result<(), Error> {
        // for convenience
        args.chip_name = args.chip_name.replace("-", "_");

        let (chip, probe_target_name) = self.get_target_info(&args.chip_name)?;

        self.create_proj(&args.name)?;

        if let Family::NRF = chip.family {
            self.init_memory_x(&chip)?;
        }

        self.init_config(&chip.target, &probe_target_name)?;
        self.init_toolchain(&chip.target)?;
        self.init_embed(&probe_target_name)?;
        self.init_build(&chip.family)?;
        self.init_manifest(&args.name, &chip, &args.commit, &args.panic_handler)?;
        self.init_fmt()?;
        self.init_main(&chip.family, &args.panic_handler)?;

        Ok(())
    }

    fn create_proj(&self, name: &str) -> Result<(), Error> {
        self.pb.set_message("Create cargo project");
        Command::new("cargo")
            .args(["new", &name])
            .output()
            .map_err(|_| Error::CreateCargo)?;

        set_current_dir(&name).map_err(|_| Error::ChangeDir)
    }

    fn get_target_info(&self, name: &str) -> Result<(Chip, String), Error> {
        self.pb.set_message("Searching chips");
        if let Ok(chips) = search_chips(&name) {
            let probe_target = get_target_by_name(
                chips
                    .first()
                    .map_or(Err(Error::InvalidChip(InvalidChip::Unknown)), |t| Ok(t))?,
            )
            .unwrap();

            Ok((Chip::try_from(name)?, probe_target.name))
        } else {
            Err(Error::InvalidChip(InvalidChip::Unknown))
        }
    }

    fn init_config(&self, target: &Target, chip: &str) -> Result<(), Error> {
        fs::create_dir_all(".cargo").map_err(|_| Error::CreateFile(".cargo/config.toml".into()))?;

        self.create_file(
            ".cargo/config.toml",
            &format!(
                include_str!("templates/config.toml.template"),
                target = target,
                chip = chip
            ),
        )
    }

    fn init_toolchain(&self, target: &Target) -> Result<(), Error> {
        self.create_file(
            "rust-toolchain.toml",
            &format!(
                include_str!("templates/rust-toolchain.toml.template"),
                target = target
            ),
        )
    }

    fn init_embed(&self, chip: &str) -> Result<(), Error> {
        self.create_file(
            "Embed.toml",
            &format!(include_str!("templates/Embed.toml.template"), chip = chip),
        )
    }

    fn init_build(&self, family: &Family) -> Result<(), Error> {
        match family {
            Family::STM32 => self.create_file(
                "build.rs",
                include_str!("templates/build.rs.stm32.template"),
            ),
            Family::NRF => {
                self.create_file("build.rs", include_str!("templates/build.rs.nrf.template"))
            }
        }
    }

    fn init_manifest(
        &self,
        name: &str,
        chip: &Chip,
        commit: &Option<String>,
        panic_handler: &PanicHandler,
    ) -> Result<(), Error> {
        let source = if let Some(commit) = commit {
            format!(r#"rev = "{commit}""#)
        } else {
            r#"branch = "main""#.into()
        };

        let features = match chip.family {
            Family::STM32 => {
                format!(
                    r#"["memory-x", "{}", "time-driver-any", "exti", "unstable-pac"]"#,
                    chip.name
                )
            }
            Family::NRF => {
                format!(r#"["{}", "gpiote", "time-driver-rtc1"]"#, chip.name)
            }
        };

        self.create_file(
            "Cargo.toml",
            &format!(
                include_str!("templates/Cargo.toml.template"),
                name = name,
                family = chip.family,
                features = features,
                source = source
            ),
        )?;

        // NOTE: should be threaded proably
        self.cargo_add(
            "cortex-m",
            Some(vec!["inline-asm", "critical-section-single-core"]),
            false,
        )?;
        self.cargo_add("cortex-m-rt", None, false)?;
        self.cargo_add("defmt", None, true)?;
        self.cargo_add("defmt-rtt", None, true)?;
        self.cargo_add("panic-probe", Some(vec!["print-defmt"]), true)?;
        self.cargo_add(panic_handler.str(), None, false)?;

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open("Cargo.toml")
            .map_err(|_| Error::CreateFile("Cargo.toml".into()))?;

        file.write_all(
            format!(
                include_str!("templates/Cargo.toml.append"),
                family = chip.family
            )
            .as_bytes(),
        )
        .map_err(|_| Error::CreateFile("Cargo.toml".into()))?;

        Ok(())
    }

    fn init_fmt(&self) -> Result<(), Error> {
        self.create_file("src/fmt.rs", include_str!("templates/fmt.rs.template"))
    }

    fn init_main(&self, family: &Family, panic_handler: &PanicHandler) -> Result<(), Error> {
        match family {
            Family::STM32 => self.create_file(
                "src/main.rs",
                &format!(
                    include_str!("templates/main.rs.stm32.template"),
                    panic_handler = inflector::cases::snakecase::to_snake_case(panic_handler.str())
                ),
            ),
            Family::NRF => self.create_file(
                "src/main.rs",
                &format!(
                    include_str!("templates/main.rs.nrf.template"),
                    panic_handler = inflector::cases::snakecase::to_snake_case(panic_handler.str())
                ),
            ),
        }
    }

    fn init_memory_x(&self, chip: &Chip) -> Result<(), Error> {
        if let Some(memory) = &chip.memory {
            self.create_file(
                "memory.x",
                &format!(
                    include_str!("templates/memory.x.template"),
                    flash_origin = memory.flash_origin,
                    flash_len = memory.flash_length,
                    ram_origin = memory.ram_origin,
                    ram_len = memory.ram_length,
                ),
            )?;
        } else {
            unreachable!("NRF chip *must* have memory region specified.");
        }

        Ok(())
    }

    fn create_file(&self, name: &str, content: &str) -> Result<(), Error> {
        self.pb.set_message(format!("Create file: {name}"));

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

    fn cargo_add(
        &self,
        name: &str,
        features: Option<Vec<&str>>,
        optional: bool,
    ) -> Result<(), Error> {
        self.pb.set_message(format!("Cargo add: {name}"));

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
}
