mod command;

use video_editor_lib::editor::Editor;
use std::env;

extern crate video_editor_lib;
fn main() {
    // output_path src_path inpoint duration start is_video is_audio is_render_target [src_path inpoint duration start is_video is_audio is_render_target]
    let args: Vec<String> = env::args().collect();
    let (output, timeline) = command::process_command_line(&args).unwrap();

    let mut editor = Editor::new(timeline);

    editor.init();

    editor.compile(output);

    println!("Succesfully Compiled: {}", output);
}
