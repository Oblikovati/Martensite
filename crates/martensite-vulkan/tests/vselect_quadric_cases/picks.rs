//! A 600x400 selection session over named quadrics, with select commands checked against the
//! expected selection state of every object.
use crate::case_view::viewer_session::DisplayRequest;
use crate::session_run::SessionRun;
use martensite_tkbrep::topods::Shape;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
use std::error::Error;

/// One select command at pixels, then the expected state of every displayed object in display
/// order ('+' selected, '-' not), with the original's error message on any mismatch.
pub(crate) struct Pick {
    pub(crate) pixels: &'static [(i32, i32)],
    pub(crate) overlap: bool,
    pub(crate) expected: &'static str,
    pub(crate) message: &'static str,
}

/// A view change between select commands.
pub(crate) enum ViewStep {
    /// A standard projection: top, right and the other shortcuts.
    Project(V3dTypeOfOrientation),
    /// Fit the view.
    Fit,
}

/// The session of one quadric regression and the names it displayed, in order.
pub(crate) struct QuadricRun {
    pub(crate) run: SessionRun,
    names: Vec<&'static str>,
}

impl QuadricRun {
    /// A 600x400 view with each shape displayed shaded under its name, then fitted.
    /// Example: `QuadricRun::shaded(&[("s", sphere(1.)?)])?`.
    pub(crate) fn shaded(shapes: &[(&'static str, Shape)]) -> Result<Self, Box<dyn Error>> {
        let mut run = SessionRun::with_view_size(600, 400)?;
        let shaded = DisplayRequest::default().with_display_mode(1);
        for (name, shape) in shapes {
            run.session().display(name, shape, &shaded)?;
        }
        run.session().fit()?;
        let names = shapes.iter().map(|(name, _)| *name).collect();
        Ok(Self { run, names })
    }

    /// Display `shape` shaded as `name` in a running session; `run.show("cone", &shape)?`.
    pub(crate) fn show(&mut self, name: &'static str, shape: &Shape) -> Result<(), Box<dyn Error>> {
        let shaded = DisplayRequest::default().with_display_mode(1);
        self.run.session().display(name, shape, &shaded)?;
        self.names.push(name);
        Ok(())
    }

    /// Remove the object `name`; `run.remove("cyl")?`.
    pub(crate) fn remove(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        self.run.session().remove(&[name])?;
        self.names.retain(|bound| *bound != name);
        Ok(())
    }

    /// Apply view steps in order; `run.view(&[ViewStep::Fit])?`.
    pub(crate) fn view(&mut self, steps: &[ViewStep]) -> Result<(), Box<dyn Error>> {
        for step in steps {
            match step {
                ViewStep::Project(orientation) => {
                    self.run.session().set_standard_projection(*orientation)?
                }
                ViewStep::Fit => self.run.session().fit()?,
            }
        }
        Ok(())
    }

    /// Run each select command and print its message when the selection state differs.
    /// Example: `run.picks(&[Pick { pixels: &[(300, 200)], overlap: false, expected: "+", message: "..." }])?`.
    pub(crate) fn picks(&mut self, picks: &[Pick]) -> Result<(), Box<dyn Error>> {
        for pick in picks {
            self.run
                .session()
                .select_pixels(pick.pixels, pick.overlap)?;
            let wanted = pick.expected.chars().map(|state| state == '+');
            let session = self.run.session();
            let matches = self
                .names
                .iter()
                .zip(wanted)
                .all(|(name, selected)| session.is_selected(name) == selected);
            if !matches {
                println!("{}", pick.message);
            }
        }
        Ok(())
    }
}
