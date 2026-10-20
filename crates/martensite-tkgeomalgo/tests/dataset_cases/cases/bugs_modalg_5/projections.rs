//! `bugs_modalg_5::bug23706_16` .. bug23706_61
//! ("Cannot project point on curve"): `proj c x y z` on a restored curve, and each
//! printed `parameter i` must equal the expected real exactly (script compares the numbers).
use crate::cases::script_words::number_after;
use crate::support::commands::extrema::proj;
use crate::support::numbers::script_real;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptError, ScriptStep};

/// The case body: banner, `restore`, `proj`, the `regexp`s and the comparison.
fn projected_parameters(d: &mut Script, file: &str, xyz: [f64; 3], expected: &[f64]) -> ScriptStep {
    d.puts("========");
    d.puts("OCC23706");
    d.puts("========");
    d.puts("");
    restore(d, file, "c")?;
    let info = proj(d, "c", xyz)?;
    let mut correct = true;
    for (index, expected) in expected.iter().enumerate() {
        let prefix = format!("parameter {} =", index + 1);
        let Some(found) = number_after(&info, &prefix) else {
            let variable = index + 1;
            return Err(ScriptError(format!(
                "can't read \"pp{variable}\": no such variable"
            )));
        };
        correct &= script_real(found) == *expected;
    }
    if correct {
        d.puts("OK: Projection is correct");
    } else {
        d.puts("Error : Projection is not correct");
    }
    Ok(())
}

pub fn bug23706_16(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c.draw",
        [3.999999652077201, 5.0000000062915735, 5.00002142991819367],
        &[0.99910795389207607],
    )
}

pub fn bug23706_17(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c.draw",
        [
            3.99999991301930024,
            5.00000000157289337,
            5.00000535747954842,
        ],
        &[0.99955486819730044],
    )
}

pub fn bug23706_19(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c.draw",
        [3.99999999837571056, 5.0000000000293724, 5.0000001000463034],
        &[0.99993927567419039],
    )
}

pub fn bug23706_20(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c2.draw",
        [3.9999965207720098, 5.0000000629157348, 5.0002142991819367],
        &[0.99715423329884789],
    )
}

pub fn bug23706_21(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c2.draw",
        [3.999999652077201, 5.0000000062915735, 5.00002142991819367],
        &[0.99910795390933049],
    )
}

pub fn bug23706_22(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c2.draw",
        [
            3.99999991301930024,
            5.00000000157289337,
            5.00000535747954842,
        ],
        &[0.99955486819835271],
    )
}

pub fn bug23706_24(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c2.draw",
        [3.99999999837571056, 5.0000000000293724, 5.0000001000463034],
        &[0.99993927567399621],
    )
}

pub fn bug23706_26(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c03.draw",
        [11.0, -6.0, 5.0],
        &[0.22894170490369881, 1.0000000014907082],
    )
}

pub fn bug23706_27(d: &mut Script) -> ScriptStep {
    projected_parameters(d, "bug23706_c03.draw", [3.0, 6.0, -3.0], &[1.0])
}

pub fn bug23706_28(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c03.draw",
        [5.0, 8.0, -2.0],
        &[1.0, 1.1865241781930462],
    )
}

pub fn bug23706_29(d: &mut Script) -> ScriptStep {
    projected_parameters(d, "bug23706_c03.draw", [-4.0, 4.0, 1.0], &[0.0])
}

pub fn bug23706_31(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c04.draw",
        [11.0, -6.0, 5.0],
        &[0.22894170490369881, 1.0000000014907084],
    )
}

pub fn bug23706_32(d: &mut Script) -> ScriptStep {
    projected_parameters(d, "bug23706_c04.draw", [3.0, 6.0, -3.0], &[1.0])
}

pub fn bug23706_33(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c04.draw",
        [5.0, 8.0, -2.0],
        &[0.81347582180695377, 1.0],
    )
}

pub fn bug23706_34(d: &mut Script) -> ScriptStep {
    projected_parameters(d, "bug23706_c04.draw", [-4.0, 4.0, 1.0], &[2.0])
}

pub fn bug23706_36(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c05.draw",
        [11.0, -6.0, 5.0],
        &[0.22894170490369881, 1.0000000014908634],
    )
}

pub fn bug23706_37(d: &mut Script) -> ScriptStep {
    projected_parameters(d, "bug23706_c05.draw", [3.0, 6.0, -3.0], &[1.0])
}

pub fn bug23706_38(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c05.draw",
        [5.0, 8.0, -2.0],
        &[1.0, 1.0371228345434986],
    )
}

pub fn bug23706_39(d: &mut Script) -> ScriptStep {
    projected_parameters(d, "bug23706_c05.draw", [-4.0, 4.0, 1.0], &[0.0])
}

