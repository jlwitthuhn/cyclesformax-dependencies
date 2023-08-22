# Intro

This repo contains a rust program that can build all of the dependencies with the configuration required by Cycles for Max. Not that this repo only directly contains the code for the smaller dependencies and you must download the source code for a few heavier libraries separately. See the 'Libraries Not Included' section below.

After downloading all archives, run this command in the root of this repository with a recent version of rust installed to configure and build everything:
`cargo run --manifest-path ./builder/Cargo.toml -- -t v140`

`v140` targets visual studio 2015. You can also use the values `v141` for 2017 and `v142` for 2019.

# Modified Libraries

The file `archive/shader_editor-21-04-04.tar.gz` contains a copy of the (cycles-shader-editor-imgui)[https://github.com/jlwitthuhn/cycles-shader-editor-imgui] repo.

ImGui has been packaged into a library that is buildable by CMake, the source for this is located in `archive/imgui-1.88-custom.tar.gz`

OneTBB has been lightly modified to build with Visual Studio 2015, needed to support 3ds Max 2017-2019. See `MY_CHANGES.txt` in the onetbb archive for a more detailed description of this change. If you do not intend to build with VS 2015 you can use an unmodified OneTBB instead.

# Libraries Not Included

This repo does not contain a few particularly heavy dependencies to keep the repo size down. The source code for the following libraries must be downloaded separately and moved to the 'archive' directory. These are:
* [Boost 1.80](https://boostorg.jfrog.io/artifactory/main/release/1.80.0/source/)
* [OpenImageIO 2.2.21.0](https://github.com/OpenImageIO/oiio/releases/tag/v2.2.21.0)
* [OpenEXR 2.5.8](https://github.com/AcademySoftwareFoundation/openexr/releases/tag/v2.5.8)
