//! Graphic3d_CLight and Graphic3d_LightSet.
use super::common::{Reader, dir, end, flag, int, pnt, real, text};
use super::pbr::vec4 as vec4f;
use super::print::Output;
use super::print::show;
use super::tried;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::{
    Graphic3dCLight, Graphic3dLightSet, Graphic3dLightSetIterationFilter,
    Graphic3dLightSetIterator, Graphic3dTypeOfLightSource,
};

fn light_type(value: i32) -> Graphic3dTypeOfLightSource {
    use Graphic3dTypeOfLightSource as T;
    [T::Ambient, T::Directional, T::Positional, T::Spot][value as usize]
}

fn rgb(r: &mut Reader<'_>) -> Color {
    let [red, green, blue] = [0; 3].map(|_| r.real());
    Color::new(red, green, blue, ColorSpace::Rgb).unwrap_or_default()
}

fn light_state(output: &Output, l: &Graphic3dCLight) {
    vec4f(output, l.packed_params());
    vec4f(output, l.packed_color());
    vec4f(output, l.packed_direction_range());
    pnt(output, l.position());
    real(output, f64::from(l.smoothness()));
    real(output, f64::from(l.intensity()));
    int(output, l.revision() as i32);
    flag(output, l.is_enabled());
    flag(output, l.to_cast_shadows());
    flag(output, l.is_headlight());
    flag(output, l.has_range());
    show(output, l.direction(), |value| dir(output, value));
}

/// The source light of operation 15: a light of `kind` with fixed settings.
fn copy_source(output: &Output, kind: Graphic3dTypeOfLightSource) -> Handle<Graphic3dCLight> {
    let source = Graphic3dCLight::new(kind);
    source.set_name(&AsciiString::from("source"));
    tried(output, source.set_intensity(3.0));
    source.set_color(Color::new(0.25, 0.5, 0.75, ColorSpace::Rgb).unwrap_or_default());
    tried(output, source.set_direction_coords(0.0, 0.6, -0.8));
    tried(output, source.set_position_coords(1.0, 2.0, 3.0));
    tried(output, source.set_headlight(true));
    tried(output, source.set_cast_shadows(true));
    tried(output, source.set_angle(0.25));
    tried(output, source.set_smooth_radius(0.5));
    tried(output, source.set_smooth_angle(0.125));
    Handle::new(source)
}

fn point_op(output: &Output, r: &mut Reader<'_>, l: &Graphic3dCLight, op: i32) {
    let [x, y, z] = [0; 3].map(|_| r.real());
    tried(
        output,
        match op {
            5 => l.set_position_coords(x, y, z),
            6 => l.set_display_position(&Point3::new(x, y, z)),
            _ => l.set_direction_coords(x, y, z),
        },
    );
}

fn scalar_op(output: &Output, r: &mut Reader<'_>, l: &Graphic3dCLight, op: i32) {
    let value = r.real() as f32;
    tried(
        output,
        match op {
            8 => l.set_intensity(value),
            9 => l.set_angle(value),
            11 => l.set_concentration(value),
            12 => l.set_smooth_radius(value),
            13 => l.set_smooth_angle(value),
            _ => l.set_range(value),
        },
    );
}

fn light_op(output: &Output, r: &mut Reader<'_>, l: &Graphic3dCLight) {
    match r.integer() {
        1 => l.set_color(rgb(r)),
        2 => l.set_enabled(r.flag()),
        3 => tried(output, l.set_cast_shadows(r.flag())),
        4 => tried(output, l.set_headlight(r.flag())),
        op @ 5..=7 => point_op(output, r, l, op),
        10 => {
            let (c, li) = (r.real() as f32, r.real() as f32);
            tried(output, l.set_attenuation(c, li));
        }
        15 => {
            tried(
                output,
                l.copy_from(&copy_source(output, light_type(r.integer()))),
            );
            flag(output, l.name().as_bytes() == b"source");
        }
        op => scalar_op(output, r, l, op),
    }
}

/// `sv_light`: a light, its state after construction and after each operation.
pub fn light_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let light = Graphic3dCLight::new(light_type(r.integer()));
    light_state(output, &light);
    for _ in 0..r.integer() {
        light_op(output, &mut r, &light);
        light_state(output, &light);
    }
    end(output);
}

fn set_state(output: &Output, s: &Graphic3dLightSet) {
    use Graphic3dLightSetIterationFilter as F;
    int(output, s.update_revision() as i32);
    int(output, s.extent());
    for kind in 0..4 {
        int(output, s.nb_lights_of_type(light_type(kind)));
        int(output, s.nb_enabled_lights_of_type(light_type(kind)));
    }
    int(output, s.nb_enabled());
    int(output, s.nb_cast_shadows());
    vec4f(output, s.ambient_color());
    text(output, s.key_enabled_long().as_bytes());
    text(output, s.key_enabled_short().as_bytes());
    let filters = [
        F::None,
        F::ExcludeAmbient,
        F::ExcludeDisabled,
        F::ExcludeNoShadow,
        F::ExcludeDisabledAndAmbient,
        F::ActiveShadowCasters,
    ];
    for filter in filters {
        let mut it = Graphic3dLightSetIterator::new(s, filter);
        let mut count = 0;
        while it.more() {
            int(output, it.value().unwrap().light_type() as i32);
            count += 1;
            it.next();
        }
        int(output, count);
    }
}

/// Reads one light of the set request and adds it twice.
fn read_set_light(
    output: &Output,
    r: &mut Reader<'_>,
    s: &Graphic3dLightSet,
) -> Handle<Graphic3dCLight> {
    let light = Handle::new(Graphic3dCLight::new(light_type(r.integer())));
    light.set_enabled(r.flag());
    if r.flag() {
        tried(output, light.set_cast_shadows(true));
    }
    tried(output, light.set_intensity(r.real() as f32));
    light.set_color(rgb(r));
    show(output, s.add(&light), |value| flag(output, value));
    show(output, s.add(&light), |value| flag(output, value));
    light
}

/// `sv_light_set`: lights added to a set, its state, then a removal and a toggle.
pub fn light_set_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let set = Graphic3dLightSet::default();
    let lights: Vec<_> = (0..r.integer())
        .map(|_| read_set_light(output, &mut r, &set))
        .collect();
    show(output, set.add(&Handle::default()), |_| ());
    int(output, set.revision() as i32);
    set_state(output, &set);
    int(output, set.update_revision() as i32);
    let (removed, toggled) = (r.integer() as usize, r.integer() as usize);
    if removed > 0 {
        flag(output, set.remove(&lights[removed - 1]));
        flag(output, set.remove(&lights[removed - 1]));
        flag(output, set.contains(&lights[removed - 1]));
    }
    for index in 1..=set.extent() {
        show(output, set.value(index), |light| {
            int(output, light.light_type() as i32)
        });
    }
    show(output, set.value(0), |_| ());
    if toggled > 0 {
        let light = &lights[toggled - 1];
        light.set_enabled(!light.is_enabled());
    }
    set_state(output, &set);
    end(output);
}
