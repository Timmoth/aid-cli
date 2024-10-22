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
[Download the latest release](https://github.com/Timmoth/aid-cli/releases)

### Build
If you'd like to build the latest version from source:
```
//Install rust https://www.rust-lang.org/tools/install
git clone https://github.com/Timmoth/aid-cli
cd aid-cli
cargo build --release
.\target\release\aid.exe
```

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
| command                | description                                                |
|------------------------|------------------------------------------------------------|
| aid http req           | Make a HTTP request                                        |
| aid http serve         | Start a dummy HTTP server                                  |
| aid ip local           | Show my local IP address                                   |
| aid ip public          | Show my public IP address                                  |
| aid ip scan            | Scan a specified IP address subnet for active ip addresses |
| aid ip status          | Try to connect to the specified IP address                 |
| aid port status        | Check if the specified port is 'open' or 'closed'.         |
| aid port scan          | Scan for open ports on a specified IP address              |
| aid cpu info           | Show CPU information                                       |
| aid cpu usage          | Monitor CPU usage                                          |
| aid mem usage          | Monitor memory usage                                       |
| aid disk info          | Show disk information                                      |
| aid network info       | Show network information                                   |
| aid network usage      | Display network usage                                      |
| aid process usage      | Display process usage                                      |
| aid json extract       | Extract a property from JSON data                          |
| aid json jwt-decode    | Decode a JWT                                               |
| aid csv search         | Sql search over csv                                        |
| aid text base64-encode | encodes a base64 string                                    |
| aid text base64-decode | decodes a base64 string                                    |
| aid text lines         | reads and prints lines from a text file                    |
| aid file info          | prints file metadata                                       |
| aid file md5           | calculates the files Md5 checksum                          |
| aid file sha1          | calculates the files Sha1 checksum                         |
| aid file sha256        | calculates the files Sha256 checksum                       |
| aid file zip           | zips the files in the source directory                     |
| aid time unix          | Display unix timestamp                                     |
| aid time dt            | Display the datetime                                       |
| aid bits board         | Display the number in bitboard representation              |
| aid bits to-bin        | Converts a number to it's binary representation            |
| aid bits to-dec        | Converts a number to it's decimal representation           |
| aid bits to-hex        | Converts a number to it's hex representation               |
| aid math eval          | Evaluates a math expression                                |
| aid math plot          | Plot a math expression                                     |
```
