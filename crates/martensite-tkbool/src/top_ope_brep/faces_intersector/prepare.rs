//! Construction of face-domain-clipped lines before publishing traversal records.
use super::TopOpeBRepFacesIntersector;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::int_patch::{IntPatchLine, IntPatchLineConstructor};

impl TopOpeBRepFacesIntersector {
    /// Clips and prepares the last surface intersection; `pair.prepare_lines()?`.
    pub fn prepare_lines(&mut self) -> Result<(), Failure> {
        self.line_count = 0;
        let count = self.intersection.nb_lines()?;
        self.lines = Handle::new(
            martensite_tkernel::ncollection::HArray1::with_bounds(0, count).map_err(|error| {
                Failure::with_message(
                    "Standard_RangeError",
                    &format!("PrepareLines: range=0..={count}; expected valid bounds: {error}"),
                )
            })?,
        );
        if count == 0 {
            return Ok(());
        }
        let mut prepared = Vec::new();
        for index in 1..=self.intersection.nb_lines()? {
            let mut split = self.split_line(&self.intersection.line(index)?)?;
            let sides = [
                super::restrictions::Side {
                    surface: &self.surfaces[0],
                    domain: &self.domains[0],
                    first: true,
                    tolerance: self.tolerances.0,
                },
                super::restrictions::Side {
                    surface: &self.surfaces[1],
                    domain: &self.domains[1],
                    first: false,
                    tolerance: self.tolerances.0,
                },
            ];
            super::restrictions::convert_groups(&mut split, sides)?;
            prepared.extend(split);
        }
        super::restrictions::merge_restriction_segments(&mut prepared, self.restriction_sides())?;
        self.install_lines(prepared)
    }

    fn split_line(
        &self,
        line: &Handle<dyn IntPatchLine>,
    ) -> Result<Vec<Handle<dyn IntPatchLine>>, Failure> {
        let mut constructor = IntPatchLineConstructor::new(2);
        constructor.perform(
            self.intersection.sequence_of_line(),
            line,
            &self.surfaces[0].cast(),
            &self.domains[0],
            &self.surfaces[1].cast(),
            &self.domains[1],
            self.tolerances.0,
        )?;
        (1..=constructor.nb_lines())
            .map(|index| constructor.line(index))
            .collect()
    }

    fn restriction_sides(&self) -> [super::restrictions::Side<'_>; 2] {
        [
            super::restrictions::Side {
                surface: &self.surfaces[0],
                domain: &self.domains[0],
                first: true,
                tolerance: self.tolerances.0,
            },
            super::restrictions::Side {
                surface: &self.surfaces[1],
                domain: &self.domains[1],
                first: false,
                tolerance: self.tolerances.0,
            },
        ]
    }

    fn install_lines(&mut self, sources: Vec<Handle<dyn IntPatchLine>>) -> Result<(), Failure> {
        self.line_count = i32::try_from(sources.len()).map_err(|_| {
            Failure::with_message(
                "Standard_RangeError",
                &format!(
                    "PrepareLines: count={}; expected 0..={}",
                    sources.len(),
                    i32::MAX
                ),
            )
        })?;
        if self.line_count == 0 {
            return Ok(());
        }
        let array = martensite_tkernel::ncollection::HArray1::with_bounds(1, self.line_count)
            .map_err(|error| {
                Failure::with_message(
                    "Standard_RangeError",
                    &format!(
                        "PrepareLines: count={}; expected positive line count: {error}",
                        self.line_count
                    ),
                )
            })?;
        self.lines = Handle::new(array);
        for (index, source) in sources.into_iter().enumerate() {
            let mut records = self.lines.change_array1();
            let line = records.change_value(index as i32 + 1).map_err(|error| {
                Failure::with_message(
                    "Standard_OutOfRange",
                    &format!(
                        "PrepareLines: index={}; expected allocated line slot: {error}",
                        index + 1
                    ),
                )
            })?;
            line.set_line(&source, &self.surfaces[0], &self.surfaces[1])?;
            line.set_index(index as i32 + 1);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failed_line_load_keeps_the_new_array_and_uncompleted_operation() {
        let mut pair = TopOpeBRepFacesIntersector::new();
        assert!(pair.install_lines(vec![Handle::default()]).is_err());
        assert!(!pair.is_done());
        assert_eq!(pair.nb_lines(), 1);
        let shared = pair.lines();
        assert_eq!(shared.array1().lower(), 1);
        assert_eq!(shared.array1().upper(), 1);
        assert_eq!(pair.change_line(1).unwrap().nb_vpoint(), 0);
    }
}
