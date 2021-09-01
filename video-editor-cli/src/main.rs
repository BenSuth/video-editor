use video_editor_lib::editor::Editor;

extern crate video_editor_lib;
fn main() {
    let mut vector = Vec::new();

    let timeline = video_editor_lib::timeline::Timeline {
        clips: vector
    };

    let mut editor = Editor::new(timeline);

    editor.init();

    editor.compile("C:/Users/suthe/Desktop/Source/Test.mp4");
}
