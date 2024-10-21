### aid port status
```
  aid port status  Check if the specified port is 'open' or 'closed'.
            -i, --ip <IP>  The IP address to check (optional).
            -p <PORT>      The port number to check the status of.
            -j, --json     Output port status in JSON format.

-----input-----
aid port status -i 192.168.0.1 -p 80
-----output-----
open
```
### aid port scan
```
  aid port scan    Scan for open ports on a specified IP address
            -i, --ip <IP>  The IP address to scan (optional).
            -j, --json     Output scan results in JSON format.

-----input-----
aid port scan -i 192.168.0.1
-----output-----
80
443
22
```