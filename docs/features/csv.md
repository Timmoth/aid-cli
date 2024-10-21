### aid csv search
```
  aid csv search  Sql search over csv
            -s, --sql <SQL>        Sql query e.g SELECT 'first name',age FROM people.csv WHERE age >= 25 AND age < 30 ORDER BY 'age' ASC.
            -o, --output <OUTPUT>  Output file path.

-----input-----
aid csv search -s "csv search -s "SELECT Platform,COUNT(Name) FROM vgsales.csv GROUP BY Platform ORDER BY COUNT(Name) DESC"
-----output-----
Platform,COUNT(Name)
DS,2163
PS2,2161
PS3,1329
Wii,1325
X360,1265
PSP,1213
PS,1196
PC,960
XB,824
GBA,822
GC,556
3DS,509
PSV,413
PS4,336
```
