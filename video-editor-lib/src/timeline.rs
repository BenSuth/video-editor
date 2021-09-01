use crate::clip;

#[derive(Clone)]
pub struct Timeline {
    pub clips: Vec<clip::Clip>
}