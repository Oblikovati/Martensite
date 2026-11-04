//! One ordered description stream serves interpreters and native typed consumers.
use super::TopologyRendering;
use martensite_tkbrep::topods::TShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs;

impl TopologyRendering {
    /// Writes the existing description in order, stopping at the first sink failure.
    /// Collect text with `drawable.write_description(|part| { text.push_str(part); Ok(()) })?`.
    pub fn write_description(
        &self,
        mut emit: impl FnMut(&str) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        if self.shape.is_null() {
            return Ok(());
        }
        let kind = top_abs::shape_type_to_string(self.shape.shape_type()?);
        let orientation = top_abs::shape_orientation_to_string(self.shape.orientation());
        emit("shape ")?;
        emit(kind)?;
        emit(" ")?;
        emit(orientation)?;
        self.write_description_flags(&mut emit)
    }

    fn write_description_flags(
        &self,
        emit: &mut impl FnMut(&str) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let Some(topology) = self.shape.tshape().get() else {
            return Ok(());
        };
        for (set, text) in description_flags(topology) {
            if set {
                emit(text)?;
            }
        }
        Ok(())
    }
}

fn description_flags(topology: &dyn TShape) -> [(bool, &'static str); 6] {
    [
        (topology.free(), " Free"),
        (topology.modified(), " Modified"),
        (topology.orientable(), " Orientable"),
        (topology.closed(), " Closed"),
        (topology.infinite(), " Infinite"),
        (topology.convex(), " Convex"),
    ]
}
