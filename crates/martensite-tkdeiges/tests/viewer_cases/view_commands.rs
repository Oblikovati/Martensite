//! The viewer commands of the scripts, with what each writes to the stream.
use crate::host::ViewerHost;
use crate::scene::ViewScene;
use crate::support::session::Session as _;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::{
    Graphic3dCameraProjection, Graphic3dPresentationAttributesSource as _,
    Graphic3dTypeOfShadingModel,
};
use martensite_tkv3d::ais::AisSelectionModesConcurrency;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_tkv3d::prs3d::{Prs3dIsoAspect, Prs3dTypeOfHighlight};
use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorResultMembers as _;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// Shaded.
pub const SHADED: i32 = 1;

impl ViewerHost {
    /// `vinit`: opens the viewer the first time; the viewer's name is what the command returns.
    pub fn vinit(&mut self) {
        println!("vinit ");
        self.open_scene();
        println!("Driver1/Viewer1/View1");
    }

    /// `vinit name` of a view the viewer does not have yet.
    pub fn vinit_named(&mut self, view: &str) {
        println!("vinit {view} ");
        if let Some(scene) = self.scene.as_mut() {
            scene.open_further_view();
        } else {
            self.open_scene();
        }
        println!("Driver1/Viewer1/{view}");
    }

    /// The first viewer command also loads the driver plugin, as the scripts' logs show.
    pub fn open_scene(&mut self) {
        if self.scene.is_none() {
            println!("pload OPENGL ");
            self.scene = Some(ViewScene::open());
        }
    }

    pub fn vclear(&mut self) {
        println!("vclear ");
        if let Some(scene) = self.scene.as_mut() {
            for name in scene.object_names() {
                println!("Remove {name}");
            }
            scene.clear();
        }
    }

    /// `vdisplay name`.
    pub fn vdisplay(&mut self, name: &str) {
        println!("vdisplay {name} ");
        let shape = self.shape(name);
        self.scene
            .as_mut()
            .expect("an open viewer")
            .display_shape(name, &shape, None);
    }

    /// `vdisplay name -dispMode mode`, the options in the case's order.
    pub fn vdisplay_shaded(&mut self, name: &str, echoed: &str) {
        println!("vdisplay {echoed} ");
        let shape = self.shape(name);
        self.scene
            .as_mut()
            .expect("an open viewer")
            .display_shape(name, &shape, Some(SHADED));
    }

    pub fn vsetdispmode_shaded(&self) {
        println!("vsetdispmode 1 ");
        self.live_scene().set_default_display_mode(SHADED);
    }

    pub fn vfit(&self) {
        println!("vfit ");
        self.live_scene().fit();
    }

    pub fn vtop(&self) {
        println!("vtop ");
        self.live_scene()
            .set_orientation(V3dTypeOfOrientation::ZUP_TOP);
    }

    pub fn vbottom(&self) {
        println!("vbottom ");
        self.live_scene()
            .set_orientation(V3dTypeOfOrientation::ZUP_BOTTOM);
    }

    pub fn vright(&self) {
        println!("vright ");
        self.live_scene()
            .set_orientation(V3dTypeOfOrientation::ZUP_RIGHT);
    }

    pub fn vaxo(&self) {
        println!("vaxo ");
        self.live_scene()
            .set_orientation(V3dTypeOfOrientation::ZUP_AXO_RIGHT);
    }

    /// `vdump imagedir/name.png`: the view must have been written; how many pixels it drew.
    pub fn vdump(&self, name: &str) -> usize {
        println!("vdump {name}.png ");
        self.live_scene().dump(&self.image_stem(name))
    }

    /// `checkview -screenshot -3d -path imagedir/name.png`.
    pub fn checkview_screenshot(&self, name: &str) {
        println!("checkview -screenshot -3d -path {name}.png");
        self.vdump(name);
    }

