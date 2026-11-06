use crate::table::*;

de_case!(
    de_step_2_c5,
    DeRow {
        source: "tests/de/step_2/C5",
        file: "trj4_k1_geo-ac-214.stp",
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
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 33  ( 35 )  Summary  = 33  ( 35 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 60  ( 0 )  Face     = 60  ( 60 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 60  ( 0 )  Face     = 60  ( 60 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0003019315208  ( 0.009909450702 )  AvgTol   =  1.037801807e-005  (  0.0001843244859 )",
            "LABELS      : N0Labels = 3  ( 3 )  N1Labels = 64  ( 64 )  N2Labels = 0  ( 0 )   TotalLabels = 67  ( 67 )   NameLabels = 5  ( 5 )   ColorLabels = 62  ( 62 )   LayerLabels = 62  ( 62 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = CYAN YELLOW4  ( CYAN YELLOW4 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 0  ( 0 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_2_c8,
    DeRow {
        source: "tests/de/step_2/C8",
        file: "trj4_s1-pe-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: TPSTAT : Faulty",
            "TODO CR23096 ALL: PROPS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 2  ( 0 )  Summary  = 2  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 8  ( 8 )  Shell    = 8  ( 8 )  Face     = 146  ( 146 ) ",
            "STATSHAPE   : Solid    = 11  ( 11 )  Shell    = 11  ( 11 )  Face     = 188  ( 188 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.0007596213789  ( 0.0007596213789 )  AvgTol   =  1.028294947e-005  (  1.028294947e-005 )",
            "LABELS      : N0Labels = 13  ( 13 )  N1Labels = 17  ( 17 )  N2Labels = 0  ( 0 )   TotalLabels = 30  ( 30 )   NameLabels = 27  ( 27 )   ColorLabels = 11  ( 11 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 13  ( 27 )  Volume   = 13  ( 13 )  Area     = 13  ( 13 )",
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
    de_step_2_e6,
    DeRow {
        source: "tests/de/step_2/E6",
        file: "trj3_s1-al-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL:  Update of 3D-Parameters has failed",
            "TODO CR23096 ALL: TPSTAT : Faulty",
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty",
            "TODO CR23096 ALL: LABELS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 101  ( 91 )  Summary  = 101  ( 91 )",
            "CHECKSHAPE  : Wires    = 12  ( 12 )  Faces    = 12  ( 12 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 5  ( 5 )  Shell    = 18  ( 14 )  Face     = 94  ( 94 ) ",
            "STATSHAPE   : Solid    = 5  ( 5 )  Shell    = 18  ( 14 )  Face     = 94  ( 94 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =    1.000111199  (    1.000111199 )  AvgTol   =   0.06532000174  (   0.07891516343 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 18  ( 17 )  N2Labels = 0  ( 0 )   TotalLabels = 19  ( 18 )   NameLabels = 1  ( 1 )   ColorLabels = 18  ( 17 )   LayerLabels = 7  ( 8 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = RED YELLOW  ( RED YELLOW )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = feet main_body tail_middle  ( feet main_body tail_middle )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_2_n2,
    DeRow {
        source: "tests/de/step_2/N2",
        file: "ec_soapbox-A.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 86  ( 86 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 86  ( 86 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.00140940647  (   0.0077001715 )  AvgTol   =  4.939486697e-005  (  0.0008378892132 )",
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
    de_step_2_n3,
    DeRow {
        source: "tests/de/step_2/N3",
        file: "ec_turbine-A.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 7 )  Warnings = 0  ( 0 )  Summary  = 0  ( 7 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 3  ( 1 )  Summary  = 3  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 53  ( 53 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 53  ( 53 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 3.83146359e-005  ( 4.239003075e-005 )  AvgTol   =  6.96329612e-006  (  7.809397867e-006 )",
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
    de_step_2_p1,
    DeRow {
        source: "tests/de/step_2/P1",
        file: "r103ec.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 28  ( 0 )  Summary  = 28  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 146  ( 146 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 146  ( 146 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 5.024535408e-007  ( 5.024535217e-007 )  AvgTol   =  1.93079097e-007  (  1.401218781e-007 )",
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
    de_step_2_q2,
    DeRow {
        source: "tests/de/step_2/Q2",
        file: "trj5_k1-al-214.stp",
        product_mode_off: false,
        length_unit: Some("in"),
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 24  ( 24 )  Summary  = 24  ( 24 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 39  ( 39 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 4.0017609e-005  ( 4.172680481e-005 )  AvgTol   =  5.844755983e-006  (  6.806025269e-006 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GRAY61  ( GRAY61 )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_2_r7,
    DeRow {
        source: "tests/de/step_2/R7",
        file: "trj10_b2-oc-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 5  ( 3 )  Summary  = 5  ( 3 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 251  ( 251 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 251  ( 251 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.002562014753  (  0.00500444492 )  AvgTol   =  2.347098051e-05  (  7.280140961e-05 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 0  ( 0 )   LayerLabels = 0  ( 0 )",
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
    de_step_2_s5,
    DeRow {
        source: "tests/de/step_2/S5",
        file: "trj4_k1_top-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 13  ( 0 )  Summary  = 13  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 47  ( 47 )  Face     = 47  ( 47 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 47  ( 47 )  Face     = 47  ( 47 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =         1e-005  ( 0.0001275978398 )  AvgTol   =  7.252436172e-007  (  5.085661783e-006 )",
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
    de_step_2_s9,
    DeRow {
        source: "tests/de/step_2/S9",
        file: "trj6_b1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 7  ( 13 )  Summary  = 7  ( 13 )",
            "CHECKSHAPE  : Wires    = 4  ( 4 )  Faces    = 4  ( 4 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 292  ( 292 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 292  ( 292 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =   0.1186503045  (    0.118648605 )  AvgTol   =  0.002710455224  (   0.01157963666 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 19  ( 19 )  N2Labels = 0  ( 0 )   TotalLabels = 20  ( 20 )   NameLabels = 1  ( 1 )   ColorLabels = 20  ( 20 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE4 MATRAGRAY  ( BLUE4 MATRAGRAY )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_2_t4,
    DeRow {
        source: "tests/de/step_2/T4",
        file: "trj6_k1-al-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 23  ( 23 )  Summary  = 23  ( 23 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 39  ( 39 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 39  ( 39 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.000908804869  ( 0.0009088048691 )  AvgTol   =  0.0001543943434  (  0.0001543943431 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 1 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 1 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GRAY61  ( GRAY61 )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_2_t5,
    DeRow {
        source: "tests/de/step_2/T5",
        file: "trj6_k1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 45  ( 45 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 45  ( 45 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 1.140736942e-007  ( 1.140736942e-007 )  AvgTol   =  1.003404926e-007  (  1.003404926e-007 )",
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
    de_step_2_t6,
    DeRow {
        source: "tests/de/step_2/T6",
        file: "trj6_k1-td-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &["TODO CR23096 ALL: TPSTAT : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 2  ( 1 )  Summary  = 2  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 59  ( 59 )  Face     = 59  ( 59 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 59  ( 59 )  Face     = 59  ( 59 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 3.463377008e-005  ( 3.463377008e-005 )  AvgTol   =  7.696812446e-007  (  7.746346804e-007 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 59  ( 59 )  N2Labels = 0  ( 0 )   TotalLabels = 60  ( 60 )   NameLabels = 1  ( 1 )   ColorLabels = 59  ( 59 )   LayerLabels = 59  ( 59 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 3  ( 3 )",
            "COLORS      : Colors   = YELLOW3 YELLOW3 YELLOW4  ( YELLOW3 YELLOW3 YELLOW4 )",
            "NLAYERS     : NLayers  = 3  ( 3 )",
            "LAYERS      : Layers   = 001 002 003  ( 001 002 003 )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_2_t9,
    DeRow {
        source: "tests/de/step_2/T9",
        file: "trj7_b1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[
            "TODO CR23096 ALL: NBSHAPES : Faulty",
            "TODO CR23096 ALL: STATSHAPE : Faulty",
            "TODO CR23096 ALL: TOLERANCE : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 2 )  Warnings = 4  ( 30 )  Summary  = 4  ( 32 )",
            "CHECKSHAPE  : Wires    = 1  ( 2 )  Faces    = 1  ( 2 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 416  ( 415 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 416  ( 415 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =    133200.3972  (   0.9492292985 )  AvgTol   =     320.1208627  (   0.04011190067 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 28  ( 28 )  N2Labels = 0  ( 0 )   TotalLabels = 29  ( 29 )   NameLabels = 1  ( 1 )   ColorLabels = 29  ( 29 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE4 MATRAGRAY  ( BLUE4 MATRAGRAY )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Known
    }
);
de_case!(
    de_step_2_u4,
    DeRow {
        source: "tests/de/step_2/U4",
        file: "trj7_k1-al-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 23  ( 23 )  Summary  = 23  ( 23 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 3  ( 3 )  Face     = 57  ( 57 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 3  ( 3 )  Face     = 57  ( 57 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.001141394308  ( 0.009811709864 )  AvgTol   =  0.0001692868406  (  0.000755108356 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 3  ( 3 )  N2Labels = 0  ( 0 )   TotalLabels = 4  ( 4 )   NameLabels = 1  ( 1 )   ColorLabels = 3  ( 3 )   LayerLabels = 2  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GRAY61  ( GRAY61 )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = L1  ( L1 )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_2_u5,
    DeRow {
        source: "tests/de/step_2/U5",
        file: "trj7_k1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 45  ( 45 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 45  ( 45 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 1.140736942e-007  ( 1.140736942e-007 )  AvgTol   =  1.003404926e-007  (  1.003404926e-007 )",
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
    de_step_2_u8,
    DeRow {
        source: "tests/de/step_2/U8",
        file: "trj8_b1-ec-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 1  ( 27 )  Summary  = 1  ( 27 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 415  ( 415 ) ",
            "STATSHAPE   : Solid    = 1  ( 1 )  Shell    = 1  ( 1 )  Face     = 415  ( 415 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.09895613597  (   0.9492292985 )  AvgTol   =   0.01328296919  (   0.04097063947 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 28  ( 28 )  N2Labels = 0  ( 0 )   TotalLabels = 29  ( 29 )   NameLabels = 1  ( 1 )   ColorLabels = 29  ( 29 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 1  ( 1 )  Volume   = 1  ( 1 )  Area     = 1  ( 1 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = BLUE4 MATRAGRAY  ( BLUE4 MATRAGRAY )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
de_case!(
    de_step_2_v2,
    DeRow {
        source: "tests/de/step_2/V2",
        file: "trj8_k1-db-214.stp",
        product_mode_off: false,
        length_unit: None,
        healing: &[],
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 15  ( 15 )  Summary  = 15  ( 15 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 1  ( 1 )  Face     = 58  ( 58 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   = 0.006251011624  ( 0.006251011625 )  AvgTol   =  0.001224689492  (  0.001224689492 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 58  ( 58 )  N2Labels = 0  ( 0 )   TotalLabels = 59  ( 59 )   NameLabels = 1  ( 1 )   ColorLabels = 59  ( 59 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = DEEPPINK2 DEEPPINK2  ( DEEPPINK2 DEEPPINK2 )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            ""
        ],
        recorded: RecordedVerdict::Ok
    }
);
