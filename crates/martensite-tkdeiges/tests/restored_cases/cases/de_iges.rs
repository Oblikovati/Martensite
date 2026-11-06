//! `de/iges_*` with a public input.
use crate::table::{DeRow, RecordedVerdict};

de_case!(
    de_iges_1_b2,
    DeRow {
        source: "tests/de/iges_1/B2",
        file: "BUC50025.igs",
        todos: &[],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 7  ( 7 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 7  ( 7 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =          1e-07  (          1e-07 )  AvgTol   =           1e-07  (           1e-07 )",
            "LABELS      : N0Labels = 7  ( 7 )  N1Labels = 0  ( 0 )  N2Labels = 0  ( 0 )   TotalLabels = 7  ( 7 )   NameLabels = 7  ( 7 )   ColorLabels = 7  ( 7 )   LayerLabels = 7  ( 7 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = YELLOW  ( YELLOW )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            "",
        ],
        recorded: RecordedVerdict::Ok,
    }
);

de_case!(
    de_iges_1_p3,
    DeRow {
        source: "tests/de/iges_1/P3",
        file: "bad_header.igs",
        todos: &["TODO known ALL: LABELS : Faulty"],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =          1e-07  (          1e-05 )  AvgTol   =           1e-07  (  9.238461538e-06 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 1 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 2 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 2 )   LayerLabels = 1  ( 2 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = GREEN  ( GREEN )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 1  ( 1 )",
            "",
        ],
        recorded: RecordedVerdict::Known,
    }
);

de_case!(
    de_iges_2_c6,
    DeRow {
        source: "tests/de/iges_2/C6",
        file: "PRO19800.igs",
        todos: &[
            "TODO known ALL: LABELS : Faulty",
            "TODO known ALL: COLORS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 1  ( 1 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 1  ( 1 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =  0.04476896343  (  0.04476636368 )  AvgTol   =   0.02404075037  (   0.02403970427 )",
            "LABELS      : N0Labels = 1  ( 1 )  N1Labels = 0  ( 5 )  N2Labels = 0  ( 0 )   TotalLabels = 1  ( 6 )   NameLabels = 1  ( 1 )   ColorLabels = 1  ( 6 )   LayerLabels = 1  ( 6 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 2 )",
            "COLORS      : Colors   = WHITE  ( CYAN WHITE )",
            "NLAYERS     : NLayers  = 1  ( 1 )",
            "LAYERS      : Layers   = 31  ( 31 )",
            "",
        ],
        recorded: RecordedVerdict::Known,
    }
);

de_case!(
    de_iges_2_c8,
    DeRow {
        source: "tests/de/iges_2/C8",
        file: "Disc.igs",
        todos: &[
            "TODO known ALL: LABELS : Faulty",
            "TODO known ALL: LAYERS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 0 )  Summary  = 0  ( 0 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 0  ( 0 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =          1e-07  (          1e-07 )  AvgTol   =           1e-07  (           1e-07 )",
            "LABELS      : N0Labels = 1  ( 3 )  N1Labels = 46  ( 47 )  N2Labels = 0  ( 0 )   TotalLabels = 47  ( 50 )   NameLabels = 1  ( 50 )   ColorLabels = 46  ( 46 )   LayerLabels = 0  ( 47 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 2  ( 2 )",
            "COLORS      : Colors   = WHITE YELLOW  ( WHITE YELLOW )",
            "NLAYERS     : NLayers  = 0  ( 2 )",
            "LAYERS      : Layers   =   ( 1 2 )",
            "",
        ],
        recorded: RecordedVerdict::Known,
    }
);

de_case!(
    de_iges_2_d6,
    DeRow {
        source: "tests/de/iges_2/D6",
        file: "UKI60998.igs",
        todos: &[
            "TODO known ALL: TOLERANCE : Faulty",
            "TODO known ALL: LABELS : Faulty"
        ],
        reference: &[
            "",
            "DATA        : Faulties = 0  ( 0 )  Warnings = 0  ( 1 )  Summary  = 0  ( 1 )",
            "TPSTAT      : Faulties = 0  ( 0 )  Warnings = 0  ( 11 )  Summary  = 0  ( 11 )",
            "CHECKSHAPE  : Wires    = 0  ( 0 )  Faces    = 0  ( 0 )  Shells   = 0  ( 0 )   Solids   = 0 ( 0 )",
            "NBSHAPES    : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 11  ( 11 ) ",
            "STATSHAPE   : Solid    = 0  ( 0 )  Shell    = 0  ( 0 )  Face     = 11  ( 11 )   FreeWire = 0  ( 0 ) ",
            "TOLERANCE   : MaxTol   =          1e-05  ( 1.210776757e-07 )  AvgTol   =  1.545256205e-06  (  1.010720359e-07 )",
            "LABELS      : N0Labels = 11  ( 11 )  N1Labels = 4  ( 13 )  N2Labels = 0  ( 0 )   TotalLabels = 15  ( 24 )   NameLabels = 11  ( 24 )   ColorLabels = 15  ( 24 )   LayerLabels = 0  ( 0 )",
            "PROPS       : Centroid = 0  ( 0 )  Volume   = 0  ( 0 )  Area     = 0  ( 0 )",
            "NCOLORS     : NColors  = 1  ( 1 )",
            "COLORS      : Colors   = ORANGERED  ( ORANGERED )",
            "NLAYERS     : NLayers  = 0  ( 0 )",
            "LAYERS      : Layers   =   (  )",
            "",
        ],
        recorded: RecordedVerdict::Known,
    }
);
