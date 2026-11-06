use crate::table::*;

de_case!(
    de_step_1_a7,
    DeRow {
        source: "tests/de/step_1/A7",
        file: "trj3_s1-id-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 32  ( 32 )  Summary  = 32  ( 32 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 101  ( 101 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 134  ( 134 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0002663644062  ( 0.004247173074 )  AvgTol   =  6.037675043e-006  (  9.943491328e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 18  ( 23 )  N2Labels = 0  ( 5 )   TotalLabels = 31  ( 41 )   NameLabels = 27  ( 32 )   ColorLabels = 12  ( 12 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_a8,
    DeRow {
        source: "tests/de/step_1/A8",
        file: "trj7_as1-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 14  ( 14 )  Summary  = 14  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 11  ( 11 )  N1Labels = 30  ( 30 )  N2Labels = 0  ( 0 )   TotalLabels = 41  ( 41 )   NameLabels = 26  ( 26 )   ColorLabels = 20  ( 20 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 14 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = BLUE DODGERBLUE2 GREEN RED  ( BLUE DODGERBLUE2 GREEN RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_a9,
    DeRow {
        source: "tests/de/step_1/A9",
        file: "trj7_as1-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 3  ( 3 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 22 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_b2,
    DeRow {
        source: "tests/de/step_1/B2",
        file: "trj7_as1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 3  ( 3 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 22 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_b3,
    DeRow {
        source: "tests/de/step_1/B3",
        file: "trj10_ap209-eb.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 37 )  Summary  = 0  ( 37 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 38  ( 38 )  Face     = 115  ( 115 ) ",
            "STATSHAPE   : Solid    = 2  ( 2 )  Shell    = 76  ( 76 )  Face     = 230  ( 230 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 41  ( 41 )  N1Labels = 41  ( 41 )  N2Labels = 0  ( 0 )   TotalLabels = 82  ( 82 )   NameLabels = 82  ( 81 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_b5,
    DeRow {
        source: "tests/de/step_1/B5",
        file: "trj8_as1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 22 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_b6,
    DeRow {
        source: "tests/de/step_1/B6",
        file: "trj7_k1-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 15  ( 18 )  Summary  = 15  ( 18 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 46  ( 46 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 46  ( 46 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.01475021965  (  0.01475021965 )  AvgTol   =  0.0004055200631  (  0.0004055200648 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 59  ( 61 )  N2Labels = 0  ( 0 )   TotalLabels = 60  ( 62 )   NameLabels = 1  ( 1 )   ColorLabels = 60  ( 62 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = DEEPSKYBLUE2 MAGENTA ORANGE ORANGERED RED  ( DEEPSKYBLUE2 MAGENTA ORANGE ORANGERED RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_c1,
    DeRow {
        source: "tests/de/step_1/C1",
        file: "trj8_as1-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 19 )  Summary  = 0  ( 19 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 35  ( 35 )  N2Labels = 0  ( 0 )   TotalLabels = 44  ( 44 )   NameLabels = 22  ( 22 )   ColorLabels = 27  ( 27 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 22 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 7  ( 7 )",
            "COLORS      : Colors   = BLUE DARKORANGE DARKORANGE GREEN RED YELLOW3 YELLOW3  ( BLUE DARKORANGE DARKORANGE GREEN RED YELLOW3 YELLOW3 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 256  ( 256 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_c5,
    DeRow {
        source: "tests/de/step_1/C5",
        file: "trj2_as1-id-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 14  ( 14 )  Summary  = 14  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 17 )  N2Labels = 0  ( 4 )   TotalLabels = 22  ( 30 )   NameLabels = 22  ( 26 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 6  ( 6 )  Volume   = 6  ( 6 )  Area     = 6  ( 6 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_c7,
    DeRow {
        source: "tests/de/step_1/C7",
        file: "trj8_k1-hc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 8 )  Summary  = 0  ( 8 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 52  ( 52 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 52  ( 52 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.002730674215  (   0.0116718272 )  AvgTol   =  0.0002131881112  (  0.001022773665 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 105  ( 53 )  N2Labels = 0  ( 0 )   TotalLabels = 107  ( 55 )   NameLabels = 3  ( 3 )   ColorLabels = 104  ( 53 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_c9,
    DeRow {
        source: "tests/de/step_1/C9",
        file: "trj3_as1-id-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 14  ( 14 )  Summary  = 14  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 17 )  N2Labels = 0  ( 4 )   TotalLabels = 22  ( 30 )   NameLabels = 22  ( 26 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 6  ( 6 )  Volume   = 6  ( 6 )  Area     = 6  ( 6 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_d9,
    DeRow {
        source: "tests/de/step_1/D9",
        file: "trj4_k1_geo-md-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 6  ( 6 )  Summary  = 6  ( 6 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 39  ( 0 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 39  ( 0 )  Face     = 39  ( 39 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.003673630603  ( 0.003673630602 )  AvgTol   =  0.0002911716538  (  0.0002911716546 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_e1,
    DeRow {
        source: "tests/de/step_1/E1",
        file: "trj4_k1_geo-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[("FromSTEP.FixShape.FixAddNaturalBoundMode", "0")],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 8 )  Summary  = 0  ( 8 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 52  ( 0 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 52  ( 0 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.007319568061  (  0.01121716749 )  AvgTol   =  0.0001836876899  (  0.0003721357713 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 5  ( 5 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_e2,
    DeRow {
        source: "tests/de/step_1/E2",
        file: "trj4_k1_geo-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[("FromSTEP.FixShape.FixAddNaturalBoundMode", "0")],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 8 )  Summary  = 0  ( 8 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 52  ( 0 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 52  ( 0 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.007319568061  (  0.01121716749 )  AvgTol   =  0.0001836876899  (  0.0003721357713 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 5  ( 5 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_e6,
    DeRow {
        source: "tests/de/step_1/E6",
        file: "trj4_s1-ai-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TOLERANCE : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 54 )  Warnings = 0  ( 0 )  Summary  = 0  ( 54 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 6  ( 53 )  Summary  = 6  ( 53 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 104  ( 104 ) ",
            "STATSHAPE   : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 104  ( 104 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =    0.949881784  ( 0.002238556699 )  AvgTol   =   0.05811636349  (  3.300213579e-005 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 8  ( 8 )  N2Labels = 0  ( 0 )   TotalLabels = 17  ( 17 )   NameLabels = 17  ( 17 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_e7,
    DeRow {
        source: "tests/de/step_1/E7",
        file: "trj5_k1-ct-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 1 )  Summary  = 1  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 58  ( 58 )  Face     = 58  ( 58 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 58  ( 58 )  Face     = 58  ( 58 )   FreeWire = 0  ( 16 ) ",
            "TOLERANCE   : MaxTol   = 0.0008167207419  ( 0.0009854215582 )  AvgTol   =  5.200297373e-005  (  0.0001118094952 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 60  ( 76 )  N2Labels = 0  ( 0 )   TotalLabels = 63  ( 79 )   NameLabels = 5  ( 5 )   ColorLabels = 58  ( 74 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_f1,
    DeRow {
        source: "tests/de/step_1/F1",
        file: "trj6_as1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 22 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_f2,
    DeRow {
        source: "tests/de/step_1/F2",
        file: "trj6_as1-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 14  ( 14 )  Summary  = 14  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 11  ( 11 )  N1Labels = 15  ( 15 )  N2Labels = 0  ( 0 )   TotalLabels = 26  ( 26 )   NameLabels = 26  ( 26 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 6  ( 19 )  Volume   = 6  ( 6 )  Area     = 6  ( 6 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_f3,
    DeRow {
        source: "tests/de/step_1/F3",
        file: "trj6_as1-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 22 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_f4,
    DeRow {
        source: "tests/de/step_1/F4",
        file: "trj6_as1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 22 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_f5,
    DeRow {
        source: "tests/de/step_1/F5",
        file: "trj6_as1-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 19 )  Summary  = 0  ( 19 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 22 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = BLUE DARKORANGE GREEN RED YELLOW3  ( BLUE DARKORANGE GREEN RED YELLOW3 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_f6,
    DeRow {
        source: "tests/de/step_1/F6",
        file: "trj6_as1-td-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 28  ( 50 )  Summary  = 28  ( 50 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 57  ( 57 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 172  ( 172 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 7.326474254e-006  ( 7.326473916e-006 )  AvgTol   =  2.405026732e-006  (  2.435678917e-006 )",
            "LABELS      : N0Labels = 14  ( 14 )  N1Labels = 20  ( 20 )  N2Labels = 0  ( 0 )   TotalLabels = 34  ( 34 )   NameLabels = 32  ( 32 )   ColorLabels = 7  ( 7 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 11  ( 24 )  Volume   = 11  ( 11 )  Area     = 11  ( 11 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_f7,
    DeRow {
        source: "tests/de/step_1/F7",
        file: "trj6_as1-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 19 )  Summary  = 0  ( 19 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 35  ( 35 )  N2Labels = 0  ( 0 )   TotalLabels = 44  ( 44 )   NameLabels = 22  ( 22 )   ColorLabels = 27  ( 27 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 22 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 7  ( 7 )",
            "COLORS      : Colors   = BLUE DARKORANGE DARKORANGE GREEN RED YELLOW3 YELLOW3  ( BLUE DARKORANGE DARKORANGE GREEN RED YELLOW3 YELLOW3 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 256  ( 256 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_g4,
    DeRow {
        source: "tests/de/step_1/G4",
        file: "trj8_k1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: LABELS : Faulty",
            "TODO CR23096 ALL: LAYERS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.002782096138  ( 0.008046347919 )  AvgTol   =  0.0002755551136  (  0.0008383045331 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 7  ( 9 )  N2Labels = 0  ( 0 )   TotalLabels = 10  ( 12 )   NameLabels = 5  ( 5 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 7 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = YELLOW  ( YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 2 )",
            "LAYERS      : Layers   = 02___PRT_ALL_AXES  ( 02___PRT_ALL_AXES 04___PRT_ALL_DTM_PNT )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_g5,
    DeRow {
        source: "tests/de/step_1/G5",
        file: "trj3_s1-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: TPSTAT : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty",
            "TODO CR23096 ALL: LAYERS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 8 )  Warnings = 0  ( 0 )  Summary  = 0  ( 8 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 36  ( 13 )  Summary  = 36  ( 13 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 9  ( 9 )  Shell    = 9  ( 9 )  Face     = 152  ( 152 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 178  ( 178 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =   0.0182458548  (   0.0182458548 )  AvgTol   =  0.0001972882708  (  0.0002289170005 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 12  ( 12 )  N2Labels = 0  ( 0 )   TotalLabels = 21  ( 21 )   NameLabels = 19  ( 19 )   ColorLabels = 9  ( 9 )   LayerLabels = 9  ( 19 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = DODGERBLUE2 MAGENTA RED WHITE YELLOW  ( DODGERBLUE2 MAGENTA RED WHITE YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 2 )",
            "LAYERS      : Layers   = 0  ( {  0} 255 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_g8,
    DeRow {
        source: "tests/de/step_1/G8",
        file: "trj4_k1_geo-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty",
            "TODO CR23096 ALL: LAYERS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 8 )  Summary  = 0  ( 8 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 59  ( 0 )  Face     = 59  ( 59 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 59  ( 0 )  Face     = 59  ( 59 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.003826736784  ( 0.003826736784 )  AvgTol   =  0.0001445169498  (  0.0001445176005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 59 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 60 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 59 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 1 )",
            "LAYERS      : Layers   =   ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_g9,
    DeRow {
        source: "tests/de/step_1/G9",
        file: "trj4_k1_geo-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[("FromSTEP.FixShape.FixAddNaturalBoundMode", "0")],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty",
            "TODO CR23096 ALL: LAYERS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 10 )  Summary  = 1  ( 10 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 44  ( 0 )  Face     = 44  ( 44 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 44  ( 0 )  Face     = 44  ( 44 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0004382667275  ( 0.007688098235 )  AvgTol   =  1.733347738e-005  (  0.0002416763952 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 44  ( 44 )  N2Labels = 0  ( 0 )   TotalLabels = 45  ( 45 )   NameLabels = 1  ( 1 )   ColorLabels = 44  ( 44 )   LayerLabels = 0  ( 44 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 0  ( 1 )",
            "LAYERS      : Layers   =   ( 001 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_k5,
    DeRow {
        source: "tests/de/step_1/K5",
        file: "bm2_ec_exhaust-A.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 6  ( 6 )  Shell    = 6  ( 6 )  Face     = 58  ( 58 ) ",
            "STATSHAPE   : Solid    = 6  ( 6 )  Shell    = 6  ( 6 )  Face     = 58  ( 58 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =    3.747413396  (    3.773005335 )  AvgTol   =    0.2774744124  (    0.2792979158 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_k6,
    DeRow {
        source: "tests/de/step_1/K6",
        file: "bm2_ec_lever-A.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 49  ( 49 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 49  ( 49 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 1.254371526e-007  ( 1.254371526e-007 )  AvgTol   =  1.008458897e-007  (  1.008458897e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_k7,
    DeRow {
        source: "tests/de/step_1/K7",
        file: "bm2_ec_leverrod-A.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 49  ( 49 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 49  ( 49 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 1.254371526e-007  ( 1.254371526e-007 )  AvgTol   =  1.008458897e-007  (  1.008458897e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_k8,
    DeRow {
        source: "tests/de/step_1/K8",
        file: "bm2_ec_t4-A.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 3  ( 3 )  Face     = 13  ( 13 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 3  ( 3 )  Face     = 13  ( 13 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0004295938988  ( 0.0007438683677 )  AvgTol   =  0.0001494340596  (  0.0002069211073 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_p8,
    DeRow {
        source: "tests/de/step_1/P8",
        file: "ec_exhaust-A.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 12 )  Warnings = 0  ( 0 )  Summary  = 0  ( 12 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 46  ( 46 ) ",
            "STATSHAPE   : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 46  ( 46 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.02183056591  (  0.02190890861 )  AvgTol   =  0.002096975786  (  0.002169792963 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 4  ( 4 )   NameLabels = 4  ( 4 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_s3,
    DeRow {
        source: "tests/de/step_1/S3",
        file: "t4mec.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 12  ( 12 )  Face     = 12  ( 12 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 12  ( 12 )  Face     = 12  ( 12 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.004636374094  ( 0.005314340756 )  AvgTol   =  0.001546342873  (  0.001821396818 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_s6,
    DeRow {
        source: "tests/de/step_1/S6",
        file: "tr10_r0901_ec.stp",
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
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_t2,
    DeRow {
        source: "tests/de/step_1/T2",
        file: "trj10_ap209-td.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 1 )  Summary  = 1  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.008505412228  ( 0.008990434575 )  AvgTol   =  0.0006167184304  (  0.0006516400657 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_t3,
    DeRow {
        source: "tests/de/step_1/T3",
        file: "trj7_k1-ct-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.000476839593  ( 0.000476839593 )  AvgTol   =  6.422491185e-005  (  6.422968681e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = LIGHTSTEELBLUE1  ( LIGHTSTEELBLUE1 )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_t4,
    DeRow {
        source: "tests/de/step_1/T4",
        file: "trj7_k1-sd-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 2  ( 2 )  Face     = 57  ( 57 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 2  ( 2 )  Face     = 57  ( 57 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 4.530986906e-007  ( 4.530984849e-007 )  AvgTol   =  1.197062502e-007  (  1.197062485e-007 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 2  ( 2 )   NameLabels = 2  ( 2 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = MEDIUMTURQUOISE  ( MEDIUMTURQUOISE )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_t5,
    DeRow {
        source: "tests/de/step_1/T5",
        file: "trj2_as1-ec-214.stp",
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
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_t6,
    DeRow {
        source: "tests/de/step_1/T6",
        file: "trj2_as1-in-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 4  ( 4 )  Summary  = 4  ( 4 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_t7,
    DeRow {
        source: "tests/de/step_1/T7",
        file: "trj2_as1-pe-203.stp",
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
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 6  ( 6 )  N1Labels = 71  ( 71 )  N2Labels = 0  ( 0 )   TotalLabels = 77  ( 77 )   NameLabels = 24  ( 24 )   ColorLabels = 8  ( 8 )   LayerLabels = 53  ( 53 )",
            "PROPS       : Centroid = 6  ( 6 )  Volume   = 6  ( 6 )  Area     = 6  ( 6 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN4 RED3  ( BLUE GREEN4 RED3 )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = LAYER_1 LAYER_2 LAYER_3  ( LAYER_1 LAYER_2 LAYER_3 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_t8,
    DeRow {
        source: "tests/de/step_1/T8",
        file: "trj2_as1-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 19 )  Summary  = 0  ( 19 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = BLUE DARKORANGE GREEN RED YELLOW3  ( BLUE DARKORANGE GREEN RED YELLOW3 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_t9,
    DeRow {
        source: "tests/de/step_1/T9",
        file: "trj2_as1-ug-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 14 )  Summary  = 0  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  2.879942693e-006 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN RED  ( BLUE GREEN RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_u1,
    DeRow {
        source: "tests/de/step_1/U1",
        file: "trj2_io1-ca-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 99  ( 99 )  N2Labels = 0  ( 0 )   TotalLabels = 100  ( 100 )   NameLabels = 1  ( 1 )   ColorLabels = 99  ( 99 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_u2,
    DeRow {
        source: "tests/de/step_1/U2",
        file: "trj3_as1-ec-203.stp",
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
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_u3,
    DeRow {
        source: "tests/de/step_1/U3",
        file: "trj3_as1-ec-214.stp",
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
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_u4,
    DeRow {
        source: "tests/de/step_1/U4",
        file: "trj3_as1-pe-203.stp",
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
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 16  ( 16 )  N2Labels = 0  ( 0 )   TotalLabels = 25  ( 25 )   NameLabels = 22  ( 22 )   ColorLabels = 8  ( 8 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN4 RED3  ( BLUE GREEN4 RED3 )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_u5,
    DeRow {
        source: "tests/de/step_1/U5",
        file: "trj3_as1-tc-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 19 )  Summary  = 0  ( 19 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_u6,
    DeRow {
        source: "tests/de/step_1/U6",
        file: "trj3_as1-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 19 )  Summary  = 0  ( 19 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = BLUE DARKORANGE GREEN RED YELLOW3  ( BLUE DARKORANGE GREEN RED YELLOW3 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_u7,
    DeRow {
        source: "tests/de/step_1/U7",
        file: "trj3_s1-ac-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 21 )  Warnings = 0  ( 0 )  Summary  = 0  ( 21 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 4  ( 4 )  Summary  = 4  ( 4 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 100  ( 100 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 138  ( 138 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.01954002732  (  0.01954002723 )  AvgTol   =  0.0005199499289  (  0.0005199499271 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 14  ( 14 )  N2Labels = 0  ( 0 )   TotalLabels = 27  ( 27 )   NameLabels = 27  ( 27 )   ColorLabels = 8  ( 8 )   LayerLabels = 8  ( 8 )",
            "PROPS       : Centroid = 13  ( 13 )  Volume   = 13  ( 13 )  Area     = 13  ( 13 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = YELLOW  ( YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_u8,
    DeRow {
        source: "tests/de/step_1/U8",
        file: "trj3_s1-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 18  ( 18 )  Summary  = 18  ( 18 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 132  ( 132 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 178  ( 178 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.06901335768  (  0.07236095994 )  AvgTol   =  0.0008239372231  (  0.0008694204078 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 21  ( 21 )  N2Labels = 0  ( 0 )   TotalLabels = 34  ( 34 )   NameLabels = 27  ( 27 )   ColorLabels = 15  ( 15 )   LayerLabels = 8  ( 8 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_u9,
    DeRow {
        source: "tests/de/step_1/U9",
        file: "trj3_s1-ct-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 13 )  Summary  = 0  ( 13 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 18  ( 18 )  Summary  = 18  ( 18 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 132  ( 132 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 178  ( 178 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.06901335768  (  0.07236095994 )  AvgTol   =  0.0008239372231  (  0.0008694204078 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 21  ( 21 )  N2Labels = 0  ( 0 )   TotalLabels = 34  ( 34 )   NameLabels = 27  ( 27 )   ColorLabels = 15  ( 15 )   LayerLabels = 8  ( 8 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_v1,
    DeRow {
        source: "tests/de/step_1/V1",
        file: "trj3_s1-ec-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 3  ( 2 )  Summary  = 3  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 90  ( 90 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 121  ( 121 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 2.374473352e-006  ( 2.374473352e-006 )  AvgTol   =  1.581134559e-007  (  1.614672336e-007 )",
            "LABELS      : N0Labels = 14  ( 14 )  N1Labels = 19  ( 19 )  N2Labels = 0  ( 0 )   TotalLabels = 33  ( 33 )   NameLabels = 30  ( 30 )   ColorLabels = 11  ( 11 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_v2,
    DeRow {
        source: "tests/de/step_1/V2",
        file: "trj3_s1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 3  ( 2 )  Summary  = 3  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 90  ( 90 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 121  ( 121 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 2.374473352e-006  ( 2.374473352e-006 )  AvgTol   =  1.581134559e-007  (  1.614672336e-007 )",
            "LABELS      : N0Labels = 14  ( 14 )  N1Labels = 19  ( 19 )  N2Labels = 0  ( 0 )   TotalLabels = 33  ( 33 )   NameLabels = 30  ( 30 )   ColorLabels = 11  ( 11 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_v3,
    DeRow {
        source: "tests/de/step_1/V3",
        file: "trj3_s1-md-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 8  ( 39 )  Summary  = 8  ( 39 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 9  ( 9 )  Shell    = 9  ( 9 )  Face     = 116  ( 116 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 130  ( 130 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0009770882201  (  0.08941549678 )  AvgTol   =  2.388262758e-005  (  0.0007912796777 )",
            "LABELS      : N0Labels = 14  ( 14 )  N1Labels = 14  ( 14 )  N2Labels = 0  ( 0 )   TotalLabels = 28  ( 28 )   NameLabels = 28  ( 28 )   ColorLabels = 9  ( 9 )   LayerLabels = 9  ( 9 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 7  ( 7 )",
            "COLORS      : Colors   = CYAN DARKORANGE1 DARKORANGE3 GREEN MAGENTA RED YELLOW  ( CYAN DARKORANGE1 DARKORANGE3 GREEN MAGENTA RED YELLOW )",
            "NLAYERS     : NLayers  = 4  ( 4 )",
            "LAYERS      : Layers   = 1 110 28263 5  ( 1 110 28263 5 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_v4,
    DeRow {
        source: "tests/de/step_1/V4",
        file: "trj3_s1-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 2  ( 0 )  Summary  = 2  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 146  ( 146 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 188  ( 188 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0007596213789  ( 0.0007596213789 )  AvgTol   =  1.028294947e-005  (  1.028294947e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 17  ( 17 )  N2Labels = 0  ( 0 )   TotalLabels = 30  ( 30 )   NameLabels = 27  ( 27 )   ColorLabels = 11  ( 11 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 13  ( 13 )  Volume   = 13  ( 13 )  Area     = 13  ( 13 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_v5,
    DeRow {
        source: "tests/de/step_1/V5",
        file: "trj3_s1-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 7 )  Warnings = 0  ( 0 )  Summary  = 0  ( 7 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 16  ( 21 )  Summary  = 16  ( 21 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 145  ( 145 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 189  ( 189 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 6.233869107e-005  ( 6.542585968e-005 )  AvgTol   =  1.170283613e-006  (  1.215811622e-006 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 19  ( 19 )  N2Labels = 0  ( 0 )   TotalLabels = 32  ( 32 )   NameLabels = 27  ( 27 )   ColorLabels = 13  ( 13 )   LayerLabels = 8  ( 8 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 8  ( 8 )",
            "LAYERS      : Layers   = 000 010 020 030 040 050 060 070  ( 000 010 020 030 040 050 060 070 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_v6,
    DeRow {
        source: "tests/de/step_1/V6",
        file: "trj3_s1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 6  ( 35 )  Summary  = 6  ( 35 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 105  ( 105 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 138  ( 138 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.002131642002  ( 0.002131642024 )  AvgTol   =  3.775281321e-005  (  4.00755825e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 15  ( 15 )  N2Labels = 0  ( 0 )   TotalLabels = 28  ( 28 )   NameLabels = 27  ( 27 )   ColorLabels = 9  ( 9 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_v7,
    DeRow {
        source: "tests/de/step_1/V7",
        file: "trj4_k1_top-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 1 )  Summary  = 1  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 58  ( 58 )  Face     = 58  ( 58 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 58  ( 58 )  Face     = 58  ( 58 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0008167207419  ( 0.0009854215582 )  AvgTol   =  5.46365282e-005  (  0.0001174776299 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_v8,
    DeRow {
        source: "tests/de/step_1/V8",
        file: "trj4_k1_top-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 15  ( 16 )  Summary  = 15  ( 16 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 46  ( 46 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 46  ( 46 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.01475021965  (  0.01475021965 )  AvgTol   =  0.0004055200631  (  0.0004055200648 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 59  ( 59 )  N2Labels = 0  ( 0 )   TotalLabels = 60  ( 60 )   NameLabels = 1  ( 1 )   ColorLabels = 60  ( 60 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = DEEPSKYBLUE2 MAGENTA ORANGE ORANGERED RED  ( DEEPSKYBLUE2 MAGENTA ORANGE ORANGERED RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_v9,
    DeRow {
        source: "tests/de/step_1/V9",
        file: "trj4_k1_top-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.005901949404  ( 0.008046534679 )  AvgTol   =  0.0002996449651  (  0.0008772059548 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_w1,
    DeRow {
        source: "tests/de/step_1/W1",
        file: "trj4_k1_top-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.005901949404  (  0.00804653468 )  AvgTol   =  0.0002996449654  (  0.001190168005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_w2,
    DeRow {
        source: "tests/de/step_1/W2",
        file: "trj4_k1_top-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 4  ( 5 )  Summary  = 4  ( 5 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 59  ( 59 )  Face     = 59  ( 59 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 59  ( 59 )  Face     = 59  ( 59 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.003826736784  ( 0.003826736784 )  AvgTol   =  0.0001445083741  (  0.0001445083741 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 59  ( 59 )  N2Labels = 0  ( 0 )   TotalLabels = 60  ( 60 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 59  ( 59 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_w3,
    DeRow {
        source: "tests/de/step_1/W3",
        file: "trj4_k1_top-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 10 )  Summary  = 1  ( 10 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 44  ( 44 )  Face     = 44  ( 44 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 44  ( 44 )  Face     = 44  ( 44 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0004382667275  ( 0.007688098235 )  AvgTol   =  1.734137364e-005  (  0.0002416764036 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 44  ( 44 )  N2Labels = 0  ( 0 )   TotalLabels = 45  ( 45 )   NameLabels = 1  ( 1 )   ColorLabels = 44  ( 44 )   LayerLabels = 44  ( 44 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 001  ( 001 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_w5,
    DeRow {
        source: "tests/de/step_1/W5",
        file: "trj4_s1-ac-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 21 )  Warnings = 0  ( 0 )  Summary  = 0  ( 21 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 4  ( 4 )  Summary  = 4  ( 4 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 100  ( 100 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 138  ( 138 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.001628335609  ( 0.001628335603 )  AvgTol   =  4.341834315e-005  (  4.341834307e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 14  ( 14 )  N2Labels = 0  ( 0 )   TotalLabels = 27  ( 27 )   NameLabels = 27  ( 27 )   ColorLabels = 8  ( 8 )   LayerLabels = 8  ( 8 )",
            "PROPS       : Centroid = 13  ( 13 )  Volume   = 13  ( 13 )  Area     = 13  ( 13 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = YELLOW  ( YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_w6,
    DeRow {
        source: "tests/de/step_1/W6",
        file: "trj4_s1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 0 )  Summary  = 1  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 90  ( 90 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 121  ( 121 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 2.374473352e-006  ( 5.010077728e-006 )  AvgTol   =  1.602903834e-007  (  2.267835824e-007 )",
            "LABELS      : N0Labels = 14  ( 14 )  N1Labels = 18  ( 18 )  N2Labels = 0  ( 0 )   TotalLabels = 32  ( 32 )   NameLabels = 30  ( 30 )   ColorLabels = 9  ( 9 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 14  ( 14 )  Volume   = 14  ( 14 )  Area     = 14  ( 14 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_w7,
    DeRow {
        source: "tests/de/step_1/W7",
        file: "trj4_s1-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 2  ( 0 )  Summary  = 2  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 146  ( 146 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 188  ( 188 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0007596213789  ( 0.0007596213789 )  AvgTol   =  1.023887043e-005  (  1.028294947e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 16  ( 16 )  N2Labels = 0  ( 0 )   TotalLabels = 29  ( 29 )   NameLabels = 27  ( 27 )   ColorLabels = 10  ( 10 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 13  ( 13 )  Volume   = 13  ( 13 )  Area     = 13  ( 13 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_w8,
    DeRow {
        source: "tests/de/step_1/W8",
        file: "trj4_s1-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 11  ( 21 )  Summary  = 11  ( 21 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 137  ( 137 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 180  ( 180 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0004751822636  ( 0.0004814441399 )  AvgTol   =  9.226809631e-006  (  1.529223584e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 14  ( 14 )  N2Labels = 0  ( 0 )   TotalLabels = 27  ( 27 )   NameLabels = 27  ( 27 )   ColorLabels = 8  ( 8 )   LayerLabels = 8  ( 8 )",
            "PROPS       : Centroid = 13  ( 13 )  Volume   = 13  ( 13 )  Area     = 13  ( 13 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 8  ( 8 )",
            "LAYERS      : Layers   = 010 020 030 040 050 060 070 256  ( 010 020 030 040 050 060 070 256 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_w9,
    DeRow {
        source: "tests/de/step_1/W9",
        file: "trj4_s1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 6  ( 35 )  Summary  = 6  ( 35 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 105  ( 105 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 138  ( 138 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.002131642002  ( 0.002131642024 )  AvgTol   =  3.775281321e-005  (  4.00755825e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 15  ( 15 )  N2Labels = 0  ( 0 )   TotalLabels = 28  ( 28 )   NameLabels = 27  ( 27 )   ColorLabels = 9  ( 9 )   LayerLabels = 8  ( 8 )",
            "PROPS       : Centroid = 13  ( 13 )  Volume   = 13  ( 13 )  Area     = 13  ( 13 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 11  ( 11 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_x1,
    DeRow {
        source: "tests/de/step_1/X1",
        file: "trj5_k1-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 1 )  Summary  = 1  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 58  ( 58 )  Face     = 58  ( 58 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 58  ( 58 )  Face     = 58  ( 58 )   FreeWire = 0  ( 16 ) ",
            "TOLERANCE   : MaxTol   = 0.0008167207419  ( 0.0009854215582 )  AvgTol   =  5.200297373e-005  (  0.0001118094952 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 5  ( 5 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_x2,
    DeRow {
        source: "tests/de/step_1/X2",
        file: "trj5_k1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 1 )  Summary  = 1  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 47  ( 47 )  Face     = 47  ( 47 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 47  ( 47 )  Face     = 47  ( 47 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0001716884488  ( 0.0009724780716 )  AvgTol   =  5.509425424e-006  (  7.086779369e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_x3,
    DeRow {
        source: "tests/de/step_1/X3",
        file: "trj5_k1-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 15  ( 16 )  Summary  = 15  ( 16 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 46  ( 46 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 46  ( 46 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.01475021965  (  0.01475021965 )  AvgTol   =  0.0004055200631  (  0.0004055200648 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 59  ( 59 )  N2Labels = 0  ( 0 )   TotalLabels = 60  ( 60 )   NameLabels = 1  ( 1 )   ColorLabels = 60  ( 60 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = DEEPSKYBLUE2 MAGENTA ORANGE ORANGERED RED  ( DEEPSKYBLUE2 MAGENTA ORANGE ORANGERED RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_x4,
    DeRow {
        source: "tests/de/step_1/X4",
        file: "trj5_k1-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.003855375762  ( 0.005725131766 )  AvgTol   =  0.001273929561  (  0.001970794874 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_x5,
    DeRow {
        source: "tests/de/step_1/X5",
        file: "trj5_k1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.003855375762  ( 0.005725131766 )  AvgTol   =  0.001273929561  (  0.001970794874 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_x6,
    DeRow {
        source: "tests/de/step_1/X6",
        file: "trj5_k1-tc-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 4  ( 5 )  Summary  = 4  ( 5 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 59  ( 59 )  Face     = 59  ( 59 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 59  ( 59 )  Face     = 59  ( 59 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.003826736784  ( 0.003826736784 )  AvgTol   =  0.0001445083741  (  0.0001445083741 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 59  ( 59 )  N2Labels = 0  ( 0 )   TotalLabels = 60  ( 60 )   NameLabels = 1  ( 1 )   ColorLabels = 59  ( 59 )   LayerLabels = 59  ( 59 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = YELLOW2 YELLOW3 YELLOW4  ( YELLOW2 YELLOW3 YELLOW4 )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = 001 002 003  ( 001 002 003 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_x8,
    DeRow {
        source: "tests/de/step_1/X8",
        file: "trj5_s1-ac-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 21 )  Warnings = 0  ( 0 )  Summary  = 0  ( 21 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 4  ( 4 )  Summary  = 4  ( 4 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 100  ( 100 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 138  ( 138 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0006410770116  ( 0.000641077009 )  AvgTol   =  1.715282274e-005  (  1.715282269e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 14  ( 14 )  N2Labels = 0  ( 0 )   TotalLabels = 27  ( 27 )   NameLabels = 27  ( 27 )   ColorLabels = 8  ( 8 )   LayerLabels = 8  ( 8 )",
            "PROPS       : Centroid = 13  ( 13 )  Volume   = 13  ( 13 )  Area     = 13  ( 13 )",
            "NCOLORS     : NColors  = 8  ( 8 )",
            "COLORS      : Colors   = BLUE CYAN DARKGOLDENROD1 GREEN LIGHTPINK4 RED TURQUOISE4 YELLOW  ( BLUE CYAN DARKGOLDENROD1 GREEN LIGHTPINK4 RED TURQUOISE4 YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_x9,
    DeRow {
        source: "tests/de/step_1/X9",
        file: "trj5_s1-in-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 8  ( 9 )  Summary  = 8  ( 9 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 108  ( 108 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 146  ( 146 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0006380346848  ( 0.0006380346818 )  AvgTol   =  1.220920021e-005  (  1.220920017e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 14  ( 14 )  N2Labels = 0  ( 0 )   TotalLabels = 27  ( 27 )   NameLabels = 27  ( 27 )   ColorLabels = 8  ( 8 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 8  ( 8 )",
            "COLORS      : Colors   = BLUE3 CYAN DEEPSKYBLUE3 INDIANRED2 RED3 SANDYBROWN SPRINGGREEN3 YELLOW3  ( BLUE3 CYAN DEEPSKYBLUE3 INDIANRED2 RED3 SANDYBROWN SPRINGGREEN3 YELLOW3 )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_y1,
    DeRow {
        source: "tests/de/step_1/Y1",
        file: "trj5_s1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 6  ( 35 )  Summary  = 6  ( 35 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 105  ( 105 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 138  ( 138 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.002131642002  ( 0.002131642024 )  AvgTol   =  3.775281321e-005  (  4.00755825e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 15  ( 15 )  N2Labels = 0  ( 0 )   TotalLabels = 28  ( 28 )   NameLabels = 27  ( 27 )   ColorLabels = 9  ( 9 )   LayerLabels = 8  ( 8 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 11  ( 11 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_y2,
    DeRow {
        source: "tests/de/step_1/Y2",
        file: "trj6_as1-hc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 14 )  Summary  = 0  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  2.879942693e-006 )",
            "LABELS      : N0Labels = 15  ( 15 )  N1Labels = 66  ( 66 )  N2Labels = 0  ( 0 )   TotalLabels = 81  ( 81 )   NameLabels = 42  ( 42 )   ColorLabels = 39  ( 39 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 5  ( 5 )  Volume   = 5  ( 5 )  Area     = 5  ( 5 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN RED  ( BLUE GREEN RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_y3,
    DeRow {
        source: "tests/de/step_1/Y3",
        file: "trj6_k1-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 6  ( 10 )  Summary  = 6  ( 10 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 62  ( 62 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 62  ( 62 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0003235803061  ( 0.0003235803061 )  AvgTol   =  2.911253993e-005  (  4.080049452e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_y4,
    DeRow {
        source: "tests/de/step_1/Y4",
        file: "trj6_k1-ct-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 6  ( 10 )  Summary  = 6  ( 10 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 62  ( 62 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 62  ( 62 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0005893670486  ( 0.0005893670486 )  AvgTol   =  4.371317513e-005  (  5.96507567e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = YELLOW  ( YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_y5,
    DeRow {
        source: "tests/de/step_1/Y5",
        file: "trj6_k1-id-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 15  ( 16 )  Summary  = 15  ( 16 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 46  ( 46 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 46  ( 46 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.01475021965  (  0.01475021965 )  AvgTol   =  0.0004055200631  (  0.0004055200648 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_y6,
    DeRow {
        source: "tests/de/step_1/Y6",
        file: "trj6_k1-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.005901949404  ( 0.008046534679 )  AvgTol   =  0.0002996449651  (  0.0008772059548 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_y7,
    DeRow {
        source: "tests/de/step_1/Y7",
        file: "trj6_k1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.005901949404  ( 0.008046534679 )  AvgTol   =  0.0002996449651  (  0.0008772059548 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_y8,
    DeRow {
        source: "tests/de/step_1/Y8",
        file: "trj6_k1-sd-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 2  ( 2 )  Face     = 57  ( 57 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 2  ( 2 )  Face     = 57  ( 57 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 4.530986906e-007  ( 4.530984849e-007 )  AvgTol   =  1.197062502e-007  (  1.197062486e-007 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 3  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 7  ( 7 )   NameLabels = 7  ( 7 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = MEDIUMTURQUOISE  ( MEDIUMTURQUOISE )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_y9,
    DeRow {
        source: "tests/de/step_1/Y9",
        file: "trj6_k1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 9 )  Summary  = 0  ( 9 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 44  ( 44 )  Face     = 44  ( 44 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 44  ( 44 )  Face     = 44  ( 44 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.004260881238  ( 0.004260881238 )  AvgTol   =  0.0001729387311  (  0.0001739274816 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 44  ( 44 )  N2Labels = 0  ( 0 )   TotalLabels = 45  ( 45 )   NameLabels = 1  ( 1 )   ColorLabels = 44  ( 44 )   LayerLabels = 44  ( 44 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_z3,
    DeRow {
        source: "tests/de/step_1/Z3",
        file: "trj7_as1-hc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 14 )  Summary  = 0  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  2.879942693e-006 )",
            "LABELS      : N0Labels = 15  ( 15 )  N1Labels = 35  ( 35 )  N2Labels = 0  ( 0 )   TotalLabels = 50  ( 50 )   NameLabels = 42  ( 42 )   ColorLabels = 13  ( 13 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 6  ( 6 )  Volume   = 6  ( 6 )  Area     = 6  ( 6 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN RED  ( BLUE GREEN RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_z4,
    DeRow {
        source: "tests/de/step_1/Z4",
        file: "trj7_k1-ac-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 29  ( 39 )  Summary  = 29  ( 39 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 2  ( 2 )  Face     = 76  ( 76 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 2  ( 2 )  Face     = 76  ( 76 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.01009476798  (  0.01009476798 )  AvgTol   =  0.001193227277  (  0.001195883452 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 3  ( 3 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_z5,
    DeRow {
        source: "tests/de/step_1/Z5",
        file: "trj7_k1-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.000476839593  ( 0.000476839593 )  AvgTol   =  6.422491185e-005  (  6.422968681e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_z6,
    DeRow {
        source: "tests/de/step_1/Z6",
        file: "trj7_k1-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.005901949404  ( 0.008046534679 )  AvgTol   =  0.0002996449651  (  0.0008772059548 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_z7,
    DeRow {
        source: "tests/de/step_1/Z7",
        file: "trj7_k1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.005901949404  ( 0.008046534679 )  AvgTol   =  0.0002996449651  (  0.0008772059548 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_z8,
    DeRow {
        source: "tests/de/step_1/Z8",
        file: "trj7_k1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 9 )  Summary  = 0  ( 9 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 44  ( 44 )  Face     = 44  ( 44 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 44  ( 44 )  Face     = 44  ( 44 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.004260881238  ( 0.004260881238 )  AvgTol   =  0.0001729387311  (  0.0001739274816 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 44  ( 44 )  N2Labels = 0  ( 0 )   TotalLabels = 45  ( 45 )   NameLabels = 1  ( 1 )   ColorLabels = 44  ( 44 )   LayerLabels = 44  ( 44 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_z9,
    DeRow {
        source: "tests/de/step_1/Z9",
        file: "trj8_k1-ac-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 29  ( 39 )  Summary  = 29  ( 39 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 2  ( 2 )  Shell    = 2  ( 2 )  Face     = 76  ( 76 ) ",
            "STATSHAPE   : Solid    = 2  ( 2 )  Shell    = 2  ( 2 )  Face     = 76  ( 76 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.01009476798  (  0.01009476798 )  AvgTol   =  0.001193227277  (  0.001195883452 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 3  ( 3 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_za1,
    DeRow {
        source: "tests/de/step_1/ZA1",
        file: "trj8_k1-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.000476839593  ( 0.000476839593 )  AvgTol   =  6.422491185e-005  (  6.422968681e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_za2,
    DeRow {
        source: "tests/de/step_1/ZA2",
        file: "trj8_k1-ct-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.000476839593  ( 0.000476839593 )  AvgTol   =  6.422491185e-005  (  6.422968681e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = LIGHTSTEELBLUE1  ( LIGHTSTEELBLUE1 )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_za3,
    DeRow {
        source: "tests/de/step_1/ZA3",
        file: "trj8_k1-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 52  ( 52 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.005901949404  ( 0.008046534679 )  AvgTol   =  0.0002996449651  (  0.0008772059548 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_za4,
    DeRow {
        source: "tests/de/step_1/ZA4",
        file: "trj8_k1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 9 )  Summary  = 0  ( 9 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 44  ( 44 )  Face     = 44  ( 44 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 44  ( 44 )  Face     = 44  ( 44 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.004260881238  ( 0.004260881238 )  AvgTol   =  0.0001729387311  (  0.0001739274816 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 44  ( 44 )  N2Labels = 0  ( 0 )   TotalLabels = 45  ( 45 )   NameLabels = 1  ( 1 )   ColorLabels = 44  ( 44 )   LayerLabels = 44  ( 44 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_za5,
    DeRow {
        source: "tests/de/step_1/ZA5",
        file: "trj9_ap209-td.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 1 )  Summary  = 1  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.008505412228  ( 0.008990434575 )  AvgTol   =  0.0006167184304  (  0.0006516400657 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zb6,
    DeRow {
        source: "tests/de/step_1/ZB6",
        file: "trj9_io1-sy-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 22  ( 22 )  Summary  = 22  ( 22 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 26  ( 26 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 26  ( 26 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 4.201871225e-005  ( 4.297474004e-005 )  AvgTol   =  1.189150257e-005  (  1.226153684e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zd8,
    DeRow {
        source: "tests/de/step_1/ZD8",
        file: "tr8_ha1_ug.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 1 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 2  ( 2 )  Shell    = 4  ( 2 )  Face     = 14  ( 14 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 5  ( 5 )  N2Labels = 0  ( 0 )   TotalLabels = 9  ( 9 )   NameLabels = 8  ( 8 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = CYAN GREEN  ( CYAN GREEN )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zd9,
    DeRow {
        source: "tests/de/step_1/ZD9",
        file: "trj7_c2-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 6 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_ze1,
    DeRow {
        source: "tests/de/step_1/ZE1",
        file: "trj7_c2-al-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 2  ( 2 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = MIDNIGHTBLUE  ( MIDNIGHTBLUE )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_ze3,
    DeRow {
        source: "tests/de/step_1/ZE3",
        file: "trj7_c2-ct-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 6 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_ze4,
    DeRow {
        source: "tests/de/step_1/ZE4",
        file: "trj7_c2-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = DODGERBLUE2 GREEN  ( DODGERBLUE2 GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_ze5,
    DeRow {
        source: "tests/de/step_1/ZE5",
        file: "trj7_c2-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 6 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_ze6,
    DeRow {
        source: "tests/de/step_1/ZE6",
        file: "trj7_c2-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 1 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 3  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 6  ( 7 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE YELLOW  ( BLUE YELLOW )",
            "NLAYERS     : NLayers  = 2  ( 2 )",
            "LAYERS      : Layers   = 03___PRT_ALL_CURVES 06___PRT_ALL_SURFS  ( 03___PRT_ALL_CURVES 06___PRT_ALL_SURFS )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_ze7,
    DeRow {
        source: "tests/de/step_1/ZE7",
        file: "trj7_c2-tc-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 10 )  Warnings = 0  ( 0 )  Summary  = 0  ( 10 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 6 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE4 DARKORANGE  ( BLUE4 DARKORANGE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 005  ( 005 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_ze8,
    DeRow {
        source: "tests/de/step_1/ZE8",
        file: "trj7_c2-td-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 12 )  Warnings = 0  ( 0 )  Summary  = 0  ( 12 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 4  ( 0 )  Face     = 4  ( 4 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 4  ( 0 )  Face     = 4  ( 4 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 26  ( 26 )  N2Labels = 0  ( 0 )   TotalLabels = 29  ( 29 )   NameLabels = 5  ( 5 )   ColorLabels = 24  ( 24 )   LayerLabels = 4  ( 4 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 7  ( 7 )",
            "COLORS      : Colors   = CYAN GOLD3 GREEN GREEN4 MAGENTA MAGENTA3 YELLOW  ( CYAN GOLD3 GREEN GREEN4 MAGENTA MAGENTA3 YELLOW )",
            "NLAYERS     : NLayers  = 4  ( 4 )",
            "LAYERS      : Layers   = 001 002 003 004  ( 001 002 003 004 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_ze9,
    DeRow {
        source: "tests/de/step_1/ZE9",
        file: "trj7_c2-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 18 )  Warnings = 0  ( 0 )  Summary  = 0  ( 18 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 1 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 3  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 6  ( 7 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE YELLOW  ( BLUE YELLOW )",
            "NLAYERS     : NLayers  = 2  ( 2 )",
            "LAYERS      : Layers   = 03___PRT_ALL_CURVES 06___PRT_ALL_SURFS  ( 03___PRT_ALL_CURVES 06___PRT_ALL_SURFS )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zf1,
    DeRow {
        source: "tests/de/step_1/ZF1",
        file: "trj9_c2-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 7 )  Warnings = 0  ( 0 )  Summary  = 0  ( 7 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 4 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = DARKORANGE1 INDIANRED1  ( DARKORANGE1 INDIANRED1 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 001  ( 001 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zf4,
    DeRow {
        source: "tests/de/step_1/ZF4",
        file: "trj8_c2-ct-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 6 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zf5,
    DeRow {
        source: "tests/de/step_1/ZF5",
        file: "trj8_c2-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 1 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 3  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 6  ( 7 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE YELLOW  ( BLUE YELLOW )",
            "NLAYERS     : NLayers  = 2  ( 2 )",
            "LAYERS      : Layers   = 03___PRT_ALL_CURVES 06___PRT_ALL_SURFS  ( 03___PRT_ALL_CURVES 06___PRT_ALL_SURFS )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zf6,
    DeRow {
        source: "tests/de/step_1/ZF6",
        file: "trj2_io1-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: COLORS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 2 )  Warnings = 0  ( 0 )  Summary  = 0  ( 2 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 1  ( 1 )   ColorLabels = 3  ( 3 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 1  ( 2 )",
            "COLORS      : Colors   = YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zf8,
    DeRow {
        source: "tests/de/step_1/ZF8",
        file: "trj9_c2-ct-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 6 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zf9,
    DeRow {
        source: "tests/de/step_1/ZF9",
        file: "trj9_c2-sy-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 4 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = DODGERBLUE2 MAGENTA  ( DODGERBLUE2 MAGENTA )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zg3,
    DeRow {
        source: "tests/de/step_1/ZG3",
        file: "trj4_xr1-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: TPSTAT : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 28  ( 0 )  Summary  = 28  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 7.325240325e-006  ( 7.325239943e-006 )  AvgTol   =  2.485827911e-006  (  2.517563139e-006 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 26  ( 35 )  N2Labels = 0  ( 9 )   TotalLabels = 39  ( 57 )   NameLabels = 26  ( 31 )   ColorLabels = 5  ( 5 )   LayerLabels = 16  ( 16 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = CYAN GOLD3 RED3 TURQUOISE4 YELLOW  ( CYAN GOLD3 RED3 TURQUOISE4 YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zg4,
    DeRow {
        source: "tests/de/step_1/ZG4",
        file: "trj4_xr1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 5 )  Warnings = 0  ( 0 )  Summary  = 0  ( 5 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =              0  (              0 )  AvgTol   =               0  (               0 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 18 )  N2Labels = 0  ( 5 )   TotalLabels = 22  ( 32 )   NameLabels = 22  ( 27 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zg5,
    DeRow {
        source: "tests/de/step_1/ZG5",
        file: "trj4_xr1-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 5 )  Warnings = 0  ( 0 )  Summary  = 0  ( 5 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =              0  (              0 )  AvgTol   =               0  (               0 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 18 )  N2Labels = 0  ( 5 )   TotalLabels = 22  ( 32 )   NameLabels = 22  ( 27 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zg6,
    DeRow {
        source: "tests/de/step_1/ZG6",
        file: "trj4_xr1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 7 )  Warnings = 0  ( 0 )  Summary  = 0  ( 7 )",
            "TPSTAT      : Faulties = 0  ( 7 )  Warnings = 0  ( 0 )  Summary  = 0  ( 7 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =              0  (              0 )  AvgTol   =               0  (               0 )",
            "LABELS      : N0Labels = 10  ( 10 )  N1Labels = 13  ( 19 )  N2Labels = 0  ( 6 )   TotalLabels = 23  ( 35 )   NameLabels = 23  ( 29 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zg7,
    DeRow {
        source: "tests/de/step_1/ZG7",
        file: "trj4_xr1-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 26 )  Summary  = 0  ( 26 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 18 )  N2Labels = 0  ( 5 )   TotalLabels = 22  ( 32 )   NameLabels = 22  ( 27 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = BLUE DARKORANGE GREEN RED YELLOW3  ( BLUE DARKORANGE GREEN RED YELLOW3 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zg8,
    DeRow {
        source: "tests/de/step_1/ZG8",
        file: "trj4_xr1-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 26 )  Summary  = 0  ( 26 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 160  ( 160 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 18 )  N2Labels = 0  ( 5 )   TotalLabels = 22  ( 32 )   NameLabels = 22  ( 27 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = BLUE DARKORANGE GREEN RED YELLOW3  ( BLUE DARKORANGE GREEN RED YELLOW3 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 256  ( 256 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zh1,
    DeRow {
        source: "tests/de/step_1/ZH1",
        file: "bm2_pe_t4-B.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 7 )  Summary  = 0  ( 7 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 9  ( 0 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 9  ( 0 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.003863207397  (  0.03790164928 )  AvgTol   =  0.0004713629464  (  0.003090287092 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 5  ( 5 )  N2Labels = 0  ( 0 )   TotalLabels = 6  ( 6 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = ACHSEN FLAECHEN VERBFLAECHE  ( ACHSEN FLAECHEN VERBFLAECHE )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zh2,
    DeRow {
        source: "tests/de/step_1/ZH2",
        file: "trj6_c1-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 14  ( 15 )  N2Labels = 0  ( 0 )   TotalLabels = 18  ( 19 )   NameLabels = 7  ( 7 )   ColorLabels = 14  ( 14 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = BLUE GREEN MAGENTA RED YELLOW  ( BLUE GREEN MAGENTA RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zh3,
    DeRow {
        source: "tests/de/step_1/ZH3",
        file: "trj7_c2-ac-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 10 )  Warnings = 0  ( 0 )  Summary  = 0  ( 10 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 3  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 6  ( 7 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = GREEN RED  ( GREEN RED )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zh4,
    DeRow {
        source: "tests/de/step_1/ZH4",
        file: "trj8_c2-ac-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 10 )  Warnings = 0  ( 0 )  Summary  = 0  ( 10 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 3  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 6  ( 7 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = GREEN RED  ( GREEN RED )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zh5,
    DeRow {
        source: "tests/de/step_1/ZH5",
        file: "trj6_c1-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: LABELS : Faulty",
            "TODO CR23096 ALL: COLORS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 )   FreeWire = 0  ( 1 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 4  ( 5 )  N2Labels = 0  ( 0 )   TotalLabels = 8  ( 9 )   NameLabels = 7  ( 7 )   ColorLabels = 2  ( 3 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 3 )",
            "COLORS      : Colors   = BLUE WHITE  ( BLUE GREEN WHITE )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zh6,
    DeRow {
        source: "tests/de/step_1/ZH6",
        file: "trj6_c1-ct-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: LABELS : Faulty",
            "TODO CR23096 ALL: COLORS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 )   FreeWire = 0  ( 1 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 5  ( 6 )  N2Labels = 0  ( 0 )   TotalLabels = 9  ( 10 )   NameLabels = 7  ( 7 )   ColorLabels = 3  ( 4 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 4 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE GREEN RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zh7,
    DeRow {
        source: "tests/de/step_1/ZH7",
        file: "tr8_ha1_db.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: LABELS : Faulty",
            "TODO CR23096 ALL: LAYERS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 ) ",
            "STATSHAPE   : Solid    = 2  ( 2 )  Shell    = 2  ( 2 )  Face     = 12  ( 12 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 4  ( 5 )   NameLabels = 4  ( 4 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 3 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 2 )",
            "LAYERS      : Layers   = 0  ( {  0} 255 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zh8,
    DeRow {
        source: "tests/de/step_1/ZH8",
        file: "trj9_c2-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zh9,
    DeRow {
        source: "tests/de/step_1/ZH9",
        file: "trj9_c2-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty",
            "TODO CR23096 ALL: PROPS : Faulty",
            "TODO CR23096 ALL: LAYERS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 4 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE YELLOW  ( BLUE YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 1 )",
            "LAYERS      : Layers   =   ( 06___PRT_ALL_SURFS )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zi1,
    DeRow {
        source: "tests/de/step_1/ZI1",
        file: "trj9_c2-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty",
            "TODO CR23096 ALL: PROPS : Faulty",
            "TODO CR23096 ALL: LAYERS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 0 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 4 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE YELLOW  ( BLUE YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 1 )",
            "LAYERS      : Layers   =   ( 06___PRT_ALL_SURFS )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_1_zi4,
    DeRow {
        source: "tests/de/step_1/ZI4",
        file: "PRO10363.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 15  ( 15 ) ",
            "STATSHAPE   : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 15  ( 15 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 4.774870401e-005  ( 4.883502558e-005 )  AvgTol   =  7.657990925e-006  (  7.830422835e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 6  ( 6 )  N2Labels = 0  ( 0 )   TotalLabels = 7  ( 7 )   NameLabels = 1  ( 1 )   ColorLabels = 6  ( 6 )   LayerLabels = 4  ( 4 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = BLUE GREEN RED YELLOW  ( BLUE GREEN RED YELLOW )",
            "NLAYERS     : NLayers  = 2  ( 2 )",
            "LAYERS      : Layers   = 1 15  ( 1 15 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zi6,
    DeRow {
        source: "tests/de/step_1/ZI6",
        file: "PRO7071.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 1 )  Summary  = 1  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 15  ( 15 ) ",
            "STATSHAPE   : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 15  ( 15 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 1.259252781e-007  ( 1.298605592e-007 )  AvgTol   =  1.021947325e-007  (  1.02527878e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 4  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 1  ( 1 )   ColorLabels = 4  ( 4 )   LayerLabels = 4  ( 4 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = DODGERBLUE2 GREEN RED YELLOW  ( DODGERBLUE2 GREEN RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zl1,
    DeRow {
        source: "tests/de/step_1/ZL1",
        file: "bm2_pe_t4-A.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 7  ( 7 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 7  ( 7 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.003863207397  (  0.04018695152 )  AvgTol   =  0.0003664438252  (  0.001787191105 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 5  ( 5 )  N2Labels = 0  ( 0 )   TotalLabels = 6  ( 6 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = ACHSEN FLAECHEN VERBFLAECHE  ( ACHSEN FLAECHEN VERBFLAECHE )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zm1,
    DeRow {
        source: "tests/de/step_1/ZM1",
        file: "pr1ct.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 3 )  Summary  = 0  ( 3 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 15  ( 15 ) ",
            "STATSHAPE   : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 15  ( 15 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 4  ( 4 )   NameLabels = 3  ( 3 )   ColorLabels = 1  ( 1 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zm2,
    DeRow {
        source: "tests/de/step_1/ZM2",
        file: "pr1db.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 8  ( 8 )  Summary  = 8  ( 8 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 15  ( 15 ) ",
            "STATSHAPE   : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 15  ( 15 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 4.77497716e-005  ( 4.88343781e-005 )  AvgTol   =  1.125867746e-005  (  1.194868418e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 4  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 1  ( 1 )   ColorLabels = 4  ( 4 )   LayerLabels = 4  ( 4 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = DODGERBLUE2 GREEN RED YELLOW  ( DODGERBLUE2 GREEN RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zm5,
    DeRow {
        source: "tests/de/step_1/ZM5",
        file: "pr1sy.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 11  ( 11 ) ",
            "STATSHAPE   : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 11  ( 11 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 2.345200447e-007  (         1e-005 )  AvgTol   =  1.111528995e-007  (  1.16322096e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 4  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 1  ( 1 )   ColorLabels = 4  ( 4 )   LayerLabels = 4  ( 4 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = GOLD GREEN RED ROYALBLUE  ( GOLD GREEN RED ROYALBLUE )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = 0 254 X  ( 0 254 X )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zm6,
    DeRow {
        source: "tests/de/step_1/ZM6",
        file: "pr2ct.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE GREEN  ( BLUE GREEN )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zn1,
    DeRow {
        source: "tests/de/step_1/ZN1",
        file: "pr_3-sy.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 11  ( 11 ) ",
            "STATSHAPE   : Solid    = 4  ( 4 )  Shell    = 4  ( 4 )  Face     = 11  ( 11 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (     1.2088e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 4  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 1  ( 1 )   ColorLabels = 4  ( 4 )   LayerLabels = 4  ( 4 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = GOLD GREEN RED ROYALBLUE  ( GOLD GREEN RED ROYALBLUE )",
            "NLAYERS     : NLayers  = 2  ( 2 )",
            "LAYERS      : Layers   = 1 A  ( 1 A )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zn2,
    DeRow {
        source: "tests/de/step_1/ZN2",
        file: "trj4_k1_top-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 8 )  Summary  = 0  ( 8 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 44  ( 44 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 44  ( 44 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.004260881238  ( 0.004260881238 )  AvgTol   =  0.0002558536624  (  0.0002570745577 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = CYAN  ( CYAN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zn3,
    DeRow {
        source: "tests/de/step_1/ZN3",
        file: "screw.step",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 10  ( 10 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 10  ( 10 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0003002881797  ( 0.0003002696046 )  AvgTol   =  7.012259046e-005  (  7.009554128e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zo3,
    DeRow {
        source: "tests/de/step_1/ZO3",
        file: "tr8_bv1_ct.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zo4,
    DeRow {
        source: "tests/de/step_1/ZO4",
        file: "tr8_bv1_db.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zo5,
    DeRow {
        source: "tests/de/step_1/ZO5",
        file: "tr8_bv1_ec.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zo7,
    DeRow {
        source: "tests/de/step_1/ZO7",
        file: "tr8_bv1a_ec.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 17  ( 17 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 17  ( 17 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zo8,
    DeRow {
        source: "tests/de/step_1/ZO8",
        file: "tr8_ha1_ct.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 2  ( 2 )  Shell    = 4  ( 4 )  Face     = 14  ( 14 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 5  ( 5 )  N2Labels = 0  ( 0 )   TotalLabels = 9  ( 9 )   NameLabels = 8  ( 8 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zp1,
    DeRow {
        source: "tests/de/step_1/ZP1",
        file: "tr8_ha1_tc.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 2  ( 2 )  Shell    = 4  ( 4 )  Face     = 14  ( 14 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 4  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 6  ( 6 )   NameLabels = 4  ( 4 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = DEEPSKYBLUE1 RED  ( DEEPSKYBLUE1 RED )",
            "NLAYERS     : NLayers  = 2  ( 2 )",
            "LAYERS      : Layers   = 001 002  ( 001 002 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zp3,
    DeRow {
        source: "tests/de/step_1/ZP3",
        file: "tr9_asd-db.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 51 )  Warnings = 0  ( 0 )  Summary  = 0  ( 51 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 12  ( 12 )  N2Labels = 0  ( 0 )   TotalLabels = 13  ( 13 )   NameLabels = 1  ( 1 )   ColorLabels = 12  ( 12 )   LayerLabels = 12  ( 12 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zp4,
    DeRow {
        source: "tests/de/step_1/ZP4",
        file: "tr9_bv1-ec.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zp7,
    DeRow {
        source: "tests/de/step_1/ZP7",
        file: "tr9_bv1-tc.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 005  ( 005 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zp8,
    DeRow {
        source: "tests/de/step_1/ZP8",
        file: "tr9_exd-db.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 18 )  Warnings = 0  ( 0 )  Summary  = 0  ( 18 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 12  ( 12 )  N2Labels = 0  ( 0 )   TotalLabels = 13  ( 13 )   NameLabels = 1  ( 1 )   ColorLabels = 12  ( 12 )   LayerLabels = 12  ( 12 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zp9,
    DeRow {
        source: "tests/de/step_1/ZP9",
        file: "trj10_ch1-id-chs.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 4 )  Warnings = 0  ( 0 )  Summary  = 0  ( 4 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 3  ( 3 )  Summary  = 3  ( 3 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 12  ( 12 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 12  ( 12 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zq1,
    DeRow {
        source: "tests/de/step_1/ZQ1",
        file: "trj10_ch1-tu-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 4 )  Warnings = 0  ( 0 )  Summary  = 0  ( 4 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zq2,
    DeRow {
        source: "tests/de/step_1/ZQ2",
        file: "trj12_gd1-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 1 )  Warnings = 0  ( 2 )  Summary  = 0  ( 3 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 1 )  Summary  = 1  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 7  ( 7 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 6  ( 6 )  N2Labels = 0  ( 0 )   TotalLabels = 7  ( 7 )   NameLabels = 1  ( 1 )   ColorLabels = 4  ( 4 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = MAGENTA RED  ( MAGENTA RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zq3,
    DeRow {
        source: "tests/de/step_1/ZQ3",
        file: "trj12_is1-oc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 ) ",
            "STATSHAPE   : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 30  ( 30 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 5  ( 5 )  N2Labels = 0  ( 0 )   TotalLabels = 7  ( 7 )   NameLabels = 7  ( 7 )   ColorLabels = 3  ( 3 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zq4,
    DeRow {
        source: "tests/de/step_1/ZQ4",
        file: "trj2_as1-ac-214.stp",
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
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = BLUE CYAN GREEN RED YELLOW  ( BLUE CYAN GREEN RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zq5,
    DeRow {
        source: "tests/de/step_1/ZQ5",
        file: "trj2_as1-md-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 14 )  Summary  = 0  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  2.814605484e-006 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN RED  ( BLUE GREEN RED )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = 1 2 3  ( 1 2 3 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zq6,
    DeRow {
        source: "tests/de/step_1/ZQ6",
        file: "trj2_as1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 14 )  Summary  = 0  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  2.879942693e-006 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN RED  ( BLUE GREEN RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zq7,
    DeRow {
        source: "tests/de/step_1/ZQ7",
        file: "trj2_d2-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 172 )  Warnings = 0  ( 0 )  Summary  = 0  ( 172 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zq8,
    DeRow {
        source: "tests/de/step_1/ZQ8",
        file: "trj2_io1-ac-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 19 )  Warnings = 0  ( 0 )  Summary  = 0  ( 19 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 3  ( 3 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = YELLOW  ( YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zq9,
    DeRow {
        source: "tests/de/step_1/ZQ9",
        file: "trj2_io1-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0007775016544  ( 0.0008554238004 )  AvgTol   =  6.182444176e-005  (  6.860637478e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 26  ( 26 )  N2Labels = 0  ( 0 )   TotalLabels = 27  ( 27 )   NameLabels = 1  ( 1 )   ColorLabels = 27  ( 27 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zr1,
    DeRow {
        source: "tests/de/step_1/ZR1",
        file: "trj2_io1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 14  ( 14 )   NameLabels = 1  ( 1 )   ColorLabels = 14  ( 14 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zr2,
    DeRow {
        source: "tests/de/step_1/ZR2",
        file: "trj2_io1-id-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 12  ( 12 )  Summary  = 12  ( 12 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 16  ( 16 )  N2Labels = 0  ( 0 )   TotalLabels = 17  ( 17 )   NameLabels = 1  ( 1 )   ColorLabels = 17  ( 17 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zr3,
    DeRow {
        source: "tests/de/step_1/ZR3",
        file: "trj2_io1-in-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zr4,
    DeRow {
        source: "tests/de/step_1/ZR4",
        file: "trj2_io1-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 7 )  Warnings = 0  ( 0 )  Summary  = 0  ( 7 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 16 )  Summary  = 0  ( 16 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 32  ( 32 )  N2Labels = 0  ( 0 )   TotalLabels = 33  ( 33 )   NameLabels = 1  ( 1 )   ColorLabels = 33  ( 33 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 010  ( 010 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zr5,
    DeRow {
        source: "tests/de/step_1/ZR5",
        file: "trj2_io1-ug-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 12 )  Summary  = 0  ( 12 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  6.417509728e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 2  ( 2 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zr6,
    DeRow {
        source: "tests/de/step_1/ZR6",
        file: "trj2_io1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 12 )  Summary  = 0  ( 12 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  6.417509728e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 2  ( 2 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zr7,
    DeRow {
        source: "tests/de/step_1/ZR7",
        file: "trj3_as1-ac-214.stp",
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
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN RED  ( BLUE GREEN RED )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zr8,
    DeRow {
        source: "tests/de/step_1/ZR8",
        file: "trj3_as1-md-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 14 )  Summary  = 0  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  2.814605484e-006 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN RED  ( BLUE GREEN RED )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = 1 2 3  ( 1 2 3 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zr9,
    DeRow {
        source: "tests/de/step_1/ZR9",
        file: "trj3_as1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 14 )  Summary  = 0  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  2.879942693e-006 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN RED  ( BLUE GREEN RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zs1,
    DeRow {
        source: "tests/de/step_1/ZS1",
        file: "trj3_d1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 8 )  Warnings = 0  ( 0 )  Summary  = 0  ( 8 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zs2,
    DeRow {
        source: "tests/de/step_1/ZS2",
        file: "trj3_d1-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 5 )  Warnings = 0  ( 6 )  Summary  = 0  ( 11 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zs3,
    DeRow {
        source: "tests/de/step_1/ZS3",
        file: "trj3_t2-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 15 )  Warnings = 0  ( 10 )  Summary  = 0  ( 25 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 8  ( 8 )  Summary  = 8  ( 8 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 2  ( 2 )  Shell    = 2  ( 2 )  Face     = 16  ( 16 ) ",
            "STATSHAPE   : Solid    = 2  ( 2 )  Shell    = 2  ( 2 )  Face     = 16  ( 16 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 2.387426923e-005  ( 2.441746656e-005 )  AvgTol   =  6.169442064e-006  (  6.250668193e-006 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 5  ( 5 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zs4,
    DeRow {
        source: "tests/de/step_1/ZS4",
        file: "trj4_d2-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 21 )  Warnings = 0  ( 0 )  Summary  = 0  ( 21 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zs5,
    DeRow {
        source: "tests/de/step_1/ZS5",
        file: "trj4_d2-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 32 )  Warnings = 0  ( 6 )  Summary  = 0  ( 38 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zs6,
    DeRow {
        source: "tests/de/step_1/ZS6",
        file: "trj4_k1_top-md-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 11 )  Summary  = 1  ( 11 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 38  ( 38 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 38  ( 38 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.004142079219  ( 0.004142079219 )  AvgTol   =  0.0002795795242  (  0.0002808044906 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zs7,
    DeRow {
        source: "tests/de/step_1/ZS7",
        file: "trj5_d3-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 20 )  Warnings = 0  ( 0 )  Summary  = 0  ( 20 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zs8,
    DeRow {
        source: "tests/de/step_1/ZS8",
        file: "trj5_d3-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 34 )  Warnings = 0  ( 0 )  Summary  = 0  ( 34 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zs9,
    DeRow {
        source: "tests/de/step_1/ZS9",
        file: "trj5_k1-md-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 11 )  Summary  = 1  ( 11 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 38  ( 38 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 38  ( 38 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.004142079219  ( 0.004142079219 )  AvgTol   =  0.0002795795242  (  0.0002808044906 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zt1,
    DeRow {
        source: "tests/de/step_1/ZT1",
        file: "trj5_k1-sh-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 2  ( 14 )  Summary  = 2  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 44  ( 44 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 44  ( 44 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0002407439301  ( 0.0002407439302 )  AvgTol   =  5.378469181e-006  (  6.563018344e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zt2,
    DeRow {
        source: "tests/de/step_1/ZT2",
        file: "trj6_as1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 14 )  Summary  = 0  ( 14 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 5  ( 5 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 18  ( 18 )  Shell    = 18  ( 18 )  Face     = 125  ( 125 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  2.879942693e-006 )",
            "LABELS      : N0Labels = 9  ( 9 )  N1Labels = 13  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 22  ( 22 )   NameLabels = 22  ( 22 )   ColorLabels = 5  ( 5 )   LayerLabels = 5  ( 5 )",
            "PROPS       : Centroid = 9  ( 9 )  Volume   = 9  ( 9 )  Area     = 9  ( 9 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE GREEN RED  ( BLUE GREEN RED )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = 1 2 3  ( 1 2 3 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zt3,
    DeRow {
        source: "tests/de/step_1/ZT3",
        file: "trj6_c1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 5  ( 5 )  N2Labels = 0  ( 0 )   TotalLabels = 8  ( 8 )   NameLabels = 5  ( 5 )   ColorLabels = 4  ( 4 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = BLUE GREEN RED YELLOW  ( BLUE GREEN RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zt4,
    DeRow {
        source: "tests/de/step_1/ZT4",
        file: "trj6_c1-hc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 7  ( 7 )  N2Labels = 0  ( 0 )   TotalLabels = 9  ( 9 )   NameLabels = 3  ( 3 )   ColorLabels = 6  ( 6 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zt5,
    DeRow {
        source: "tests/de/step_1/ZT5",
        file: "trj6_c1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 19 )  Warnings = 0  ( 0 )  Summary  = 0  ( 19 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 4 )  Summary  = 0  ( 4 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 )   FreeWire = 0  ( 3 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  ( 1.001604854e-007 )  AvgTol   =          1e-007  (  1.000067289e-007 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 8  ( 8 )  N2Labels = 0  ( 0 )   TotalLabels = 12  ( 12 )   NameLabels = 7  ( 7 )   ColorLabels = 7  ( 7 )   LayerLabels = 3  ( 3 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 2  ( 2 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = DODGERBLUE2 RED WHITE YELLOW  ( DODGERBLUE2 RED WHITE YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 03___PRT_ALL_CURVES  ( 03___PRT_ALL_CURVES )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zt6,
    DeRow {
        source: "tests/de/step_1/ZT6",
        file: "trj6_c1-sd-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 18 )  Warnings = 0  ( 0 )  Summary  = 0  ( 18 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 5  ( 5 )  N1Labels = 4  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 9  ( 9 )   NameLabels = 7  ( 7 )   ColorLabels = 5  ( 5 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = BLUE MATRAGRAY MEDIUMTURQUOISE RED  ( BLUE MATRAGRAY MEDIUMTURQUOISE RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zt7,
    DeRow {
        source: "tests/de/step_1/ZT7",
        file: "trj6_c1-tc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 9  ( 9 )  N2Labels = 0  ( 0 )   TotalLabels = 10  ( 10 )   NameLabels = 1  ( 1 )   ColorLabels = 9  ( 9 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 5  ( 5 )",
            "COLORS      : Colors   = BLUE CYAN GREEN RED YELLOW  ( BLUE CYAN GREEN RED YELLOW )",
            "NLAYERS     : NLayers  = 2  ( 2 )",
            "LAYERS      : Layers   = 001 002  ( 001 002 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zt8,
    DeRow {
        source: "tests/de/step_1/ZT8",
        file: "trj6_c1-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 4  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 1  ( 1 )   ColorLabels = 4  ( 4 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = BLUE CYAN RED YELLOW  ( BLUE CYAN RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 010  ( 010 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zt9,
    DeRow {
        source: "tests/de/step_1/ZT9",
        file: "trj6_c1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 2  ( 2 )  Face     = 7  ( 7 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 4  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 1  ( 1 )   ColorLabels = 4  ( 4 )   LayerLabels = 3  ( 3 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 4  ( 4 )",
            "COLORS      : Colors   = BLUE RED SALMON1 YELLOW  ( BLUE RED SALMON1 YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zu1,
    DeRow {
        source: "tests/de/step_1/ZU1",
        file: "trj6_d3-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 23 )  Warnings = 0  ( 0 )  Summary  = 0  ( 23 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 0  ( 0 )",
            "COLORS      : Colors   =   (  )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zu2,
    DeRow {
        source: "tests/de/step_1/ZU2",
        file: "trj6_k1-md-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 11 )  Summary  = 1  ( 11 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 38  ( 38 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 38  ( 38 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.01052088122  (  0.01052088122 )  AvgTol   =  0.0007091310596  (  0.000710669262 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GRAY94  ( GRAY94 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 5  ( 5 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zu4,
    DeRow {
        source: "tests/de/step_1/ZU4",
        file: "trj7_c1-hc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 7  ( 7 )  N2Labels = 0  ( 0 )   TotalLabels = 9  ( 9 )   NameLabels = 3  ( 3 )   ColorLabels = 7  ( 7 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 2  ( 2 )  Volume   = 2  ( 2 )  Area     = 2  ( 2 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zu5,
    DeRow {
        source: "tests/de/step_1/ZU5",
        file: "trj7_c2-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 3  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 6  ( 6 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE RED  ( BLUE RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zu6,
    DeRow {
        source: "tests/de/step_1/ZU6",
        file: "trj7_c2-sd-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 18 )  Warnings = 0  ( 0 )  Summary  = 0  ( 18 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 3  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 7  ( 7 )   NameLabels = 6  ( 6 )   ColorLabels = 3  ( 3 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE CYAN MEDIUMTURQUOISE  ( BLUE CYAN MEDIUMTURQUOISE )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zu7,
    DeRow {
        source: "tests/de/step_1/ZU7",
        file: "trj7_c2-tu-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 8 )  Warnings = 0  ( 0 )  Summary  = 0  ( 8 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 2 )  Summary  = 0  ( 2 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 4  ( 4 )  N2Labels = 0  ( 0 )   TotalLabels = 7  ( 7 )   NameLabels = 5  ( 5 )   ColorLabels = 3  ( 3 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = DARKSLATEGRAY2 GREEN ROYALBLUE3  ( DARKSLATEGRAY2 GREEN ROYALBLUE3 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 001  ( 001 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zu8,
    DeRow {
        source: "tests/de/step_1/ZU8",
        file: "trj7_c2-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE SALMON1  ( BLUE SALMON1 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zu9,
    DeRow {
        source: "tests/de/step_1/ZU9",
        file: "trj8_c2-ct-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zv1,
    DeRow {
        source: "tests/de/step_1/ZV1",
        file: "trj8_c2-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.001285959159  (  0.00128595916 )  AvgTol   =  0.0007707941989  (  0.000770794199 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = DEEPPINK2 DODGERBLUE2  ( DEEPPINK2 DODGERBLUE2 )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zv2,
    DeRow {
        source: "tests/de/step_1/ZV2",
        file: "trj8_c2-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE RED  ( BLUE RED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zv3,
    DeRow {
        source: "tests/de/step_1/ZV3",
        file: "trj8_c2-hc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 2  ( 2 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 3  ( 3 )   ColorLabels = 1  ( 1 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = BLUE  ( BLUE )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zv4,
    DeRow {
        source: "tests/de/step_1/ZV4",
        file: "trj8_c2-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 7 )  Warnings = 0  ( 0 )  Summary  = 0  ( 7 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zv5,
    DeRow {
        source: "tests/de/step_1/ZV5",
        file: "trj8_c2-pe-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 2  ( 2 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = BLUE  ( BLUE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 06___PRT_ALL_SURFS  ( 06___PRT_ALL_SURFS )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zv6,
    DeRow {
        source: "tests/de/step_1/ZV6",
        file: "trj8_c2-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 7 )  Warnings = 0  ( 0 )  Summary  = 0  ( 7 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = DARKORANGE1 INDIANRED1  ( DARKORANGE1 INDIANRED1 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 001  ( 001 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zv7,
    DeRow {
        source: "tests/de/step_1/ZV7",
        file: "trj9_c2-oc-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 5  ( 5 )   NameLabels = 5  ( 5 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zv8,
    DeRow {
        source: "tests/de/step_1/ZV8",
        file: "trj9_c2-oc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE MAGENTA  ( BLUE MAGENTA )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zv9,
    DeRow {
        source: "tests/de/step_1/ZV9",
        file: "trj9_io1-id-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 12  ( 12 )  Summary  = 12  ( 12 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 16  ( 16 )  N2Labels = 0  ( 0 )   TotalLabels = 17  ( 17 )   NameLabels = 1  ( 1 )   ColorLabels = 17  ( 17 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zw1,
    DeRow {
        source: "tests/de/step_1/ZW1",
        file: "trj9_io1-oc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 7  ( 7 )  N2Labels = 0  ( 0 )   TotalLabels = 8  ( 8 )   NameLabels = 1  ( 1 )   ColorLabels = 8  ( 8 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = BLUE RED YELLOW  ( BLUE RED YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zw2,
    DeRow {
        source: "tests/de/step_1/ZW2",
        file: "trj9_io1-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 3 )  Warnings = 0  ( 0 )  Summary  = 0  ( 3 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 16 )  Summary  = 0  ( 16 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 29  ( 29 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 2  ( 2 )  N2Labels = 0  ( 0 )   TotalLabels = 3  ( 3 )   NameLabels = 1  ( 1 )   ColorLabels = 3  ( 3 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 010  ( 010 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zw3,
    DeRow {
        source: "tests/de/step_1/ZW3",
        file: "trj9_io1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 12 )  Summary  = 0  ( 12 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (  6.417509728e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 1  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 2  ( 2 )   NameLabels = 1  ( 1 )   ColorLabels = 2  ( 2 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zw6,
    DeRow {
        source: "tests/de/step_1/ZW6",
        file: "tr9_3dt-ct.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 2 )  Warnings = 0  ( 1 )  Summary  = 0  ( 3 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = MAGENTA  ( MAGENTA )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zw7,
    DeRow {
        source: "tests/de/step_1/ZW7",
        file: "tr9_3dt-tc.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 3 )  Warnings = 0  ( 0 )  Summary  = 0  ( 3 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = DARKVIOLET  ( DARKVIOLET )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 015  ( 015 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zw8,
    DeRow {
        source: "tests/de/step_1/ZW8",
        file: "tr9_3dt-td.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 3 )  Warnings = 0  ( 0 )  Summary  = 0  ( 3 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zw9,
    DeRow {
        source: "tests/de/step_1/ZW9",
        file: "tr9_bv1-ct.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zx1,
    DeRow {
        source: "tests/de/step_1/ZX1",
        file: "tr9_bv1-db.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zx2,
    DeRow {
        source: "tests/de/step_1/ZX2",
        file: "tr9_bv1-td.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 000  ( 000 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zx3,
    DeRow {
        source: "tests/de/step_1/ZX3",
        file: "brep_wv2.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 1 )  Warnings = 0  ( 0 )  Summary  = 0  ( 1 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zx4,
    DeRow {
        source: "tests/de/step_1/ZX4",
        file: "trj2_f1-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 25 )  Warnings = 0  ( 9 )  Summary  = 0  ( 34 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 6  ( 6 )  Summary  = 6  ( 6 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 12  ( 12 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 12  ( 12 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 2.387832344e-005  ( 2.442119623e-005 )  AvgTol   =  7.482707945e-006  (  7.582542794e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zx5,
    DeRow {
        source: "tests/de/step_1/ZX5",
        file: "trj8_k1-md-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 2  ( 10 )  Summary  = 2  ( 10 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 30  ( 30 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 30  ( 30 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0003868703974  ( 0.0006146351007 )  AvgTol   =  2.289212428e-005  (  3.880746786e-005 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 2  ( 2 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zx6,
    DeRow {
        source: "tests/de/step_1/ZX6",
        file: "trj2_io1-md-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 12 )  Summary  = 0  ( 12 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 17  ( 17 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-005 )  AvgTol   =          1e-007  (      6.148e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = YELLOW  ( YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zx7,
    DeRow {
        source: "tests/de/step_1/ZX7",
        file: "trj3_d1-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 44 )  Warnings = 0  ( 0 )  Summary  = 0  ( 44 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zx8,
    DeRow {
        source: "tests/de/step_1/ZX8",
        file: "trj4_d2-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 57 )  Warnings = 0  ( 0 )  Summary  = 0  ( 57 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zx9,
    DeRow {
        source: "tests/de/step_1/ZX9",
        file: "trj4_d2-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 22 )  Warnings = 0  ( 6 )  Summary  = 0  ( 28 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 001  ( 001 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zy1,
    DeRow {
        source: "tests/de/step_1/ZY1",
        file: "pr2db.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 6  ( 6 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = RED  ( RED )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zy2,
    DeRow {
        source: "tests/de/step_1/ZY2",
        file: "trj5_d3-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 58 )  Warnings = 0  ( 0 )  Summary  = 0  ( 58 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zy3,
    DeRow {
        source: "tests/de/step_1/ZY3",
        file: "trj5_d3-tu-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 26 )  Warnings = 0  ( 0 )  Summary  = 0  ( 26 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 001  ( 001 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zy4,
    DeRow {
        source: "tests/de/step_1/ZY4",
        file: "trj5_k1-ug-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 8 )  Summary  = 0  ( 8 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 44  ( 44 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 44  ( 44 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.004260881238  ( 0.004260881238 )  AvgTol   =  0.0002558536624  (  0.0002570745577 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = CYAN  ( CYAN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zy8,
    DeRow {
        source: "tests/de/step_1/ZY8",
        file: "trj6_d3-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 57 )  Warnings = 0  ( 0 )  Summary  = 0  ( 57 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 9  ( 9 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = WHITE  ( WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( {  0} )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zy9,
    DeRow {
        source: "tests/de/step_1/ZY9",
        file: "tr8_bv1_sy.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = RED  ( RED )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 2  ( 2 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zz1,
    DeRow {
        source: "tests/de/step_1/ZZ1",
        file: "tr8_bv1_tc.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 1  ( 1 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 005  ( 005 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_1_zz2,
    DeRow {
        source: "tests/de/step_1/ZZ2",
        file: "tr8_bv1_ug.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 3  ( 3 )  Face     = 18  ( 18 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-007  (         1e-007 )  AvgTol   =          1e-007  (          1e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
