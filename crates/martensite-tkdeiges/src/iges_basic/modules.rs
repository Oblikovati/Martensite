use super::{
    IgesBasicAssocGroupType, IgesBasicExternalRefFile, IgesBasicExternalRefFileIndex,
    IgesBasicExternalRefFileName, IgesBasicExternalRefLibName, IgesBasicExternalRefName,
    IgesBasicExternalReferenceFile, IgesBasicGroup, IgesBasicGroupWithoutBackP, IgesBasicHierarchy,
    IgesBasicName, IgesBasicOrderedGroup, IgesBasicOrderedGroupWithoutBackP, IgesBasicSingleParent,
    IgesBasicSingularSubfigure, IgesBasicSubfigureDef, IgesBasicToolAssocGroupType,
    IgesBasicToolExternalRefFile, IgesBasicToolExternalRefFileIndex,
    IgesBasicToolExternalRefFileName, IgesBasicToolExternalRefLibName,
    IgesBasicToolExternalRefName, IgesBasicToolExternalReferenceFile, IgesBasicToolGroup,
    IgesBasicToolGroupWithoutBackP, IgesBasicToolHierarchy, IgesBasicToolName,
    IgesBasicToolOrderedGroup, IgesBasicToolOrderedGroupWithoutBackP, IgesBasicToolSingleParent,
    IgesBasicToolSingularSubfigure, IgesBasicToolSubfigureDef,
};
use crate::iges_data::{iges_data_init, iges_data_protocol};
use crate::iges_package_tables;
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkxsbase::interface::InterfaceCategory;

/// The case of a type and form, 0 when not recognised.
fn iges_basic_case_iges(typenum: i32, formnum: i32) -> i32 {
    match (typenum, formnum) {
        (308, _) => 16,
        (402, 1) => 8,
        (402, 7) => 9,
        (402, 9) => 14,
        (402, 12) => 3,
        (402, 14) => 12,
        (402, 15) => 13,
        (406, 10) => 10,
        (406, 12) => 7,
        (406, 15) => 11,
        (406, 23) => 1,
        (408, _) => 15,
        (416, 0) => 4,
        (416, 1) => 2,
        (416, 2) => 4,
        (416, 3) => 6,
        (416, 4) => 5,
        _ => 0,
    }
}

/// The category of a case.
fn iges_basic_category(cn: i32, _ent: &Handle<StandardTransient>) -> i32 {
    if cn == 10 || cn == 11 {
        return InterfaceCategory::number("Auxiliary");
    }
    if (2..=6).contains(&cn) {
        return InterfaceCategory::number("Description");
    }
    InterfaceCategory::number("Structure")
}

iges_package_tables! {
package: IgesBasic,
protocol: IgesBasicProtocol "IgesBasicProtocol",
read_write: IgesBasicReadWriteModule "IgesBasicReadWriteModule",
general: IgesBasicGeneralModule "IgesBasicGeneralModule",
specific: IgesBasicSpecificModule "IgesBasicSpecificModule",
resources: [1 => iges_data_protocol()],
prior_init: [iges_data_init()],
case_iges: iges_basic_case_iges,
category: iges_basic_category,
cases: [
1 => IgesBasicAssocGroupType, IgesBasicToolAssocGroupType;
2 => IgesBasicExternalRefFile, IgesBasicToolExternalRefFile;
3 => IgesBasicExternalRefFileIndex, IgesBasicToolExternalRefFileIndex;
4 => IgesBasicExternalRefFileName, IgesBasicToolExternalRefFileName;
5 => IgesBasicExternalRefLibName, IgesBasicToolExternalRefLibName;
6 => IgesBasicExternalRefName, IgesBasicToolExternalRefName;
7 => IgesBasicExternalReferenceFile, IgesBasicToolExternalReferenceFile;
8 => IgesBasicGroup, IgesBasicToolGroup;
9 => IgesBasicGroupWithoutBackP, IgesBasicToolGroupWithoutBackP;
10 => IgesBasicHierarchy, IgesBasicToolHierarchy;
11 => IgesBasicName, IgesBasicToolName;
12 => IgesBasicOrderedGroup, IgesBasicToolOrderedGroup;
13 => IgesBasicOrderedGroupWithoutBackP, IgesBasicToolOrderedGroupWithoutBackP;
14 => IgesBasicSingleParent, IgesBasicToolSingleParent;
15 => IgesBasicSingularSubfigure, IgesBasicToolSingularSubfigure;
16 => IgesBasicSubfigureDef, IgesBasicToolSubfigureDef;
],
corrected: [
1 => IgesBasicAssocGroupType, IgesBasicToolAssocGroupType;
8 => IgesBasicGroup, IgesBasicToolGroup;
9 => IgesBasicGroupWithoutBackP, IgesBasicToolGroupWithoutBackP;
10 => IgesBasicHierarchy, IgesBasicToolHierarchy;
11 => IgesBasicName, IgesBasicToolName;
12 => IgesBasicOrderedGroup, IgesBasicToolOrderedGroup;
13 => IgesBasicOrderedGroupWithoutBackP, IgesBasicToolOrderedGroupWithoutBackP;
14 => IgesBasicSingleParent, IgesBasicToolSingleParent;
],
new_void: [
1 => Handle::new(IgesBasicAssocGroupType::new()).cast();
2 => Handle::new(IgesBasicExternalRefFile::new()).cast();
3 => Handle::new(IgesBasicExternalRefFileIndex::new()).cast();
4 => Handle::new(IgesBasicExternalRefFileName::new()).cast();
5 => Handle::new(IgesBasicExternalRefLibName::new()).cast();
6 => Handle::new(IgesBasicExternalRefName::new()).cast();
7 => Handle::new(IgesBasicExternalReferenceFile::new()).cast();
8 => Handle::new(IgesBasicGroup::new()).cast();
9 => Handle::new(IgesBasicGroupWithoutBackP::new()).cast();
10 => Handle::new(IgesBasicHierarchy::new()).cast();
11 => Handle::new(IgesBasicName::new()).cast();
12 => Handle::new(IgesBasicOrderedGroup::new()).cast();
13 => Handle::new(IgesBasicOrderedGroupWithoutBackP::new()).cast();
14 => Handle::new(IgesBasicSingleParent::new()).cast();
15 => Handle::new(IgesBasicSingularSubfigure::new()).cast();
16 => Handle::new(IgesBasicSubfigureDef::new()).cast();
],
general_extra: {}
}
