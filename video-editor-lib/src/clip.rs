#[derive(Clone)]
pub struct Clip {
    pub path: String,
    pub inpoint: u64,
    pub duration: u64,
    pub start: u64,
    pub is_video: bool,
    pub is_audio: bool,
    pub is_render_target: bool,
    pub clip: Option<ges::UriClip>
}

impl Clip {
    pub fn new(path: &str, inpoint: u64, duration: u64, start: u64, is_video: bool, is_audio: bool, is_render_target: bool) -> Clip {
        Clip {
            path: path.to_string(),
            inpoint: inpoint,
            duration: duration,
            start: start,
            is_video: is_video,
            is_audio: is_audio,
            is_render_target: is_render_target,
            clip: None
        }
    }

    pub fn new_template() -> Clip {
        Clip {
            path: "".to_string(),
            inpoint: 0,
            duration: 0,
            start: 0,
            is_video: false,
            is_audio: false,
            is_render_target: false,
            clip: None
        }
    }
}