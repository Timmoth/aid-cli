use clap::{Parser, Subcommand};

mod cpu_commands;
mod format_utils;
mod ip_commands;
mod ip_utils;
mod mem_commands;
mod port_commands;
mod disk_commands;
mod network_commands;
mod http_commands;
mod json_commands;

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
    #[command(subcommand)]
    Disk(DiskCommands),
    #[command(subcommand)]
    Network(NetworkCommands),
    #[command(subcommand)]
    Http(HttpCommands),
    #[command(subcommand)]
    Json(JsonCommands),
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
        #[arg(short = 'i', long = "ip", default_value = "")]
        ip: Option<String>,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
    Status {
        #[arg(short = 'i', long = "ip")]
        ip: String,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum PortCommands {
    Status {
        #[arg(short = 'i', long = "ip")]
        ip: Option<String>,
        #[arg(short = 'p')]
        port: u16,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
    Scan {
        #[arg(short = 'i', long = "ip")]
        ip: Option<String>,
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum CpuCommands {
    Info {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
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

#[derive(Subcommand, Debug, Clone)]
enum DiskCommands {
    Info {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum NetworkCommands {
    Info {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum HttpCommands {
    Get {
        #[arg(short = 'u', long = "url")]
        url: String,
    },
    Serve{
        #[arg(short = 'p', long = "port", default_value = "80")]
        port: u16,
    }
}

#[derive(Subcommand, Debug, Clone)]
enum JsonCommands {
    Extract{
        #[arg(short = 'p', long = "prop")]
        property: String
    }
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
            CpuCommands::Info {json } => cpu_commands::system_cpu_info(json),
            CpuCommands::Usage { watch, json } => cpu_commands::system_cpu_usage(watch, json).await,
        },
        Commands::Mem(sub_command) => match sub_command {
            MemoryCommands::Usage { watch, json } => {
                mem_commands::system_mem_usage(watch, json).await
            }
        },
        Commands::Disk(sub_command) => match sub_command {
            DiskCommands::Info { json } => {
                disk_commands::system_disk_info(json).await
            }
        },
        Commands::Network(sub_command) => match sub_command {
            NetworkCommands::Info { json } => {
                network_commands::system_network_info(json).await
            }
        },
        Commands::Http(sub_command) => match sub_command {
            HttpCommands::Get { url } => {
                http_commands::http_get_request(url).await
            },
            HttpCommands::Serve { port } => {
                http_commands::http_serve(port).await
            }
        },

        Commands::Json(sub_command) => match sub_command {
            JsonCommands::Extract { property } => {
                json_commands::json_extract(property).await
            },
        },
    }
}
