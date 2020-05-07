extern crate ffmpeg;
extern crate image;

use std::env;

fn main() {
  ffmpeg::init().unwrap();
  match ffmpeg::format::input(&env::args().nth(1).expect("missing file")) {
    Ok(mut context) => {
      println!("duration = {}", context.duration() as f64 / 1000.0);

      let snapshot_frame_index = 5;
      
      match context.streams().best(ffmpeg::media::Type::Video) {
        Some(stream) => {
          println!("Best video stream index: {}, duration: {}, frames: {}", 
            stream.index(), stream.duration(), stream.frames());

          let index = stream.index();
          let codec_context = stream.codec();
          let mut codec_video = codec_context.decoder().video().unwrap();

          let mut i = 0;
          let mut rgb_frame = ffmpeg::frame::Video::empty();

          for (s, p) in context.packets() {
            println!("packets: {:?} duration: {}", p.pts(), p.duration());

            if s.index() == index {
              let mut frame = ffmpeg::util::frame::Video::empty();
              match codec_video.decode(&p, &mut frame) {
                Ok(_) => {
                  println!("decode success");
                  i += 1;
                  if i != snapshot_frame_index {
                    continue;
                  }
                  let name = format!("frame{}.png", i);
                  let mut rgb_converter = frame.converter(ffmpeg::format::Pixel::RGB24).unwrap();
                  rgb_converter.run(&frame, &mut rgb_frame).unwrap();
                  let buffer = rgb_frame.data(0);

                  image::save_buffer(name, buffer, frame.width(), frame.height(), 
                    image::ColorType::Rgb8).unwrap()
                },
                Err(error) => {
                  println!("decode error: {}", error);
                }
              }
            }
          }

          println!("video width = {}, height = {}", codec_video.width(), codec_video.height());
        },
        None => {
          println!("No video stream found");
        },
      };
    },
    Err(error) => println!("error: {}", error),
  };
}
