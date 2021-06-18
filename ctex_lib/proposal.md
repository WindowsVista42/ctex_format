## INTRODUCTION

This data representation is built around SIMD and easily supports up to avx512.
At the time of this being created, it is the latest and greatest.
While future technologies might prove to be better,
designing around likely, yet still theoretical, future hardware
would be even more overkill than this current approach is.

The following section gives a brief overview of the *simd friendly* layout of CTEX 0.2+.  
The section after gives a visual overview of the layout.

## OUTLINE

### DECODING

All CTEX 0.2+ files are chunked into 512 bit / 64 byte sectors.  
From one sector to the next, elements can be thought of as continuous.  
However, within each sector, elements are discontinuous.  

If we were to look at the data horizontally, it would follow the pattern  
```
00 16 32 48 01 17 33 49 ...  
```

If we instead look at the pattern vertically, such that each column is the size of a dword, it follows the pattern  
```
00 16 32 48  
01 17 33 49  
...  
```

CTEX 0.2+ packs adjacent elements into the nth byte of each dword sequence  
```
00 through 15 is packed into the 1st byte of the first 16 dwords.  
16 through 31 is packed into the 2nd byte of the next 16 dwords.  
...  
```

What this allows us to do is load each sector into a 64 byte vector,
do some bit manipulation and shifting, and arrive at
```
00 through 15 in the first 16 dwords.
16 through 31 in the next 16 dwords.
...
```

Considering that SIMD gather instructions require us to use dword offsets, 
generating it in this manner allows us to forgo shuffling the data into order.
With this out of the way, we can now gather the data at the dword offsets provided,
to get the colors of each pixel.  

We now write out the data generated to memory, and process the next sector.  
Once all of the sectors have been processed, we have a decoded image in memory.  

Modifying this to approach to also work with avx2 and sse2 is relatively trivial.  

### ENCODING
Encoding an image works the same way.  
We chunk it into sectors of 64 bytes, and order the bytes such that  
```
00 through 15 is packed into the 1st byte of the first 16 dwords.  
16 through 31 is packed into the 2nd byte of the next 16 dwords.  
...
```

### FLAGS

CTEX 0.2+ has moved to a flags system.  
The first 8 bytes are dedicated to flags.  

```
Packed 64 bit Flags Layout

00  vers_0 // CTEX Version
01  comp_0 // Compression
02  lutw_0 // Lut Width
03  offw_0 // Offsets Width
04  offw_0 // Offsets Width
05  resv_0 // Reserved
06  resv_1 // Reserved
07  resv_2 // Reserved
```


## LAYOUTS

Visual representations of the CTEX 0.2+ data layout throughout each processing step.

```
Packed 512 bit Sector Layout
sect_0 (u8x64)
00 16 32 48
01 17 33 49

02 18 34 50
03 19 35 51

04 20 36 52
05 21 37 53

06 22 38 54
07 23 39 55

08 24 40 56
09 25 41 57

10 26 42 58
11 27 43 59

12 28 44 60
13 29 45 61

14 30 46 62
15 31 47 63

... etc


De-Shuffled 2048 bit Layout
shfl_0 (u32x8)
00 01 02 03
04 05 06 07

shfl_1 (u32x8)
08 09 10 11
12 13 14 15

shfl_2 (u32x8)
16 17 18 19
20 21 22 23

shfl_3 (u32x8)
24 25 26 27
28 29 30 31

shfl_4 (u32x8)
32 33 34 35
36 37 38 39

shfl_5 (u32x8)
40 41 42 43
44 45 46 47

shfl_6 (u32x8)
48 49 50 51
52 53 54 55

shfl_7 (u32x8)
56 57 58 59
60 61 62 63

... etc


Gathered 2048 bit Layout
gath_0 (u32x8)
00 01 02 03
04 05 06 07

gath_1 (u32x8)
08 09 10 11
12 13 14 15

gath_2 (u32x8)
16 17 18 19
20 21 22 23

gath_3 (u32x8)
24 25 26 27
28 29 30 31

gath_4 (u32x8)
32 33 34 35
36 37 38 39

gath_5 (u32x8)
40 41 42 43
44 45 46 47

gath_6 (u32x8)
48 49 50 51
52 53 54 55

gath_7 (u32x8)
56 57 58 59
60 61 62 63

... etc
```
