//! The Media package without FFmpeg.
use super::common::{Reader, end, flag, int, real, text};
use super::print::Output;
use super::tried;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkservice::media::{
    MediaBufferPool, MediaCodecContext, MediaFormatContext, MediaFrame, MediaPacket, MediaScaler,
    MediaTimer,
};

fn frame_values(output: &Output, frame: &MediaFrame) {
    flag(output, frame.is_empty());
    [frame.size_x(), frame.size_y(), frame.format()]
        .into_iter()
        .for_each(|value| int(output, value));
    flag(output, frame.is_full_range_yuv());
    flag(output, frame.plane(0).is_none());
    int(output, frame.line_size(0));
    int(output, frame.best_effort_timestamp() as i32);
    real(output, frame.pts());
    real(output, f64::from(frame.pixel_aspect_ratio()));
    flag(output, frame.is_locked());
}

fn frames(output: &Output, code: i32) -> (Handle<MediaFrame>, Handle<MediaFrame>) {
    let (frame, other) = (
        Handle::new(MediaFrame::default()),
        Handle::new(MediaFrame::default()),
    );
    frame_values(output, &frame);
    frame.set_pts(2.5);
    frame.set_pixel_aspect_ratio(1.25);
    frame.set_locked(true);
    MediaFrame::swap(&frame, &other);
    real(output, frame.pts());
    real(output, f64::from(frame.pixel_aspect_ratio()));
    flag(output, frame.is_locked());
    flag(output, frame.frame().is_none());
    flag(output, frame.init_wrapper(&Handle::default()));
    flag(
        output,
        frame.init_wrapper(&Handle::new(ImagePixMap::default())),
    );
    int(output, MediaFrame::format_from_ffmpeg(code) as i32);
    int(output, MediaFrame::format_to_ffmpeg(ImageFormat::Rgb));
    (frame, other)
}

fn packet(output: &Output) -> Handle<MediaPacket> {
    let packet = Handle::new(MediaPacket::default());
    int(output, packet.size());
    [packet.pts(), packet.dts(), packet.duration()]
        .into_iter()
        .for_each(|v| int(output, v as i32));
    int(output, packet.stream_index());
    packet.set_key_frame();
    flag(output, packet.is_key_frame());
    flag(output, packet.data().is_none());
    packet.set_duration_seconds(0.04);
    real(output, packet.duration_seconds());
    packet
}

fn contexts(
    output: &Output,
    frame: &Handle<MediaFrame>,
    other: &Handle<MediaFrame>,
    packet: &Handle<MediaPacket>,
) {
    let scaler = MediaScaler::default();
    flag(output, scaler.init(Vec2::new(4, 4), 0, Vec2::new(2, 2), 0));
    flag(output, scaler.convert(frame, other));
    flag(output, scaler.is_valid());
    let codec = MediaCodecContext::default();
    [codec.size_x(), codec.size_y(), codec.stream_index()]
        .into_iter()
        .for_each(|value| int(output, value));
    flag(output, codec.can_process_packet(packet));
    flag(output, codec.can_process_packet(&Handle::default()));
    flag(output, codec.send_packet(packet));
    flag(output, codec.receive_frame(frame));
    flag(output, codec.context().is_none());
    let format = MediaFormatContext::default();
    int(output, format.nb_steams() as i32);
    tried(output, format.stream(0).map(|_| ()));
    text(output, format.stream_info(0, None).as_bytes());
    flag(output, format.read_packet(packet));
    flag(output, format.seek_stream(0, 1.0, false));
    flag(output, format.seek(1.0, true));
    real(output, format.pts_start_base());
    real(output, format.duration());
    flag(output, format.context().is_none());
}

fn timer_and_pool(output: &Output, r: &mut Reader<'_>, t: f64, d: f64) {
    let timer = MediaTimer::default();
    timer.seek(t);
    real(output, timer.elapsed_time());
    timer.set_playback_speed(d);
    real(output, timer.playback_speed());
    real(output, timer.elapsed_time());
    timer.pause();
    real(output, timer.elapsed_time());
    timer.stop();
    real(output, timer.elapsed_time());
    flag(output, timer.is_started());
    let pool = MediaBufferPool::default();
    for _ in 0..r.integer() {
        flag(output, pool.init(r.integer()));
        int(output, pool.buffer_size());
    }
    flag(output, pool.get_buffer().is_none());
    pool.release();
    int(output, pool.buffer_size());
}

/// `sv_media`: the time texts, a stopped timer, a buffer pool and the FFmpeg-free objects.
pub fn media_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (t, d, code) = (r.real(), r.real(), r.integer());
    text(output, MediaFormatContext::format_time(t).as_bytes());
    text(
        output,
        MediaFormatContext::format_time_progress(t, d).as_bytes(),
    );
    text(
        output,
        MediaFormatContext::format_av_error_description(code).as_bytes(),
    );
    real(
        output,
        MediaFormatContext::format_units_to_seconds(i64::from(code)),
    );
    int(output, MediaFormatContext::seconds_to_units(t) as i32);
    timer_and_pool(output, &mut r, t, d);
    let (frame, other) = frames(output, code);
    let packet = packet(output);
    contexts(output, &frame, &other, &packet);
    end(output);
}
