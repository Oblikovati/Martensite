use crate::table::*;

de_case!(
    de_step_5_b7,
    DeRow {
        source: "tests/de/step_5/B7",
        file: "bug21802_as1-oc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 2.825576197e-005  ( 2.825576197e-005 )  AvgTol   =  6.514039857e-006  (  6.514039857e-006 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = BLUE DARKORANGE1 GREEN RED YELLOW2  ( BLUE DARKORANGE1 GREEN RED YELLOW2 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 256  ( 256 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_5_b8,
    DeRow {
        source: "tests/de/step_5/B8",
        file: "bug29241.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: TPSTAT : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 22 )  Warnings = 0  ( 22 )  Summary  = 0  ( 44 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 2  ( 1 )  Summary  = 2  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 205  ( 205 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 205  ( 205 )   FreeWire = 0  ( 1 ) ",
            "TOLERANCE   : MaxTol   = 0.000584860357  ( 0.0009118447559 )  AvgTol   =  2.354671626e-005  (  3.258060117e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 3  ( 18 )  N2Labels = 0  ( 0 )   TotalLabels = 4  ( 19 )   NameLabels = 1  ( 1 )   ColorLabels = 3  ( 3 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = LIGHTSTEELBLUE1 PALEGOLDENROD WHITE  ( LIGHTSTEELBLUE1 PALEGOLDENROD WHITE )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
