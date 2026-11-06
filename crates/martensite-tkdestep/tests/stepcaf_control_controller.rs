//! The cases of `controller`: translators used in parallel, the initialization of
//! the shape processing parameters, and documents with visibility and names through streams.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkdestep::de_step::{
    DeStepParameters, DeStepWriteModeVertexMode, StepControlStepModelType,
};
use martensite_tkdestep::step_control::{
    StepControlReader, StepControlWriter, shape_fix_parameters_for_processing,
};
use martensite_tkdestep::stepcaf_control::{
    STEPCAFControlController, STEPCAFControlReader, STEPCAFControlWriter,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::precision;
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::Point3;
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisShapeContents;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex};
use martensite_tkxcaf::xcaf_doc::{XCAFDocColorType, XCAFDocDocumentTool};
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::xs_algo::XSAlgoParameterMap;
use martensite_tkxsbase::xs_control::XSControlReaderOps;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};

/// The write actor belongs to the controller, of which a process has one, and an XDE writer changes its
/// mode during its transfer:, where each case has the process alone, one case at a time.
fn alone() -> MutexGuard<'static, ()> {
    static ONE_CASE_AT_A_TIME: Mutex<()> = Mutex::new(());
    ONE_CASE_AT_A_TIME
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// `parallel loop::For(0, count, ...)`, on as many threads as the budget of a test run.
fn parallel_for(count: i32, body: impl Fn() + Sync) {
    let next = AtomicI32::new(0);
    std::thread::scope(|scope| {
        for _ in 0..6 {
            scope.spawn(|| {
                while next.fetch_add(1, Ordering::Relaxed) < count {
                    body();
                }
            });
        }
    });
}

fn a_box(dx: f64, dy: f64, dz: f64) -> Shape {
    BRepPrimAPIMakeBox::from_sizes(dx, dy, dz)
        .expect("box")
        .solid()
        .expect("solid")
        .into_shape()
}

fn range() -> ProgressRange<'static> {
    ProgressRange::default()
}

/// The stream of a shape written through `StepControlWriter`.
fn written_stream(writer: &StepControlWriter, shape: &Shape) -> Vec<u8> {
    writer
        .transfer_with(
            shape,
            StepControlStepModelType::AS_IS,
            &DeStepParameters::default(),
            true,
            &range(),
        )
        .expect("transfer");
    let mut stream = Vec::new();
    writer.write_stream(&mut stream).expect("write");
    stream
}

/// The solids, shells, faces, wires, edges and vertices of a shape.
fn contents(shape: &Shape) -> [i32; 6] {
    let mut analyzer = ShapeAnalysisShapeContents::new();
    analyzer.perform(shape).expect("contents");
    [
        analyzer.nb_solids(),
        analyzer.nb_shells(),
        analyzer.nb_faces(),
        analyzer.nb_wires(),
        analyzer.nb_edges(),
        analyzer.nb_vertices(),
    ]
}

fn new_document() -> Handle<TDocStdDocument> {
    TDocStdDocument::new(&ExtendedString::from_utf8("dummy")).expect("document")
}

#[test]
fn bug33657_parallel_constructors() {
    let _alone = alone();
    STEPCAFControlController::init().expect("controller");
    parallel_for(1000, || {
        let mut reader = STEPCAFControlReader::new().expect("reader");
        reader.set_color_mode(true);
        let mut writer = STEPCAFControlWriter::new().expect("writer");
        writer.set_dim_tol_mode(true);
    });
}

#[test]
fn bug33657_parallel_writers_to_buffer() {
    let _alone = alone();
    STEPCAFControlController::init().expect("controller");
    let all_ok = AtomicBool::new(true);
    parallel_for(100, || {
        let shape = a_box(10.0, 20.0, 30.0);
        let writer = StepControlWriter::new().expect("writer");
        let defaults =
            shape_fix_parameters_for_processing(&DeStepParameters::default_shape_fix_parameters());
        writer
            .set_shape_fix_parameters_from(&defaults, XSAlgoParameterMap::default())
            .expect("parameters");
        if written_stream(&writer, &shape).is_empty() {
            all_ok.store(false, Ordering::Relaxed);
        }
    });
    assert!(all_ok.load(Ordering::Relaxed));
}

#[test]
fn bug33657_parallel_readers_from_stream() {
    let _alone = alone();
    STEPCAFControlController::init().expect("controller");
    let shape = a_box(10.0, 20.0, 30.0);
    let content = written_stream(&StepControlWriter::new().expect("writer"), &shape);
    assert!(!content.is_empty(), "STEP content should not be empty");
    parallel_for(100, || {
        let mut reader = StepControlReader::new().expect("reader");
        reader
            .read_stream_with("", &DeStepParameters::default(), &mut content.as_slice())
            .expect("read");
        reader.transfer_roots(&range()).expect("transfer");
    });
}

