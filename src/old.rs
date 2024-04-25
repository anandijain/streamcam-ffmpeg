
fn list_codecs() {
    unsafe {
        let mut ptr = std::ptr::null_mut();
        loop {
            // or av_demuxer_iterate
            let muxer = av_codec_iterate(&mut ptr);
            if muxer.is_null() {
                break;
            }

            let c = *muxer;
            let ln = CStr::from_ptr((*muxer).long_name).to_str().unwrap();

            println!("{:?}: {:?}", c.id, ln);
            // do something with (*muxer)
            // ID: 226, av1: librav1e AV1 (Video)
        }
    }
}