//! The data of an IGES file: the directory part all entities share, the models and protocols, the
//! parameter reader and the file writer.
mod basic_editor;
#[cfg(test)]
mod core_tests;
#[cfg(test)]
mod core_tests_model;
#[cfg(test)]
mod core_tests_tools;
mod def_switch;
mod default_modules;
mod dir_checker;
mod dir_part;
mod dump;
mod dump_levels;
mod dumper;
mod entity;
mod entity_lists;
mod entity_macro;
mod enums;
mod file_recognizer;
mod free_format_entity;
mod global_section;
pub(crate) mod global_section_dates;
mod global_section_fields;
mod iges_type;
mod kinds;
mod model;
mod model_dump;
mod model_verify;
mod module_bodies;
mod modules;
mod package;
mod param_cursor;
mod param_reader;
mod param_reader_entity;
mod param_reader_items;
mod param_reader_lists;
mod param_reader_read;
mod protocol;
mod reader_data;
mod reader_tool;
mod reader_tool_dir;
mod tool_location;
mod undefined_entity;
mod writer;
mod writer_directory;
mod writer_param;
mod writer_print;

pub use basic_editor::IgesDataBasicEditor;
pub use def_switch::IgesDataDefSwitch;
pub use default_modules::{IgesDataDefaultGeneral, IgesDataDefaultSpecific};
pub use dir_checker::IgesDataDirChecker;
pub use dir_part::{IgesDataDirPart, IgesDataDirText, IgesDataDirValues};
pub use dump::{
    IgesDataDumpValue, iges_dump_list_header, iges_dump_list_val, iges_dump_list_xy,
    iges_dump_list_xyz, iges_dump_string, iges_dump_xy, iges_dump_xy_l, iges_dump_xy_lz,
    iges_dump_xy_t, iges_dump_xy_tz, iges_dump_xyz, iges_dump_xyz_l, iges_dump_xyz_t,
    iges_real_text,
};
pub use dump_levels::{
    iges_dump_entities, iges_dump_list_xy_l, iges_dump_list_xy_lz, iges_dump_list_xyz_l,
    iges_dump_rect_vals, iges_dump_strings, iges_dump_vals,
};
pub use dumper::IgesDataIgesDumper;
pub use entity::{IgesDataEntityKind, IgesDataIgesEntity, iges_entity_of};
pub use entity_macro::{
    iges_array1_item, iges_array1_len, iges_array1_set, iges_array2_item, iges_array2_set,
    iges_live, iges_new_array1, iges_new_array2,
};
pub use enums::{IgesDataDefList, IgesDataDefType, IgesDataReadStage, IgesDataStatus};
pub use file_recognizer::{IgesDataFileRecognizer, IgesDataFileRecognizerBase};
pub use free_format_entity::IgesDataFreeFormatEntity;
pub use global_section::IgesDataGlobalSection;
pub use iges_type::IgesDataIgesType;
pub use kinds::{
    IgesDataColorEntity, IgesDataLabelDisplayEntity, IgesDataLevelListEntity,
    IgesDataLineFontEntity, IgesDataNameEntity, IgesDataSingleParentEntity, IgesDataTransfEntity,
    IgesDataViewKindEntity,
};
pub use model::IgesDataIgesModel;
pub use module_bodies::{
    iges_check_case, iges_copy_case, iges_fill_shared_case, iges_list_implied_case, iges_name_of,
    iges_renew_implied_case, iges_when_delete_case,
};
pub use modules::{
    IgesDataDumpError, IgesDataDumpResult, IgesDataGeneralModule, IgesDataGlobalNodeOfSpecificLib,
    IgesDataGlobalNodeOfWriterLib, IgesDataNodeOfSpecificLib, IgesDataNodeOfWriterLib,
    IgesDataReadWriteModule, IgesDataSpecificLib, IgesDataSpecificLibKind, IgesDataSpecificModule,
    IgesDataWriterLib, IgesDataWriterLibKind, iges_module_case_num,
};
pub use package::{iges_data_init, iges_data_protocol};
pub use param_cursor::IgesDataParamCursor;
pub use param_reader::{IgesDataParamReader, IgesDataReadMessage, iges_atoi, iges_msg};
pub(crate) use param_reader_items::{
    iges_read_integer_item, iges_read_real_item, iges_read_text_item,
};
pub use protocol::{
    IgesDataFileProtocol, IgesDataProtocol, IgesDataProtocolOps, iges_protocol_is_suitable_model,
    iges_protocol_is_unknown_entity, iges_protocol_new_model, iges_protocol_unknown_entity,
};
pub use reader_data::{IgesDataIgesReaderData, IgesDataTextLines};
pub use reader_tool::IgesDataIgesReaderTool;
pub use tool_location::IgesDataToolLocation;
pub use undefined_entity::IgesDataUndefinedEntity;
pub use writer::IgesDataIgesWriter;
pub use writer_param::IgesDataWriterParam;
