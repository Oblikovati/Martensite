//! Checking a rendered pixel's colour against an expected RGB, searching the 6x6 neighbourhood
//! when the pixel itself differs, with the regressions' messages.
use super::*;

/// Channels compare as integers scaled by 1e5, with a relative tolerance computed in integer
/// arithmetic (as the original check does): any difference below the expected value passes.
fn channel_off(expected: i64, actual: i64) -> bool {
    let tolerance = if expected != 0 {
        (expected - actual).abs() / expected
    } else {
        actual
    };
    tolerance as f64 > 0.2
}

fn scaled(value: f64) -> i64 {
    (value * 1.0e5) as i64
}

impl ViewerSession {
    /// Check that pixel (x, y), or one of the 6x6 pixels around it, has the linear RGB colour
    /// `expected`; prints the check's report and "Error" lines; `session.check_color(58, 324, [0., 1., 1.])?`.
    pub fn check_color(&self, x: i32, y: i32, expected: [f64; 3]) -> Result<bool, Failure> {
        let [r, g, b] = expected;
        println!("Coordinate x = {x}\nCoordinate y = {y}");
        println!("RED color of RGB is {r}\nGREEN color of RGB is {g}\nBLUE color of RGB is {b}");
        if x <= 1 || y <= 1 {
            println!("Error : minimal coordinate is x = 2, y = 2. But we have x = {x} y = {y}");
            return Ok(false);
        }
        let wanted = expected.map(scaled);
        let off = self.channels_off(x, y, wanted)?;
        let names = ["RED", "GREEN", "BLUE"];
        for (name, _) in names.iter().zip(off).filter(|(_, off)| *off) {
            println!("Warning : {name} light of additive color model RGB is invalid");
        }
        if !off.contains(&true) {
            return Ok(true);
        }
        println!("Warning : Colors of default coordinate are not equal");
        self.search_neighbourhood(x, y, wanted)
    }

    fn channels_off(&self, x: i32, y: i32, wanted: [i64; 3]) -> Result<[bool; 3], Failure> {
        let color = self.pixel_color(x, y)?;
        let actual = [color.red(), color.green(), color.blue()].map(scaled);
        Ok([0, 1, 2].map(|i| channel_off(wanted[i], actual[i])))
    }

    fn search_neighbourhood(&self, x: i32, y: i32, wanted: [i64; 3]) -> Result<bool, Failure> {
        for row in 0..=5 {
            for column in 0..=5 {
                let (px, py) = (x - 2 + column, y - 2 + row);
                println!("{px}\n{py}");
                let (width, height) = self.source_window_size();
                if px < 0 || py < 0 || px >= width || py > height {
                    println!("Warning : Pixel coordinates ({px}; {py}) are out of view");
                    continue;
                }
                if self.channels_off(px, py, wanted)?.contains(&true) {
                    println!(
                        "Warning : Point with true color was not found near default coordinates"
                    );
                    continue;
                }
                println!("Point with valid color was found");
                return Ok(true);
            }
        }
        println!(
            "Error : Colors are not equal in default coordinate and in the near coordinates too"
        );
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_tolerance_uses_integer_division() {
        assert!(!channel_off(100_000, 50_000));
        assert!(channel_off(100_000, 200_001));
        assert!(channel_off(0, 1));
        assert!(!channel_off(0, 0));
    }
}
