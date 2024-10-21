## aid file

### aid file info
```
  aid file info  prints file metadata
            -f, --file <FILE>   file path.

-----input-----
aid file info -f .\README.md
-----output-----
Type: File
Size: 5582 bytes
Permissions: Read-only: false
Modified: 2024-10-21 20:52:25.0 +00:00:00
Accessed: 2024-10-21 20:52:25.0 +00:00:00
Created: 2024-10-18 20:45:22.0 +00:00:00
```

### aid file md5
```
  aid file md5  calculates the files Md5 checksum
            -f, --file <FILE>   file path.

-----input-----
aid file md5 -f .\README.md
-----output-----
7d417daec5c2663c7a224297d1fc9d95
```

### aid file sha1
```
  aid file sha1  calculates the files Sha1 checksum
            -f, --file <FILE>   file path.

-----input-----
aid file sha1 -f .\README.md
-----output-----
aaaa9aa5119da904c84eca8b0a1db46947732737
```

### aid file sha256
```
  aid file sha256  calculates the files Sha256 checksum
            -f, --file <FILE>   file path.

-----input-----
aid file sha256 -f .\README.md
-----output-----
06e033f7ca19a5ef5a1be805a41dec6bd5b36cfdd231d6d3959373d6d4fe7ae7
```

### aid file zip
```
  aid file zip  zips the files in the source directory
            -d, --dir <DIRECTORY>   file path.
            -f, --file <FILE>   output zip file.

-----input-----
aid file zip -dir ./src ./src.zip
-----output-----
Successfully zipped directory './src'
```
