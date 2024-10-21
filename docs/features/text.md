## aid text
### aid text base64-encode
```
  aid text base64-encode  encodes a base64 string
            -i, --input <INPUT>   Input text to base64 encode.

-----input-----
aid text base64-encode -i "Hello, world!"
-----output-----
SGVsbG8sIHdvcmxkIQ==
```
### aid text base64-decode
```
  aid text base64-decode  decodes a base64 string
            -i, --input <INPUT>   Input text to base64 decode.

-----input-----
aid text base64-decode -i SGVsbG8sIHdvcmxkIQ==
-----output-----
Hello, world!
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