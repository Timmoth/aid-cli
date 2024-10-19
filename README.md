# aid-cli
A CLI toolkit featuring a variety of helpful utilities.

## Networking
| command          | options                          | outputs                                            |
|------------------|----------------------------------|----------------------------------------------------|
| aid ip local     | -j, --json                       | your local ip address                              |
| aid ip public    | -j, --json                       | your public ip address                             |
| aid ip scan      | -i, --ip, -j, --json             | scans for all hosts for on given subnet            |
| aid ip status    | -i, --ip, -j, --json             | checks if you can connect to a given ip            |
| aid port scan    | -i, --ip, -j, --json             | returns all open ports for a given ip              |
| aid port status  | -i, --ip, -p, --port, -j, --json | checks if a given ip / port is open                |
| aid network info | -j, --json                       | network device info                                |
| aid http get     | -u --url                         | http response body text                            |
| aid http serve   | -u --url                         | runs a simple http webserver at the specified port |

## System

| command       | options                 | outputs              |
|---------------|-------------------------|----------------------|
| aid cpu usage | -w, --watch, -j, --json | system cpu usage     |
| aid mem usage | -w, --watch, -j, --json | sytstem memory usage |
| aid cpu into  | -j, --json              | cpu info             |
| aid disk into | -j, --json              | disk info            |

## Text

| command          | options    | outputs                                                              |
|------------------|------------|----------------------------------------------------------------------|
| aid json extract | -p, --prop | extracts the value from the specified property of a piped json input |