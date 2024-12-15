use crate::{
    chip::{
        family::{esp::Variant, mem_region::MemRegion, Family},
        Chip,
    },
    cli::init_args::{panic_handler::PanicHandler, soft_device::Softdevice, InitArgs},
    error::{Error, InvalidChip},
};
use indicatif::ProgressBar;
use inflector::cases::snakecase::to_snake_case;
use probe_rs::config::{get_target_by_name, search_chips};
use std::{
    env::set_current_dir,
    fs,
    io::{Read, Write},
    process::Command,
    time::Duration,
};

use serde_json::Value;

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
        if let Err(e) = self.run_inner(args) {
            self.pb
                .abandon_with_message(format!("Failed with error: {e:#?}."))
        } else {
            self.pb
                .finish_with_message(format!("Finished in {}s", self.pb.elapsed().as_secs()))
        }
    }

    fn run_inner(&self, mut args: InitArgs) -> Result<(), Error> {
        // for convenience
        args.chip_name = args.chip_name.replace('-', "_").to_lowercase();

        let (chip, probe_target_name) = self.get_target_info(&args.chip_name)?;

        // validate softdevice <--> nrf
        if args.softdevice.is_some() && !matches!(chip.family, Family::NRF(_)) {
            return Err(Error::ErroneousSoftdevice);
        }

        self.create_project(&args.name)?;

        self.init_config(&chip, &probe_target_name)?;
        if args.vscode {
            self.init_debug_config(&chip, &probe_target_name, &args.name)?;
        }
        self.init_toolchain(&chip)?;
        if !matches!(&chip.family, Family::ESP(_)) {
            self.init_embed(&probe_target_name)?;
        }
        self.init_build(&chip.family)?;
        self.init_manifest(
            &args.name,
            &chip,
            &args.panic_handler,
            args.softdevice.as_ref(),
        )?;
        if !matches!(&chip.family, Family::ESP(_)) {
            self.init_fmt()?;
        }
        self.init_main(&chip.family, &args.panic_handler, args.softdevice.as_ref())?;

        if let Family::NRF(mem_reg) = chip.family {
            self.init_memory_x(mem_reg)?;
            self.pb.println("[ACTION NEEDED] You must now flash the Softdevice and configure memory.x. Instructions can be found here: https://github.com/embassy-rs/nrf-softdevice#running-examples.");
        }

        Ok(())
    }

    fn create_project(&self, name: &str) -> Result<(), Error> {
        self.pb.set_message("Create cargo project");
        Command::new("cargo")
            .args(["new", &name])
            .output()
            .map_err(|_| Error::CreateCargo)?;

        set_current_dir(name).map_err(|_| Error::ChangeDir)
    }

    fn get_target_info(&self, name: &str) -> Result<(Chip, String), Error> {
        self.pb.set_message("Searching chips");
        if let Ok(chips) = search_chips(name) {
            let probe_target = get_target_by_name(
                chips
                    .first()
                    .ok_or(Error::InvalidChip(InvalidChip::Unknown))?,
            )
            .unwrap();

            Ok((name.parse()?, probe_target.name))
        } else {
            Err(Error::InvalidChip(InvalidChip::Unknown))
        }
    }

    fn init_debug_config(&self, chip: &Chip, name: &str, project_name: &str) -> Result<(), Error> {
        fs::create_dir_all(".vscode").map_err(|_| Error::CreateFolder(".vscode".into()))?;

        let contents = include_str!("templates/launch.json.template").to_string();
        let mut contents =
            serde_json::from_str::<Value>(&contents).expect("failed to convert to json");

        // update chip name
        contents["configurations"][0]["chip"] = Value::String(name.to_string());

        // update target binary name
        let target = format!("target/{}/debug/{}", chip.target, project_name);
        contents["configurations"][0]["coreConfigs"][0]["programBinary"] = Value::String(target);

        self.create_file(
            ".vscode/launch.json",
            serde_json::to_string_pretty(&contents).unwrap().as_str(),
        )
    }

    fn init_config(&self, chip: &Chip, name: &str) -> Result<(), Error> {
        fs::create_dir_all(".cargo").map_err(|_| Error::CreateFolder(".cargo".into()))?;

        self.create_file(
            ".cargo/config.toml",
            &match &chip.family {
                Family::ESP(variant) => format!(
                    include_str!("templates/config.toml.esp.template"),
                    target = chip.target,
                    rustflags = match variant {
                        Variant::C3 => "rustflags = [\n\"-C\", \"force-frame-pointers\",\n]",
                        Variant::S2 | Variant::S3 =>
                            "rustflags = [\n\"-C\", \"link-arg=-nostartfiles\",\n]",
                    }
                ),
                _ => format!(
                    include_str!("templates/config.toml.template"),
                    target = chip.target,
                    chip = name
                ),
            },
        )
    }

    fn init_toolchain(&self, chip: &Chip) -> Result<(), Error> {
        self.create_file(
            "rust-toolchain.toml",
            &match chip.family {
                Family::ESP(_) => include_str!("templates/rust-toolchain.toml.esp.template").into(),
                _ => format!(
                    include_str!("templates/rust-toolchain.toml.template"),
                    target = chip.target
                ),
            },
        )
    }

    fn init_embed(&self, chip: &str) -> Result<(), Error> {
        self.create_file(
            "Embed.toml",
            &format!(include_str!("templates/Embed.toml.template"), chip = chip),
        )
    }

    fn init_build(&self, family: &Family) -> Result<(), Error> {
        let template = match family {
            Family::STM => include_str!("templates/build.rs.stm.template"),
            Family::NRF(_) => include_str!("templates/build.rs.nrf.template"),
            Family::ESP(_) => include_str!("templates/build.rs.esp.template"),
        };

        self.create_file("build.rs", template)
    }

    fn init_manifest(
        &self,
        name: &str,
        chip: &Chip,
        panic_handler: &PanicHandler,
        softdevice: Option<&Softdevice>,
    ) -> Result<(), Error> {
        self.create_file(
            "Cargo.toml",
            &format!(include_str!("templates/Cargo.toml.template"), name = name),
        )?;

        // NOTE: should be threaded proably
        self.cargo_add(
            "embassy-executor",
            match &chip.family {
                Family::ESP(_) => Some(&["executor-thread"]),
                _ => Some(&["arch-cortex-m", "executor-thread", "integrated-timers"]),
            },
            false,
        )?;
        self.cargo_add("embassy-sync", None, false)?;
        self.cargo_add("embassy-futures", None, false)?;
        self.cargo_add(
            "embassy-time",
            match &chip.family {
                Family::ESP(_) => None,
                _ => Some(&["tick-hz-32_768"]),
            },
            false,
        )?;

        match &chip.family {
            Family::STM => {
                self.cargo_add(
                    "embassy-stm32",
                    Some(&[
                        "memory-x",
                        chip.name.as_str(),
                        "time-driver-any",
                        "exti",
                        "unstable-pac",
                    ]),
                    false,
                )?;
            }
            Family::NRF(_) => {
                self.cargo_add(
                    "embassy-nrf",
                    Some(&[chip.name.as_str(), "gpiote", "time-driver-rtc1"]),
                    false,
                )?;
            }
            Family::ESP(variant) => {
                let name = variant.to_string();

                self.cargo_add("embassy-time-driver", None, false)?;
                self.cargo_add(
                    "esp-backtrace",
                    Some(&[&name, "exception-handler", "panic-handler", "println"]),
                    false,
                )?;
                self.cargo_add("esp-hal", Some(&[&name]), false)?;
                self.cargo_add(
                    "esp-hal-embassy",
                    Some(&[&name, "integrated-timers"]),
                    false,
                )?;
                self.cargo_add("esp-println", Some(&[&name, "log"]), false)?;
                self.cargo_add("log", None, false)?;
                self.cargo_add("static_cell", None, false)?;
            }
        };

        if let Some(softdevice) = softdevice {
            self.cargo_add(
                "nrf-softdevice",
                Some(&[
                    chip.name.as_str(),
                    softdevice.str(),
                    "ble-peripheral",
                    "ble-gatt-server",
                    "critical-section-impl",
                ]),
                false,
            )?;
            self.cargo_add(&format!("nrf-softdevice-{}", softdevice.str()), None, false)?;
        }

        if let Family::ESP(_) = &chip.family {
            println!("[NOTICE] ESP32s have their own panic handler system.");
            if panic_handler.ne(&PanicHandler::default()) {
                Err(Error::ErroneousPanicHandler)?
            }
        } else {
            self.cargo_add(
                "cortex-m",
                Some(if softdevice.is_some() {
                    &["inline-asm"]
                } else {
                    &["inline-asm", "critical-section-single-core"]
                }),
                false,
            )?;
            self.cargo_add("cortex-m-rt", None, false)?;
            self.cargo_add("defmt", None, true)?;
            self.cargo_add("defmt-rtt", None, true)?;
            self.cargo_add("panic-probe", Some(&["print-defmt"]), true)?;
            self.cargo_add(panic_handler.str(), None, false)?;

            let mut file = fs::OpenOptions::new()
                .read(true)
                .append(true)
                .open("Cargo.toml")
                .map_err(|_| Error::CreateFile("Cargo.toml".into()))?;

            // really gross patch for cargo version discontinuity
            // somewhere between cargo 1.72 and 1.76 the behavior of "cargo add" changed
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            if !buf.contains("[features]") {
                file.write_all(
                    include_str!("templates/Cargo.toml.feature-patch.template").as_bytes(),
                )
                .map_err(|_| Error::CreateFile("Cargo.toml".into()))?;
            }

            file.write_all(
                if softdevice.is_some() {
                    include_str!("templates/Cargo.toml.sd.append").into()
                } else {
                    format!(
                        include_str!("templates/Cargo.toml.append"),
                        family = chip.family
                    )
                }
                .as_bytes(),
            )
            .map_err(|_| Error::CreateFile("Cargo.toml".into()))?;
        }

        Ok(())
    }

    fn init_fmt(&self) -> Result<(), Error> {
        self.create_file("src/fmt.rs", include_str!("templates/fmt.rs.template"))
    }

    fn init_main(
        &self,
        family: &Family,
        panic_handler: &PanicHandler,
        softdevice: Option<&Softdevice>,
    ) -> Result<(), Error> {
        let panic_handler = to_snake_case(panic_handler.str());

        self.create_file(
            "src/main.rs",
            &match (family, softdevice) {
                (Family::STM, _) => format!(
                    include_str!("templates/main.rs.stm.template"),
                    panic_handler = panic_handler
                ),
                (Family::NRF(_), Some(_)) => {
                    format!(
                        include_str!("templates/main.rs.nrf.sd.template"),
                        panic_handler = panic_handler
                    )
                }
                (Family::NRF(_), None) => {
                    format!(
                        include_str!("templates/main.rs.nrf.template"),
                        panic_handler = panic_handler
                    )
                }
                (Family::ESP(_), _) => include_str!("templates/main.rs.esp.template").into(),
            },
        )
    }

    fn init_memory_x(&self, memory: MemRegion) -> Result<(), Error> {
        self.create_file(
            "memory.x",
            &format!(
                include_str!("templates/memory.x.template"),
                flash_origin = memory.flash_origin,
                flash_len = memory.flash_length,
                ram_origin = memory.ram_origin,
                ram_len = memory.ram_length,
            ),
        )
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
        features: Option<&[&str]>,
        optional: bool,
    ) -> Result<(), Error> {
        self.pb.set_message(format!("Cargo add: {name}"));

        let features = features.unwrap_or_default().join(",");
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
