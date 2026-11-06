//! The static parameters `StepControlController` defines, in their order. The default of
//! `write.step.product.name` names Martensite, per the release's naming rule.
use martensite_tkernel::standard::Failure;
use martensite_tkxsbase::interface::InterfaceStatic;

/// One statement of the constructor.
enum Row {
    /// `InterfaceStatic::init(family, name, kind, init)`.
    Init(&'static str, &'static str, u8, &'static str),
    /// An enumeration: its declaration, the first value's number and each value's text.
    Enum(&'static str, &'static str, i32, &'static [&'static str]),
    /// `InterfaceStatic::set_c_val(name, value)`.
    Text(&'static str, &'static str),
    /// `InterfaceStatic::set_i_val(name, value)`.
    Int(&'static str, i32),
}

const ROWS: &[Row] = &[
    Row::Init(
        "step",
        "write.step.product.name",
        b't',
        "Martensite STEP translator",
    ),
    Row::Enum("step", "write.step.assembly", 0, &["Off", "On", "Auto"]),
    Row::Text("write.step.assembly", "Auto"),
    Row::Enum("step", "step.angleunit.mode", 0, &["File", "Rad", "Deg"]),
    Row::Text("step.angleunit.mode", "File"),
    Row::Enum(
        "step",
        "write.step.schema",
        1,
        &["AP214CD", "AP214DIS", "AP203", "AP214IS", "AP242DIS"],
    ),
    Row::Text("write.step.schema", "AP214IS"),
    Row::Enum(
        "step",
        "read.step.shape.repr",
        1,
        &["All", "ABSR", "MSSR", "GBSSR", "FBSR", "EBWSR", "GBWSR"],
    ),
    Row::Text("read.step.shape.repr", "All"),
    Row::Enum("step", "read.step.shape.relationship", 0, &["OFF", "ON"]),
    Row::Text("read.step.shape.relationship", "ON"),
    Row::Enum("step", "read.step.shape.aspect", 0, &["OFF", "ON"]),
    Row::Text("read.step.shape.aspect", "ON"),
    Row::Enum("step", "read.step.product.mode", 0, &["OFF", "ON"]),
    Row::Text("read.step.product.mode", "ON"),
    Row::Enum(
        "step",
        "read.step.product.context",
        1,
        &["all", "design", "analysis"],
    ),
    Row::Text("read.step.product.context", "all"),
    Row::Enum(
        "step",
        "read.step.assembly.level",
        1,
        &["all", "assembly", "structure", "shape"],
    ),
    Row::Text("read.step.assembly.level", "all"),
    Row::Enum(
        "step",
        "write.step.unit",
        1,
        &[
            "INCH", "MM", "??", "FT", "MI", "M", "KM", "MIL", "UM", "CM", "UIN",
        ],
    ),
    Row::Text("write.step.unit", "MM"),
    Row::Enum("step", "read.step.nonmanifold", 0, &["Off", "On"]),
    Row::Int("read.step.nonmanifold", 0),
    Row::Enum("step", "write.step.nonmanifold", 0, &["Off", "On"]),
    Row::Int("write.step.nonmanifold", 0),
    Row::Enum("step", "read.step.ideas", 0, &["Off", "On"]),
    Row::Int("read.step.ideas", 0),
    Row::Enum(
        "step",
        "write.step.vertex.mode",
        0,
        &["One Compound", "Single Vertex"],
    ),
    Row::Int("write.step.vertex.mode", 0),
    Row::Init("XSTEP", "write.step.resource.name", b't', "STEP"),
    Row::Init("XSTEP", "read.step.resource.name", b't', "STEP"),
    Row::Init("XSTEP", "write.step.sequence", b't', "ToSTEP"),
    Row::Init("XSTEP", "read.step.sequence", b't', "FromSTEP"),
    Row::Init(
        "XSTEP",
        "ToSTEP.exec.op",
        b't',
        "SplitCommonVertex,DirectFaces",
    ),
    Row::Init("XSTEP", "FromSTEP.exec.op", b't', "FixShape"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.Tolerance3d",
        b't',
        "&Runtime.Tolerance",
    ),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.MaxTolerance3d",
        b't',
        "&Runtime.MaxTolerance",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.MinTolerance3d", b't', "1.e-7"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixFreeShellMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixFreeFaceMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixFreeWireMode", b't', "-1"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixSameParameterMode",
        b't',
        "-1",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixSolidMode", b't', "-1"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixShellOrientationMode",
        b't',
        "-1",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.CreateOpenSolidMode", b't', "0"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixShellMode", b't', "-1"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixFaceOrientationMode",
        b't',
        "-1",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixFaceMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixWireMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixOrientationMode", b't', "-1"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixAddNaturalBoundMode",
        b't',
        "-1",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixMissingSeamMode", b't', "-1"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixSmallAreaWireMode",
        b't',
        "-1",
    ),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.RemoveSmallAreaFaceMode",
        b't',
        "-1",
    ),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixIntersectingWiresMode",
        b't',
        "-1",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixLoopWiresMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixSplitFaceMode", b't', "-1"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.AutoCorrectPrecisionMode",
        b't',
        "1",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.ModifyTopologyMode", b't', "0"),
    Row::Init("XSTEP", "FromSTEP.FixShape.ModifyGeometryMode", b't', "1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.ClosedWireMode", b't', "1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.PreferencePCurveMode", b't', "1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixReorderMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixSmallMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixConnectedMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixEdgeCurvesMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixDegeneratedMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixLackingMode", b't', "-1"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixSelfIntersectionMode",
        b't',
        "-1",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.RemoveLoopMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixReversed2dMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixRemovePCurveMode", b't', "-1"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixRemoveCurve3dMode",
        b't',
        "-1",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixAddPCurveMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixAddCurve3dMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixSeamMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixShiftedMode", b't', "-1"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixEdgeSameParameterMode",
        b't',
        "0",
    ),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixNotchedEdgesMode", b't', "-1"),
    Row::Init("XSTEP", "FromSTEP.FixShape.FixTailMode", b't', "0"),
    Row::Init("XSTEP", "FromSTEP.FixShape.MaxTailAngle", b't', "0.0"),
    Row::Init("XSTEP", "FromSTEP.FixShape.MaxTailWidth", b't', "-1.0"),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixSelfIntersectingEdgeMode",
        b't',
        "-1",
    ),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixIntersectingEdgesMode",
        b't',
        "-1",
    ),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixNonAdjacentIntersectingEdgesMode",
        b't',
        "-1",
    ),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixVertexPositionMode",
        b't',
        "0",
    ),
    Row::Init(
        "XSTEP",
        "FromSTEP.FixShape.FixVertexToleranceMode",
        b't',
        "-1",
    ),
    Row::Init("XSTEP", "FromSTEP.FixFaceSize.Tolerance", b't', "1.e-7"),
    Row::Enum("step", "read.step.all.shapes", 0, &["Off", "On"]),
    Row::Int("read.step.all.shapes", 0),
    Row::Enum(
        "step",
        "read.step.constructivegeom.relationship",
        0,
        &["OFF", "ON"],
    ),
    Row::Text("read.step.constructivegeom.relationship", "OFF"),
    Row::Enum("step", "read.step.root.transformation", 0, &["OFF", "ON"]),
    Row::Text("read.step.root.transformation", "ON"),
    Row::Enum(
        "step",
        "read.step.codepage",
        0,
        &[
            "SJIS",
            "EUC",
            "NoConversion",
            "GB",
            "UTF8",
            "SystemLocale",
            "CP1250",
            "CP1251",
            "CP1252",
            "CP1253",
            "CP1254",
            "CP1255",
            "CP1256",
            "CP1257",
            "CP1258",
            "iso8859-1",
            "iso8859-2",
            "iso8859-3",
            "iso8859-4",
            "iso8859-5",
            "iso8859-6",
            "iso8859-7",
            "iso8859-8",
            "iso8859-9",
            "CP850",
        ],
    ),
    Row::Text("read.step.codepage", "UTF8"),
    Row::Enum(
        "step",
        "read.step.tessellated",
        0,
        &["Off", "On", "OnNoBRep"],
    ),
    Row::Text("read.step.tessellated", "On"),
    Row::Enum(
        "step",
        "write.step.tessellated",
        0,
        &["Off", "On", "OnNoBRep"],
    ),
    Row::Text("write.step.tessellated", "OnNoBRep"),
];

/// Runs the statements.
pub(super) fn define_statics() -> Result<(), Failure> {
    for row in ROWS {
        match row {
            Row::Init(family, name, kind, init) => {
                InterfaceStatic::init_char(family, name, *kind, init)?;
            }
            Row::Enum(family, name, first, values) => {
                InterfaceStatic::init_char(family, name, b'e', "")?;
                InterfaceStatic::init_char(family, name, b'&', &format!("enum {first}"))?;
                for value in *values {
                    InterfaceStatic::init_char(family, name, b'&', &format!("eval {value}"))?;
                }
            }
            Row::Text(name, value) => {
                InterfaceStatic::set_c_val(name, value)?;
            }
            Row::Int(name, value) => {
                InterfaceStatic::set_i_val(name, *value)?;
            }
        }
    }
    Ok(())
}
