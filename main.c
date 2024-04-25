#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavdevice/avdevice.h>
#include <libswscale/swscale.h>
#include <libavutil/imgutils.h>

int main()
{
    AVFormatContext *formatCtx = NULL;
    AVCodecContext *codecCtx = NULL;
    AVCodec *codec = NULL;
    AVInputFormat *inputFormat = NULL;
    AVFrame *frame = NULL, *frameRGB = NULL;
    struct SwsContext *sws_ctx = NULL;
    int videoStreamIndex = -1;

    // Register all devices
    avdevice_register_all();

    // Set up the format context for the device
    formatCtx = avformat_alloc_context();

    // Open input from the webcam
    inputFormat = av_find_input_format("dshow");
    // if (avformat_open_input(&formatCtx, "video=Microsoft® LifeCam Studio(TM)", inputFormat, NULL) != 0)
    if (avformat_open_input(&formatCtx, "video=Integrated Webcam", inputFormat, NULL) != 0)
    {
        fprintf(stderr, "Cannot open input device\n");
        return -1;
    }

    if (avformat_find_stream_info(formatCtx, NULL) < 0)
    {
        fprintf(stderr, "Cannot find stream information\n");
        return -1;
    }

    // Find the first video stream
    for (int i = 0; i < formatCtx->nb_streams; i++)
    {
        if (formatCtx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO)
        {
            videoStreamIndex = i;
            break;
        }
    }
    if (videoStreamIndex == -1)
    {
        fprintf(stderr, "No video stream found\n");
        return -1;
    }

    // Get a pointer to the codec for the video stream
    codec = avcodec_find_decoder(formatCtx->streams[videoStreamIndex]->codecpar->codec_id);
    codecCtx = avcodec_alloc_context3(codec);
    avcodec_parameters_to_context(codecCtx, formatCtx->streams[videoStreamIndex]->codecpar);

    // Open codec
    if (avcodec_open2(codecCtx, codec, NULL) < 0)
    {
        fprintf(stderr, "Failed to open codec\n");
        return -1;
    }

    // Allocate video frame and frameRGB
    frame = av_frame_alloc();
    frameRGB = av_frame_alloc();
    int numBytes = av_image_get_buffer_size(AV_PIX_FMT_RGB24, codecCtx->width, codecCtx->height, 32);
    uint8_t *buffer = (uint8_t *)av_malloc(numBytes * sizeof(uint8_t));
    printf("Buffer size: %d\n", numBytes);
    // Assign appropriate parts of buffer to image planes in frameRGB
    av_image_fill_arrays(frameRGB->data, frameRGB->linesize, buffer, AV_PIX_FMT_RGB24, codecCtx->width, codecCtx->height, 1);

    // Initialize SWS context for software scaling
    sws_ctx = sws_getContext(codecCtx->width, codecCtx->height, codecCtx->pix_fmt, codecCtx->width, codecCtx->height, AV_PIX_FMT_RGB24, SWS_BILINEAR, NULL, NULL, NULL);

    // Read frames and save first one to jpg
    AVPacket packet;
    while (av_read_frame(formatCtx, &packet) >= 0)
    {
        printf("Packet stream index: %d\n", packet.stream_index);  
         
        if (packet.stream_index == videoStreamIndex)
        {
            int response = avcodec_send_packet(codecCtx, &packet);
            if (response < 0)
            {
                fprintf(stderr, "Failed to send packet to decoder\n");
                return -1;
            }

            while (response >= 0)
            {
                response = avcodec_receive_frame(codecCtx, frame);
                if (response == AVERROR(EAGAIN) || response == AVERROR_EOF)
                {
                    break;
                }
                else if (response < 0)
                {
                    fprintf(stderr, "Failed to receive frame from decoder\n");
                    return -1;
                }

                if (response >= 0)
                {
                    sws_scale(sws_ctx, (uint8_t const *const *)frame->data, frame->linesize, 0, codecCtx->height, frameRGB->data, frameRGB->linesize);

                    AVCodecContext *pOutCtx = NULL;
                    AVCodec *jpegCodec = avcodec_find_encoder(AV_CODEC_ID_MJPEG);
                    pOutCtx = avcodec_alloc_context3(jpegCodec);
                    pOutCtx->pix_fmt = AV_PIX_FMT_YUVJ420P;
                    pOutCtx->height = codecCtx->height;
                    pOutCtx->width = codecCtx->width;
                    pOutCtx->time_base = (AVRational){1, 25};

                    if (!jpegCodec || avcodec_open2(pOutCtx, jpegCodec, NULL) < 0)
                    {
                        fprintf(stderr, "Could not open JPEG codec\n");
                        return -1;
                    }

                    AVPacket outPacket;
                    av_init_packet(&outPacket);
                    outPacket.data = NULL;
                    outPacket.size = 0;
                    avcodec_send_frame(pOutCtx, frameRGB);
                    avcodec_receive_packet(pOutCtx, &outPacket);
                    printf("Packet size: %d\n", outPacket.size);
                    FILE *JPEGFile = fopen("me2.jpg", "wb");
                    fwrite(outPacket.data, 1, outPacket.size, JPEGFile);
                    fclose(JPEGFile);
                    av_packet_unref(&outPacket);

                    break; // Break out after saving first frame
                }
            }
            av_packet_unref(&packet);
        }
    }
}
