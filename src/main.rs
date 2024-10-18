use clap::{Parser, Subcommand};

mod cpu_commands;
mod format_utils;
mod ip_commands;
mod ip_utils;
mod mem_commands;
mod port_commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(subcommand)]
    Ip(IpCommands),
    #[command(subcommand)]
    Port(PortCommands),
    #[command(subcommand)]
    Cpu(CpuCommands),
    #[command(subcommand)]
    Mem(MemoryCommands),
}

#[derive(Subcommand, Debug, Clone)]
enum IpCommands {
    Local {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
    Public {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
    Scan {
        #[arg(short = 'i', long="ip", default_value = "")]
        ip: Option<String>,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
    Status {
        #[arg(short = 'i', long="ip")]
        ip: String,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum PortCommands {
    Status {
        #[arg(short = 'i', long="ip")]
        ip: String,
        #[arg(short = 'p')]
        port: u16,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
    Scan {
        #[arg(short = 'i', long="ip")]
        ip: String,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum CpuCommands {
    Details,
    Usage {
        #[arg(short = 'w', long = "watch", action = clap::ArgAction::SetTrue)]
        watch: bool,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum MemoryCommands {
    Usage {
        #[arg(short = 'w', long = "watch", action = clap::ArgAction::SetTrue)]
        watch: bool,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Ip(command) => match command {
            IpCommands::Local { json } => ip_commands::ip_local(json).await,
            IpCommands::Public { json } => ip_commands::ip_public(json).await,
            IpCommands::Scan { ip, json } => ip_commands::ip_scan(ip, json).await,
            IpCommands::Status { ip, json } => ip_commands::ip_status(ip, json).await,
        },
        Commands::Port(command) => match command {
            PortCommands::Scan { ip, json } => port_commands::port_scan(ip, json).await,
            PortCommands::Status { ip, port, json } => {
                port_commands::port_status(ip, port, json).await
            }
        },
        Commands::Cpu(sub_command) => match sub_command {
            CpuCommands::Details => cpu_commands::system_cpu_details(),
            CpuCommands::Usage { watch, json } => cpu_commands::system_cpu_usage(watch, json).await,
        },
        Commands::Mem(sub_command) => match sub_command {
            MemoryCommands::Usage { watch, json } => {
                mem_commands::system_mem_usage(watch, json).await
            }
        },
    }
}
