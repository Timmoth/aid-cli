### aid network info
```
  aid network info  Show network information
            -j, --json  Output network information in JSON format.

-----input-----
aid network info -j
-----output-----
[
  {
    "name": "WiFi",
    "transmitted": 0.2,
    "received": 1.5,
    "mac": "00:00:00:00:00:00"
  }
]
```

```
  aid network usage  Display network usage
            -w, --watch  Continuously monitor network usage.

-----input-----
aid network usage -w
-----output-----
WiFi: rx 1.05 kB/s, tx 0 B/s
WiFi-QoS Packet Scheduler-0000: rx 1.05 kB/s, tx 0 B/s
```