    /// `checkview -display name -3d -path imagedir/image.png`: a name that is no shape draws nothing.
    pub fn checkview_display(&mut self, name: &str, image: &str) {
        println!("checkview -display {name} -3d -path {image}.png");
        let is_drawn = self.session.raster.shape(name).is_some();
        self.vinit();
        self.vclear();
        if is_drawn {
            self.vdisplay(name);
        }
        self.vsetdispmode_shaded();
        self.vfit();
        self.vdump(image);
    }

    /// `vselmode mode 1`: the mode is active on every displayed object, beside the others it has.
    pub fn vselmode_on(&self, mode: i32) {
        println!("vselmode {mode} 1 ");
        let scene = self.live_scene();
        for object in scene.objects() {
            scene
                .context
                .set_selection_mode_active(
                    object,
                    mode,
                    true,
                    AisSelectionModesConcurrency::GlobalOrLocal,
                    false,
                )
                .unwrap();
        }
    }

    /// `vmoveto x y`: the picked point of what is detected there, as the command returns it.
    pub fn vmoveto(&self, x: i32, y: i32) {
        println!("vmoveto {x} {y} ");
        let scene = self.live_scene();
        scene.context.move_to(x, y, scene.view(), true).unwrap();
        let point = detected_point(scene);
        println!(
            "{} {} {}",
            script_real(point.x()),
            script_real(point.y()),
            script_real(point.z())
        );
    }

    /// `vselprops dynHighlight -dispMode -1`: the highlight takes the object's own display mode.
    pub fn vselprops_dynamic_highlight_own_mode(&self) {
        println!("vselprops dynHighlight -dispMode -1 ");
        let scene = self.live_scene();
        let style = (*scene
            .context
            .highlight_style(Prs3dTypeOfHighlight::Dynamic)
            .unwrap())
        .clone();
        style
            .get()
            .expect("a highlight drawer")
            .set_display_mode(-1);
    }

    /// `vreadpixel x y rgb name`: the nearest named colour.
    pub fn vreadpixel_name(&self, x: i32, y: i32) -> &'static str {
        println!("vreadpixel {x} {y} rgb name ");
        let color = self.live_scene().pixel(x, y).expect("a pixel in the view");
        let name = Color::string_name(color.name());
        println!("{name}");
        name
    }

    /// `checkcolor x y r g b`: the pixel or one around it has the colour; `$stat` is what it returns.
    pub fn checkcolor(&self, x: i32, y: i32, expected: [f64; 3]) -> bool {
        let [r, g, b] = expected;
        println!("Coordinate x = {x}\nCoordinate y = {y}");
        println!("RED color of RGB is {r}\nGREEN color of RGB is {g}\nBLUE color of RGB is {b}");
        let wanted = expected.map(scaled);
        let off = self.channels_off(x, y, wanted);
        for (name, _) in ["RED", "GREEN", "BLUE"]
            .iter()
            .zip(off)
            .filter(|(_, off)| *off)
        {
            println!("Warning : {name} light of additive color model RGB is invalid");
        }
        if !off.contains(&true) {
            return true;
        }
        println!("Warning : Colors of default coordinate are not equal");
        self.search_neighbourhood(x, y, wanted)
    }

    fn channels_off(&self, x: i32, y: i32, wanted: [i64; 3]) -> [bool; 3] {
        println!("vreadpixel {x} {y} rgb ");
        let color = self.live_scene().pixel(x, y).expect("a pixel in the view");
        let channels = [color.red(), color.green(), color.blue()];
        println!(
            "{} {} {}",
            script_real(channels[0]),
            script_real(channels[1]),
            script_real(channels[2])
        );
        let actual = channels.map(scaled);
        [0, 1, 2].map(|at| channel_off(wanted[at], actual[at]))
    }

    fn search_neighbourhood(&self, x: i32, y: i32, wanted: [i64; 3]) -> bool {
        for row in 0..=5 {
            for column in 0..=5 {
                let (at_x, at_y) = (x - 2 + column, y - 2 + row);
                println!("{at_x}\n{at_y}");
                if !self.channels_off(at_x, at_y, wanted).contains(&true) {
                    println!("Point with valid color was found");
                    return true;
                }
                println!("Warning : Point with true color was not found near default coordinates");
            }
        }
        println!(
            "Error : Colors are not equal in default coordinate and in the near coordinates too"
        );
        false
    }

    /// `vaspects name -faceBoundaryDraw 1`.
    pub fn vaspects_face_boundary(&self, name: &str) {
        println!("vaspects {name} -faceBoundaryDraw 1 ");
        let scene = self.live_scene();
        let object = scene.object(name);
        object
            .get()
            .unwrap()
            .attributes()
            .get()
            .unwrap()
            .set_face_boundary_draw(true);
        scene.context.redisplay(object, false, false).unwrap();
        scene.context.update_current_viewer();
    }
}