#[test]
fn bug33657_parallel_readers_and_writers() {
    let _alone = alone();
    STEPCAFControlController::init().expect("controller");
    let source = a_box(10.0, 20.0, 30.0);
    let expected = contents(&source);
    let all_ok = AtomicBool::new(true);
    parallel_for(100, || {
        if !all_ok.load(Ordering::Relaxed) {
            return;
        }
        let content = written_stream(&StepControlWriter::new().expect("writer"), &source);
        let mut reader = StepControlReader::new().expect("reader");
        reader
            .read_stream_with("", &DeStepParameters::default(), &mut content.as_slice())
            .expect("read");
        reader.transfer_roots(&range()).expect("transfer");
        if contents(&reader.xs_reader().one_shape()) != expected {
            all_ok.store(false, Ordering::Relaxed);
        }
    });
    assert!(all_ok.load(Ordering::Relaxed));
}

#[test]
fn step_control_writer_initializes_missing_shape_processing_parameters() {
    let _alone = alone();
    let shape = a_box(1.0, 2.0, 3.0);
    let writer = StepControlWriter::new().expect("writer");
    writer
        .set_shape_fix_parameters(XSAlgoParameterMap::default())
        .expect("parameters");
    assert!(
        writer
            .shape_fix_parameters()
            .expect("parameters")
            .is_empty()
    );
    let status = writer
        .transfer(&shape, StepControlStepModelType::AS_IS, true, &range())
        .expect("transfer");
    assert_eq!(status, IFSelectReturnStatus::RetDone);
    assert!(
        !writer
            .shape_fix_parameters()
            .expect("parameters")
            .is_empty()
    );
}

#[test]
fn stepcaf_control_writer_initializes_missing_shape_processing_parameters() {
    let _alone = alone();
    let shape = a_box(1.0, 2.0, 3.0);
    let document = new_document();
    let main = document.get().expect("document").main().expect("main");
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("tool");
    tool.set_shape(&tool.new_shape().expect("label"), &shape)
        .expect("shape");
    let mut writer = STEPCAFControlWriter::new().expect("writer");
    writer
        .set_shape_fix_parameters(XSAlgoParameterMap::default())
        .expect("parameters");
    assert!(
        writer
            .shape_fix_parameters()
            .expect("parameters")
            .is_empty()
    );
    assert!(
        writer
            .transfer_document(&document, StepControlStepModelType::AS_IS, None, &range())
            .expect("transfer")
    );
    assert!(
        !writer
            .shape_fix_parameters()
            .expect("parameters")
            .is_empty()
    );
}

/// Writes a document to a stream, and reads it into a new document.
fn written_and_read(writer: &STEPCAFControlWriter) -> (String, Handle<TDocStdDocument>) {
    let mut stream = Vec::new();
    let status = writer.write_stream(&mut stream).expect("write");
    assert_eq!(
        status,
        IFSelectReturnStatus::RetDone,
        "WriteStream did not return RetDone"
    );
    assert!(
        !stream.is_empty(),
        "Written STEP stream should not be empty"
    );
    let read_document = new_document();
    let mut reader = STEPCAFControlReader::new().expect("reader");
    reader.set_color_mode(true);
    assert_eq!(
        reader
            .read_stream("", &mut stream.as_slice())
            .expect("read"),
        IFSelectReturnStatus::RetDone,
        "ReadStream failed on written STEP content"
    );
    assert!(
        reader.transfer(&read_document, &range()).expect("transfer"),
        "Transfer to XCAF document failed"
    );
    (String::from_utf8_lossy(&stream).into_owned(), read_document)
}

/// The shape of the first free shape of a document.
fn first_root(document: &Handle<TDocStdDocument>) -> Shape {
    let main = document.get().expect("document").main().expect("main");
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("tool");
    let roots = tool.get_free_shapes().expect("roots");
    assert!(!roots.is_empty(), "No shapes in read-back document");
    let mut shape = Shape::default();
    martensite_tkxcaf::xcaf_doc::XCAFDocShapeTool::get_shape(&roots[0], &mut shape).expect("shape");
    assert!(!shape.is_null(), "Read-back shape should not be null");
    shape
}

/// A colour of the faces of a shape, or of the shape.
fn found_color(document: &Handle<TDocStdDocument>, shape: &Shape) -> Option<ColorRgba> {
    let main = document.get().expect("document").main().expect("main");
    let tool = XCAFDocDocumentTool::color_tool(&main).expect("tool");
    let of = |item: &Shape, kind| tool.get_shape_color(item, kind).expect("color");
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).expect("explorer");
    while faces.more() {
        let face = faces.current().clone();
        let color = of(&face, XCAFDocColorType::ColorSurf)
            .or_else(|| of(&face, XCAFDocColorType::ColorGen));
        if color.is_some() {
            return color;
        }
        faces.next().expect("next");
    }
    of(shape, XCAFDocColorType::ColorGen).or_else(|| of(shape, XCAFDocColorType::ColorSurf))
}

