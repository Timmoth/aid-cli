## aid ip 
### aid ip local
```
  aid ip local   Show my local IP address
            -j, --json  Output the local IP address in JSON format.

-----input-----
aid ip local
-----output-----
192.168.0.10
```

### aid ip public
```
  aid ip public  Show my public IP address
            -j, --json  Output the local IP address in JSON format.

-----input-----
aid ip public
-----output-----
1.2.3.4
```
### aid ip scan
```
  aid ip scan    Scan a specified IP address subnet for active ip addresses
            -i, --ip <IP>  The IP subnet to scan. If not provided, the local subnet will be used. [default: ]        
            -j, --json     Output scan results in JSON format.

-----input-----
aid ip scan
-----output-----
1.2.3.0
1.2.3.2
1.2.3.4
```
### aid ip status
```
  aid ip status  Try to connect to the specified IP address
            -i, --ip <IP>  The IP address to check the status of.
            -j, --json     Output status in JSON format.

-----input-----
aid ip status -i 1.2.3.2
-----output-----
online
```