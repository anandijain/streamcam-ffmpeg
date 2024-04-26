extern crate ffmpeg_the_third as ffmpeg;

use crate::ffmpeg::codec::packet::Packet;
use crate::ffmpeg::codec::Context as CodecContext;
use crate::ffmpeg::format::{input, Pixel};
use crate::ffmpeg::media::Type;
use crate::ffmpeg::software::scaling::{context::Context, flag::Flags};
use crate::ffmpeg::util::frame::video::Video;
use crate::ffmpeg::Dictionary;

use crate::ffmpeg::format::context::Output;
use anyhow::{self};
use ffmpeg::encoder::{self, find_by_name, Encoder};
use ffmpeg::format::output;
use ffmpeg::{codec, Codec};
use ffmpeg_sys_the_third::{av_find_input_format, avformat_open_input};
use std::ffi::CString;
use std::{env, ptr};

fn main() -> Result<(), anyhow::Error> {
    ffmpeg::init().unwrap();

    let device_name = CString::new("video=Microsoft® LifeCam Studio(TM)").unwrap();
    let format_name = CString::new("dshow").unwrap();
    let output_file = "output.mp4";
    let output_path = std::path::Path::new(output_file);
    let mut ps = ptr::null_mut();
    let mut options = ptr::null_mut();

    let mut ictx = unsafe {
        ffmpeg::device::register_all();
        let fmt = av_find_input_format(format_name.as_ptr());
        avformat_open_input(&mut ps, device_name.as_ptr(), fmt, &mut options);
        ffmpeg::format::context::Input::wrap(ps)
    };

    let input_stream = ictx.streams().best(Type::Video).unwrap();
    let video_stream_index = input_stream.index();
    let decoder = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?
        .decoder()
        .video()?;

    let mut octx = output(&output_path)?;
    let c = ffmpeg::codec::id::Id::AV1;

    let mut encoder = encoder::find(c).unwrap();
    let enc = encoder::find_by_name("libx264").unwrap();
    let vid = encoder.video().unwrap();
    codec::Context::encoder()?;
// Encoder::

    println!("Encoder : {:?}", encoder.name());
    println!("Encoder : {:?}", encoder.description());
    unsafe {
        println!("Encoder : {:?}", *(encoder.as_ptr()));

    }
    // encoder.capabilities()
    // unsafe {
    // }
    // let mut encoder = Codec::from().unwrap().video()?;

    // encoder.
    // encoder.set_width(decoder.width());
    // encoder.set_height(decoder.height());
    // encoder.set_format(Pixel::YUV420P);
    // encoder.set_time_base((1, 30));
    let mut stream = octx.add_stream(encoder).unwrap();
    // let mut encoder = stream.codec().encoder().video()?;
    // encoder.set_time_base((1, 30));

    // encoder.open_as(
    //     Codec::find_encoder(ffmpeg::codec::id::Id::H264).unwrap(),
    //     &Dictionary::new(),
    // )?;
    // stream.set_time_base((1, 30));
    // octx.write_header()?;

    // let mut scaler = Context::get(
    //     decoder.format(),
    //     decoder.width(),
    //     decoder.height(),
    //     Pixel::YUV420P,
    //     decoder.width(),
    //     decoder.height(),
    //     Flags::BILINEAR,
    // )?;

    // for (stream, packet) in ictx.packets() {
    //     if stream.index() == video_stream_index {
    //         decoder.send_packet(&packet)?;
    //         let mut decoded = Video::empty();
    //         while decoder.receive_frame(&mut decoded).is_ok() {
    //             let mut converted = Video::empty();
    //             scaler.run(&decoded, &mut converted)?;
    //             let mut encoded = Packet::empty();
    //             encoder.send_frame(&converted)?;
    //             while encoder.receive_packet(&mut encoded).is_ok() {
    //                 encoded.set_stream(0);
    //                 octx.interleaved_write_frame(&encoded)?;
    //             }
    //         }
    //     }
    // }

    // encoder.send_eof()?;
    // let mut encoded = Packet::empty();
    // while encoder.receive_packet(&mut encoded).is_ok() {
    //     encoded.set_stream(0);
    //     octx.interleaved_write_frame(&encoded)?;
    // }

    // octx.write_trailer()?;

    Ok(())
}