#[test]
fn bug23951_write_document_with_visibility() {
    let _alone = alone();
    STEPCAFControlController::init().expect("controller");
    let document = new_document();
    let shape = a_box(1.0, 1.0, 1.0);
    let main = document.get().expect("document").main().expect("main");
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("tool");
    let label = tool.new_shape().expect("label");
    tool.set_shape(&label, &shape).expect("shape");
    TDataStdName::set(&label, &ExtendedString::from_utf8("Box1")).expect("name");
    let yellow = Color::named(NamedColor::YELLOW);
    let colors = XCAFDocDocumentTool::color_tool(&main).expect("tool");
    colors
        .set_color(
            &label,
            &ColorRgba::from_rgb(yellow, 1.0),
            XCAFDocColorType::ColorGen,
        )
        .expect("color");
    colors.set_visibility(&label, false).expect("visibility");
    let mut writer = STEPCAFControlWriter::new().expect("writer");
    assert!(
        writer
            .transfer_document(&document, StepControlStepModelType::AS_IS, None, &range())
            .expect("transfer"),
        "Transfer failed"
    );
    let (content, read_document) = written_and_read(&writer);
    assert!(
        content.contains("DRAUGHTING_PRE_DEFINED_COLOUR('yellow')"),
        "Yellow DRAUGHTING_PRE_DEFINED_COLOUR entity not found in STEP stream"
    );
    assert!(
        content.contains("INVISIBILITY"),
        "INVISIBILITY entity not found in STEP stream"
    );
    let result = first_root(&read_document);
    let (source, found) = (contents(&shape), contents(&result));
    assert_eq!(found[0], source[0], "Solid count mismatch");
    assert_eq!(found[2], source[2], "Face count mismatch");
    assert_eq!(found[4], source[4], "Edge count mismatch");
    assert_eq!(found[5], source[5], "Vertex count mismatch");
    let color = *found_color(&read_document, &result)
        .expect("Yellow color not found on read-back shape")
        .rgb();
    assert!(
        (color.red() - yellow.red()).abs() <= 0.05,
        "Color red channel mismatch"
    );
    assert!(
        (color.green() - yellow.green()).abs() <= 0.05,
        "Color green channel mismatch"
    );
    assert!(
        (color.blue() - yellow.blue()).abs() <= 0.05,
        "Color blue channel mismatch"
    );
}

#[test]
fn bug23950_write_document_with_vertex_name() {
    let _alone = alone();
    STEPCAFControlController::init().expect("controller");
    let document = new_document();
    let mut maker = BRepBuilderAPIMakeVertex::new(Point3::new(75., 0., 0.)).expect("vertex");
    let vertex = maker.shape().expect("shape").clone();
    let main = document.get().expect("document").main().expect("main");
    let tool = XCAFDocDocumentTool::shape_tool(&main).expect("tool");
    let label = tool.new_shape().expect("label");
    tool.set_shape(&label, &vertex).expect("shape");
    TDataStdName::set(&label, &ExtendedString::from_utf8("Point1")).expect("name");
    let assembly = tool.new_shape().expect("label");
    TDataStdName::set(&assembly, &ExtendedString::from_utf8("ASSEMBLY")).expect("name");
    let component = tool
        .add_component(&assembly, &label, &Location::default())
        .expect("component");
    tool.update_assemblies().expect("update");
    let colors = XCAFDocDocumentTool::color_tool(&main).expect("tool");
    colors
        .set_color(
            &component,
            &ColorRgba::from_rgb(Color::named(NamedColor::YELLOW), 1.0),
            XCAFDocColorType::ColorGen,
        )
        .expect("color");
    colors
        .set_visibility(&component, false)
        .expect("visibility");
    let parameters = DeStepParameters {
        write_vertex_mode: DeStepWriteModeVertexMode::SINGLE_VERTEX,
        ..Default::default()
    };
    let mut writer = STEPCAFControlWriter::new().expect("writer");
    assert!(
        writer
            .transfer_document_with(
                &document,
                &parameters,
                StepControlStepModelType::AS_IS,
                None,
                &range()
            )
            .expect("transfer"),
        "Transfer failed"
    );
    let (content, read_document) = written_and_read(&writer);
    assert!(
        content.contains("Point1"),
        "Vertex name 'Point1' not found in STEP stream"
    );
    assert!(
        content.contains("DRAUGHTING_PRE_DEFINED_COLOUR('yellow')"),
        "Yellow DRAUGHTING_PRE_DEFINED_COLOUR entity not found in STEP stream"
    );
    assert!(
        content.contains("INVISIBILITY"),
        "INVISIBILITY entity not found in STEP stream"
    );
    let root = first_root(&read_document);
    let vertices = Explorer::new(&root, ShapeEnum::Vertex, ShapeEnum::Shape).expect("explorer");
    assert!(vertices.more(), "No vertex found in read-back shape");
    let point = BRepTool::pnt(&topods::vertex(vertices.current()).expect("vertex")).expect("point");
    assert!(
        (point.x() - 75.0).abs() <= precision::confusion(),
        "Vertex X coordinate mismatch"
    );
    assert!(
        point.y().abs() <= precision::confusion(),
        "Vertex Y coordinate mismatch"
    );
    assert!(
        point.z().abs() <= precision::confusion(),
        "Vertex Z coordinate mismatch"
    );
}
