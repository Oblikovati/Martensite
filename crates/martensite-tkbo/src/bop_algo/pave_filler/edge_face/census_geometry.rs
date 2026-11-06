//! Read actual joined geometry without initializing extra face or edge pools.
use crate::bop_algo::{AlertUserBreak, BOPAlgoPaveFiller};
use crate::bop_ds::{DataStructure, PaveBlockHandle};
use martensite_tkbrep::{brep::BRepTool, topods::Vertex};
use martensite_tkernel::standard::RuntimeTyped;
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn ef_census_geometry(filler: &BOPAlgoPaveFiller, polls: usize) -> String {
    let ds = filler.data_structure().unwrap();
    let mut fields = ef_status_fields(filler);
    fields.extend([
        format!("ef={}", ef_records(ds)),
        format!("ve={}", ve_records(ds)),
        format!("shapes={}", shape_records(ds)),
        format!("blocks={}", source_blocks(ds)),
        format!("faces={}", face_records(ds)),
    ]);
    fields.extend(ef_report_fields(filler, polls));
    fields.join("|")
}

fn ef_status_fields(filler: &BOPAlgoPaveFiller) -> Vec<String> {
    let state = ef_final_counts(filler);
    vec![
        format!("state={}", scalar_list(&state)),
        format!("iterator_more={}", filler.iterator().unwrap().more()),
        format!("has_errors={}", filler.options().has_errors()),
        format!("has_warnings={}", filler.options().has_warnings()),
        format!(
            "user_break={}",
            filler.options().has_error(AlertUserBreak::static_type())
        ),
    ]
}

fn ef_records(ds: &DataStructure) -> String {
    row_list(ds.interf_ef_records().iter().map(|record| {
        let part = record.common_part();
        let (first, last) = part.range1_bounds();
        format!(
            "[{},{},{},{},{},{},{}]",
            record.index1(),
            record.index2(),
            record.index_new(),
            part.shape_type() as i32,
            first.to_bits(),
            last.to_bits(),
            part.vertex_parameter1().to_bits()
        )
    }))
}

fn ve_records(ds: &DataStructure) -> String {
    row_list(ds.interf_ve_records().iter().map(|record| {
        format!(
            "[{},{},{},{}]",
            record.index1(),
            record.index2(),
            record.index_new(),
            record.parameter().to_bits()
        )
    }))
}

fn shape_records(ds: &DataStructure) -> String {
    row_list((0..ds.nb_shapes()).map(|index| {
        let kind = ds.shape_info(index).unwrap().shape_type();
        let suffix = vertex_coordinates(ds, index, kind);
        format!(
            "[{index},{},{},{suffix}]",
            kind as i32,
            ds.is_new_shape(index)
        )
        .replace(",]", "]")
    }))
}

fn vertex_coordinates(ds: &DataStructure, index: i32, kind: ShapeEnum) -> String {
    if kind != ShapeEnum::Vertex {
        return String::new();
    }
    let vertex = Vertex::unchecked(ds.shape(index).unwrap().clone());
    let point = BRepTool::pnt(&vertex).unwrap();
    let tolerance = BRepTool::vertex_tolerance(&vertex).unwrap();
    format!(
        "{},{},{},{}",
        point.x().to_bits(),
        point.y().to_bits(),
        point.z().to_bits(),
        tolerance.to_bits()
    )
}

fn source_blocks(ds: &DataStructure) -> String {
    row_list(
        (0..ds.nb_source_shapes())
            .filter(|index| ds.shape_info(*index).unwrap().shape_type() == ShapeEnum::Edge)
            .map(|index| {
                let blocks = row_list(ds.pave_blocks(index).unwrap().iter().map(block_record));
                format!("[{index},{blocks}]")
            }),
    )
}

fn block_record(block: &PaveBlockHandle) -> String {
    let block = block
        .lock()
        .expect("EF census block: expected an unpoisoned lock");
    let (first, last) = block.indices();
    let (begin, end) = block.range();
    format!("[{first},{last},{},{}]", begin.to_bits(), end.to_bits())
}

fn face_records(ds: &DataStructure) -> String {
    row_list(
        (0..ds.nb_source_shapes())
            .filter(|index| {
                ds.shape_info(*index).unwrap().shape_type() == ShapeEnum::Face
                    && ds.has_face_info(*index).unwrap()
            })
            .map(|index| {
                let face = ds.face_info(index).unwrap();
                format!(
                    "[{index},{},{},{},{}]",
                    face.vertices_in().extent(),
                    face.vertices_on().extent(),
                    face.pave_blocks_in().extent(),
                    face.pave_blocks_on().extent()
                )
            }),
    )
}

fn row_list(rows: impl Iterator<Item = String>) -> String {
    let members = rows.collect::<Vec<_>>().join(",");
    format!("[{members}]")
}

fn scalar_list(values: &impl std::fmt::Debug) -> String {
    // Callers supply only numeric arrays and diagnostic bytes, never scope names.
    format!("{values:?}").replace(' ', "")
}

fn ef_report_fields(filler: &BOPAlgoPaveFiller, polls: usize) -> Vec<String> {
    let (mut errors, mut warnings) = (Vec::new(), Vec::new());
    filler.options().dump_errors(&mut errors).unwrap();
    filler.options().dump_warnings(&mut warnings).unwrap();
    vec![
        format!("error_bytes={}", scalar_list(&errors)),
        format!("warning_bytes={}", scalar_list(&warnings)),
        format!("poll_count={polls}"),
    ]
}

fn ef_final_counts(filler: &BOPAlgoPaveFiller) -> [usize; 5] {
    let ds = filler.data_structure().unwrap();
    [
        ds.nb_shapes() as usize,
        ds.interf_ef_records().len(),
        ds.interf_ve_records().len(),
        ds.same_domain_binding_count(),
        filler.fpb_done.len(),
    ]
}
