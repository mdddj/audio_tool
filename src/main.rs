use clap::Parser;
use rodio::{Decoder, Source};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let config = Config::parse();
    let file = config.file;
    if config.covert {
        let d = get_audio_dur(&file);
        let f = convert_to_minutes_and_seconds(d);
        println!("{}:{}",f.0,f.1);
    } else {
        let d = get_audio_dur(&file);
        println!("{}", d);
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
struct Config {
    #[clap(long, short, value_parser)]
    file: String,

    #[clap(long, short, default_value = "false")]
    covert: bool,
}

fn convert_to_minutes_and_seconds(millis: u128) -> (u32, u32) {
    let seconds = millis as f32 / 1000.0;
    let minutes = (seconds / 60.0) as u32;
    let remaining_seconds = ((seconds % 60.0) + 0.5).floor() as u32; // 四舍五入加 1
    (minutes, remaining_seconds)
}

///获取歌曲时长
fn get_audio_dur(path: &str) -> u128 {
    let file = File::open(path).expect("Failed to open file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

    // 获取时长
    let duration = source.total_duration().expect("Failed to get duration");
    duration.as_millis()
}

#[cfg(test)]
mod test {
    use crate::{convert_to_minutes_and_seconds, get_audio_dur};

    #[test]
    fn test_get_duration() {
        let flac1 = "/Users/ldd/Music/歌曲/flac/02.The Magnificent Seven 七宗罪 - 辛辛那提流行乐管弦乐队演奏.flac"; //30m
        let mp31 = "/Users/ldd/Music/歌曲/14.春江花月夜[歌手]赵聪.mp3";
        let dur = get_audio_dur(flac1);
        println!("{:#?}", convert_to_minutes_and_seconds(dur));
        let dur2 = get_audio_dur(mp31);
        println!("{:#?}", convert_to_minutes_and_seconds(dur2));
    }
}
