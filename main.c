#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>
#include <libavdevice/avdevice.h>

void list_devices()
{
    AVFormatContext *pFormatCtx = NULL;
    AVInputFormat *inputFormat = av_find_input_format("dshow");
    AVDictionary *options = NULL;

    av_dict_set(&options, "list_devices", "true", 0);
    int ret = avformat_open_input(&pFormatCtx, "dummy", inputFormat, &options);
    if (ret < 0)
    {
        av_log(NULL, AV_LOG_ERROR, "Cannot open input device\n");
    }
    else
    {
        av_log(NULL, AV_LOG_INFO, "Device list (above) provided by dshow\n");
    }

    avformat_close_input(&pFormatCtx);
    av_dict_free(&options);
}

int main(int argc, char **argv)
{
    avdevice_register_all();
    // list_devices();
    // AVFormatContext *pFormatCtx = NULL;
    AVFormatContext *formatCtx = avformat_alloc_context();
    AVInputFormat *inputFormat = av_find_input_format("dshow"); // Use "v4l2" for Linux
    
    return 0;
}
