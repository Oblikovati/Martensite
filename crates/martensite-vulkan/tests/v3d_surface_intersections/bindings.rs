use super::*;
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkgeomalgo::geom_int::GeomIntIntSS;
use martensite_tkmath::gp::Point3;

pub(super) struct IntersectionBindings {
    pub curves: Vec<(String, Handle<dyn GeomCurve>)>,
    pub points: Vec<(String, Point3)>,
}

impl IntersectionBindings {
    pub(super) fn actual(operation: &GeomIntIntSS) -> Result<Self, Failure> {
        let mut curves = Vec::new();
        for index in 1..=operation.nb_lines() {
            let name = if operation.nb_lines() == 1 {
                "result".to_owned()
            } else {
                format!("result_{index}")
            };
            curves.push((name, operation.line(index)?));
        }
        let mut points = Vec::new();
        for index in 1..=operation.nb_points()? {
            points.push((format!("result_p_{index}"), operation.point(index)?));
        }
        Ok(Self { curves, points })
    }

    pub(super) fn contains(&self, name: &str) -> bool {
        self.curves.iter().any(|(key, _)| key == name)
            || self.points.iter().any(|(key, _)| key == name)
    }

    pub(super) fn authored_res_curve_count(&self) -> usize {
        (1..=10)
            .take_while(|index| {
                self.curves
                    .iter()
                    .any(|(name, _)| name == &format!("res_{index}"))
            })
            .count()
    }
}
