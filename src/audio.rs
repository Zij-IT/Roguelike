use super::raws::config::Config;
use rodio::Source;

type AudioResult<T> = Result<T, Box<dyn ::std::error::Error>>;

pub fn configure_music(configs: &Config, handle: &rodio::OutputStreamHandle) -> AudioResult<rodio::Sink> {
    let master_volume: f32 = configs.audio.master_volume as f32 / 25.0;
    let music_volume: f32 = configs.audio.music_volume as f32 / 25.0;

    let file = std::fs::File::open("./resources/audio/dungeon_sewer.ogg")?;
    let source = rodio::Decoder::new(std::io::BufReader::new(file))?.repeat_infinite();
    let music_sink = rodio::Sink::try_new(handle)?;

    music_sink.set_volume(master_volume * music_volume);
    music_sink.append(source);

    Ok(music_sink)
}

pub fn configure_sfx(configs: &Config, handle: &rodio::OutputStreamHandle) -> AudioResult<rodio::Sink> {
    let master_volume: f32 = configs.audio.master_volume as f32 / 25.0;
    let sfx_volume: f32 = configs.audio.sfx_volume as f32 / 25.0;

    let sfx_sink = rodio::Sink::try_new(handle)?;
    sfx_sink.set_volume(master_volume * sfx_volume);

    Ok(sfx_sink)
}
