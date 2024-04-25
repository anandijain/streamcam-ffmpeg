# learning ffmpeg in c and rust

### to compile main.c 
C:\msys64\ucrt64\bin\gcc.exe -o main.exe main.c -LC:\Users\anand\Downloads\ffmpeg-6.1.1-full_build-shared\lib -IC:\Users\anand\Downloads\ffmpeg-6.1.1-full_build-shared\include -lavdevice -lavfilter -lavformat -lavcodec -lavutil -lswresample -lswscale -lbcrypt -Bdynamic

`ffmpeg -list_devices true -f dshow -i dummy` 

this captures a single frame from a webcam to file. `examples/webcam_fram_cap.rs` does this currently 
`ffmpeg -f "dshow" -i "video=Microsoft® LifeCam Studio(TM)" -frames:v 1 output.jpg`

the next step is to capture a video from the webcam using rust 
`ffmpeg -f "dshow" -i "video=Microsoft® LifeCam Studio(TM)" -frames:v 30 output.mp4`

