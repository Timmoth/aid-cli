### aid time chron

```
  aid time chron <EXPRESSION> Describes a chron job

-----input-----
aid time chron 0 30 3 ? * 2#3 *
-----output-----
At 3:30 AM, on the third Tuesday of the month
```    

### aid time count-down

```
  aid time count-down <MM:SS> Starts a countdown timer

-----input-----
aid time count-down 10:30
-----output-----
Time left: 10 minutes 30 seconds
```         

### aid time unix
```
  aid time unix  Display unix timestamp
            -d, --dt  Use the specified datetime.
            -m, --milli  Output the timestamp as unix milliseconds.

-----input-----
aid time unix 
-----output-----
1729546694

-----input-----
aid time unix -m
-----output-----
1729715103358

-----input-----
aid time unix -d 'Fri, 14 Jul 2017 02:40:00 +0000'
-----output-----
1500000000
```          

### aid time dt
```
  aid time dt  Display the datetime
            -l, --local  Use the local datetime.
            -u, --unix  Use the specified unix second timestamp.
            -r, --rfc   Output the datetime in Rfc3339 format.

-----input-----
aid time dt
-----output-----
2024-10-21 21:38:34

-----input-----
aid time dt -l
-----output-----
2024-10-21 22:38:14

-----input-----
aid time dt -u 1729546694
-----output-----
2024-10-21 21:38:14

-----input-----
aid time dt --rfc
-----output-----
2024-10-23T20:38:00.086663200+00:00
```