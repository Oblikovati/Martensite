//! `StepControlActorRead`: closing the shells of an I-DEAS file with the faces of the shells
//! suspected to close them.
use super::StepControlActorRead;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Compound, Shape, Shell};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::{BRepCheckShell, BRepCheckStatus};

/// Whether the shell checks as closed.
fn shell_is_closed(shell: &Shell) -> Result<bool, Failure> {
    let checker = BRepCheckShell::new(shell)?;
    Ok(checker.closed(false)? == BRepCheckStatus::NoError)
}

/// The shells a compound explores to.
fn shells_of(comp: &Compound) -> Result<Vec<Shape>, Failure> {
    let mut shells = Vec::new();
    let mut exp = Explorer::new(comp, ShapeEnum::Shell, ShapeEnum::Shape)?;
    while exp.more() {
        shells.push(exp.current().clone());
        exp.next()?;
    }
    Ok(shells)
}

impl StepControlActorRead {
    /// `closeIDEASShell(shell, closingShells)`: the shell with the faces that close it, least of
    /// them kept; the shell itself when they cannot close it.
    fn close_ideas_shell(&self, shell: &Shell, closing_shells: &[Shape]) -> Result<Shell, Failure> {
        let builder = BRepBuilder::default();
        let mut result = Shell::default();
        builder.make_shell(&mut result);
        let mut faces = Explorer::new(shell, ShapeEnum::Face, ShapeEnum::Shape)?;
        while faces.more() {
            builder.add(&mut result, faces.current())?;
            faces.next()?;
        }
        let mut closing_faces = Vec::new();
        for closing in closing_shells {
            let mut exp = Explorer::new(closing, ShapeEnum::Face, ShapeEnum::Shape)?;
            while exp.more() {
                builder.add(&mut result, exp.current())?;
                closing_faces.push(exp.current().clone());
                exp.next()?;
            }
        }
        if !shell_is_closed(&result)? {
            return Ok(shell.clone()); // Cannot close this shell, skip it so...
        }
        result.set_closed(true)?;
        for face in &closing_faces {
            builder.remove(&mut result, face)?;
            if !shell_is_closed(&result)? {
                builder.add(&mut result, face)?;
            }
        }
        Ok(result)
    }
    /// `computeIDEASClosings(comp, shellClosingMap)`: each shell with the shells that may close it.
    fn compute_ideas_closings(&self, comp: &Compound) -> Result<Vec<(Shape, Vec<Shape>)>, Failure> {
        let shells = shells_of(comp)?;
        let state = self.state.read();
        let mut closings = Vec::new();
        for shell_a in &shells {
            let mut closing = Vec::new();
            for shell_b in &shells {
                if !shell_a.is_same(shell_b)
                    && state.nm_tool.is_suspected_as_closing(shell_a, shell_b)?
                {
                    closing.push(shell_b.clone());
                }
            }
            if !closing.is_empty()
                && !closings
                    .iter()
                    .any(|(known, _): &(Shape, Vec<Shape>)| known.is_same(shell_a))
            {
                closings.push((shell_a.clone(), closing));
            }
        }
        Ok(closings)
    }
    /// The compound of the closed shells, then of the shells neither pure non-manifold nor closed
    /// above.
    pub(super) fn ideas_closed_compound(&self, comp: &Compound) -> Result<Compound, Failure> {
        let builder = BRepBuilder::default();
        let closings = self.compute_ideas_closings(comp)?;
        let mut result = Compound::default();
        builder.make_compound(&mut result);
        for (shell, closing) in &closings {
            let adjusted = self.close_ideas_shell(&topods::shell(shell)?, closing)?;
            builder.add(&mut result, &adjusted)?;
        }
        let state = self.state.read();
        for shell in shells_of(comp)? {
            let closed_above = closings.iter().any(|(known, _)| known.is_same(&shell));
            if !state.nm_tool.is_pure_nm_shell(&shell)? && !closed_above {
                builder.add(&mut result, &shell)?;
            }
        }
        Ok(result)
    }
}
