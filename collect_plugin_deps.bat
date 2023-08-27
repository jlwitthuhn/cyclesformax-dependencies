rmdir _collected /s /q

mkdir .\_collected
mkdir _collected\include
mkdir _collected\lib

xcopy /e .\boost-1.80.0\include\boost-1_80\* .\_collected\include
xcopy /e .\oiio-2.2.21.0\include\* .\_collected\include
xcopy /e .\oneTBB-2021.3.0\include\* .\_collected\include
xcopy /e .\openexr-2.5.8\include\* .\_collected\include

xcopy /e .\shader_editor\include\* .\_collected\include

xcopy .\boost-1.80.0\lib\libboost_filesystem-*.lib .\_collected\lib
xcopy .\boost-1.80.0\lib\libboost_regex-*.lib .\_collected\lib
xcopy .\boost-1.80.0\lib\libboost_system-*.lib .\_collected\lib
xcopy .\boost-1.80.0\lib\libboost_thread-*.lib .\_collected\lib
xcopy .\glew-2.1.0\lib\libglew32.lib .\_collected\lib
xcopy .\glfw-3.3.8\lib\glfw3.lib .\_collected\lib
xcopy .\imgui-1.88\lib\imgui.lib .\_collected\lib
xcopy .\libjpeg-turbo-2.1.5.1\lib\jpeg-static.lib .\_collected\lib
xcopy .\libpng-1.6.39\lib\libpng16_static.lib .\_collected\lib
xcopy .\libtiff-4.5.0\lib\tiff.lib .\_collected\lib
xcopy .\oiio-2.2.21.0\lib\OpenImageIO.lib .\_collected\lib
xcopy .\oneTBB-2021.3.0\lib\tbb.lib .\_collected\lib
xcopy .\openexr-2.5.8\lib\Half-2_5.lib .\_collected\lib
xcopy .\openexr-2.5.8\lib\Iex-2_5.lib .\_collected\lib
xcopy .\openexr-2.5.8\lib\IlmImf-2_5.lib .\_collected\lib
xcopy .\openexr-2.5.8\lib\IlmThread-2_5.lib .\_collected\lib
xcopy .\openexr-2.5.8\lib\Imath-2_5.lib .\_collected\lib
xcopy .\zlib-1.2.13\lib\zlibstatic.lib .\_collected\lib

xcopy .\shader_editor\lib\shader_editor.lib .\_collected\lib
