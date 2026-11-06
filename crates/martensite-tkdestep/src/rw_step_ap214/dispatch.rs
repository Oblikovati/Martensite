//! The modules that tie every STEP entity type to its reader and writer, by case number.
//!
//! The switches are the tables of this folder (`table_*.rs`, generated). A arm with no
//! body falls through to the next arm, and the tables keep that: such a case reaches the next RW
//! class with an entity of another type, which is the null dereference (a null object failure).
use super::complex_cases::{COMPLEX_CASES, COMPLEX_TYPES, complex_case};
use super::table_index::*;
use super::tables::*;
use crate::rw_header_section::RWHeaderSection;
use crate::step_ap214::StepAP214;
use crate::step_rw::*;
use martensite_tkernel::impl_transient;
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCategory, InterfaceCopyTool, InterfaceGeneralLib, InterfaceGeneralModule,
    InterfaceProtocol, InterfaceReaderLib, InterfaceReaderModule,
};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{LazyLock, OnceLock};

/// The first text of the recognition, tested before the maps because it is the most frequent.
const CARTESIAN_POINT: (&[u8], i32) = (b"CARTESIAN_POINT", 59);
/// `PasReco`: the type name of a case that has none.
const NOT_RECOGNIZED: &str = "?";

/// Merges the parts of a generated table.
fn table_of<K: Eq + Hash + Copy, V: Copy>(parts: &[&[(K, V)]]) -> HashMap<K, V> {
    parts.iter().flat_map(|part| part.iter().copied()).collect()
}

static READERS: LazyLock<HashMap<i32, ReadFn>> = LazyLock::new(|| table_of(TABLE_READ_PARTS));
static WRITERS: LazyLock<HashMap<i32, WriteFn>> = LazyLock::new(|| table_of(TABLE_WRITE_PARTS));
static SHARES: LazyLock<HashMap<i32, ShareFn>> = LazyLock::new(|| table_of(TABLE_SHARE_PARTS));
static CHECKS: LazyLock<HashMap<i32, CheckFn>> = LazyLock::new(|| table_of(TABLE_CHECK_PARTS));
static NEW_ENTITIES: LazyLock<HashMap<i32, NewFn>> = LazyLock::new(|| table_of(TABLE_NEW_PARTS));
static CATEGORIES: LazyLock<HashMap<i32, StepCategory>> =
    LazyLock::new(|| table_of(TABLE_CATEGORY_PARTS));
static STEP_TYPES: LazyLock<HashMap<i32, &'static str>> =
    LazyLock::new(|| table_of(TABLE_STEP_TYPE_PARTS));
static TYPE_NUMBERS: LazyLock<HashMap<&'static str, i32>> =
    LazyLock::new(|| table_of(TABLE_TYPENUMS_PARTS));
static SHORT_NUMBERS: LazyLock<HashMap<&'static str, i32>> =
    LazyLock::new(|| table_of(TABLE_TYPESHOR_PARTS));

/// The number the constructor gives each category.
static CATEGORY_NUMBERS: LazyLock<[(StepCategory, i32); 5]> = LazyLock::new(|| {
    InterfaceCategory::init();
    [
        (StepCategory::Sh, InterfaceCategory::number("Shape")),
        (StepCategory::Dr, InterfaceCategory::number("Drawing")),
        (StepCategory::Str, InterfaceCategory::number("Structure")),
        (StepCategory::Dsc, InterfaceCategory::number("Description")),
        (StepCategory::Aux, InterfaceCategory::number("Auxiliary")),
    ]
});

/// The category number of a case, 0 for a case of no category.
fn category_of(cn: i32) -> i32 {
    let Some(category) = CATEGORIES.get(&cn) else {
        return 0;
    };
    CATEGORY_NUMBERS
        .iter()
        .find(|(known, _)| known == category)
        .map_or(0, |(_, number)| *number)
}

/// `RWStepAP214`: installs the modules of the AP214 protocol.
#[derive(Debug, Clone, Copy, Default)]
pub struct RwStepAP214;

impl RwStepAP214 {
    /// `init`: registers the modules once, after those of the header section.
    ///
    /// ```
    /// martensite_tkdestep::rw_step_ap214::RwStepAP214::init();
    /// ```
    pub fn init() {
        static DONE: OnceLock<()> = OnceLock::new();
        DONE.get_or_init(|| {
            RWHeaderSection::init();
            let protocol = StepAP214::protocol();
            let any: Handle<dyn InterfaceProtocol> = protocol.cast();
            let step: Handle<dyn StepDataProtocolOps> = protocol.cast();
            let general = Handle::new(RwStepAP214GeneralModule::default());
            InterfaceGeneralLib::set_global(&general.cast(), &any);
            let module = Handle::new(RwStepAP214ReadWriteModule::default());
            InterfaceReaderLib::set_global(&module.cast(), &any);
            StepDataWriterLib::set_global(&module.cast(), &step);
        });
    }
}

