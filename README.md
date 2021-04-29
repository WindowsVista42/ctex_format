# CTEX Image Encoding Format

CTEX (.ctex) is an image encoding format intended for use with 256 color images.  
It supports very fast loads and small file sizes.  
This format is being developed for a custom game engine, but I decided that the technology here might be useful to someone else.

## Limitations and Considerations
Input images can only contain 256 unique colors.
This palette is naively generated from the base image,
and if there are more than 256 unique colors, the encoding will fail.

This format will typically reduce file sizes, however it was not created with this sole purpose.
LZ4 compression is used to reduce file sizes further, as well as reduce io bottlenecks.

The primary goal of this format is to be easy to implement and quick to decode.
Encoding times will naturally be longer, but not slow by any means.

## Specification
[proposal](ctex_lib/proposal.md)
