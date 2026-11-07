//! The shells and solids of the BRep entities of IGES.
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_solid::{IgesSolidManifoldSolid, IgesSolidShell};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Shape, Shell, Solid};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;

use super::IgesToBRepBRepEntity;

const WHERE: &str = "IgesToBRepBRepEntity";

impl IgesToBRepBRepEntity {
    fn built_shell(
        &self,
        start: &Handle<IgesSolidShell>,
        range: &ProgressRange,
    ) -> Result<(), Failure> {
        let live = iges_live(start, WHERE)?;
        let nb_faces = live.nb_faces()?;
        if nb_faces == 0 {
            self.fail_key(start, "XSTEP_200");
            return Ok(());
        }
        let builder = BRepBuilder::default();
        let mut shell = Shell::default();
        builder.make_shell(&mut shell);
        let mut closed = true;
        let scope = ProgressScope::with_literal(range, "Face", f64::from(nb_faces), false);
        for rank in 1..=nb_faces {
            if !scope.more() {
                break;
            }
            let mut shape = self.transfer_face(&live.face(rank)?)?;
            let _ = scope.next(1.0);
            if shape.is_null() {
                closed = false;
                continue;
            }
            if !live.orientation(rank)? {
                shape.reverse();
            }
            builder.add(&mut shell, &shape)?;
        }
        if !closed {
            self.fail_key(start, "IGES_1360");
        }
        shell.set_closed(closed)?;
        shell.set_orientable(true)?;
        self.set_shape_result(&start.cast(), &shell)
    }
    /// The shell of a shell entity, once; `tool.transfer_shell(&entity, &range)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shell.
    pub fn transfer_shell(
        &self,
        start: &Handle<IgesSolidShell>,
        range: &ProgressRange,
    ) -> Result<Shape, Failure> {
        let entity: Handle<IgesDataIgesEntity> = start.cast();
        if !self.has_shape_result(&entity)? {
            self.built_shell(start, range)?;
        }
        let shape = self.get_shape_result(&entity)?;
        if shape.is_null() {
            self.fail_label(start, b"shell")?;
        }
        Ok(shape)
    }
    fn built_solid(
        &self,
        start: &Handle<IgesSolidManifoldSolid>,
        range: &ProgressRange,
    ) -> Result<(), Failure> {
        let live = iges_live(start, WHERE)?;
        let builder = BRepBuilder::default();
        let mut solid = Solid::default();
        builder.make_solid(&mut solid);
        let oriented = live.orientation_flag();
        let add_shell = |solid: &mut Solid, shape: &Shape| -> Result<(), Failure> {
            if shape.shape_type()? != ShapeEnum::Shell {
                return Ok(());
            }
            let mut shell = topods::shell(shape)?;
            if !oriented {
                shell.reverse();
            }
            BRepBuilder::default().add(solid, &shell)
        };
        let outer = self.transfer_shell(&live.shell(), range)?;
        if !outer.is_null() {
            add_shell(&mut solid, &outer)?;
            let nb_voids = live.nb_void_shells();
            if nb_voids != 0 {
                let scope = ProgressScope::unnamed(range, f64::from(nb_voids), false);
                for rank in 1..=nb_voids {
                    if !scope.more() {
                        break;
                    }
                    let step = scope.next(1.0);
                    let void = self.transfer_shell(&live.void_shell(rank)?, &step)?;
                    if void.is_null() {
                        builder.add(&mut solid, &Shell::default())?;
                    } else {
                        add_shell(&mut solid, &void)?;
                    }
                }
            }
        }
        self.set_shape_result(&start.cast(), &solid)
    }
    /// The solid of a manifold solid entity, once; `tool.transfer_manifold_solid(&entity, &range)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the solid.
    pub fn transfer_manifold_solid(
        &self,
        start: &Handle<IgesSolidManifoldSolid>,
        range: &ProgressRange,
    ) -> Result<Shape, Failure> {
        let entity: Handle<IgesDataIgesEntity> = start.cast();
        if !self.has_shape_result(&entity)? {
            self.built_solid(start, range)?;
        }
        let shape = self.get_shape_result(&entity)?;
        if shape.is_null() {
            self.fail_label(start, b"solid")?;
        }
        Ok(shape)
    }
}
