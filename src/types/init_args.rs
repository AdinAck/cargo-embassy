pub mod panic_handler;
pub mod soft_device;

use clap::Args;
use panic_handler::PanicHandler;
use soft_device::Softdevice;

#[derive(Debug, Clone, Args)]
pub struct InitArgs {
    #[arg(help = "The name of the Embassy project to create.")]
    pub name: String,

    #[arg(long = "chip", help = "Specifies the target chip.")]
    pub chip_name: String,

    #[arg(value_enum, long, help = "Selects the panic handler.", default_value_t = PanicHandler::Halt)]
    pub panic_handler: PanicHandler,

    #[arg(long, help = "Configure for use with a Softdevice (NRF only).")]
    pub softdevice: Option<Softdevice>,
}