/// Channels compare as integers scaled by 1e5, with a relative tolerance in integer arithmetic, as
/// `checkcolor` does: any difference below the expected value passes.
fn channel_off(expected: i64, actual: i64) -> bool {
    let tolerance = if expected != 0 {
        (expected - actual).abs() / expected
    } else {
        actual
    };
    tolerance as f64 > 0.2
}

fn scaled(value: f64) -> i64 {
    (value * 1.0e5) as i64
}

fn detected_point(scene: &ViewScene) -> Point3 {
    let owner = scene.context.detected_owner();
    let selector = scene.context.main_selector();
    for rank in 1..=selector.nb_picked() {
        if selector.picked(rank).unwrap() == *owner {
            return selector.picked_point(rank).unwrap();
        }
    }
    Point3::new(f64::MAX, f64::MAX, f64::MAX)
}

/// A real as the interpreter prints it: `%.17g`.
pub fn script_real(value: f64) -> String {
    let text = format!("{value:.16e}");
    let (mantissa, exponent) = text.split_once('e').expect("an exponent");
    let exponent: i32 = exponent.parse().expect("an exponent value");
    if !(-5..17).contains(&exponent) {
        let digits = mantissa.trim_end_matches('0').trim_end_matches('.');
        let sign = if exponent < 0 { '-' } else { '+' };
        return format!("{digits}e{sign}{:02}", exponent.abs());
    }
    let fixed = format!("{value:.*}", (16 - exponent) as usize);
    if fixed.contains('.') {
        return fixed.trim_end_matches('0').trim_end_matches('.').to_owned();
    }
    fixed
}

impl ViewerHost {
    /// `vrenderparams -shadingModel PHONG`.
    pub fn vrenderparams_phong(&self) {
        println!("vrenderparams -shadingModel PHONG ");
        let scene = self.live_scene();
        scene
            .view()
            .view_base()
            .set_shading_model(Graphic3dTypeOfShadingModel::Phong)
            .unwrap();
        scene.view().redraw();
    }

    /// `vlight -change index -intensity value`.
    pub fn vlight_intensity(&self, index: usize, value: f32) {
        println!("vlight -change {index} -intensity {value} ");
        let scene = self.live_scene();
        let lights = scene.view().view_base().active_lights();
        let light = lights
            .get(index)
            .unwrap_or_else(|| panic!("vlight {index}: no such light"));
        light.get().unwrap().set_intensity(value).unwrap();
        scene.view().redraw();
    }

    /// `vcamera -orthographic`.
    pub fn vcamera_orthographic(&self) {
        println!("vcamera -orthographic ");
        let scene = self.live_scene();
        scene
            .view()
            .view_base()
            .camera()
            .get()
            .unwrap()
            .set_projection_type(Graphic3dCameraProjection::Orthographic);
        scene.view().redraw();
    }
}

impl ViewerHost {
    /// `vdisplay -dispMode mode name`.
    pub fn vdisplay_in_mode(&mut self, name: &str, mode: i32) {
        println!("vdisplay -dispMode {mode} {name} ");
        let shape = self.shape(name);
        self.scene
            .as_mut()
            .expect("an open viewer")
            .display_shape(name, &shape, Some(mode));
    }