/// `RwStepAP214ReadWriteModule`: names, reading and writing by case number.
#[derive(Default)]
pub struct RwStepAP214ReadWriteModule {
    transient: StandardTransient,
}
impl_transient!(
    RwStepAP214ReadWriteModule,
    "RwStepAP214ReadWriteModule",
    dyn StepDataReadWriteModule,
    transient,
    interfaces: [dyn InterfaceReaderModule, dyn StepDataReadWriteModule]
);
crate::impl_step_read_write_module!(RwStepAP214ReadWriteModule);

impl RwStepAP214ReadWriteModule {
    /// The complex case of names converted to long names and sorted, by the bubble sort.
    fn sorted_case(&self, types: &[AsciiString]) -> i32 {
        let mut sorted: Vec<&[u8]> = types.iter().map(AsciiString::as_bytes).collect();
        sorted.sort_unstable();
        let eq = |rank: usize, case: i32| {
            sorted
                .get(rank - 1)
                .is_some_and(|name| *name == self.step_type(case).as_bytes())
        };
        let eq_text = |rank: usize, text: &str| {
            sorted
                .get(rank - 1)
                .is_some_and(|name| *name == text.as_bytes())
        };
        complex_case(sorted.len(), &eq, &eq_text)
    }
}

impl StepDataReadWriteModule for RwStepAP214ReadWriteModule {
    fn case_step(&self, atype: &AsciiString) -> i32 {
        let key = atype.as_bytes();
        if key == CARTESIAN_POINT.0 {
            return CARTESIAN_POINT.1;
        }
        let Ok(text) = std::str::from_utf8(key) else {
            return 0;
        };
        TYPE_NUMBERS
            .get(text)
            .or_else(|| SHORT_NUMBERS.get(text))
            .copied()
            .unwrap_or(0)
    }
    /// A list holding a short name is converted to long names first.
    fn case_step_complex(&self, types: &[AsciiString]) -> i32 {
        if types.len() < 2 {
            return 0;
        }
        let short_case = |name: &AsciiString| {
            std::str::from_utf8(name.as_bytes())
                .ok()
                .and_then(|text| SHORT_NUMBERS.get(text).copied())
        };
        if !types.iter().any(|name| short_case(name).is_some()) {
            return self.sorted_case(types);
        }
        let longs: Vec<AsciiString> = types
            .iter()
            .map(|name| {
                short_case(name).map_or_else(
                    || name.clone(),
                    |case| AsciiString::new(self.step_type(case)),
                )
            })
            .collect();
        self.case_step_complex(&longs)
    }
    fn is_complex(&self, cn: i32) -> bool {
        COMPLEX_CASES.binary_search(&cn).is_ok()
    }
    fn step_type(&self, cn: i32) -> &'static str {
        STEP_TYPES.get(&cn).copied().unwrap_or(NOT_RECOGNIZED)
    }
    fn complex_type(&self, cn: i32, types: &mut Vec<AsciiString>) -> bool {
        let Some((_, members)) = COMPLEX_TYPES.iter().find(|(case, _)| *case == cn) else {
            return false;
        };
        types.extend(
            members
                .iter()
                .map(|member| AsciiString::new(self.step_type(*member))),
        );
        true
    }
    fn read_step(
        &self,
        cn: i32,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &mut Handle<InterfaceCheck>,
        ent: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        if cn == 0 {
            return Ok(());
        }
        let Some(read) = READERS.get(&cn) else {
            ach.add_fail_text("Type Mismatch when reading - Entity", "");
            return Ok(());
        };
        read(data, num, ach, ent)
    }
    fn write_step(
        &self,
        cn: i32,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        WRITERS.get(&cn).map_or(Ok(()), |write| write(sw, ent))
    }
}

/// `RwStepAP214GeneralModule`: sharing, checks, new entities and categories by case number.
#[derive(Default)]
pub struct RwStepAP214GeneralModule {
    transient: StandardTransient,
}
impl_transient!(
    RwStepAP214GeneralModule,
    "RwStepAP214GeneralModule",
    dyn StepDataGeneralModule,
    transient,
    interfaces: [dyn InterfaceGeneralModule, dyn StepDataGeneralModule]
);

impl StepDataGeneralModule for RwStepAP214GeneralModule {}

