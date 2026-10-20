//! The hidden-line commands the cases call: projectors and outliners are named
//! script objects, and one hidden-line algorithm (`hider`) lives for the session.
use super::restore::Restored;
use super::script::{Reply, Script, ScriptStep};
use super::session::Session;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkhlr::hlr_algo::Projector;
use martensite_tkhlr::hlr_brep::{Algo, HLRToShape};
use martensite_tkhlr::hlr_topo_brep::{MapOfShapeTool, OutLiner};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};

/// The HLRTest objects of one case session; `HlrSession::new()?`.
pub struct HlrSession {
    projectors: Vec<(String, Projector)>,
    outliners: Vec<(String, Handle<OutLiner>)>,
    hider: Handle<Algo>,
}

/// The `hres2d` outputs in HLRTest order with their the case harness names.
const HRES2D_NAMES: [&str; 10] = [
    "vl", "v1l", "vnl", "vol", "vil", "hl", "h1l", "hnl", "hol", "hil",
];

impl HlrSession {
    /// No projector or outliner and a fresh `hider`; `HlrSession::new()?`.
    pub fn new() -> Result<Self, Failure> {
        Ok(Self {
            projectors: Vec::new(),
            outliners: Vec::new(),
            hider: Handle::new(Algo::new()?),
        })
    }

    fn projector(&self, name: &str) -> Option<Projector> {
        self.projectors
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, p)| *p)
    }

    fn outliner(&self, name: &str) -> Option<Handle<OutLiner>> {
        self.outliners
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, o)| o.clone())
    }

    /// `hprj name [x y z dx dy dz dx1 dy1 dz1]`: a projector on the frame (origin,
    /// normal, X direction), gp::XOY without the nine numbers; `hlr.hprj(&mut draw, &args)?`.
    pub fn hprj(&mut self, draw: &mut Script, args: &[&str]) -> ScriptStep {
        let words: Vec<&str> = std::iter::once("hprj")
            .chain(args.iter().copied())
            .collect();
        draw.command(&words, |_| {
            let frame = if args.len() == 10 {
                frame_of(&args[1..])?
            } else {
                Axis2::default()
            };
            let projector = Projector::with_frame(frame)?;
            self.projectors.retain(|(n, _)| n != args[0]);
            self.projectors.push((args[0].to_string(), projector));
            Ok(Reply::ok(""))
        })?;
        Ok(())
    }

    /// `houtl name shape`: an outliner of the shape variable; `hlr.houtl(&mut draw, "o", "s")?`.
    pub fn houtl(&mut self, draw: &mut Script, name: &str, shape: &str) -> ScriptStep {
        draw.command(&["houtl", name, shape], |draw| {
            let source = draw.shape(shape);
            if source.is_null() {
                return Ok(Reply::error(format!("{shape} is not a shape.")));
            }
            self.outliners.retain(|(n, _)| n != name);
            self.outliners.push((
                name.to_string(),
                Handle::new(OutLiner::with_original(&source)),
            ));
            Ok(Reply::ok(""))
        })?;
        Ok(())
    }

    /// `hfill outliner projector [nbIso]`; `hlr.hfill(&mut draw, "o", "p", "0")?`.
    pub fn hfill(
        &mut self,
        draw: &mut Script,
        outliner: &str,
        projector: &str,
        nb_iso: &str,
    ) -> ScriptStep {
        draw.command(&["hfill", outliner, projector, nb_iso], |_| {
            let Some(outline) = self.outliner(outliner) else {
                return Ok(Reply::error(format!("{outliner} is not an OutLiner.")));
            };
            let Some(projector) = self.projector(projector) else {
                return Ok(Reply::error(format!("{projector} is not a projector.")));
            };
            // The integer parser: the leading integer of the word.
            let count = nb_iso.parse().unwrap_or(0);
            outline.fill(&projector, &mut MapOfShapeTool::default(), count)?;
            Ok(Reply::ok(""))
        })?;
        Ok(())
    }

    /// `hload outliner`: `hider->Load(outliner)`; `hlr.hload(&mut draw, "o")?`.
    pub fn hload(&mut self, draw: &mut Script, outliner: &str) -> ScriptStep {
        draw.command(&["hload", outliner], |_| {
            let Some(outline) = self.outliner(outliner) else {
                return Ok(Reply::error(format!("{outliner} is not an OutLiner.")));
            };
            self.hider.load(&outline, 0);
            Ok(Reply::ok(""))
        })?;
        Ok(())
    }

    /// `hsetprj projector`: `hider->Projector(P)`; `hlr.hsetprj(&mut draw, "p")?`.
    pub fn hsetprj(&mut self, draw: &mut Script, projector: &str) -> ScriptStep {
        draw.command(&["hsetprj", projector], |_| {
            let Some(projector) = self.projector(projector) else {
                return Ok(Reply::error(format!("{projector} is not a projector.")));
            };
            self.hider.set_projector(&projector);
            Ok(Reply::ok(""))
        })?;
        Ok(())
    }

    /// `hupdate`: `hider->Update()`; `hlr.hupdate(&mut draw)?`.
    pub fn hupdate(&mut self, draw: &mut Script) -> ScriptStep {
        draw.command(&["hupdate"], |_| {
            self.hider.update()?;
            Ok(Reply::ok(""))
        })?;
        Ok(())
    }

    /// `hhide`: `hider->Hide()`; `hlr.hhide(&mut draw)?`.
    pub fn hhide(&mut self, draw: &mut Script) -> ScriptStep {
        draw.command(&["hhide"], |_| {
            self.hider.hide()?;
            Ok(Reply::ok(""))
        })?;
        Ok(())
    }

    /// `hres2d`: every HLRToShape output of the whole scene is built first, then each
    /// non-null one is set as `vl`, `v1l`, ... `hil`; `hlr.hres2d(&mut draw, &mut view)?`.
    pub fn hres2d(&mut self, draw: &mut Script, view: &mut dyn Session) -> ScriptStep {
        draw.command(&["hres2d"], |draw| {
            let outputs = hres2d_outputs(&HLRToShape::new(&self.hider))?;
            for (name, shape) in HRES2D_NAMES.iter().zip(outputs) {
                if !shape.is_null() {
                    draw.set_shape(name, &shape);
                    view.set(name, &Restored::Shape(shape));
                }
            }
            Ok(Reply::ok(""))
        })?;
        Ok(())
    }
}

/// The nine `hprj` numbers as `gp_Ax2(gp_Pnt, gp_Dir, gp_Dir)` (each word parsed as a real).
fn frame_of(words: &[&str]) -> Result<Axis2, Failure> {
    let numbers: Vec<f64> = words
        .iter()
        .map(|word| word.parse().unwrap_or(0.))
        .collect();
    let origin = Point3::new(numbers[0], numbers[1], numbers[2]);
    let normal = Direction3::new(numbers[3], numbers[4], numbers[5])?;
    let x_direction = Direction3::new(numbers[6], numbers[7], numbers[8])?;
    Axis2::with_x_direction(origin, normal, x_direction)
}

fn hres2d_outputs(extractor: &HLRToShape) -> Result<[Shape; 10], Failure> {
    Ok([
        extractor.v_compound()?,
        extractor.rg1_line_v_compound()?,
        extractor.rgn_line_v_compound()?,
        extractor.out_line_v_compound()?,
        extractor.iso_line_v_compound()?,
        extractor.h_compound()?,
        extractor.rg1_line_h_compound()?,
        extractor.rgn_line_h_compound()?,
        extractor.out_line_h_compound()?,
        extractor.iso_line_h_compound()?,
    ])
}
