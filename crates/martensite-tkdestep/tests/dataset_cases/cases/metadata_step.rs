use crate::table::*;

metadata_case!(
    metadata_step_a1,
    MetadataRow {
        source: "tests/metadata/step/A1",
        file: "bug28345_30338.stp",
        ref_size: 41611,
        check_metadata: true,
        reference: &[
            "Property for [0:1:1:1]:",
            "H_CIP : 55.545955351400004",
            "ProductID : MVE0300X02S030_ASM",
            "ProductName : MVE0300X02S030_ASM",
            "Description : NOT SPECIFIED",
            "Property for [0:1:1:2]:",
            "H : 45",
            "E : 55",
            "B : 16",
            "I : 15",
            "A : 3",
            "D : 3",
            "C : 140",
            "F : 0.29999999999999999",
            "DESCRIPTION : ",
            "MODELED_BY : ",
            "ProductID : CUT_MVE0300X02S030",
            "ProductName : CUT_MVE0300X02S030",
            "Description : NOT SPECIFIED",
            "Property for [0:1:1:3]:",
            "H : 45",
            "E : 55",
            "B : 16",
            "I : 15",
            "A : 3",
            "D : 3",
            "C : 140",
            "F : 0.29999999999999999",
            "DESCRIPTION : ",
            "MODELED_BY : ",
            "ProductID : NOCUT_MVE0300X02S030",
            "ProductName : NOCUT_MVE0300X02S030",
            "Description : NOT SPECIFIED",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
metadata_case!(
    metadata_step_a4,
    MetadataRow {
        source: "tests/metadata/step/A4",
        file: "nist_ftc_08_asme1_ap242-2.stp",
        ref_size: 334466,
        check_metadata: true,
        reference: &[
            "Property for [0:1:1:1]:",
            "ProductID : NIST PMI FTC 08 ASME1",
            "ProductDefinition :  ",
            "Revision : C",
            "PartNumber : NIST PMI FTC 08 ASME1",
            "DescriptionRef : NIST PMI test model downloaded from http://go.usa.gov/mGVm",
            "ProductName : NIST PMI FTC 08 ASME1",
            "Property for [0:1:1:2]:",
            "ProductID : SOLID",
            "ProductName : SOLID",
            "Property for [0:1:1:3]:",
            "ProductID : SHELL",
            "ProductName : SHELL",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