impl InterfaceGeneralModule for RwStepAP214GeneralModule {
    /// The `fill_shared_case` cannot report: a null dereference here stops the sharing
    /// of this entity, keeping what it already added.
    fn fill_shared_case(
        &self,
        cn: i32,
        ent: &Handle<StandardTransient>,
        iter: &mut InterfaceEntityIterator,
    ) {
        if let Some(share) = SHARES.get(&cn) {
            let _stopped_at_a_null_member = share(ent, iter);
        }
    }
    fn check_case(
        &self,
        cn: i32,
        ent: &Handle<StandardTransient>,
        shares: &InterfaceShareTool,
        ach: &mut Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        CHECKS
            .get(&cn)
            .map_or(Ok(()), |check| check(ent, shares, ach))
    }
    fn new_void(&self, cn: i32, entto: &mut Handle<StandardTransient>) -> bool {
        let Some(create) = NEW_ENTITIES.get(&cn) else {
            return false;
        };
        *entto = create();
        true
    }
    /// Empty.
    fn copy_case(
        &self,
        _cn: i32,
        _entfrom: &Handle<StandardTransient>,
        _entto: &Handle<StandardTransient>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        Ok(())
    }
    fn category_number(
        &self,
        cn: i32,
        _ent: &Handle<StandardTransient>,
        _shares: &InterfaceShareTool,
    ) -> i32 {
        category_of(cn)
    }
    /// A null name whatever the entity.
    fn name(
        &self,
        _cn: i32,
        _ent: &Handle<StandardTransient>,
        _shares: &InterfaceShareTool,
    ) -> Handle<HAsciiString> {
        Handle::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn module() -> RwStepAP214ReadWriteModule {
        RwStepAP214ReadWriteModule::default()
    }

    #[test]
    fn long_and_short_names_reach_the_same_case() {
        let module = module();
        assert_eq!(module.case_step(&AsciiString::new("CARTESIAN_POINT")), 59);
        assert_eq!(module.case_step(&AsciiString::new("ADDRESS")), 1);
        assert_eq!(module.case_step(&AsciiString::new("ADDRSS")), 1);
        assert_eq!(module.case_step(&AsciiString::new("NOT_A_STEP_TYPE")), 0);
        assert_eq!(module.step_type(1), "ADDRESS");
        assert_eq!(module.step_type(0), "?");
    }

    #[test]
    fn every_read_case_with_a_name_has_a_new_entity_of_the_read_type_kind() {
        let general = RwStepAP214GeneralModule::default();
        let mut entity = Handle::default();
        for case in READERS.keys() {
            assert!(
                general.new_void(*case, &mut entity),
                "case {case} has a reader and no new entity"
            );
            assert!(entity.is_some(), "case {case} created a null entity");
        }
    }

    /// The tables of 694 and 696 hold one name twice, so these two are not recognized
    /// from their own names; the readers of these cases are reached from the specific tolerance.
    /// `complex_type` of 800 is one fixed list, though the names of that record depend on its kind
    /// of pair: `RWActuatedKinPairAndOrderKinPair` is reached through `actuated_pair_in_a_complex_record_keeps_each_direction`.
    #[test]
    fn complex_names_are_recognized_in_any_order() {
        let module = module();
        for case in COMPLEX_CASES.iter().filter(|case| **case != 800) {
            let mut names = Vec::new();
            assert!(module.complex_type(*case, &mut names), "case {case}");
            let mut sorted: Vec<&[u8]> = names.iter().map(AsciiString::as_bytes).collect();
            sorted.sort_unstable();
            sorted.dedup();
            let repeats_a_name = sorted.len() != names.len();
            names.reverse();
            assert_eq!(
                module.case_step_complex(&names) == 0,
                repeats_a_name,
                "case {case}"
            );
        }
    }

    #[test]
    fn a_single_name_is_no_complex_entity() {
        assert_eq!(
            module().case_step_complex(&[AsciiString::new("ADDRESS")]),
            0
        );
    }

    #[test]
    fn an_unknown_case_fails_the_check_and_writes_nothing() {
        let module = module();
        let mut ach = Handle::new(InterfaceCheck::default());
        let data = Handle::default();
        module
            .read_step(9999, &data, 1, &mut ach, &Handle::default())
            .unwrap();
        assert!(ach.get().unwrap().has_failed());
        let mut clean = Handle::new(InterfaceCheck::default());
        module
            .read_step(0, &data, 1, &mut clean, &Handle::default())
            .unwrap();
        assert!(!clean.get().unwrap().has_failed());
    }

    #[test]
    fn categories_come_from_the_names() {
        assert_eq!(category_of(1), InterfaceCategory::number("Description"));
        assert_eq!(category_of(2), InterfaceCategory::number("Shape"));
        assert_eq!(category_of(9999), 0);
    }
}
