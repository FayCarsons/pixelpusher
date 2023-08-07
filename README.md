## PIXELPUSHER
A Rust pixel sorting program, making use of the excellent crate Rayon to achieve 
<1s sorting times even on large images. It currently supports vertical and 
horizontal sorting, and sorting by value and hue. At the moment it only exports 
as BMP, but can process images of any format supported by the Image crate. 

# Use
Constants deciding the thresholds in which to sort, the mode of direction, and input and output paths are stored in a RON file in the root of the crate for convenient
iteration. The mode and direction must be capitalized and the thresholds must be u8s.

# Example Outputs
go here :)

# Have fun! :)