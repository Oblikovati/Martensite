//! Complete original coefficient tables and contiguous-block offset requests.
use super::common::{self as p, Reader, end};
use super::print::Output;
use martensite_tkgeombase::app_cont::cont_matrices as cm;

pub fn table_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (kind, start, count) = (r.integer(), r.integer(), r.integer());
    let tables = [
        &cm::THE_BBMATRIX_DATA,
        &cm::THE_INV_MMATRIX_DATA,
        &cm::THE_IBPMATRIX_DATA,
        &cm::THE_IBTMATRIX_DATA,
        &cm::THE_VBMATRIX_DATA,
    ];
    for i in start..start + count {
        p::real(
            output,
            tables[kind as usize].get(i as usize).expect("corpus index"),
        );
    }
    end(output);
}

pub fn offsets_case(output: &Output, values: &[f64]) {
    let classe = Reader::new(values).integer();
    p::int(output, cm::m_matrix_offset(classe));
    p::int(output, cm::ibp_matrix_offset(classe));
    p::int(output, cm::ibt_matrix_offset(classe));
    end(output);
}
