# notes

```shell
rcmd SHLIB wk/inst/include/wk-v1-impl.c
using C compiler: 'gcc.exe (GCC) 12.2.0'
gcc  -I"C:/Users/minin/scoop/apps/r/current/include" -DNDEBUG -I"c:/rtools43/x86_64-w64-mingw32.static.posix/include" -O2 -Wall  -mfpmath=sse -msse2 -mstackrealign  -c wk/inst/include/wk-v1-impl.c -o wk/inst/include/wk-v1-impl.o
gcc -shared -s -static-libgcc -o wk/inst/include/wk-v1-impl.dll tmp.def wk/inst/include/wk-v1-impl.o -Lc:/rtools43/x86_64-w64-mingw32.static.posix/lib/x64 -Lc:/rtools43/x86_64-w64-mingw32.static.posix/lib -LC:/Users/minin/scoop/apps/r/current/bin/x64 -lR
```
