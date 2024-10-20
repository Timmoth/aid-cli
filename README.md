# aid-cli
A CLI toolkit featuring a variety of helpful utilities.

## Installation

Manual installation is simple, just download the release and add it to your PATH environment variable, if you'd like an even easier way i've added scripts to do it for you.

### Linux / Mac
```
wget https://raw.githubusercontent.com/Timmoth/aid-cli/refs/heads/main/install.sh
chmod +x install.sh
sudo ./install.sh
```
### Windows (powershell)
```
Invoke-WebRequest -Uri https://raw.githubusercontent.com/Timmoth/aid-cli/refs/heads/main/install.ps1 -OutFile install.ps1
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
.\install.ps1
```

### Releases
[Download the latest release v0.1.0](https://github.com/Timmoth/aid-cli/releases/tag/aid-0.1.0)

### Build
If you'd like to build the latest version from source:
```
//Install rust https://www.rust-lang.org/tools/install
git clone https://github.com/Timmoth/aid-cli
cd aid-cli
cargo build --release
.\target\release\aid.exe
```

## Commands:
```
  aid http     HTTP functions
  aid ip       IP information / scanning
  aid port     Port information / scanning
  aid cpu      System cpu information
  aid mem      System memory information
  aid disk     System disk information
  aid network  System network information
  aid json     JSON parsing / extraction functions
  aid csv      CSV search / transformation functions
  aid help     Print this message or the help of the given subcommand(s)
```

## aid http
```
  aid http req    Make a HTTP request
            -m, --method <METHOD>  Specify the HTTP method (e.g., GET, POST).
            -u, --url <URL>        Specify the URL for the HTTP request.
            -c, --config <CONFIG>  Path to a configuration file for the request. Specify: method, url, body, headers in json format.
            -o, --output <OUTPUT>  If specified saves http response body to a file at the given path.
            
  aid http serve  Start a HTTP server (GET: 0.0.0.0:80 -> 'Hello, World!')
            -p, --port <PORT>  Specify the port for the HTTP server (default is 80). [default: 80]
```

## aid ip 
```
  aid ip local   Show my local IP address
            -j, --json  Output the local IP address in JSON format.

  aid ip public  Show my public IP address
            -j, --json  Output the local IP address in JSON format.

  aid ip scan    Scan a specified IP address subnet for active ip addresses
            -i, --ip <IP>  The IP subnet to scan. If not provided, the local subnet will be used. [default: ]        
            -j, --json     Output scan results in JSON format.

  aid ip status  Try to connect to the specified IP address
            -i, --ip <IP>  The IP address to check the status of.
            -j, --json     Output status in JSON format.
```

## aid port
```
  aid port status  Check if the specified port is 'open' or 'closed'.
            -i, --ip <IP>  The IP address to check (optional).
            -p <PORT>      The port number to check the status of.
            -j, --json     Output port status in JSON format.

  aid port scan    Scan for open ports on a specified IP address
            -i, --ip <IP>  The IP address to scan (optional).
            -j, --json     Output scan results in JSON format.
```
## aid cpu
```
  aid cpu info   Show CPU information
            -j, --json  Output CPU information in JSON format.

  aid cpu usage  Monitor CPU usage
            -w, --watch  Continuously monitor CPU usage.
            -j, --json   Output CPU usage in JSON format.
```
## aid mem
```
  aid mem usage  Monitor memory usage
            -w, --watch  Continuously monitor memory usage.
            -j, --json   Output memory usage in JSON format.
```
## aid disk
```
  aid disk info  Show disk information
            -j, --json  Output disk information in JSON format.
```
## aid network
```
  aid network info  Show network information
            -j, --json  Output network information in JSON format.
```

## aid json
```
  aid json extract      Extract a property from JSON data
            -p, --prop <PROPERTY>  Specify the property to extract from the JSON.
  aid json jwt-decode   Decode a JWT
            -j, --jwt <PROPERTY>  Specify JWT to decode.
```

## aid csv
```
  aid csv search  Sql search over csv
            -s, --sql <SQL>        Sql query e.g SELECT 'first name',age FROM people.csv WHERE age >= 25 AND age < 30 ORDER BY 'age' ASC.
            -o, --output <OUTPUT>  Output file path.
```