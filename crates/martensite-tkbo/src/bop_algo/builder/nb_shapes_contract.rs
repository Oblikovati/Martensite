//! Each accessor pair owns its one slot, so a mixed-up index would show as a wrong count.
use super::NbShapes;

type Read = fn(&NbShapes) -> i32;
type Write = fn(&mut NbShapes) -> &mut i32;
const ACCESSORS: [(Read, Write); 8] = [
    (NbShapes::nb_vertices, NbShapes::nb_vertices_mut),
    (NbShapes::nb_edges, NbShapes::nb_edges_mut),
    (NbShapes::nb_wires, NbShapes::nb_wires_mut),
    (NbShapes::nb_faces, NbShapes::nb_faces_mut),
    (NbShapes::nb_shells, NbShapes::nb_shells_mut),
    (NbShapes::nb_solids, NbShapes::nb_solids_mut),
    (NbShapes::nb_compsolids, NbShapes::nb_compsolids_mut),
    (NbShapes::nb_compounds, NbShapes::nb_compounds_mut),
];

#[test]
fn every_count_starts_at_zero_and_only_its_own_writer_changes_it() {
    for (written, (_, write)) in ACCESSORS.iter().enumerate() {
        let mut counts = NbShapes::new();
        *write(&mut counts) = 7;
        for (index, (read, _)) in ACCESSORS.iter().enumerate() {
            let expected = if index == written { 7 } else { 0 };
            assert_eq!(read(&counts), expected, "written={written} read={index}");
        }
    }
}
