extern crate ffmpeg_the_third as ffmpeg;

use crate::ffmpeg::format::{input, Pixel};
use crate::ffmpeg::media::Type;
use crate::ffmpeg::software::scaling::{context::Context, flag::Flags};
use crate::ffmpeg::util::frame::video::Video;
use anyhow;
use ffmpeg::format::context::Input;
use ffmpeg::format::output;
use ffmpeg::time;
use ffmpeg_sys_the_third::{av_find_input_format, avdevice_register_all, avformat_open_input};
use image::{ImageBuffer, RgbImage};
use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result as IoResult;
use std::{env, ptr};

fn main() -> Result<(), anyhow::Error> {
    ffmpeg::init().unwrap();
    // input()

    Ok(())
}