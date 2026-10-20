use super::replay::brep_record_print::{self as report, RecordEmitter};
use martensite_tkbrep::topods::Edge;
use martensite_tkernel::standard::Failure;
use martensite_tkhlr::hlr_brep::EdgeData;
use martensite_tkmath::gp::Point3;
#[derive(Debug, PartialEq)]
enum RecordField {
    Flag(bool),
    Integer(i32),
    Real(f64),
    Point(Point3),
    Exception(String),
}
#[derive(Default)]
struct RecordedEdgeEmitter(Vec<RecordField>);
impl RecordEmitter for RecordedEdgeEmitter {
    fn flag(&mut self, value: bool) {
        self.0.push(RecordField::Flag(value));
    }
    fn int(&mut self, value: i32) {
        self.0.push(RecordField::Integer(value));
    }
    fn real(&mut self, value: f64) {
        self.0.push(RecordField::Real(value));
    }
    fn pnt(&mut self, value: Point3) {
        self.0.push(RecordField::Point(value));
    }
    fn attempt(&mut self, body: impl FnOnce(&mut Self) -> Result<(), Failure>) {
        if let Err(error) = body(self) {
            self.0
                .push(RecordField::Exception(error.exception_type().to_owned()));
        }
    }
}
#[test]
fn default_edge_report_reads_only_initialized_flags_and_hide_count() {
    let mut emit = RecordedEdgeEmitter::default();
    report::edge_flags(&mut emit, &EdgeData::new(), false);
    let mut expected = vec![RecordField::Flag(true)];
    expected.extend((0..12).map(|_| RecordField::Flag(false)));
    expected.push(RecordField::Integer(0));
    assert_eq!(emit.0, expected);
}
#[test]
fn null_edge_short_circuits_and_line_summary_keeps_vertex_then_adaptor_order() {
    use RecordField::{Flag, Integer, Point, Real};
    let mut emit = RecordedEdgeEmitter::default();
    report::edge_summary(&mut emit, &Edge::default()).unwrap();
    assert_eq!(emit.0, [Flag(true)]);
    emit.0.clear();
    report::edge_summary(&mut emit, &super::line_edge()).unwrap();
    assert_eq!(
        emit.0,
        [
            Flag(false),
            Flag(false),
            Point(Point3::new(1., 2., 3.)),
            Flag(false),
            Point(Point3::new(3., 2., 3.)),
            Real(0.),
            Real(2.),
            Integer(0),
            Point(Point3::new(1., 2., 3.)),
            Point(Point3::new(2., 2., 3.)),
            Point(Point3::new(3., 2., 3.))
        ]
    );
}
