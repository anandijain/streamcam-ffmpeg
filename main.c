#include <libavdevice/avdevice.h>
#include <libavformat/avformat.h>
#include <libavutil/avutil.h>
#include <libavcodec/avcodec.h>
#include <libswscale/swscale.h>
#include <libswresample/swresample.h>
#include <libavutil/pixfmt.h>
#include <libavutil/pixdesc.h>

// #include <libswscale/.h>

#include <stdio.h>

int get_stream_idx(const AVFormatContext *formatCtx, enum AVMediaType type)
{
    for (int i = 0; i < formatCtx->nb_streams; i++)
    {
        if (formatCtx->streams[i]->codecpar->codec_type == type)
        {
            return i;
        }
    }
    return -1;
}

int main()
{
    avdevice_register_all();

    AVFormatContext *formatCtx = avformat_alloc_context();
    AVInputFormat *inputFormat = av_find_input_format("dshow"); // Use "dshow" on Windows, "v4l2" on Linux
    avformat_open_input(&formatCtx, "video=Microsoft® LifeCam Studio(TM)", inputFormat, NULL);
    int stream_idx = get_stream_idx(formatCtx, AVMEDIA_TYPE_VIDEO);
    printf("Stream index: %d\n", stream_idx);
    AVCodecContext *codecCtx = avcodec_alloc_context3(NULL);
    AVCodecParameters *codecParams = formatCtx->streams[stream_idx]->codecpar;
    printf("stream codecpar res: %dx%d\n", codecParams->width, codecParams->height);

    avcodec_parameters_to_context(codecCtx, codecParams);
    AVCodec *codec = avcodec_find_decoder(codecCtx->codec_id);
    avcodec_open2(codecCtx, codec, NULL);
    // codecCtx->
    printf("Codec name: %s\n", codec->long_name);
    printf("Codec ID: %d\n", codecCtx->codec_id);

    struct SwsContext *scaler = sws_getContext(codecCtx->width, codecCtx->height, codecCtx->pix_fmt, codecCtx->width, codecCtx->height, AV_PIX_FMT_RGB24, SWS_BILINEAR, NULL, NULL, NULL);
    // AVPixelFormat
    const char *pix_fmt_name = av_get_pix_fmt_name(codecCtx->pix_fmt);
    printf("Pixel Format: %s\n", pix_fmt_name);

    AVFrame *frame = av_frame_alloc(); // this comes straight from the cam i guess
    printf("frame fmt %d\n", frame->format);

    // frame->format = AV_PIX_FMT_YUYV422;
    frame->format = codecCtx->pix_fmt;
    frame->width = codecCtx->width;
    frame->height = codecCtx->height;
    int did_get_buf = av_frame_get_buffer(frame, 32);
    printf("did get buf: %d\n", did_get_buf);
    printf("frame fmt after set %d\n", frame->format);

    AVFrame *frame_rgb = av_frame_alloc(); // this is the frame that will be converted to RGB
    frame_rgb->format = AV_PIX_FMT_RGB24;
    frame_rgb->width = codecCtx->width;
    frame_rgb->height = codecCtx->height;

    int rgb_did_get_buf = av_frame_get_buffer(frame_rgb, 32);
    printf("rgb did get buf: %d\n", rgb_did_get_buf);
    
    AVPacket *pkt = av_packet_alloc();
    av_init_packet(pkt);
    int isempty = frame->data[0] == NULL;
    printf("Frame is empty: %d\n", isempty);
    printf("Frame linesize: %d\n", frame->linesize[0]);
    // printf("Frame linesize 1: %d\n", frame->linesize[1]);
    // printf("Frame linesize 2: %d\n", frame->linesize[2]);
    // printf("Frame linesize 2: %d\n", frame->linesize[3]);
    printf("pkt size %d\n", pkt->size);
    printf("frame pkt size %d\n", frame->pkt_size);
    // frame->pkt_size
    int was_sent = avcodec_send_packet(codecCtx, pkt);
    printf("Packet sent: %d\n", was_sent);
    int f_ret = avcodec_receive_frame(codecCtx, frame_rgb);
    printf("cctx fmt %d\n", codecCtx->pix_fmt);
    printf("frame fmt %d\n", frame->format);
    printf("Frame received: %d\n", f_ret);
    // pripkt->stream_index;
    printf("pkt stream index: %d\n", pkt->stream_index);
    printf("rgb linesize[0] = %d\n", frame_rgb->linesize[0]);
    // scaler = sws_getContext(dec_ctx->width, dec_ctx->height, dec_ctx->pix_fmt, dec_ctx->width, dec_ctx->height, AV_PIX_FMT_RGB24, SWS_BILINEAR, NULL, NULL, NULL);
    int did_scale = sws_scale(scaler, (const uint8_t *const *)frame->data, frame->linesize, 0, frame->height, frame_rgb->data, frame_rgb->linesize);
    printf("Did scale: %d\n", did_scale);
    avcodec_free_context(&codecCtx);
    sws_freeContext(scaler);
    avformat_close_input(&formatCtx);
    return 0;
}
