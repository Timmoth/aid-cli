## aid cpu

### aid cpu info
```
  aid cpu info   Show CPU information
            -j, --json  Output CPU information in JSON format.

-----input-----
aid cpu info

-----output-----
AMD Ryzen 9 5900HX with Radeon Graphics
```

### aid cpu usage
```
  aid cpu usage  Monitor CPU usage
            -w, --watch  Continuously monitor CPU usage.
            -j, --json   Output CPU usage in JSON format.

-----input-----
aid cpu usage -j

-----output-----
{
  "total": 33.4,
  "core_usage": [
    54.3,
    17.0,
    60.0,
    21.4,
    47.7,
    20.9,
    43.6,
    24.7,
    39.2,
    22.9,
    36.9,
    22.1,
    42.9,
    18.9,
    40.6,
    20.6
  ]
}
```