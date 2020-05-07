extern crate ffmpeg;
extern crate image;

use std::io::Write;

use ffmpeg::{
  format,
  frame,
  codec,
  Packet,
};
use std::env;
use std::path::Path;


fn main() {
  ffmpeg::init().unwrap();
  match format::output(&env::args().nth(1).expect("missing output file")) {
    Ok(mut context) => {

      let global = {
        (&context).format()
          .flags()
          .contains(format::Flags::GLOBAL_HEADER)
      };

      let video_codec = codec::encoder::find(codec::Id::H264).unwrap();
      let video_codec = video_codec.video().unwrap();

      let mut stream = context.add_stream(video_codec).unwrap();
      let mut encoder = stream.codec().encoder().video().unwrap();

      if global {
        encoder.set_flags(codec::Flags::GLOBAL_HEADER);
      }

      encoder.set_time_base((1, 25));
      encoder.set_frame_rate(Some((25, 1)));
      encoder.set_width(640);
      encoder.set_height(480);
      encoder.set_bit_rate(64000);
      encoder.set_gop(12);
      encoder.set_format(format::Pixel::YUV420P);

      let in_time_base = (1, 25);
      let out_time_base = (1, 12800);
      stream.set_time_base(out_time_base);

      // 打开编码器
      let mut encoder = encoder.open_as(video_codec).unwrap();
   
      // 写头部
      context.write_header().unwrap();

      let (width, height) = (640, 480);

      let mut pts = 0..;
      let mut rgb_frame = frame::Video::new(format::Pixel::RGB24, width, height);
      let mut yuv_frame = frame::Video::new(format::Pixel::YUV420P, width, height);

      let im = image::open(&Path::new("examples/resources/test.png")).unwrap();
      let bytes = im.into_rgb().into_raw();
      rgb_frame.data_mut(0).write(&bytes).unwrap();

      let mut yuv_converter = rgb_frame.converter(format::Pixel::YUV420P).unwrap();
      let mut frame_generate_count = 0;

      for _ in 0..100 {

        let mut packet = Packet::empty();
        yuv_converter.run(&rgb_frame, &mut yuv_frame).unwrap();
        yuv_frame.set_pts(pts.next());

        encoder.send_frame(&yuv_frame).unwrap();
        while encoder.receive_packet(&mut packet) {
          frame_generate_count += 1;
          packet.rescale_ts(in_time_base, out_time_base);
          packet.write(&mut context).unwrap();
        }

      }

      encoder.flush();

      let mut packet = Packet::empty();
      while encoder.receive_packet(&mut packet) {
        frame_generate_count += 1;
        packet.rescale_ts(in_time_base, out_time_base);
        packet.write(&mut context).unwrap();
      }

      println!("frame generated count = {}", frame_generate_count);
      context.write_trailer().unwrap();
    },
    Err(error) => println!("error: {}", error),
  };
}
