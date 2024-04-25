extern crate ffmpeg_the_third as ffmpeg;

use crate::ffmpeg::format::{input, Pixel};
use crate::ffmpeg::media::Type;
use crate::ffmpeg::software::scaling::{context::Context, flag::Flags};
use crate::ffmpeg::util::frame::video::Video;
use anyhow;
use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use std::{env, ptr};
fn main() -> Result<(), anyhow::Error> {
    ffmpeg::init().unwrap();
    // input()

    let device_name =
        CString::new("video=Microsoft® LifeCam Studio(TM)").expect("CString::new failed");
    let format_name = CString::new("dshow").expect("CString::new failed");
    // av_find_input_format("d")
    unsafe { 

    }
    println!("b");
    let mut ps = ptr::null_mut();
    let mut options = ptr::null_mut();

    let mut ictx = unsafe {
        avdevice_register_all();
        let fmt = av_find_input_format(format_name.as_ptr());
        println!("fmt : {:?}", (*fmt).long_name);
        if avformat_open_input(&mut ps, device_name.as_ptr(), fmt, options) != 0 {
            eprintln!("Cannot open input device with specified options");
            return Err(anyhow::anyhow!(
                "Cannot open input device with specified options"
            ));
        }

        Input::wrap(ps)
    };
    println!("Input : {:?}", 55);

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    println!("Input : {:?}", input);
    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    // unsafe {
    //     println!("Context Decoder : {:?}", *context_decoder.as_ptr());
    // }
    let mut decoder = context_decoder.decoder().video()?;
    // println!("Decoder : {:?}", decoder);
    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;
    // println!("Scaler : {:?}", scaler);

    let mut frame_index = 0;

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let old_fmt = decoded.format();
                println!("old planes: {:?}", decoded.planes());
                println!("pix_fmt: {:?}", old_fmt);

                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                println!("planes: {:?}", rgb_frame.planes());
                println!("converted pix_fmt: {:?}", rgb_frame.format());
                save_file2(&rgb_frame, frame_index).unwrap();
                frame_index += 1;
            }
            Ok(())
        };

    for (i, (stream, packet)) in ictx.packets().enumerate() {
        if stream.index() == video_stream_index {
            if i == 10 {
                break;
            }
            decoder.send_packet(&packet)?;
            receive_and_process_decoded_frames(&mut decoder)?;
        }
    }

    decoder.send_eof()?;
    receive_and_process_decoded_frames(&mut decoder)?;

    Ok(())
}

use ffmpeg::format::context::Input;
use ffmpeg_sys_the_third::{av_find_input_format, avdevice_register_all, avformat_open_input};
use image::{ImageBuffer, RgbImage};
use std::io::Result as IoResult;

fn save_file2(frame: &Video, index: usize) -> IoResult<()> {
    let width = frame.width() as u32;
    let height = frame.height() as u32;
    let frame_data = frame.data(0);

    // Creating an ImageBuffer from raw frame data
    let img: RgbImage =
        ImageBuffer::from_raw(width, height, frame_data.to_vec()).ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Cannot create image buffer",
        ))?;

    // Save to file
    img.save(format!("frame{index}.png")).unwrap();

    Ok(())
}

fn save_file(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
    // frame.data(index)
    // frame.data.len()
    println!("Frame received: {:?}", frame.data(0).len());
    // let x = frame.;
    // println!("Frame received: {:?}", frame.data(1).len());
    println!("converted pix_fmt: {:?}", frame.format());
    let mut file = File::create(format!("frame{index}.ppm"))?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))?;
    Ok(())
}
