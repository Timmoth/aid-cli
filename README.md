<p align="center">
   <div style="width:640;height:320">
       <img style="width: inherit" src="./banner.png">
</div>
</p>
A CLI toolkit featuring a variety of helpful utilities.

```
aid math plot --start -20 --end 20 --step 0.5 --exp "x * sin(1 - x)"
                        |                      *
    *                   |                     **
   ***                  |                     **
   * *                  |                    ***
   * *                  |             **     * **
   * *     **           |             ***    *  *
   * *     ***          |             * *    *  *
  **  *   ** *          |     ***    *  *    *  *
  *   *   *  *    ***   |     * *    *  *    *  *
  *   *   *  **   * **  |    **  *   *  **   *  *
--*---*---*---*--**--******-**---*---*---*--*---*-
  *   *  **   *  *      | ***    *  *    *  *   **
  *   *  *    ****      |         * *    *  *
  *    * *     **       |         * *    *  *
 **    * *              |         ***    ** *
 *     ***              |                 ***
 *     **               |                 **
 *                      |                 **
**                      |
**                      |
```
## Read the [docs](https://timmoth.github.io/aid-cli/)
for all supported commands, parameters, and examples

## Installation

Manual installation is simple, just download the release and add it to your PATH environment variable, if you'd like an even easier way i've added scripts to do it for you.

### Linux / Mac (apple silicon supported)
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
[Download the latest release](https://github.com/Timmoth/aid-cli/releases)

## Top level commands:
```
 | command     | description                                               |
 |-------------|-----------------------------------------------------------|
 | aid http    | HTTP functions                                            |
 | aid ip      | IP information / scanning                                 |
 | aid port    | Port information / scanning                               |
 | aid cpu     | System cpu information                                    |
 | aid mem     | System memory information                                 |
 | aid disk    | System disk information                                   |
 | aid network | System network information                                |
 | aid json    | JSON parsing / extraction functions                       |
 | aid csv     | CSV search / transformation functions                     |
 | aid text    | Text manipulation functions                               |
 | aid file    | File info functions                                       |
 | aid time    | Time related functions                                    |
 | aid bits    | Bit manipulation functions                                |
 | aid math    | Math functions                                            |
 | aid process | Process monitoring functions                              |
 | aid help    | Print this message or the help of the given subcommand(s) |
```
## All commands:

```
| version   | command                | description                                                |
|-----------|------------------------|------------------------------------------------------------|
| [u] 0.1.3 | aid http req           | Make a HTTP request                                        |
| [u] 0.1.3 | aid http serve         | Start a dummy HTTP server                                  |
| [u] 0.1.3 | aid ip local           | Show my local IP address                                   |
| [u] 0.1.3 | aid ip public          | Show my public IP address                                  |
| [u] 0.1.3 | aid ip scan            | Scan a specified IP address subnet for active ip addresses |
| [u] 0.1.3 | aid ip status          | Try to connect to the specified IP address                 |
| [u] 0.1.3 | aid port status        | Check if the specified port is 'open' or 'closed'.         |
| [u] 0.1.3 | aid port scan          | Scan for open ports on a specified IP address              |
| [u] 0.1.3 | aid cpu info           | Show CPU information                                       |
| [u] 0.1.6 | aid cpu usage          | Monitor CPU usage                                          |
| [u] 0.1.6 | aid mem usage          | Monitor memory usage                                       |
| [u] 0.1.3 | aid disk info          | Show disk information                                      |
| [u] 0.1.3 | aid network info       | Show network information                                   |
| [u] 0.1.3 | aid network usage      | Display network usage                                      |
| [a] 0.1.7 | aid process usage      | Display process usage                                      |
| [u] 0.1.3 | aid json extract       | Extract a property from JSON data                          |
| [u] 0.1.3 | aid json jwt-decode    | Decode a JWT                                               |
| [u] 0.1.3 | aid csv search         | Sql search over csv                                        |
| [u] 0.1.3 | aid text base64-encode | encodes a base64 string                                    |
| [u] 0.1.3 | aid text base64-decode | decodes a base64 string                                    |
| [u] 0.1.3 | aid text lines         | reads and prints lines from a text file                    |
| [u] 0.1.3 | aid file info          | prints file metadata                                       |
| [u] 0.1.3 | aid file md5           | calculates the files Md5 checksum                          |
| [u] 0.1.3 | aid file sha1          | calculates the files Sha1 checksum                         |
| [u] 0.1.3 | aid file sha256        | calculates the files Sha256 checksum                       |
| [a] 0.1.4 | aid file zip           | zips the files in the source directory                     |
| [u] 0.1.9 | aid time unix          | Display unix timestamp                                     |
| [u] 0.1.9 | aid time dt            | Display the datetime                                       |
| [a] 0.1.9 | aid time chron         | Describes a chron job                                      |
| [a] 0.1.9 | aid time count-down    | Starts a countdown timer                                   |
| [a] 0.1.8 | aid bits eval          | Bitwise expression evaluation / conversion / information   |
| [u] 0.1.3 | aid math eval          | Evaluates a math expression                                |
| [u] 0.1.3 | aid math plot          | Plot a math expression                                     |

[a] added in version x.x.x
[u] lasted updated in version x.x.x
[p] last patched in version x.x.x
[d] deprecated in version x.x.x
```

### Build
If you'd like to build the latest version from source:
```
//Install rust https://www.rust-lang.org/tools/install
git clone https://github.com/Timmoth/aid-cli
cd aid-cli
cargo build --release
.\target\release\aid.exe
```