    /// `vsetdispmode name mode`.
    pub fn vsetdispmode_of(&self, name: &str, mode: i32) {
        println!("vsetdispmode {name} {mode} ");
        let scene = self.live_scene();
        scene
            .context
            .set_display_mode(scene.object(name), mode, true)
            .unwrap();
    }

    /// `vsetdispmode mode`.
    pub fn vsetdispmode_all(&self, mode: i32) {
        println!("vsetdispmode {mode} ");
        self.live_scene().set_default_display_mode(mode);
    }

    /// `vsetcolor name color`.
    pub fn vsetcolor(&self, name: &str, word: &str, color: Color) {
        println!("vsetcolor {name} {word} ");
        let scene = self.live_scene();
        scene
            .context
            .set_color(scene.object(name), color, true)
            .unwrap();
    }

    /// `visos name u v 1`: the numbers of isolines of one object, on planes too.
    pub fn visos(&self, name: &str, u_isos: i32, v_isos: i32) {
        println!("visos {name} {u_isos} {v_isos} 1 ");
        println!("New number of isos : {u_isos} {v_isos}");
        println!("New IsoOnPlane mode is ON");
        let scene = self.live_scene();
        scene.context.default_drawer().set_iso_on_plane(true);
        let object = scene.object(name);
        let live = object.get().unwrap();
        let attributes = live.attributes();
        let drawer = attributes.get().unwrap();
        // A copy of each aspect with the number asked for, as the command makes one; isolines are gray.
        let iso = |aspect: Handle<Prs3dIsoAspect>, number: i32| {
            let old = aspect.get().unwrap();
            let line = old.line_aspect().aspect();
            let line = line.get().unwrap();
            let gray = Color::named(NamedColor::GRAY75);
            let width = f64::from(line.width());
            Handle::new(Prs3dIsoAspect::new(gray, line.line_kind(), width, number).unwrap())
        };
        drawer.set_u_iso_aspect(iso(drawer.u_iso_aspect(), u_isos));
        drawer.set_v_iso_aspect(iso(drawer.v_iso_aspect(), v_isos));
        scene
            .context
            .set_local_attributes(object, &attributes, false)
            .unwrap();
        scene.context.redisplay(object, false, false).unwrap();
    }

    /// `vinit name` of the view that is open: it asks for the view, which is active already.
    pub fn vinit_again(&self, view: &str) {
        println!("vinit {view} ");
        println!("vactivate Driver1/Viewer1/{view} ");
        println!("vactivate: the view is active already");
        println!("vactivate: the view is active already");
    }

    /// `vinit` with a viewer open: a further view, the second one of the case.
    pub fn vinit_second(&mut self) {
        println!("vinit ");
        self.scene
            .as_mut()
            .expect("an open viewer")
            .open_further_view();
        println!("Driver1/Viewer1/View2");
    }

    /// `vreadpixel x y name`: the nearest named colour and its alpha.
    pub fn vreadpixel_name_only(&self, x: i32, y: i32) -> String {
        println!("vreadpixel {x} {y} name ");
        let color = self.live_scene().pixel(x, y).expect("a pixel in the view");
        let text = format!("{} 1", Color::string_name(color.name()));
        println!("{text}");
        text
    }

    /// `vzoom coefficient`.
    pub fn vzoom(&self, coefficient: f64) {
        println!("vzoom {} ", script_real(coefficient));
        self.live_scene()
            .view()
            .set_zoom(coefficient, true)
            .unwrap();
    }

    /// `vviewparams -scale s -proj x y z -up x y z -at x y z`; `words` is what the case wrote.
    pub fn vviewparams(&self, words: &str, scale: f64, proj: [f64; 3], up: [f64; 3], at: [f64; 3]) {
        println!("vviewparams {words} ");
        let view = self.live_scene().view();
        view.set_scale(scale).unwrap();
        view.set_at(at[0], at[1], at[2]).unwrap();
        view.set_proj(proj[0], proj[1], proj[2]).unwrap();
        view.set_up(up[0], up[1], up[2]).unwrap();
        view.redraw();
    }
}
