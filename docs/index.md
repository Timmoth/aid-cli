![Banner](assets/banner.png)

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
 | aid env     | Environment information                                   |
 | aid help    | Print this message or the help of the given subcommand(s) |
```
## All commands:

```
| version    | command                | description                                                |
|------------|------------------------|------------------------------------------------------------|
| [u] 0.1.3  | aid http req           | Make a HTTP request                                        |
| [u] 0.1.3  | aid http serve         | Start a dummy HTTP server                                  |
| [u] 0.1.3  | aid ip local           | Show my local IP address                                   |
| [u] 0.1.3  | aid ip public          | Show my public IP address                                  |
| [u] 0.1.3  | aid ip scan            | Scan a specified IP address subnet for active ip addresses |
| [u] 0.1.3  | aid ip status          | Try to connect to the specified IP address                 |
| [u] 0.1.3  | aid port status        | Check if the specified port is 'open' or 'closed'.         |
| [u] 0.1.3  | aid port scan          | Scan for open ports on a specified IP address              |
| [u] 0.1.3  | aid cpu info           | Show CPU information                                       |
| [u] 0.1.6  | aid cpu usage          | Monitor CPU usage                                          |
| [u] 0.1.6  | aid mem usage          | Monitor memory usage                                       |
| [u] 0.1.3  | aid disk info          | Show disk information                                      |
| [u] 0.1.3  | aid network info       | Show network information                                   |
| [u] 0.1.3  | aid network usage      | Display network usage                                      |
| [a] 0.1.7  | aid process usage      | Display process usage                                      |
| [u] 0.1.3  | aid json extract       | Extract a property from JSON data                          |
| [u] 0.1.3  | aid json jwt-decode    | Decode a JWT                                               |
| [u] 0.1.3  | aid csv search         | Sql search over csv                                        |
| [u] 0.1.3  | aid text base64-encode | encodes a base64 string                                    |
| [u] 0.1.3  | aid text base64-decode | decodes a base64 string                                    |
| [a] 0.1.10 | aid text url-encode    | url encodes a string                                       |
| [a] 0.1.10 | aid text url-decode    | decodes a url encoded string                               |
| [u] 0.1.3  | aid text lines         | reads and prints lines from a text file                    |
| [a] 0.1.10 | aid text guid          | Generates a random GUID                                    |
| [u] 0.1.3  | aid file info          | prints file metadata                                       |
| [u] 0.1.3  | aid file md5           | calculates the files Md5 checksum                          |
| [u] 0.1.3  | aid file sha1          | calculates the files Sha1 checksum                         |
| [u] 0.1.3  | aid file sha256        | calculates the files Sha256 checksum                       |
| [a] 0.1.4  | aid file zip           | zips the files in the source directory                     |
| [u] 0.1.9  | aid time unix          | Display unix timestamp                                     |
| [u] 0.1.9  | aid time dt            | Display the datetime                                       |
| [a] 0.1.9  | aid time chron         | Describes a chron job                                      |
| [a] 0.1.9  | aid time count-down    | Starts a countdown timer                                   |
| [a] 0.1.8  | aid bits eval          | Bitwise expression evaluation / conversion / information   |
| [u] 0.1.10 | aid math eval          | Evaluates a math expression                                |
| [u] 0.1.10 | aid math plot          | Plot a math expression                                     |
| [a] 0.1.10 | aid env vars           | Filter / Display environment variables                     |

[a] added in version x.x.x
[u] updated in version x.x.x
[p] patched in version x.x.x
[d] deprecated in version x.x.x
```

# Support 🛟

Need help? Ping me on [linkedin](https://www.linkedin.com/in/timmoth/) and I'd be more then happy to jump on a call to debug, help configure or answer any questions.
