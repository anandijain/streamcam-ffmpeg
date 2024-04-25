extern crate ffmpeg_sys_the_third;
extern crate image;

use ffmpeg_sys_the_third::av_get_pix_fmt_name;
use ffmpeg_sys_the_third::AVMediaType::AVMEDIA_TYPE_VIDEO;
use ffmpeg_sys_the_third::AVPixelFormat;
use ffmpeg_sys_the_third::*;
use ffmpeg_the_third as ffmpeg;
use image::{ImageBuffer, Rgb};
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::slice;

fn main() {
    unsafe {
        // Register all available file formats and codecs
        avdevice_register_all();

        let mut format_ctx = ptr::null_mut();

        let device_name =
            CString::new("video=Microsoft® LifeCam Studio(TM)").expect("CString::new failed");
            

        // let device_name =
        //     CString::new("video=Integrated Webcam").expect("CString::new failed");

        let format_name = CString::new("dshow").expect("CString::new failed");
        let fmt = av_find_input_format(format_name.as_ptr());

        let mut options = ptr::null_mut();
        // av_dict_set(
        //     &mut options,
        //     CString::new("video_size").unwrap().as_ptr(),
        //     CString::new("1920x1080").unwrap().as_ptr(),
        //     0,
        // );

        if avformat_open_input(&mut format_ctx, device_name.as_ptr(), fmt, &mut options) != 0 {
            eprintln!("Cannot open input device with specified options");
            return;
        }

        if avformat_find_stream_info(format_ctx, ptr::null_mut()) < 0 {
            eprintln!("Couldn't find stream information");
            avformat_close_input(&mut format_ctx);
            return;
        }

        let mut video_stream_index = -1;
        for i in 0..(*format_ctx).nb_streams {
            let stream = *(*format_ctx).streams.offset(i as isize);
            if (*(*stream).codecpar).codec_type == AVMEDIA_TYPE_VIDEO {
                video_stream_index = i as i32;
                let codecpar = *(*stream).codecpar;
                println!("Resolution: {}x{}", codecpar.width, codecpar.height);
                break;
            }
        }

        if video_stream_index == -1 {
            eprintln!("No video stream found");
            avformat_close_input(&mut format_ctx);
            return;
        }

        let codec_id =
            (*(*(*(*format_ctx).streams.offset(video_stream_index as isize))).codecpar).codec_id;
        let codec = avcodec_find_decoder(codec_id);
        if codec.is_null() {
            eprintln!("Codec not found");
            avformat_close_input(&mut format_ctx);
            return;
        }

        let mut codec_ctx = avcodec_alloc_context3(codec);
        avcodec_parameters_to_context(
            codec_ctx,
            (*(*(*format_ctx).streams.offset(video_stream_index as isize))).codecpar,
        );
        avcodec_open2(codec_ctx, codec, &mut options);

        let mut frame = av_frame_alloc();

        let mut packet: AVPacket = mem::zeroed();
        av_init_packet(&mut packet);

        while av_read_frame(format_ctx, &mut packet) >= 0 {
            if packet.stream_index == video_stream_index {
                if avcodec_send_packet(codec_ctx, &packet) == 0 {
                    while avcodec_receive_frame(codec_ctx, frame) == 0 {
                        let f = (*frame);
                        println!("frame pkt_size: {:?}", f.pkt_size);
                        // let foo = CStr::from_ptr((*frame).format as i8);
                        // AVPixelFormat::
                        println!("frame format: {:?}", f.format);
                        let pix_fmt = std::mem::transmute::<i32, AVPixelFormat>(f.format);
                        let format_name_ptr = CStr::from_ptr(av_get_pix_fmt_name(pix_fmt))
                            .to_str()
                            .unwrap();
                        println!("pix fmt: {:?}", format_name_ptr);
                        // let foo2 = av_get_pix_fmt_name(f.format);
                        // (*frame).fo
                        let fd = (*frame).data[0];
                        let dim = ((*frame).width, (*frame).height);
                        // println!("Frame received: {:?}", fd.);
                        let pix_fmt_desc = av_pix_fmt_desc_get(pix_fmt);
                        println!("pix_fmt_desc: {:?}", *pix_fmt_desc);


                        let buffer_bytes_size = av_image_get_buffer_size(
                            AVPixelFormat::AV_PIX_FMT_RGB24,
                            dim.0,
                            dim.1,
                            0,
                        ) as usize;
                        println!("buffer_bytes_size: {:?}", buffer_bytes_size);
                        let data = slice::from_raw_parts(
                            fd,
                            ((*frame).linesize[0] * (*frame).height) as usize,
                        );
                        
                        // sws_convert
                        println!("Frame received: {:?}", data.len());
                        let buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(
                            dim.0 as u32,
                            dim.1 as u32,
                            data.to_vec(),
                        )
                        .unwrap();
                        buffer.save("frame.png").unwrap();
                        println!("Image saved as 'frame.png'");
                        break;
                    }
                }
                av_packet_unref(&mut packet);
                break;
            }
            av_packet_unref(&mut packet);
        }

        av_frame_free(&mut frame);
        avcodec_free_context(&mut codec_ctx);
        avformat_close_input(&mut format_ctx);
    }
}
