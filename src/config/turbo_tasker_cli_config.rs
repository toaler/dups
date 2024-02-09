use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "tt", version = "0.1.0", author = "toaler", about = "Turbo Tasker - Keeping PC's organized since 2024!")]
pub struct TurboTaskerApp {
    #[command(subcommand)]
    pub cmd: Option<Command>
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Disk {
        #[arg(long = "duplicate_detection", required = false, short = 'd', help = "enable file duplicate detection")]
        duplicate_detection: bool,

        #[arg(long = "root", short = 'r', required = true, help = "root path to start resource analysis")]
        root: String
    },
    Cpu,
}