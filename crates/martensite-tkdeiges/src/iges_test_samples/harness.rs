//! The round every sample entity goes through: into a file and back, a dump at every level, a check
//! and a copy, each by the modules of the packages.
use crate::iges_appli::IgesAppli;
use crate::iges_basic::{
    IgesBasicHArray1OfHArray1OfInteger, IgesBasicHArray1OfHArray1OfReal,
    IgesBasicHArray1OfHArray1OfXyz,
};
use crate::iges_data::{
    IgesDataIgesDumper, IgesDataIgesModel, IgesDataIgesWriter, IgesDataProtocolOps,
    IgesDataSpecificLib, iges_live, iges_new_array1,
};
use crate::iges_file::iges_file_read_stream;
use crate::iges_select::IgesSelectWorkLibrary;
use crate::iges_solid::IgesSolid;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheckTool, InterfaceCopyTool, InterfaceInterfaceModel, InterfaceModelEntities,
    InterfaceProtocol,
};

/// A shared list of the given items, from rank 1.
pub(crate) fn sample_list<T: Default + Clone + Send + Sync + 'static>(
    items: [T; 2],
) -> Result<Handle<HArray1<T>>, Failure> {
    let list = iges_new_array1::<T>(1, 2)?;
    let [first, second] = items;
    crate::iges_data::iges_array1_set(&list, 1, first, "sample_list")?;
    crate::iges_data::iges_array1_set(&list, 2, second, "sample_list")?;
    Ok(list)
}

macro_rules! nested_sample {
    ($name:ident, $class:ty, $element:ty, $first:expr, $second:expr) => {
        /// An array of two lists of two items.
        pub(crate) fn $name() -> Result<Handle<$class>, Failure> {
            let nested = <$class>::with_bounds(1, 2)?;
            for rank in 1..=2 {
                nested.set_value(rank, &sample_list::<$element>([$first, $second])?)?;
            }
            Ok(Handle::new(nested))
        }
    };
}
nested_sample!(
    h_array1_of_h_array1_of_integer_sample,
    IgesBasicHArray1OfHArray1OfInteger,
    i32,
    3,
    4
);
nested_sample!(
    h_array1_of_h_array1_of_real_sample,
    IgesBasicHArray1OfHArray1OfReal,
    f64,
    1.5,
    2.5
);
nested_sample!(
    h_array1_of_h_array1_of_xyz_sample,
    IgesBasicHArray1OfHArray1OfXyz,
    Xyz,
    Xyz::new(1.5, 2.5, 0.5),
    Xyz::new(3.5, 4.5, 0.5)
);
fn protocol() -> Handle<dyn IgesDataProtocolOps> {
    IgesAppli::init();
    IgesSolid::init();
    IgesSelectWorkLibrary::define_protocol()
}

fn model_of(
    prior: &Handle<StandardTransient>,
    entity: &Handle<StandardTransient>,
) -> Result<Handle<IgesDataIgesModel>, Failure> {
    let model = IgesDataIgesModel::new();
    let any: Handle<dyn InterfaceProtocol> = protocol().cast();
    model.add_with_refs_protocol(prior, &any, 0, false)?;
    model.add_with_refs_protocol(entity, &any, 0, false)?;
    Ok(model)
}

fn file_of(model: &Handle<IgesDataIgesModel>) -> Result<Vec<u8>, Failure> {
    let mut writer = IgesDataIgesWriter::new(model);
    writer.send_model(&protocol())?;
    let mut text = Vec::new();
    assert!(writer.print(&mut text)?);
    Ok(text)
}

fn read_file(mut text: &[u8]) -> Result<Handle<IgesDataIgesModel>, Failure> {
    let model = IgesDataIgesModel::new();
    assert_eq!(
        iges_file_read_stream("sample", &mut text, &model, &protocol())?,
        0
    );
    Ok(model)
}

/// A file's content but the global section, which holds the time of writing.
fn file_sections(text: &[u8]) -> Vec<Vec<u8>> {
    text.split(|byte| *byte == b'\n')
        .filter(|line| line.get(72) != Some(&b'G'))
        .map(<[u8]>::to_vec)
        .collect()
}

fn assert_dumps(model: &Handle<IgesDataIgesModel>, name: &str) -> Result<(), Failure> {
    let dumper = IgesDataIgesDumper::new(model, &protocol());
    let any: Handle<dyn InterfaceInterfaceModel> = model.cast();
    for number in 1..=any.nb_entities() {
        let entity = any.value(number)?;
        let mut last = Vec::new();
        for level in [0, 1, 4, 5, 6, 7] {
            last.clear();
            if let Err(failure) = dumper.own_dump(&entity, &mut last, level) {
                panic!("{name}: dump of entity {number} at level {level}: {failure:?}");
            }
        }
        let class = entity.dynamic_type().map_or("", |kind| kind.name());
        assert!(
            String::from_utf8_lossy(&last).contains(class),
            "{name}: the dump of {class} should name it: {}",
            String::from_utf8_lossy(&last)
        );
    }
    Ok(())
}

fn assert_check_runs(model: &Handle<IgesDataIgesModel>) -> Result<(), Failure> {
    let any: Handle<dyn InterfaceInterfaceModel> = model.cast();
    let protocol: Handle<dyn InterfaceProtocol> = protocol().cast();
    let tool = InterfaceCheckTool::with_protocol(&any, &protocol)?;
    for number in 1..=any.nb_entities() {
        tool.check(number)?;
    }
    Ok(())
}

