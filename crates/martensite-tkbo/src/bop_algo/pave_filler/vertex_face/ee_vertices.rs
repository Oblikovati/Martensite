//! Native unranged EE-vertex classification; progress is deliberately not added here.
use super::{BOPAlgoPaveFiller, missing_structure};
use crate::bop_ds::{BOPDSSubIterator, InterfVF};
use martensite_tkbrep::topods::{Face, Vertex};
use martensite_tkernel::{
    ncollection::{List, Map},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;

impl BOPAlgoPaveFiller {
    /// Classifies every vertex created by an EE intersection against the source faces it is
    /// not On, adding VF interferences and In vertices (`TreatVerticesEE`).
    /// `filler.treat_vertices_ee()?`.
    pub fn treat_vertices_ee(&mut self) -> Result<(), Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let mut fence: Map<i32> = Map::new(100);
        let mut vertices: List<i32> = List::default();
        for record in structure.interf_ee_records().iter() {
            if let Some(vertex) = record.get_index_new()
                && fence.add(vertex)
            {
                vertices.append(vertex);
            }
        }
        if vertices.is_empty() {
            return Ok(());
        }
        let mut faces: List<i32> = List::default();
        for index in 0..structure.nb_source_shapes() {
            if structure.shape_info(index)?.shape_type() == ShapeEnum::Face {
                faces.append(index);
            }
        }
        if faces.is_empty() {
            return Ok(());
        }
        let pairs = subset_pairs(structure, &faces, &vertices)?;
        for (vertex, face) in pairs {
            self.treat_ee_vertex_on_face(vertex, face)?;
        }
        Ok(())
    }

    fn treat_ee_vertex_on_face(&mut self, vertex: i32, face: i32) -> Result<(), Failure> {
        let fuzzy = self.options.fuzzy_value();
        let context = self.context.clone();
        let structure = self.structure_mut()?;
        if structure
            .change_face_info(face)?
            .vertices_on()
            .contains(vertex)
        {
            return Ok(());
        }
        let shape = Vertex::unchecked(structure.shape(vertex)?.clone());
        let face_shape = Face::unchecked(structure.shape(face)?.clone());
        let (mut u, mut v, mut dummy) = (0., 0., 0.);
        let context = context.get().ok_or_else(missing_structure)?;
        if context.compute_vf(&shape, &face_shape, &mut u, &mut v, &mut dummy, fuzzy)? != 0 {
            return Ok(());
        }
        let record = structure.interf_vf().append(InterfVF::default());
        record.set_indices(vertex, face);
        record.set_uv(u, v);
        structure.add_interference(vertex, face);
        structure
            .change_face_info(face)?
            .change_vertices_in()
            .add(vertex);
        Ok(())
    }
}

fn subset_pairs(
    structure: &crate::bop_ds::DataStructure,
    faces: &List<i32>,
    vertices: &List<i32>,
) -> Result<Vec<(i32, i32)>, Failure> {
    let mut iterator = BOPDSSubIterator::new();
    iterator.set_ds(Some(structure));
    iterator.set_subset1(faces);
    iterator.set_subset2(vertices);
    iterator.prepare()?;
    iterator.initialize();
    let mut pairs = Vec::new();
    while iterator.more() {
        pairs.push(iterator.value()?);
        iterator.next();
    }
    Ok(pairs)
}
