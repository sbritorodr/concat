use std::ffi::CString;
use std::ptr;

fn main() {
    // Initialize the FFmpeg libraries
    unsafe {
        ffmpeg_sys::av_register_all();
    }

    // Open the input files
    let filename1 = CString::new("1.mp4").unwrap();
    let input_format_context1 = unsafe {
        let mut input_format_context: *mut ffmpeg_sys::AVFormatContext = ptr::null_mut();
        let mut error: i32 = 0;
        ffmpeg_sys::avformat_open_input(
            &mut input_format_context,
            filename1.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        input_format_context
    };

    let filename2 = CString::new("2.mp4").unwrap();
    let input_format_context2 = unsafe {
        let mut input_format_context: *mut ffmpeg_sys::AVFormatContext = ptr::null_mut();
        let mut error: i32 = 0;
        ffmpeg_sys::avformat_open_input(
            &mut input_format_context,
            filename2.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        input_format_context
    };

    // Create the output context
    let output_filename = CString::new("output.mp4").unwrap();
    let output_format_context = unsafe {
        let mut output_format_context: *mut ffmpeg_sys::AVFormatContext = ptr::null_mut();
        let mut error: i32 = 0;
        ffmpeg_sys::avformat_alloc_output_context2(
            &mut output_format_context,
            ptr::null_mut(),
            ptr::null_mut(),
            output_filename.as_ptr(),
        );
        output_format_context
    };

    // Iterate through the streams in the first input file and add them to the output context
    unsafe {
        let mut input_stream: *mut ffmpeg_sys::AVStream = ptr::null_mut();
        let mut output_stream: *mut ffmpeg_sys::AVStream = ptr::null_mut();
        for i in 0..(*input_format_context1).nb_streams {
            input_stream = *(*input_format_context1).streams.offset(i as isize);
            output_stream = ffmpeg_sys::avformat_new_stream(output_format_context, (*input_stream).codec.as_ptr());
            ffmpeg_sys::avcodec_parameters_copy(
                (*output_stream).codecpar,
                (*input_stream).codecpar,
            );
            (*output_stream).time_base = (*input_stream).time_base;
        }
    }

    // Iterate through the streams in the second input file and add them to the output context
    unsafe     {
        let mut input_stream: *mut ffmpeg_sys::AVStream = ptr::null_mut();
        let mut output_stream: *mut ffmpeg_sys::AVStream = ptr::null_mut();
        for i in 0..(*input_format_context2).nb_streams {
            input_stream = *(*input_format_context2).streams.offset(i as isize);
            output_stream = ffmpeg_sys::avformat_new_stream(output_format_context, (*input_stream).codec.as_ptr());
            ffmpeg_sys::avcodec_parameters_copy(
                (*output_stream).codecpar,
                (*input_stream).codecpar,
            );
            (*output_stream).time_base = (*input_stream).time_base;
        }
    }

    // Open the output file
    let mut error: i32 = 0;
    unsafe {
        error = ffmpeg_sys::avio_open(&mut (*output_format_context).pb, output_filename.as_ptr(), ffmpeg_sys::AVIO_FLAG_WRITE);
    }
    if error < 0 {
        println!("Error opening output file: {}", error);
        return;
    }

    // Write the output file header
    unsafe {
        error = ffmpeg_sys::avformat_write_header(output_format_context, ptr::null_mut());
    }
    if error < 0 {
        println!("Error writing output file header: {}", error);
        return;
    }

    // Set up the packet and frame objects for reading and writing
    let mut packet: ffmpeg_sys::AVPacket = ffmpeg_sys::AVPacket::default();
    let mut frame: *mut ffmpeg_sys::AVFrame = ptr::null_mut();
    unsafe {
        frame = ffmpeg_sys::av_frame_alloc();
    }

    // Read and write the packets from the first input file
    let mut eof1 = 0;
    while eof1 == 0 {
        // Read a packet from the input file
        unsafe {
            error = ffmpeg_sys::av_read_frame(input_format_context1, &mut packet);
        }
        if error < 0 {
            eof1 = 1;
            break;
        }

        // Write the packet to the output file
        unsafe {
            error = ffmpeg_sys::av_interleaved_write_frame(output_format_context, &mut packet);
        }
        if error < 0 {
            println!("Error writing packet: {}", error);
            return;
        }
    }

    // Read and write the packets from the second input file
    let mut eof2 = 0;
    while eof2 == 0 {
        // Read a packet from the input file
        unsafe {
            error = ffmpeg_sys::av_read_frame(input_format_context2, &mut packet);
        }
        if error < 0 {
            eof2 = 1;
            break;
        }

        // Write the packet to the output file
        unsafe {
            error = ffmpeg_sys::av_interleaved_write_frame(output_format_context, &mut packet);
        }
        if error < 0 {
            println!("Error writing packet: {}", error);
            return;
        }
    }

    // Flush the encoder and write the trailer
    unsafe {
        error = ffmpeg_sys::av_write_trailer(output_format_context);
    }
    if error < 0 {
        println!("Error writing output file trailer: {}", error);
        return;
    }

    // Clean up
    unsafe {
        ffmpeg_sys::avformat_close_input(&mut input_format_context1);
        ffmpeg_sys::avformat_close_input(&mut input_format_context2);
        ffmpeg_sys::avformat_free_context(output_format_context);
        ffmpeg_sys::av_frame_free(&mut frame);
    }

    println!("Successfully concatenated the input videos");
}


