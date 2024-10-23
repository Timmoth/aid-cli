### aid bits eval
```
  aid bits eval <EXPRESSION>  Evaluates a bitwise expression, converts base, visualize binary / display info
                -i, --info   Output the bitboard representation.
                -c, --chess  Output the chess bitboard representation.
                -b, --bin    Output the result in binary.
                    --hex    Output the result in hex.

-----input-----
aid bits eval 0b1001
-----output-----
9

-----input-----
aid bits eval --hex 0b10101001
-----output-----
a9

-----input-----
aid bits eval --bin 0xa9
-----output-----
10101001

-----input-----
aid bits eval --bin '0xa9 << 2 | (0b1010 & 7)'
-----output-----
1010100110

-----input-----
aid bits eval --info '!((0b11110000 ^ 0b00001111) << 8)'
-----output-----
FF | 1  1  1  1  1  1  1  1  
FF | 1  1  1  1  1  1  1  1
FF | 1  1  1  1  1  1  1  1
FF | 1  1  1  1  1  1  1  1
FF | 1  1  1  1  1  1  1  1
FF | 1  1  1  1  1  1  1  1
00 | 0  0  0  0  0  0  0  0
FF | 1  1  1  1  1  1  1  1
    -----------------------
     7  6  5  4  3  2  1  0
dec: 18446744073709486335
hex: FFFFFFFFFFFF00FF
bin: 1111111111111111111111111111111111111111111111110000000011111111
lsb: 0
msb: 63
set bits: 56

-----input-----
aid bits eval --chess '!(0xFEFEFEFEFEFEFEFE << 8)'
-----output-----
8 | 1  0  0  0  0  0  0  0  
7 | 1  0  0  0  0  0  0  0
6 | 1  0  0  0  0  0  0  0
5 | 1  0  0  0  0  0  0  0
4 | 1  0  0  0  0  0  0  0
3 | 1  0  0  0  0  0  0  0
2 | 1  0  0  0  0  0  0  0
1 | 1  1  1  1  1  1  1  1
   -----------------------
    A  B  C  D  E  F  G  H
dec: 72340172838076927
hex: 1010101010101FF
bin: 100000001000000010000000100000001000000010000000111111111
lsb: 0
msb: 56
set bits: 15
```