This code was taken from here:

https://github.com/I3ck/rust-3d/tree/master/src/io

Thanks to Martin Buck for releasing what appears to be the most
complete and working set of Mesh IO functions for Rust that I've
come across. It's not currently included as a crate because it
doesn't load UVs from geometry files, and one goal of this project
is to support rendering of textured meshes. 

My current plan (all assuming I don't get too lazy to bring this 
to completion) is to implement UV support, contact Martin, and 
ask him for advice on how to architect this functionality in a 
way that meshes well wtih his design, and get whatever functionality
I implement here adapted for merging upstream.
