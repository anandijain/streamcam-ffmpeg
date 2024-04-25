# learning ffmpeg in c and rust

### to compile main.c 
C:\msys64\ucrt64\bin\gcc.exe -o main.exe main.c -LC:\Users\anand\Downloads\ffmpeg-6.1.1-full_build-shared\lib -IC:\Users\anand\Downloads\ffmpeg-6.1.1-full_build-shared\include -lavdevice -lavfilter -lavformat -lavcodec -lavutil -lswresample -lswscale -lbcrypt -Bdynamic