### aid text base64-encode
```
  aid text base64-encode <INPUT> encodes a base64 string

-----input-----
aid text base64-encode "Hello, world!"
-----output-----
SGVsbG8sIHdvcmxkIQ==
```
### aid text base64-decode
```
  aid text base64-decode <INPUT> decodes a base64 string

-----input-----
aid text base64-decode SGVsbG8sIHdvcmxkIQ==
-----output-----
Hello, world!
```
### aid text guid
```
  aid text guid Generates a random GUID

-----input-----
aid text guid
-----output-----
af6b8756-32fb-4630-bfe4-264c9a476273
```

### aid text url-encode
```
  aid text url-encode url encodes a string

-----input-----
aid text url-encode hello, world!
-----output-----
hello%20world%21
```

### aid text url-decode
```
  aid text url-decode decodes a url encoded string

-----input-----
aid text url-decode hello%20world%21
-----output-----
hello world!
```

### aid text lines
```
  aid text lines  reads and prints lines from a text file
           -i, --input <FILE>   Input text file to search.
           -s, --start <START>  first line to print      
           -e, --end <END>      last line to print       
           -f, --first <HEAD>   number of lines from the start of the file to print
           -l, --last <TAIL>    number of lines from the end of the file to print

-----input-----
aid text lines -i .\README.md -l 10
-----output-----
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