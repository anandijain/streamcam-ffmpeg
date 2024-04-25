extern crate ffmpeg_sys_the_third;
extern crate image;

use ffmpeg::format::context::Input;
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

fn main() -> Result<(), anyhow::Error> {
    println!("a");

    println!("b");
    // unsafe {

    // };
    println!("c");
    let input = unsafe {
        avdevice_register_all();

        let device_name =
            CString::new("video=Microsoft® LifeCam Studio(TM)").expect("CString::new failed");
        // let fmt = unsafe { av_find_input_format(format_name.as_ptr()) };
        let format_name = CString::new("dshow").expect("CString::new failed");
        let fmt = av_find_input_format(format_name.as_ptr());
        println!("hi");
        // println!("fmt : {:?}", (*fmt).long_name);
        let mut ps = ptr::null_mut();
        let mut options = ptr::null_mut();

        if avformat_open_input(&mut ps, device_name.as_ptr(), fmt, &mut options) != 0 {
            eprintln!("Cannot open input device with specified options");
            return Err(anyhow::anyhow!(
                "Cannot open input device with specified options"
            ));
        }

        Input::wrap(ps)
    };
    println!("Input : {:?}", 5);
    Ok(())
}
