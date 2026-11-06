//! Existing measured-property text is shared by commands and interpreter-free source peers.
use martensite_tkernel::{standard::Failure, storage::format_general};
use martensite_tkgeombase::gprop::{GProps, PrincipalProps};
use martensite_tkmath::gp::{Matrix3, Point3};

#[derive(Clone, Copy)]
struct PropertyReportStyle {
    full: bool,
}

impl PropertyReportStyle {
    fn number(self, value: f64) -> String {
        if self.full {
            return format_general(value, 17);
        }
        format!("{:>15}", format_general(value, 6))
    }

    fn label(self, name: &str) -> String {
        if self.full {
            return format!("{name} :\n");
        }
        format!("{name} : \n")
    }
}

/// Format already measured properties without a session or new integration.
/// Example: `property_report(&properties, &principal, Some(error), false)?`.
pub fn property_report(
    properties: &GProps,
    principal: &PrincipalProps,
    epsilon: Option<f64>,
    full: bool,
) -> Result<String, Failure> {
    let style = PropertyReportStyle { full };
    let mut text = mass_report(style, properties.mass(), epsilon);
    text += &centre_report(style, properties.centre_of_mass());
    text += &matrix_report(style, properties.matrix_of_inertia())?;
    text += &moments_report(style, principal.moments());
    Ok(text)
}

fn mass_report(style: PropertyReportStyle, mass: f64, epsilon: Option<f64>) -> String {
    let mut text = format!("\n\nMass : {}\n\n", style.number(mass));
    if let Some(error) = epsilon {
        text += &format!(
            "Relative error of mass computation : {}\n\n",
            style.number(error)
        );
    }
    text
}

fn centre_report(style: PropertyReportStyle, point: Point3) -> String {
    format!(
        "Center of gravity : \nX = {}\nY = {}\nZ = {}\n\n",
        style.number(point.x()),
        style.number(point.y()),
        style.number(point.z()),
    )
}

fn matrix_report(style: PropertyReportStyle, matrix: Matrix3) -> Result<String, Failure> {
    let mut text = style.label("Matrix of Inertia");
    let separator = if style.full { "    " } else { " " };
    for row in 1..=3 {
        let numbers = [
            style.number(matrix.value(row, 1)?),
            style.number(matrix.value(row, 2)?),
            style.number(matrix.value(row, 3)?),
        ];
        text += &numbers.join(separator);
        text.push('\n');
    }
    text.push('\n');
    Ok(text)
}

fn moments_report(style: PropertyReportStyle, (ix, iy, iz): (f64, f64, f64)) -> String {
    format!(
        "{}IX = {}\nIY = {}\nIZ = {}\n\n",
        if style.full {
            "Moments :\n"
        } else {
            "Moments : \n"
        },
        style.number(ix),
        style.number(iy),
        style.number(iz),
    )
}

#[cfg(test)]
#[path = "property_report_tests.rs"]
mod tests;
