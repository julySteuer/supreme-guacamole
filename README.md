## Reproduce 
1. Install WinBGIm from https://github.com/ki9gpin/WinBGIm-64 this one is for x64 machines
2. go to the libbgi folder make a out folder cd into it
3. ``` cmake ..```
4. install or locate msbuild.exe , defaults in vs2019 at ``` C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\MSBuild\Current\Bin```
5. add msbuild.exe to the path
6. ```msbuild.exe ./Project.sln # or whatever your .sln name is```
7. copy bgi.lib to c_code