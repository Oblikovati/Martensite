//! Checking a rendered pixel's colour against an expected RGB, with the check's own report.
//!
//! The check scales channels by 1e5 and divides in integer arithmetic, so any difference below the
//! expected value passes; only a channel at more than twice its expected value, or a non-zero one
//! expected to be zero, is off. That weakness belongs to the check, and is kept.
use super::commands::CafSession;
use super::viewer::ViewerHost;

fn scaled(value: f64) -> i64 {
    (value * 1.0e5) as i64
}

fn channel_off(expected: i64, actual: i64) -> bool {
    let tolerance = if expected != 0 {
        (expected - actual).abs() / expected
    } else {
        actual
    };
    tolerance as f64 > 0.2
}

/// The shortest text of 17 significant digits that a C `%.17g` writes.
/// Example: `significant_17(0.5) == "0.5"`.
pub fn significant_17(value: f64) -> String {
    if value == 0. {
        return "0".to_owned();
    }
    let scientific = format!("{value:.16e}");
    let (_, exponent) = scientific.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    let decimals = (16 - exponent).max(0) as usize;
    let fixed = format!("{value:.decimals$}");
    if fixed.contains('.') {
        fixed.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        fixed
    }
}

fn read_and_report(host: &ViewerHost, x: i32, y: i32) -> [i64; 3] {
    println!("vreadpixel {x} {y} rgb ");
    let rgb = host.read_pixel(x, y);
    println!(
        "{} {} {}",
        significant_17(rgb[0]),
        significant_17(rgb[1]),
        significant_17(rgb[2])
    );
    rgb.map(scaled)
}

impl CafSession {
    /// `checkcolor x y r g b`: an error unless the pixel, or one of the 6x6 around it, is in range.
    /// Example: `session.checkcolor(273, 115, [0.78, 0.55, 0.9]);`.
    pub fn checkcolor(&self, x: i32, y: i32, expected: [f64; 3]) {
        let [r, g, b] = expected;
        println!("Coordinate x = {x}\nCoordinate y = {y}");
        println!("RED color of RGB is {r}\nGREEN color of RGB is {g}\nBLUE color of RGB is {b}");
        let host = self.open_host();
        let wanted = expected.map(scaled);
        let actual = read_and_report(host, x, y);
        let off = [0, 1, 2].map(|i| channel_off(wanted[i], actual[i]));
        for (name, _) in ["RED", "GREEN", "BLUE"]
            .iter()
            .zip(off)
            .filter(|pair| pair.1)
        {
            println!("Warning : {name} light of additive color model RGB is invalid");
        }
        if !off.contains(&true) {
            return;
        }
        println!("Warning : Colors of default coordinate are not equal");
        if search_neighbourhood(host, x, y, wanted) {
            println!("Point with valid color was found");
        } else {
            println!(
                "Error : Colors are not equal in default coordinate and in the near coordinates too"
            );
        }
    }
}

fn search_neighbourhood(host: &ViewerHost, x: i32, y: i32, wanted: [i64; 3]) -> bool {
    for row in 0..=5 {
        for column in 0..=5 {
            let (px, py) = (x - 2 + column, y - 2 + row);
            println!("{px}\n{py}");
            let actual = read_and_report(host, px, py);
            if ![0, 1, 2]
                .map(|i| channel_off(wanted[i], actual[i]))
                .contains(&true)
            {
                return true;
            }
            println!("Warning : Point with true color was not found near default coordinates");
        }
    }
    false
}

#[test]
fn channels_compare_in_integer_arithmetic() {
    assert!(!channel_off(100_000, 50_000));
    assert!(channel_off(100_000, 200_001));
    assert!(channel_off(0, 1));
    assert!(!channel_off(0, 0));
}

#[test]
// Each literal spells the seventeen digits it is checked against.
#[allow(clippy::excessive_precision)]
fn seventeen_digits_are_written_as_the_viewer_commands_write_them() {
    assert_eq!(significant_17(0.5), "0.5");
    assert_eq!(significant_17(0.), "0");
    assert_eq!(significant_17(1.), "1");
    assert_eq!(significant_17(0.9), "0.90000000000000002");
    assert_eq!(significant_17(0.41788515448570251), "0.41788515448570251");
    assert_eq!(
        significant_17(0.0091340569779276848),
        "0.0091340569779276848"
    );
}
