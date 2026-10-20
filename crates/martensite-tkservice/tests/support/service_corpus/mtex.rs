//! Graphic3d_MediaTexture and Graphic3d_MediaTextureSet without FFmpeg.
use super::common::{Reader, end, flag, int, real, text};
use super::print::Output;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::graphic3d::{
    Graphic3dMediaTexture, Graphic3dMediaTextureSet, Graphic3dShaderProgram, Graphic3dTextureRoot,
    Graphic3dTextureRootMembers,
};
use martensite_tkservice::image::ImageSupportedFormats;

fn program(output: &Output, program: &Handle<Graphic3dShaderProgram>) {
    let Some(program) = program.get() else { return };
    text(output, program.header().as_bytes());
    int(output, program.nb_lights_max());
    int(output, program.nb_clip_planes_max());
    for shader in program.shader_objects().iter().filter_map(Handle::get) {
        int(output, shader.shader_type() as i32);
        text(output, shader.source().as_bytes());
    }
}

fn textures(output: &Output, set: &Graphic3dMediaTextureSet) {
    int(output, set.size());
    for texture in set.textures() {
        let texture = texture.cast::<Graphic3dMediaTexture>();
        let Some(texture) = texture.get() else {
            continue;
        };
        let params = texture.get_params();
        if let Some(params) = params.get() {
            int(output, params.texture_unit() as i32);
            flag(output, params.is_modulate());
            flag(output, params.is_repeat());
            int(output, params.filter() as i32);
        }
        flag(output, texture.frame().is_null());
        flag(
            output,
            texture
                .get_image(&Handle::new(ImageSupportedFormats::default()))
                .is_null(),
        );
    }
}

fn frame_queue(output: &Output, set: &Graphic3dMediaTextureSet, nb_locks: i32) {
    let queue = set.frame_queue();
    for index in 0..nb_locks {
        let frame = queue.lock_frame();
        flag(output, frame.is_null());
        let Some(value) = frame.get() else { continue };
        flag(output, value.is_locked());
        flag(output, queue.lock_frame().is_null());
        if index % 2 == 0 {
            queue.release_frame(&frame);
            flag(output, value.is_locked());
        }
    }
}

/// `sv_mtex`: a media texture set, its programs and frame queue, then the swap without player.
pub fn mtex_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let nb_locks = r.integer();
    let Ok(set) = Graphic3dMediaTextureSet::new() else {
        return;
    };
    textures(output, &set);
    program(output, &set.shader_yuv());
    program(output, &set.shader_yuvj());
    flag(output, set.shader_program().is_null());
    flag(output, set.is_planar_yuv());
    flag(output, set.is_full_range_yuv());
    frame_queue(output, &set, nb_locks);
    flag(output, set.swap_frames());
    flag(output, set.player_context().is_null());
    set.open_input(&AsciiString::default(), false);
    text(output, set.input().as_bytes());
    flag(output, set.player_context().is_null());
    let size = set.frame_size();
    int(output, size.x);
    int(output, size.y);
    real(output, set.progress());
    real(output, set.duration());
    end(output);
}
