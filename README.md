# CTEX 
### 256 Color Texture Encoding Format (.ctex)

CTEX is a texture encoding format intended for use with 256 colors.
It supports very fast loads and small file sizes.

## Limitations and Considerations
Images can only use a palette of 256 colors.
This palette is naively generated from the base image.
If the base image contains more than 256 colors the encoding will not work.

This texture format will typically reduce file sizes, however it was not created with this sole purpose.
LZ4 compression is used to reduce file sizes further, as well as reduce io bottlenecks.

The primary goal of this format is to be easy to implement and quick to decode.
Encoding times will naturally be longer, but not slow by any means.

## Specification
```
FILE FORMAT:
     ┌──────────────────────────────────┐
     │      ┌────────────────┐          │
BOF: u32 ++ u32 ++ [[u8; 4]; n] ++ [u8; n] :EOF
     │      │      │               │
 (0) width  │      │               │
        (1) length │               │
               (2) color palette   │
                               (3) lookup table

BYTE OFFSETS:
(0) 0 .. 4
(1) 4 .. 8
(2) 8 .. (8 + 4 * length)
(3) (8 + 4 * length) .. (8 + 4 * length + width * width)
```
