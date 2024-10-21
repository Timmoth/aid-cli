### aid bits board
```
  aid bits board  Display the number in bitboard representation
            -b, --bin <BINARY>   Display the binary value as a bitboard.
            -d, --dec <DECIMAL>  Display the decimal value as a bitboard.
                --hex <HEX>      Display the decimal value as a bitboard.

-----input-----
aid bits board -d 14992333222294

-----output-----
00 | 0  0  0  0  0  0  0  0  
00 | 0  0  0  0  0  0  0  0
0D | 0  0  0  0  1  1  0  1
A2 | 1  0  1  0  0  0  1  0
AC | 1  0  1  0  1  1  0  0
B2 | 1  0  1  1  0  0  1  0
31 | 0  0  1  1  0  0  0  1
96 | 1  0  0  1  0  1  1  0
    -----------------------
     7  6  5  4  3  2  1  0
dec: 14992333222294
hex: DA2ACB23196
bin: 11011010001010101100101100100011000110010110
lsb: 1
msb: 43
set bits: 21
```

### aid bits to-bin
```
  aid bits to-bin  Converts a number to it's binary representation
            -d, --dec <DECIMAL>  Convert the decimal number to binary.
                --hex <HEX>      Converts the hex number to binary.

-----input-----
aid bits to-bin -d 134

-----output-----
10000110
```

### aid bits to-dec
```
  aid bits to-dec  Converts a number to it's decimal representation
            -b, --bin <BIN>  Converts the binary number to hedecimalx.
                --hex <HEX>  Converts the hex number to decimal.

-----input-----
aid bits to-dec -b 10000110

-----output-----
134
```

### aid bits to-hex
```
  aid bits to-hex  Converts a number to it's hex representation
            -d, --dec <DECIMAL>  Convert the decimal number to hex.
            -b, --bin <BIN>      Converts the binary number to hex.

-----input-----
aid bits to-hex -b 10000110

-----output-----
86
```