use crate::table::*;

de_case!(
    de_step_4_d5,
    DeRow {
        source: "tests/de/step_4/D5",
        file: "r0901_ec.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 15  ( 7 )  Summary  = 15  ( 7 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 438  ( 438 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 438  ( 438 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0001456659717  ( 0.0001456659717 )  AvgTol   =  8.859841366e-007  (  9.133548678e-007 )",
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
    de_step_4_e4,
    DeRow {
        source: "tests/de/step_4/E4",
        file: "tr9_r1001-ec.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 133  ( 5 )  Summary  = 133  ( 5 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 918  ( 918 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 918  ( 918 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =   0.0027785066  ( 0.005865384835 )  AvgTol   =  0.000107420207  (  0.0001402817936 )",
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
    de_step_4_e9,
    DeRow {
        source: "tests/de/step_4/E9",
        file: "trj4_pm2-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 4  ( 22 )  Summary  = 4  ( 22 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 13  ( 13 )  Shell    = 13  ( 13 )  Face     = 801  ( 801 ) ",
            "STATSHAPE   : Solid    = 32  ( 32 )  Shell    = 32  ( 32 )  Face     = 840  ( 840 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.000994818338  ( 0.001450971274 )  AvgTol   =  8.152449067e-006  (  2.901586213e-005 )",
            "LABELS      : N0Labels = 21  ( 21 )  N1Labels = 39  ( 39 )  N2Labels = 0  ( 0 )   TotalLabels = 60  ( 60 )   NameLabels = 60  ( 60 )   ColorLabels = 13  ( 13 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 21  ( 21 )  Volume   = 21  ( 21 )  Area     = 21  ( 21 )",
            "NCOLORS     : NColors  = 7  ( 7 )",
            "COLORS      : Colors   = GRAY20 GRAY80 GREEN GREENYELLOW PURPLE STEELBLUE YELLOW  ( GRAY20 GRAY80 GREEN GREENYELLOW PURPLE STEELBLUE YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
