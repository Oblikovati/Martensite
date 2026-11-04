//! `propeller`: four restored engine parts under a camera tour, the propeller and crank arm
//! turned by object animations, with frame callbacks that print the presentation time in a
//! screen-corner label and fade the propeller in and out.
use super::camera_moves::{CameraSet, camera_move};
use super::command_animation::{CommandAnimation, FrameCommand, substituted, substituted_text};
use super::prelude::*;
use super::propeller_keys::{ARM_KEYS, CAMERA_MOVES, CameraEnd, PROPELLER_TURNS};
use super::timeline::{play_at, schedule, timeline};
use crate::case_view::viewer_session::{ViewTrihedronSettings, ViewerSession};
use crate::dataset_shape::read_dataset_shape;
use crate::session_run::ScriptedControl;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkmath::gp::{Quaternion, Transform3, Vector3};
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_TOP_OSD, Graphic3dCameraProjection, Graphic3dTransModeFlags,
    Graphic3dTransformPers,
};
use martensite_tkv3d::ais::{
    AisAnimationMembers as _, AisAnimationObject, AisAnimationSource, AisInteractiveContext,
    AisInteractiveObject, AisTextLabel,
};
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _,
};
use std::sync::atomic::{AtomicBool, Ordering};

type Animation = Handle<dyn AisAnimationSource>;

/// `propeller`: eleven frames from 0 to 2 seconds.
pub(crate) fn propeller_tour() -> Outcome {
    banner(&[
        "==================================",
        "Viewer animation - Propeller animation",
        "Regressions should be detected visually",
        "==================================",
    ]);
    let mut run = SessionRun::with_view_size(912, 512)?;
    engine_scene(run.session())?;
    let root = timeline("anim");
    let label = AisTextLabel::new();
    schedule_commands(run.session(), &root, &label)?;
    schedule_camera(run.session(), &root)?;
    schedule_turns(run.session(), &root)?;
    for (i, image) in FRAMES.into_iter().enumerate() {
        play_at(run.session(), &root, 2.0 * i as f64 / 10.)?;
        if image == WALL_CLOCK_FRAME {
            dump_with_label_control(&mut run, image, &label)?;
        } else {
            run.dump(image)?;
        }
    }
    println!("Put the following command to start interactive animation:");
    println!("    vanimation anim -play -playSpeed 0.2");
    run.finish()
}

const FRAMES: [&str; 11] = [
    "propeller_00.png",
    "propeller_01.png",
    "propeller_02.png",
    "propeller_03.png",
    "propeller_04.png",
    "propeller_05.png",
    "propeller_06.png",
    "propeller_07.png",
    "propeller_08.png",
    "propeller_09.png",
    "propeller_10.png",
];

/// The last frame: native's label there reads wall-clock time (the playback holder's running
/// timer), so the batch masks the label by its footprint (user decision 2026-10-29).
const WALL_CLOCK_FRAME: &str = "propeller_10.png";

/// Dump `image`, then its `nolabel` control with the time label erased;
/// the label is displayed again afterwards.
fn dump_with_label_control(
    run: &mut SessionRun,
    image: &'static str,
    label: &Handle<AisTextLabel>,
) -> Outcome {
    let object: Handle<dyn AisInteractiveObject> = label.clone().cast();
    let control = ScriptedControl {
        suffix: "nolabel",
        apply: &|session| Ok(session.context().erase(&object, true)?),
        restore: &|session| Ok(session.context().display(&object, true)?),
    };
    run.dump_with_scripted_controls(image, &[control])
}

/// The restored arm, head, propeller and block, shaded and mutable, in a fitted perspective
/// view (60 degree field) under the view trihedron, coloured for the case.
fn engine_scene(session: &mut ViewerSession) -> Outcome {
    session.clear()?;
    session.show_view_trihedron(&ViewTrihedronSettings::default())?;
    session.set_projection(Graphic3dCameraProjection::Perspective)?;
    let view = session.view().view_base();
    view.camera().get().ok_or("a view camera")?.set_fov_y(60.);
    let request = DisplayRequest::default().with_display_mode(1).mutable();
    for (name, file) in [
        ("a", "occ/CrankArm.rle"),
        ("h", "occ/CylinderHead.rle"),
        ("p", "occ/Propeller.rle"),
        ("e", "occ/EngineBlock.rle"),
    ] {
        session.display(name, &read_dataset_shape(file)?, &request)?;
    }
    session.fit()?;
    for (name, color) in [
        ("p", NamedColor::GREEN),
        ("e", NamedColor::RED),
        ("h", NamedColor::PURPLE),
    ] {
        session.set_color(name, Color::named(color))?;
    }
    Ok(())
}

/// A group `name` of the root timeline, as a path component of a child name creates it.
fn group(root: &Animation, name: &str) -> Result<Animation, Failure> {
    let child = timeline(name);
    root.get().expect("a root animation").add(&child)?;
    Ok(child)
}

