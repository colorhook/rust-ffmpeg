extern crate ffmpeg;

fn main() {
    ffmpeg::init().unwrap();

    println!("device input audio ---");
    for i in ffmpeg::device::input::audio() {
      println!("name: {}",  i.name());
      println!("description: {}",  i.description());
      println!("extensions: {:?}",  i.extensions());
      println!("mime_types: {:?}",  i.mime_types());
    }
    println!("device input video ---");
    for i in ffmpeg::device::input::video() {
      println!("name: {}",  i.name());
      println!("description: {}",  i.description());
      println!("extensions: {:?}",  i.extensions());
      println!("mime_types: {:?}",  i.mime_types());
    }
    println!("device output audio ---");
    for i in ffmpeg::device::output::audio() {
      println!("name: {}",  i.name());
      println!("description: {}",  i.description());
      println!("extensions: {:?}",  i.extensions());
      println!("mime_types: {:?}",  i.mime_types());
    }
    println!("device output video ---");
    for i in ffmpeg::device::output::video() {
      println!("name: {}",  i.name());
      println!("description: {}",  i.description());
      println!("extensions: {:?}",  i.extensions());
      println!("mime_types: {:?}",  i.mime_types());
    }

    println!("format list:");

    for i in ffmpeg::format::list() {
      println!("name: {}, description: {}",  i.name(), i.description());
    }
}
