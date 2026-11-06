//! Mouse clicks retain the tutorial's deterministic color generator and alternating animation.
use super::{object::TutorialObject, owner::TutorialOwner};
use martensite_tkernel::{
    quantity::{Color, ColorSpace},
    standard::Handle,
};
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3};
use martensite_tkv3d::{
    ais::{AisAnimationMembers, AisAnimationObject, AisInteractiveObjectMembers},
    prs_mgr::*,
    select_mgr::*,
};
impl TutorialOwner {
    pub(super) fn click(&self) {
        let object = self.selectable().cast::<TutorialObject>();
        let color = self.next_color();
        let shading = object.attributes().shading_aspect();
        shading
            .set_color(
                color,
                martensite_tkservice::aspect::AspectTypeOfFacingModel::BothSide,
            )
            .unwrap();
        object.synchronize_aspects();
        if !object.animation.is_null() {
            self.animate(&object);
        }
    }
    fn next_color(&self) -> Color {
        let mut random = self.random.write();
        let red = (random.next_int() % 256) as f32 / 255_f32;
        let green = (random.next_int() % 256) as f32 / 255_f32;
        let blue = (random.next_int() % 256) as f32 / 255_f32;
        Color::new(red as f64, green as f64, blue as f64, ColorSpace::Srgb).unwrap()
    }
    fn next_target(&self) -> Transform3 {
        let mut first = self.first_click.write();
        *first = !*first;
        let angle = if *first {
            std::f64::consts::FRAC_PI_2
        } else {
            -std::f64::consts::FRAC_PI_2
        };
        let axis = Axis1::new(Point3::default(), Direction3::new(1., 0., 0.).unwrap());
        Transform3::rotation(axis, angle).unwrap()
    }
    fn animate(&self, object: &Handle<TutorialObject>) {
        let target = self.next_target();
        let animation = Handle::new(AisAnimationObject::new(
            "MyAnim",
            &object.interactive_context().upgrade(),
            &object.cast(),
            object.local_transformation(),
            target,
        ));
        animation.set_own_duration(2.);
        object.animation.clear();
        object.animation.add(&animation.cast()).unwrap();
        object.animation.start_timer(0., 1., true, false).unwrap();
    }
}