/// `anim/cmd/1..3`: the time label for two seconds; the propeller fading out over 0.1 s from
/// 1 s and back in from 1.5 s.
fn schedule_commands(
    session: &ViewerSession,
    root: &Animation,
    label: &Handle<AisTextLabel>,
) -> Outcome {
    let commands = group(root, "cmd")?;
    let context = session.context().clone();
    let propeller = session
        .object("p")
        .cloned()
        .ok_or("the displayed propeller p")?;
    schedule(&commands, time_label(&context, label), 0., 2.0)?;
    let fade_out = fade(&context, &propeller, |n| n * 0.9);
    schedule(
        &commands,
        CommandAnimation::scheduled("2", fade_out),
        1.0,
        0.1,
    )?;
    let fade_in = fade(&context, &propeller, |n| (1.0 - n) * 0.9);
    schedule(
        &commands,
        CommandAnimation::scheduled("3", fade_in),
        1.5,
        0.1,
    )?;
    Ok(())
}

/// The "Current PTS" text at every frame: a
/// label kept 30 px from the upper-left corner in the top overlay layer, displayed the first
/// time and redisplayed with its new text afterwards.
fn time_label(context: &Handle<AisInteractiveContext>, label: &Handle<AisTextLabel>) -> Animation {
    let (context, label) = (context.clone(), label.clone());
    let shown = AtomicBool::new(false);
    let command: FrameCommand = Box::new(move |progress| {
        let text = format!("Current PTS: {} sec", substituted_text(progress.pts));
        label.set_text(&ExtendedString::new(&text));
        let corner = Graphic3dTransformPers::with_corner(
            Graphic3dTransModeFlags::TMF_2D,
            Corner::LEFT_UPPER,
            Vec2::new(30, 30),
        )?;
        let object: Handle<dyn AisInteractiveObject> = label.clone().cast();
        context.set_transform_persistence(&object, &Handle::new(corner).cast())?;
        label.set_z_layer(GRAPHIC3D_ZLAYER_ID_TOP_OSD);
        if shown.swap(true, Ordering::Relaxed) {
            return context.redisplay(&object, false, true);
        }
        context.display(&object, false)
    });
    CommandAnimation::scheduled("1", command)
}

/// Set the transparency of `p` from the normalized local time, without an update.
fn fade(
    context: &Handle<AisInteractiveContext>,
    object: &Handle<dyn AisInteractiveObject>,
    transparency: fn(f64) -> f64,
) -> FrameCommand {
    let (context, object) = (context.clone(), object.clone());
    Box::new(move |progress| {
        let value = transparency(substituted(progress.local_normalized));
        context.set_transparency(&object, value, false)?;
        object
            .get()
            .expect("a displayed object")
            .synchronize_aspects();
        Ok(())
    })
}

/// `anim/cam/1..4`: the camera tour between the listed ends.
fn schedule_camera(session: &ViewerSession, root: &Animation) -> Outcome {
    let cameras = group(root, "cam")?;
    for (index, (start, duration, first, second)) in CAMERA_MOVES.iter().enumerate() {
        let name = (index + 1).to_string();
        let animation = camera_move(session, &name, &settings(first), &settings(second))?;
        schedule(&cameras, animation.cast(), *start, *duration)?;
    }
    Ok(())
}

/// The settings of one end in order: up, centre, eye, scale.
fn settings(end: &CameraEnd) -> [CameraSet; 4] {
    [
        CameraSet::Up(end.up),
        CameraSet::At(end.at),
        CameraSet::Eye(end.eye),
        CameraSet::Scale(end.scale),
    ]
}

/// `anim/p/1..4` (half a second each) and `anim/a/1..64` (1/32 s each): placements of the
/// propeller and the crank arm between consecutive key frames.
fn schedule_turns(session: &ViewerSession, root: &Animation) -> Outcome {
    let propeller = group(root, "p")?;
    for (k, pair) in PROPELLER_TURNS.windows(2).enumerate() {
        let ends = [
            [pair[0][0], pair[0][1], 0., 0., 0.],
            [pair[1][0], pair[1][1], 0., 0., 0.],
        ];
        let turn = placement_move(session, &(k + 1).to_string(), "p", ends)?;
        schedule(&propeller, turn, k as f64 * 0.5, 0.5)?;
    }
    let arm = group(root, "a")?;
    for (k, pair) in ARM_KEYS.windows(2).enumerate() {
        let swing = placement_move(session, &(k + 1).to_string(), "a", [pair[0], pair[1]])?;
        schedule(&arm, swing, k as f64 * 0.03125, 0.03125)?;
    }
    Ok(())
}

/// A placement animation between two rotations and locations, the scale kept from the
/// object's local transformation.
fn placement_move(
    session: &ViewerSession,
    child: &str,
    name: &str,
    ends: [[f64; 5]; 2],
) -> Result<Animation, Failure> {
    let object = session.object(name).cloned().unwrap_or_default();
    let scale = object
        .get()
        .map_or(1., |o| o.local_transformation().scale_factor());
    let placement = |[z, w, x, y, h]: [f64; 5]| -> Result<Transform3, Failure> {
        let mut trsf = Transform3::default();
        trsf.set_rotation_quaternion(Quaternion::new(0., 0., z, w));
        trsf.set_translation_part(Vector3::new(x, y, h));
        trsf.set_scale_factor(scale)?;
        Ok(trsf)
    };
    let (start, end) = (placement(ends[0])?, placement(ends[1])?);
    let animation = AisAnimationObject::new(child, session.context(), &object, start, end);
    Ok(Handle::new(animation).cast())
}