pub fn bug23706_40(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c07.draw",
        [-3.0, 15.0, -9.0],
        &[0.31967360381308063],
    )
}

pub fn bug23706_41(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c07.draw",
        [11.0, -6.0, 5.0],
        &[1.7205732840814361],
    )
}

pub fn bug23706_42(d: &mut Script) -> ScriptStep {
    projected_parameters(d, "bug23706_c07.draw", [3.0, 6.0, -3.0], &[1.0])
}

pub fn bug23706_43(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c07.draw",
        [5.0, 8.0, -2.0],
        &[1.0371228345434986, 0.99999999682789309],
    )
}

pub fn bug23706_44(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c07.draw",
        [-4.0, 4.0, 1.0],
        &[0.034819847916144751],
    )
}

pub fn bug23706_45(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c08.draw",
        [-3.0, 15.0, -9.0],
        &[0.51963826852813999],
    )
}

pub fn bug23706_46(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c08.draw",
        [11.0, -6.0, 5.0],
        &[0.48292970726418566, 1.7205732840814361],
    )
}

pub fn bug23706_47(d: &mut Script) -> ScriptStep {
    projected_parameters(d, "bug23706_c08.draw", [3.0, 6.0, -3.0], &[1.0])
}

pub fn bug23706_48(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c08.draw",
        [5.0, 8.0, -2.0],
        &[1.0371228345434986, 0.99999999851019361],
    )
}

pub fn bug23706_49(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c08.draw",
        [-4.0, 4.0, 1.0],
        &[0.087689905182099168],
    )
}

pub fn bug23706_50(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c11.draw",
        [3.0, 3.0, 2.0],
        &[
            2.261838779028444,
            2.7514388736312116,
            3.5195936992321921,
            3.9600115496393977,
            5.5000000059308434,
            6.838813244759355,
            7.8261046366621292,
        ],
    )
}

pub fn bug23706_51(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c11.draw",
        [5.0, 7.0, 8.0],
        &[
            1.0,
            2.8126840147763663,
            3.5195936992321926,
            3.9600115496393977,
            5.5000000059308434,
            7.2883607799598096,
        ],
    )
}

pub fn bug23706_52(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c11.draw",
        [11.0, -2.0, -2.0],
        &[
            2.947326959460205,
            4.4416670680933228,
            5.5000000000000036,
            6.6582576262308306,
            7.7414573419084736,
        ],
    )
}

pub fn bug23706_53(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c12.draw",
        [3.0, 3.0, 2.0],
        &[
            1.1738953633378706,
            2.1611867552406454,
            3.5000000059308389,
            5.0399884503606023,
            5.4804063007678074,
            6.2485611263687888,
            6.7381612209715556,
        ],
    )
}

pub fn bug23706_54(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c12.draw",
        [5.0, 7.0, 8.0],
        &[
            1.7116392200401909,
            3.5000000059308389,
            5.0399884503606023,
            5.4804063007678074,
            6.1873159852236332,
            8.0,
        ],
    )
}

pub fn bug23706_55(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c12.draw",
        [11.0, -2.0, -2.0],
        &[
            1.2585426580915264,
            2.3417423737691694,
            3.4999999999999991,
            4.5583329319066772,
            6.052673040539795,
        ],
    )
}

pub fn bug23706_56(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c13.draw",
        [3.0, 3.0, 2.0],
        &[
            1.8318851868378956,
            3.0397214383562297,
            5.5,
            6.838813244759355,
            7.8261046366621292,
        ],
    )
}

pub fn bug23706_57(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c13.draw",
        [5.0, 7.0, 8.0],
        &[1.0, 3.0397214383562297, 5.5, 7.2883607799598096],
    )
}

pub fn bug23706_58(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c13.draw",
        [11.0, -2.0, -2.0],
        &[
            2.2389225099869194,
            3.219764556283669,
            5.4999999999999991,
            6.6582576262308306,
            7.7414573419084736,
        ],
    )
}

pub fn bug23706_59(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c14.draw",
        [3.0, 3.0, 2.0],
        &[
            1.1738953633378706,
            2.1611867552406454,
            3.5000000000000004,
            5.9602785616437703,
            7.1681148131621049,
        ],
    )
}

pub fn bug23706_60(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c14.draw",
        [5.0, 7.0, 8.0],
        &[1.7116392200401909, 3.5, 5.9602785616437703, 8.0],
    )
}

pub fn bug23706_61(d: &mut Script) -> ScriptStep {
    projected_parameters(
        d,
        "bug23706_c14.draw",
        [11.0, -2.0, -2.0],
        &[
            1.2585426580915264,
            2.3417423737691694,
            3.4999999999999991,
            5.7802354437163306,
            6.761077490013081,
        ],
    )
}
