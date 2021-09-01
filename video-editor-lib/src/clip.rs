use ges::prelude::*;

#[derive(Clone)]
pub struct Clip {
    pub name: String,
    pub inpoint: u64,
    pub duration: u64,
    pub start: u64,
    pub is_audio: bool,
    pub is_render_target: bool,
    clip: Option<ges::UriClip>
}

impl Clip {
    pub fn new(name: &str, inpoint: u64, duration: u64, start: u64, is_audio: bool, is_render_target: bool) -> Clip {
        Clip {
            name: name.to_string(),
            inpoint: inpoint,
            duration: duration,
            start: start,
            is_audio: is_audio,
            is_render_target: is_render_target,
            clip: None
        }
    }
}