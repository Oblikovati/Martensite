//! Lossless typed field encoding keeps numerical payloads separate from graph aliases.
use super::types::*;

/// Encode every stored field without evaluating geometry; `field_words(&input)`.
pub(super) fn field_words(input: &GraphInput) -> Vec<u64> {
    let mut output = FieldWords(vec![0x4d53_5446_574f_5244, 1]);
    output.occurrence(input.root);
    output.0.push(input.owners.len() as u64);
    for owner in input.owners {
        output.owner(*owner);
    }
    output.0.push(input.datums.len() as u64);
    for datum in input.datums {
        output.transform(*datum);
    }
    output.0
}

struct FieldWords(Vec<u64>);
fn signed(value: i32) -> u64 {
    i64::from(value) as u64
}
impl FieldWords {
    fn scalars(&mut self, values: &[u64]) {
        self.0.push(values.len() as u64);
        self.0.extend_from_slice(values);
    }
    fn integers(&mut self, values: &[i32]) {
        self.0.push(values.len() as u64);
        self.0.extend(values.iter().copied().map(signed));
    }
    fn references(&mut self, values: &[usize]) {
        self.0.push(values.len() as u64);
        self.0.extend(values.iter().map(|value| *value as u64));
    }
    fn rows<const D: usize>(&mut self, values: &[[u64; D]]) {
        self.0.push(values.len() as u64);
        for row in values {
            self.scalars(row);
        }
    }
    fn transform(&mut self, value: TransformInput) {
        self.0.extend([signed(value.form), value.scale]);
        self.rows(&value.matrix);
        self.scalars(&value.translation);
    }
    fn location(&mut self, value: LocationInput) {
        self.0.push(value.steps.len() as u64);
        for (datum, power) in value.steps {
            self.scalars(&[*datum as u64, signed(*power)]);
        }
        self.transform(value.effective);
    }
    fn occurrence(&mut self, value: OccurrenceInput) {
        self.0.push(value.owner as u64);
        self.location(value.location);
        self.0.push(signed(value.orientation));
    }
    fn topology(&mut self, tag: u64, state: u16, children: &[OccurrenceInput]) {
        self.0
            .extend([tag, u64::from(state), children.len() as u64]);
        for child in children {
            self.occurrence(*child);
        }
    }
    fn owner(&mut self, input: OwnerInput) {
        match input {
            OwnerInput::Face { .. } => self.face(input),
            OwnerInput::Wire { .. } => self.wire(input),
            OwnerInput::Edge { .. } => self.edge(input),
            OwnerInput::Vertex { .. } => self.vertex(input),
            OwnerInput::Spatial { .. } => self.spatial(input),
            OwnerInput::Parametric { .. } => self.parametric(input),
            OwnerInput::Seam { .. } => self.seam(input),
            OwnerInput::Line3 { origin, direction } => self.line(8, origin, direction),
            OwnerInput::Line2 { origin, direction } => self.line(9, origin, direction),
            OwnerInput::Spline3(value) => self.spline(10, value),
            OwnerInput::Spline2(value) => self.spline(11, value),
            OwnerInput::Plane { .. } => self.plane(input),
            OwnerInput::Extrusion { .. } => self.extrusion(input),
        }
    }
    fn face(&mut self, input: OwnerInput) {
        let OwnerInput::Face {
            state,
            children,
            surface,
            location,
            tolerance,
            natural,
        } = input
        else {
            return;
        };
        self.topology(1, state, children);
        self.0.push(surface as u64);
        self.location(location);
        self.0.extend([tolerance, u64::from(natural)]);
    }
    fn wire(&mut self, input: OwnerInput) {
        let OwnerInput::Wire { state, children } = input else {
            return;
        };
        self.topology(2, state, children);
    }
    fn edge(&mut self, input: OwnerInput) {
        let OwnerInput::Edge {
            state,
            children,
            tolerance,
            flags,
            representations,
        } = input
        else {
            return;
        };
        self.topology(3, state, children);
        self.0.extend([tolerance, signed(flags)]);
        self.references(representations);
    }
    fn vertex(&mut self, input: OwnerInput) {
        let OwnerInput::Vertex {
            state,
            children,
            point,
            tolerance,
        } = input
        else {
            return;
        };
        self.topology(4, state, children);
        self.scalars(&point);
        self.0.push(tolerance);
    }
    fn spatial(&mut self, input: OwnerInput) {
        let OwnerInput::Spatial {
            location,
            range,
            curve,
        } = input
        else {
            return;
        };
        self.0.push(5);
        self.location(location);
        self.scalars(&range);
        self.0.push(curve as u64);
    }
    fn parametric(&mut self, input: OwnerInput) {
        let OwnerInput::Parametric {
            location,
            range,
            curve,
            surface,
            endpoints,
        } = input
        else {
            return;
        };
        self.0.push(6);
        self.location(location);
        self.scalars(&range);
        self.0.extend([curve as u64, surface as u64]);
        self.rows(&endpoints);
    }
    fn seam(&mut self, input: OwnerInput) {
        let OwnerInput::Seam {
            location,
            range,
            curves,
            surface,
            continuity,
            endpoints,
        } = input
        else {
            return;
        };
        self.0.push(7);
        self.location(location);
        self.scalars(&range);
        self.references(&curves);
        self.0.extend([surface as u64, signed(continuity)]);
        self.0.push(endpoints.len() as u64);
        for branch in endpoints {
            self.rows(&branch);
        }
    }
    fn line<const D: usize>(&mut self, tag: u64, origin: [u64; D], direction: [u64; D]) {
        self.0.push(tag);
        self.scalars(&origin);
        self.scalars(&direction);
    }
    fn plane(&mut self, input: OwnerInput) {
        let OwnerInput::Plane {
            origin,
            x,
            y,
            normal,
        } = input
        else {
            return;
        };
        self.0.push(12);
        for vector in [origin, x, y, normal] {
            self.scalars(&vector);
        }
    }
    fn extrusion(&mut self, input: OwnerInput) {
        let OwnerInput::Extrusion { basis, direction } = input else {
            return;
        };
        self.0.extend([13, basis as u64]);
        self.scalars(&direction);
    }
    fn spline<const D: usize>(&mut self, tag: u64, value: SplineInput<D>) {
        self.0.push(tag);
        self.rows(value.poles);
        self.scalars(value.weights);
        self.scalars(value.knots);
        self.integers(value.multiplicities);
        self.scalars(value.flat_knots);
        self.scalars(&value.domain);
        self.0.extend([
            signed(value.degree),
            u64::from(value.periodic),
            u64::from(value.rational),
            u64::from(value.evaluation_override),
        ]);
        self.integers(&value.lower_bounds);
    }
}
