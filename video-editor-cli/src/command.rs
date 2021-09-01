extern crate video_editor_lib;

use std::fmt;

#[derive(Debug)]
pub struct InvalidArgument;

impl fmt::Display for InvalidArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid command line arguments!")
    }
}

pub fn process_command_line(args: &Vec<String>) -> Result<(&str, video_editor_lib::timeline::Timeline), InvalidArgument> {
    let mut counter = 0;
    let mut output: &str = "";
    let mut clip = video_editor_lib::clip::Clip::new_template();
    let mut clips: Vec<video_editor_lib::clip::Clip> = Vec::new();

    for x in args {
        match counter {
            0 => {
                counter += 1;
                continue
            }
            1 => {
                output = &x;
                counter += 1;
            },
            2 => {
                clip.path = x.to_string();
                counter += 1;
            },
            3 => {
                clip.inpoint = match x.parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => {
                        print!("Error: {} not a valid digit", x);
                        return Err(InvalidArgument);
                    }
                };
                counter += 1;
            }
            4 => {
                clip.duration = match x.parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => {
                        print!("Error: {} not a valid digit", x);
                        return Err(InvalidArgument);
                    }
                };
                counter += 1;  
            },
            5 => {
                clip.start = match x.parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => {
                        print!("Error: {} not a valid digit", x);
                        return Err(InvalidArgument);
                    }
                };
                counter += 1;
            },
            6 => {
                if x == "Y" {
                    clip.is_video = true;
                } else if x == "N" {
                    clip.is_video = false;
                } else { 
                    return Err(InvalidArgument);
                }
                counter += 1;
            }
            7 => {
                if x == "Y" {
                    clip.is_audio = true;
                } else if x == "N" {
                    clip.is_audio = false;
                } else { 
                    return Err(InvalidArgument);
                }
                counter += 1;
            },
            8 => {
                if x == "Y" {
                    clip.is_render_target = true;
                } else if x == "N" {
                    clip.is_render_target = false;
                } else { 
                    return Err(InvalidArgument);
                }
                counter = 2;
                clips.push(video_editor_lib::clip::Clip::new(&clip.path, clip.inpoint, clip.duration, clip.start, clip.is_video, clip.is_audio, clip.is_render_target))
            },
            _ => {
                return Err(InvalidArgument);
            }
        }        
    };
    
    if clips.len() == 0 {
        return Err(InvalidArgument);
    }

    let timeline = video_editor_lib::timeline::Timeline {
        clips: clips
    };

    return Ok((output, timeline));
}  
