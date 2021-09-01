use gst::prelude::*;
use ges::prelude::*;

pub struct Editor {
    timeline: crate::timeline::Timeline,
}

fn generate_render_profile(output: &str) -> gst_pbutils::EncodingProfile {
    let discoverer = gst_pbutils::Discoverer::new(gst::ClockTime::SECOND * 60).unwrap();
    let discoverer_info = discoverer.discover_uri(&format!("file:///{}", output)).unwrap();

    gst_pbutils::EncodingProfile::from_discoverer(&discoverer_info).unwrap()
}

impl Editor {
    pub fn new(timeline: crate::timeline::Timeline) -> Editor {
        Editor {
            timeline: timeline
        }
    } 

    pub fn init(&mut self) {
        gst::init().expect("Failed to initialize gstreamer");
        ges::init().expect("Failed to initialize gstreamer editing services");

        for x in &mut self.timeline.clips {
            x.clip = Some(ges::UriClip::new(&format!("file:///{}", x.path)).unwrap());

            x.clip.as_ref().unwrap().set_inpoint(gst::ClockTime::from_seconds(x.inpoint));
            x.clip.as_ref().unwrap().set_duration(gst::ClockTime::from_seconds(x.duration));
            x.clip.as_ref().unwrap().set_start(gst::ClockTime::from_seconds(x.start));
        }
    }

    pub fn compile(&self, output: &str) {
        let ges_timeline = ges::Timeline::new_audio_video();
        let mut target_src: &str = "";

        let video_layer = ges_timeline.append_layer();
        let audio_layer = ges_timeline.append_layer();

        let pipeline = ges::Pipeline::new();
        pipeline.set_timeline(&ges_timeline).expect("Error compiling: cannot set timeline");

        for x in &self.timeline.clips {
            if x.is_render_target {
                target_src = &x.path;
            }
            match (x.is_video, x.is_audio) {
                (true, true) => {
                    video_layer.add_clip(x.clip.as_ref().unwrap()).unwrap();
                },

                (true, false) => {
                    video_layer.add_clip(x.clip.as_ref().unwrap()).unwrap();
                    x.clip.as_ref().unwrap().set_mute(true);
                },

                (false, true) => {
                    audio_layer.add_clip(x.clip.as_ref().unwrap()).unwrap()  
                },

                (false, false) => continue,
            } 
        }

        ges_timeline.commit();

        let profile = generate_render_profile(target_src);
        pipeline.set_render_settings(&format!("file:///{}", output), &profile).unwrap();

        pipeline.set_mode(ges::PipelineFlags::RENDER).expect("Could not render");

        pipeline
            .set_state(gst::State::Playing)
            .unwrap();

        // Wait until error or EOS
        let bus = pipeline.bus().unwrap();
        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}",
                        err
                    );
                    break;
                }
                _ => (),
            }
        }

        // Shutdown pipeline
        pipeline
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
        } 
}