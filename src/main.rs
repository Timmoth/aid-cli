use aid::{
    bits_commands, cpu_commands, csv_commands, disk_commands, file_commands, http_commands, ip_commands, json_commands, mem_commands, network_commands, port_commands, text_commands, time_commands, math_commands
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(subcommand, about = "HTTP functions")]
    Http(HttpCommands),
    #[command(subcommand, about = "IP information / scanning")]
    Ip(IpCommands),
    #[command(subcommand, about = "Port information / scanning")]
    Port(PortCommands),
    #[command(subcommand, about = "System cpu information")]
    Cpu(CpuCommands),
    #[command(subcommand, about = "System memory information")]
    Mem(MemoryCommands),
    #[command(subcommand, about = "System disk information")]
    Disk(DiskCommands),
    #[command(subcommand, about = "System network information")]
    Network(NetworkCommands),
    #[command(subcommand, about = "JSON parsing / extraction functions")]
    Json(JsonCommands),
    #[command(subcommand, about = "CSV searching / filtering")]
    Csv(CsvCommands),
    #[command(subcommand, about = "Text manipulation functions")]
    Text(TextCommands),
    #[command(subcommand, about = "File information")]
    File(FileCommands),
    #[command(subcommand, about = "Time related functions")]
    Time(TimeCommands),
    #[command(subcommand, about = "Math functions")]
    Math(MathCommands),
    #[command(subcommand, about = "Bit manipulation functions")]
    Bits(BitsCommands),
}
#[derive(Subcommand, Debug, Clone)]
enum IpCommands {
    #[command(about = "Show my local IP address")]
    Local {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue, 
               help = "Output the local IP address in JSON format.")]
        json: bool,
    },

    #[command(about = "Show my public IP address")]
    Public {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output the public IP address in JSON format.")]
        json: bool,
    },

    #[command(about = "Scan a specified IP address subnet for active ip addresses")]
    Scan {
        #[arg(
            short = 'i',
            long = "ip",
            default_value = "",
            help = "The IP subnet to scan. If not provided, the local subnet will be used."
        )]
        ip: Option<String>,

        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output scan results in JSON format.")]
        json: bool,
    },

    #[command(about = "Try to connect to the specified IP address")]
    Status {
        #[arg(
            short = 'i',
            long = "ip",
            help = "The IP address to check the status of."
        )]
        ip: String,

        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output status in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum PortCommands {
    #[command(about = "Check if the specified port is 'open' or 'closed'.")]
    Status {
        #[arg(short = 'i', long = "ip", help = "The IP address to check (optional).")]
        ip: Option<String>,

        #[arg(short = 'p', help = "The port number to check the status of.")]
        port: u16,

        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output port status in JSON format.")]
        json: bool,
    },

    #[command(about = "Scan for open ports on a specified IP address")]
    Scan {
        #[arg(short = 'i', long = "ip", help = "The IP address to scan (optional).")]
        ip: Option<String>,

        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output scan results in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum CpuCommands {
    #[command(about = "Show CPU information")]
    Info {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output CPU information in JSON format.")]
        json: bool,
    },

    #[command(about = "Monitor CPU usage")]
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
    #[command(about = "Monitor memory usage")]
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
    #[command(about = "Show disk information")]
    Info {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output disk information in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum NetworkCommands {
    #[command(about = "Show network information")]
    Info {
        #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue,
               help = "Output network information in JSON format.")]
        json: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum HttpCommands {
    #[command(about = "Make a HTTP request")]
    Req {
        #[arg(
            short = 'm',
            long = "method",
            help = "Specify the HTTP method (e.g., GET, POST)."
        )]
        method: Option<String>,

        #[arg(
            short = 'u',
            long = "url",
            help = "Specify the URL for the HTTP request."
        )]
        url: Option<String>,

        #[arg(
            short = 'c',
            long = "config",
            help = "Path to a configuration file for the request. Specify: method, url, body, headers in json format."
        )]
        config: Option<String>,

        #[arg(
            short = 'o',
            long = "output",
            help = "If specified saves http response body to a file at the given path."
        )]
        output: Option<String>,
    },

    #[command(about = "Start a HTTP server (GET: 0.0.0.0:80 -> 'Hello, World!')")]
    Serve {
        #[arg(
            short = 'p',
            long = "port",
            default_value = "80",
            help = "Specify the port for the HTTP server (default is 80)."
        )]
        port: u16,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum JsonCommands {
    #[command(about = "Extract a property from JSON data")]
    Extract {
        #[arg(
            short = 'p',
            long = "prop",
            help = "Specify the property to extract from the JSON."
        )]
        property: String,
    },
    #[command(about = "Decode a JWT")]
    JwtDecode {
        #[arg(short = 'j', long = "jwt", help = "Specify JWT to decode.")]
        jwt: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum CsvCommands {
    #[command(about = "Sql search over csv")]
    Search {
        #[arg(short = 's', long = "sql", help = "Sql query.")]
        sql: String,
        #[arg(short = 'o', long = "output", help = "Output file path.")]
        output: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum TextCommands {
    #[command(about = "base64 encode")]
    Base64Encode {
        #[arg(short = 'i', long = "input", help = "Input text to base64 encode.")]
        input: String,
    },

    #[command(about = "base64 decode")]
    Base64Decode {
        #[arg(short = 'i', long = "input", help = "Input text to base 64 decode.")]
        input: String,
    },

    #[command(about = "search a text file for lines that match a regex")]
    Regex {
        #[arg(short = 'f', long = "file", help = "Input text file to search.")]
        file: String,
        #[arg(short = 'r', long = "regex", help = "regex search pattern.")]
        regex: String,
    },

    #[command(about = "print the specified range of lines.")]
    Lines {
        #[arg(short = 'i', long = "input", help = "Input text file to search.")]
        file: String,
        #[arg(short = 's', long = "start", help = "first line to print")]
        start: Option<usize>,
        #[arg(short = 'e', long = "end", help = "last line to print")]
        end: Option<usize>,
        #[arg(short = 'f', long = "first", help = "number of lines from the start of the file to print")]
        head: Option<usize>,
        #[arg(short = 'l', long = "last", help = "number of lines from the end of the file to print")]
        tail: Option<usize>,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum FileCommands {
    #[command(about = "prints file metadata")]
    Info {
        #[arg(short = 'f', long = "file", help = "Input file.")]
        file: String,
    },
    #[command(about = "calculate a files Md5 checksum")]
    Md5 {
        #[arg(short = 'f', long = "file", help = "Input file.")]
        file: String,
    },
    #[command(about = "calculate a files Sha1 checksum")]
    Sha1 {
        #[arg(short = 'f', long = "file", help = "Input file.")]
        file: String,
    },
    #[command(about = "calculate a files Sha256 checksum")]
    Sha256 {
        #[arg(short = 'f', long = "file", help = "Input file.")]
        file: String,
    },

    #[command(about = "zips the files in the source directory")]
    Zip {
        #[arg(short = 'd', long = "dir", help = "source directory.")]
        dir: String,
        #[arg(short = 'f', long = "file", help = "output zip file.")]
        file: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum TimeCommands {
    #[command(about = "Display unix timestamp")]
    Unix {
        #[arg(short = 'm', long = "milli", action = clap::ArgAction::SetTrue,
               help = "Output the timestamp as unix milliseconds.")]
        milli: bool,
    },
    #[command(about = "Display the datetime")]
    Dt{
        #[arg(short = 'l', long = "local", action = clap::ArgAction::SetTrue,
        help = "Use the local datetime.")]
        local: bool,
        #[arg(short = 'r', long = "rfc", action = clap::ArgAction::SetTrue,
        help = "Output the datetime in Rfc3339 format.")]
        rfc: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum MathCommands {
    #[command(about = "Evaluates a math expression")]
    Eval {
        #[arg(short='e', long = "exp", help = "Math expression to evaluate.")]
        expression: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum BitsCommands {
    #[command(about = "Display the number in bitboard representation")]
    Board {
        #[arg(short = 'b', long = "bin", help = "Display the binary value as a bitboard.")]
        binary: Option<String>,
        #[arg(short = 'd', long = "dec", help = "Display the decimal value as a bitboard.")]
        decimal: Option<u64>,
        #[arg(long = "hex", help = "Display the decimal value as a bitboard.")]
        hex: Option<String>,
    },
    #[command(about = "Converts a number to it's binary representation")]
    ToBin {
        #[arg(short = 'd', long = "dec", help = "Convert the decimal number to binary.")]
        decimal: Option<u64>,
        #[arg(long = "hex", help = "Converts the hex number to binary.")]
        hex: Option<String>,
    },
    #[command(about = "Converts a number to it's decimal representation")]
    ToDec {
        #[arg(short = 'b', long = "bin", help = "Converts the binary number to hedecimalx.")]
        bin: Option<String>,
        #[arg(long = "hex", help = "Converts the hex number to decimal.")]
        hex: Option<String>,
    },
    #[command(about = "Converts a number to it's hex representation")]
    ToHex {
        #[arg(short = 'd', long = "dec", help = "Convert the decimal number to hex.")]
        decimal: Option<u64>,
        #[arg(short = 'b', long = "bin", help = "Converts the binary number to hex.")]
        bin: Option<String>,
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
            CpuCommands::Info { json } => cpu_commands::system_cpu_info(json),
            CpuCommands::Usage { watch, json } => cpu_commands::system_cpu_usage(watch, json).await,
        },
        Commands::Mem(sub_command) => match sub_command {
            MemoryCommands::Usage { watch, json } => {
                mem_commands::system_mem_usage(watch, json).await
            }
        },
        Commands::Disk(sub_command) => match sub_command {
            DiskCommands::Info { json } => disk_commands::system_disk_info(json).await,
        },
        Commands::Network(sub_command) => match sub_command {
            NetworkCommands::Info { json } => network_commands::system_network_info(json).await,
        },
        Commands::Http(sub_command) => match sub_command {
            HttpCommands::Req {
                method,
                url,
                config,
                output,
            } => http_commands::http_request(method, url, config, output).await,
            HttpCommands::Serve { port } => http_commands::http_serve(port).await,
        },

        Commands::Json(sub_command) => match sub_command {
            JsonCommands::Extract { property } => json_commands::json_extract(property).await,
            JsonCommands::JwtDecode { jwt } => json_commands::json_decode_jwt(&jwt),
        },

        Commands::Csv(sub_command) => match sub_command {
            CsvCommands::Search { sql, output } => csv_commands::sql_search(sql, output).await,
        },

        Commands::Text(sub_command) => match sub_command {
            TextCommands::Base64Encode { input } => text_commands::base64_encode(input),
            TextCommands::Base64Decode { input } => text_commands::base64_decode(input),
            TextCommands::Regex { file, regex } => text_commands::regex_search(file, regex),
            TextCommands::Lines { file, start, end, head, tail } => text_commands::print_lines(file, start, end, head, tail),
        },

        Commands::File(sub_command) => match sub_command {
            FileCommands::Info { file } => file_commands::file_info(file),
            FileCommands::Md5 { file } => file_commands::md5_checksum(file),
            FileCommands::Sha1 { file } => file_commands::sha1_checksum(file),
            FileCommands::Sha256 { file } => file_commands::sha256_checksum(file),
            FileCommands::Zip { dir, file } => file_commands::zip_directory(dir, file),

        },
        Commands::Time(sub_command) => match sub_command {
            TimeCommands::Unix { milli } => time_commands::unix_timestamp(milli),
            TimeCommands::Dt { local, rfc } => time_commands::date_time(local, rfc),
        },
        Commands::Math(sub_command) => match sub_command {
            MathCommands::Eval { expression } => math_commands::evaluate(expression),

        },
          Commands::Bits(sub_command) => match sub_command{
                BitsCommands::Board { binary, decimal, hex } => bits_commands::bitboard(binary, decimal, hex),
                BitsCommands::ToBin { decimal, hex } => bits_commands::to_binary(decimal, hex),
                BitsCommands::ToDec { bin, hex } => bits_commands::to_dec(bin, hex),
                BitsCommands::ToHex { decimal, bin } => bits_commands::to_hex(decimal, bin),            
            }
    }
}
