web-geo-viewer
==============

Instructions
------------
Load one or more files by clicking "choose files" and selecting one or more files,
one or more times. Enable and disable meshes by clicking the checkbox. Remove meshes by
clicking "remove." The viewer will do its best to associate textures and metadata files
with the appropriate mesh.

Supported formats
-----------------
* OBJ files, MTL files. Load the OBJ, MTL, and any texture the MTL refers to via the
  "choose files" dialogue. If the material and texture names match with the file names,
  the render canvas should update as more information becomes available.

* PLY files as defined by VCGLib (https://github.com/cnr-isti-vclab/vcglib) and Meshlab.
  This is the unofficial standard for textured PLYs - there doesn't appear to be an
  official way of supporting textured PLYs. This is implemented by placing a comment of the
  format:

  ```
  comment TextureFile file_name.png
  ```

  and having `property list texcoord ...` in your face properties list.

  binary archives are supported at this time.

* Most kinds of images: png, jpg, bmp, webp, pnm, tiff, tga, dds, ico, hdr

Known issues
------------
* The image file names referred to in MTL files and material names must be unique
  among all of the files that have been uploaded. Otherwise they will get mixed up.

Thanks
------
* Rust image crate (https://crates.io/crates/image)
* Three-d (https://crates.io/crates/three-d)
* rust-3d (https://crates.io/crates/rust-3d)
