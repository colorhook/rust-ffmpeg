# rust-ffmpeg

基于 ffmpeg v4.2.2 改造

## example

设备信息和编解码列表

```
cargo run --example format
```

编解码详情

```
cargo run --example codec-info h264
```

查看音视频文件元信息

```
cargo run --example metadata examples/resources/audio.ogg
```

视频截帧

```
cargo run --example snapshot examples/resources/mov.mp4
```

编码生成视频

```
cargo run --example generate test.mp4
```