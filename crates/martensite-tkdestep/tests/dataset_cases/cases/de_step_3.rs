use crate::table::*;

de_case!(
    de_step_3_d7,
    DeRow {
        source: "tests/de/step_3/D7",
        file: "trj6_pm4-ec-203.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: PROPS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 4  ( 22 )  Summary  = 4  ( 22 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 13  ( 13 )  Shell    = 13  ( 13 )  Face     = 801  ( 801 ) ",
            "STATSHAPE   : Solid    = 32  ( 32 )  Shell    = 32  ( 32 )  Face     = 840  ( 840 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0009956037508  ( 0.009298658856 )  AvgTol   =  1.276056437e-005  (  0.0002831958675 )",
            "LABELS      : N0Labels = 21  ( 21 )  N1Labels = 39  ( 39 )  N2Labels = 0  ( 0 )   TotalLabels = 60  ( 60 )   NameLabels = 60  ( 60 )   ColorLabels = 13  ( 13 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 21  ( 60 )  Volume   = 21  ( 21 )  Area     = 21  ( 21 )",
            "NCOLORS     : NColors  = 7  ( 7 )",
            "COLORS      : Colors   = GRAY20 GRAY8 GREEN GREENYELLOW PURPLE STEELBLUE YELLOW  ( GRAY20 GRAY8 GREEN GREENYELLOW PURPLE STEELBLUE YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_3_e5,
    DeRow {
        source: "tests/de/step_3/E5",
        file: "trj9_ap209-mp.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL:  Update of 3D-Parameters has failed",
            "TODO CR23096 ALL: TPSTAT : Faulty",
            "TODO CR23096 ALL: NBSHAPES : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 26  ( 7 )  Summary  = 26  ( 7 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 4 )  Shell    = 8  ( 4 )  Face     = 48  ( 24 ) ",
            "STATSHAPE   : Solid    = 24  ( 24 )  Shell    = 24  ( 24 )  Face     = 144  ( 144 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0009551590893  ( 0.0009551590893 )  AvgTol   =  0.0002351426518  (  0.0002351426518 )",
            "LABELS      : N0Labels = 4  ( 4 )  N1Labels = 6  ( 6 )  N2Labels = 0  ( 0 )   TotalLabels = 10  ( 10 )   NameLabels = 10  ( 10 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
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