/// What a correction leaves can still be written; a correction may report a change each time.
fn assert_corrections_settle(model: &Handle<IgesDataIgesModel>) -> Result<(), Failure> {
    let lib = IgesDataSpecificLib::new(&protocol());
    let any: Handle<dyn InterfaceInterfaceModel> = model.cast();
    for number in 1..=any.nb_entities() {
        let entity = any.value(number)?;
        let Some((module, case)) = lib.select(&entity) else {
            continue;
        };
        let live = iges_live(&module, "corrections")?;
        live.own_correct(case, &entity.cast())?;
    }
    file_of(model).map(drop)
}

const CUTS_PER_RECORD: usize = 10;

/// The first record's parameters, when they hold on one line of the file, split at their commas.
fn first_record_parameters(lines: &[&str]) -> Option<(usize, Vec<String>)> {
    let at = lines
        .iter()
        .position(|line| line.as_bytes().get(72) == Some(&b'P'))?;
    let text = lines[at].get(..64)?.trim_end();
    let body = text.strip_suffix(';')?;
    Some((at, body.split(',').map(str::to_string).collect()))
}

/// A record cut short at any parameter is read to its end, and what was read is written or refused
/// by a failure: nothing in a file can bring the reader down.
fn assert_short_records_are_survived(file: &[u8]) -> Result<(), Failure> {
    let text = String::from_utf8_lossy(file).to_string();
    let lines: Vec<&str> = text.lines().collect();
    let Some((at, parameters)) = first_record_parameters(&lines) else {
        return Ok(());
    };
    let step = parameters.len().div_ceil(CUTS_PER_RECORD).max(1);
    for kept in (1..parameters.len()).step_by(step) {
        let mut cut = parameters[..kept].join(",");
        cut.push(';');
        let tail = lines[at].get(64..).unwrap_or("");
        let mut changed: Vec<String> = lines.iter().map(|line| (*line).to_string()).collect();
        changed[at] = format!("{cut:<64}{tail}");
        let bytes = (changed.join("\n") + "\n").into_bytes();
        // Whether what was read can be written depends on where the record was cut.
        drop(file_of(&read_file(&bytes)?));
    }
    Ok(())
}

fn copied_model(model: &Handle<IgesDataIgesModel>) -> Result<Handle<IgesDataIgesModel>, Failure> {
    let any: Handle<dyn InterfaceInterfaceModel> = model.cast();
    let protocol: Handle<dyn InterfaceProtocol> = protocol().cast();
    let mut tool = InterfaceCopyTool::with_protocol(&any, &protocol)?;
    for number in 1..=any.nb_entities() {
        tool.transfer_entity(&any.value(number)?)?;
    }
    let copy = IgesDataIgesModel::new();
    tool.fill_model(&copy.cast())?;
    Ok(copy)
}

/// The file an entity and what it refers to make.
pub(crate) fn iges_written_file<T: ?Sized + RuntimeTyped>(
    object: &Handle<T>,
) -> Result<Vec<u8>, Failure> {
    file_of(&model_of(&Handle::default(), &object.cast())?)
}

/// The model read back from the file an entity makes.
pub(crate) fn iges_round_read<T: ?Sized + RuntimeTyped>(
    object: &Handle<T>,
) -> Result<Handle<IgesDataIgesModel>, Failure> {
    read_file(&iges_written_file(object)?)
}

/// Carries an entity set up from samples through everything the modules do with it.
///
/// A file written from what was read is the file that was read: no field is lost or changed by the
/// reader or the writer. A copy writes the same parameters. A dump names the class at every level, and
/// a check runs to its end.
pub(crate) fn assert_entity_round<T: ?Sized + RuntimeTyped>(
    object: &Handle<T>,
    name: &str,
) -> Result<(), Failure> {
    assert_entity_round_after(&Handle::<StandardTransient>::default(), object, name)
}

/// [`assert_entity_round`] with an entity that must come first in the file: a reader takes an
/// attribute table's layout from its definition, loaded before it.
pub(crate) fn assert_entity_round_after<P: ?Sized + RuntimeTyped, T: ?Sized + RuntimeTyped>(
    prior: &Handle<P>,
    object: &Handle<T>,
    name: &str,
) -> Result<(), Failure> {
    let model = model_of(&prior.cast(), &object.cast())?;
    let first = file_of(&model)?;
    let again = read_file(&first)?;
    let second = file_of(&again)?;
    assert_eq!(
        String::from_utf8_lossy(&file_sections(&first).concat()),
        String::from_utf8_lossy(&file_sections(&second).concat()),
        "{name}: the file read back writes the same"
    );
    assert_dumps(&model, name)?;
    assert_check_runs(&model)?;
    assert_corrections_settle(&again)?;
    assert_short_records_are_survived(&first)?;
    let copy = copied_model(&model)?;
    assert_eq!(
        String::from_utf8_lossy(&file_sections(&first).concat()),
        String::from_utf8_lossy(&file_sections(&file_of(&copy)?).concat()),
        "{name}: the copy writes the same"
    );
    Ok(())
}
