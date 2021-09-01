extern crate video_editor_lib;
fn main() {
    let test = video_editor_lib::clip::Clip::new("bruh", 0, 0, 0, true, true);

    let test2 = video_editor_lib::clip::Clip::new("bruh", 0, 0, 0, true, true);
    
    let mut vector = Vec::new();
    vector.push(test);
    vector.push(test2);

    let timeline = video_editor_lib::timeline::Timeline {
        clips: vector
    };
}
