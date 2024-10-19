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
    #[command(subcommand, about="HTTP functions")]
    Http(HttpCommands),
    #[command(subcommand, about="IP information / scanning")]
    Ip(IpCommands),
    #[command(subcommand, about="Port information / scanning")]
    Port(PortCommands),
    #[command(subcommand, about="System cpu information")]
    Cpu(CpuCommands),
    #[command(subcommand, about="System memory information")]
    Mem(MemoryCommands),
    #[command(subcommand, about="System disk information")]
    Disk(DiskCommands),
    #[command(subcommand, about="System network information")]
    Network(NetworkCommands),
    #[command(subcommand, about="JSON parsing / extraction functions")]
    Json(JsonCommands),
}
#[derive(Subcommand, Debug, Clone)]
enum IpCommands {
    #[command(about="Show my local IP address")]
    Local {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue, 
               help = "Output the local IP address in JSON format.")]
        json: bool,
    },
    
    #[command(about="Show my public IP address")]
    Public {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output the public IP address in JSON format.")]
        json: bool,
    },
    
    #[command(about="Scan a specified IP address subnet for active ip addresses")]
    Scan {
        #[arg(short = 'i', long = "ip", default_value = "",
               help = "The IP subnet to scan. If not provided, the local subnet will be used.")]
        ip: Option<String>,
        
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output scan results in JSON format.")]
        json: bool,
    },

    #[command(about="Try to connect to the specified IP address")]
    Status {
        #[arg(short = 'i', long = "ip", 
               help = "The IP address to check the status of.")]
        ip: String,
        
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output status in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum PortCommands {
    #[command(about="Check if the specified port is 'open' or 'closed'.")]
    Status {
        #[arg(short = 'i', long = "ip", 
               help = "The IP address to check (optional).")]
        ip: Option<String>,
        
        #[arg(short = 'p', 
               help = "The port number to check the status of.")]
        port: u16,
        
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output port status in JSON format.")]
        json: bool,
    },
    
    #[command(about="Scan for open ports on a specified IP address")]
    Scan {
        #[arg(short = 'i', long = "ip", 
               help = "The IP address to scan (optional).")]
        ip: Option<String>,
        
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output scan results in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum CpuCommands {
    #[command(about="Show CPU information")]
    Info {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output CPU information in JSON format.")]
        json: bool,
    },
    
    #[command(about="Monitor CPU usage")]
    Usage {
        #[arg(short = 'w', long = "watch", action = clap::ArgAction::SetTrue,
               help = "Continuously monitor CPU usage.")]
        watch: bool,
        
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output CPU usage in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum MemoryCommands {
    #[command(about="Monitor memory usage")]
    Usage {
        #[arg(short = 'w', long = "watch", action = clap::ArgAction::SetTrue,
               help = "Continuously monitor memory usage.")]
        watch: bool,
        
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output memory usage in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum DiskCommands {
    #[command(about="Show disk information")]
    Info {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output disk information in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum NetworkCommands {
    #[command(about="Show network information")]
    Info {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output network information in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum HttpCommands {
    #[command(about="Make a HTTP request")]
    Req {
        #[arg(short = 'm', long = "method", 
               help = "Specify the HTTP method (e.g., GET, POST).")]
        method: Option<String>,
        
        #[arg(short = 'u', long = "url", 
               help = "Specify the URL for the HTTP request.")]
        url: Option<String>,
        
        #[arg(short = 'c', long = "config", 
               help = "Path to a configuration file for the request. Specify: method, url, body, headers in json format.")]
        config: Option<String>,

        #[arg(short = 'o', long = "output", 
        help = "If specified saves http response body to a file at the given path.")]
        output: Option<String>,
    },
    
    #[command(about="Start a HTTP server (GET: 0.0.0.0:80 -> 'Hello, World!')")]
    Serve {
        #[arg(short = 'p', long = "port", default_value = "80", 
               help = "Specify the port for the HTTP server (default is 80).")]
        port: u16,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum JsonCommands {
    #[command(about="Extract a property from JSON data")]
    Extract {
        #[arg(short = 'p', long = "prop", 
               help = "Specify the property to extract from the JSON.")]
        property: String,
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
            HttpCommands::Req { method, url, config, output } => {
                http_commands::http_request(method, url, config, output).await
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
