### aid process usage
```
  aid process usage  Display process information.
                    -f, --filter <FILTER>  filter the results by process name regex.
                    -s, --sort <SORT_BY>   Sort the results by [cpu, mem, disk]
                    -l, --limit <LIMIT>    Limit the number of results to display.
                    -w, --watch            Continuously monitor the processes.

-----input-----
aid process usage -w -s mem -l 10 -f host
-----output-----
[pid]   [cpu %]      [mem]       [disk read]    [disk write]    name
[5640]  1.20  %      44 MB             0 B/s           0 B/s    svchost.exe
[1568]  0.00  %      42 MB             0 B/s           0 B/s    svchost.exe
[5632]  0.00  %      30 MB             0 B/s           0 B/s    svchost.exe
[9348]  0.00  %      20 MB             0 B/s           0 B/s    svchost.exe
[14736] 0.00  %      20 MB             0 B/s           0 B/s    svchost.exe
[9388]  0.00  %      19 MB             0 B/s           0 B/s    svchost.exe
[9244]  0.00  %      19 MB             0 B/s           0 B/s    sihost.exe
[8396]  0.00  %      19 MB             0 B/s           0 B/s    svchost.exe
[1736]  0.00  %      17 MB             0 B/s           0 B/s    svchost.exe
[37596] 0.00  %      16 MB             0 B/s           0 B/s    taskhostw.exe
```