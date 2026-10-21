//! The view commands the cases call (and the `smallview`, `v2d` and `checkview`
//! procedures).
use super::variables::isdraw;
use crate::support::numbers::g17;
use crate::support::script::{Reply, Script, ScriptStep};
use crate::support::session::ViewStep;

/// A view command that logs its arguments, returns nothing and runs `step`.
fn view_command(d: &mut Script, args: &[&str], step: ViewStep<'_>) -> ScriptStep {
    d.command(args, |_| Ok(Reply::ok("")))?;
    d.view(step);
    Ok(())
}

/// `smallview [type]`: `delete`, then `view 1 type 728 450 400 400`.
pub fn smallview(d: &mut Script, kind: &str) -> ScriptStep {
    d.command(&["delete"], |_| Ok(Reply::ok("")))?;
    view_command(
        d,
        &["view", "1", kind, "728", "450", "400", "400"],
        ViewStep::SmallView(kind),
    )
}

/// `view 1 type x y w h`.
pub fn view(d: &mut Script, kind: &str, frame: [i32; 4]) -> ScriptStep {
    let texts = frame.map(|value| value.to_string());
    let args = [
        "view", "1", kind, &texts[0], &texts[1], &texts[2], &texts[3],
    ];
    view_command(d, &args, ViewStep::View(kind, frame))
}

/// `v2d`: `delete`, then `view 1 -2D- 465 20 800 800`.
pub fn v2d(d: &mut Script) -> ScriptStep {
    d.command(&["delete"], |_| Ok(Reply::ok("")))?;
    view_command(
        d,
        &["view", "1", "-2D-", "465", "20", "800", "800"],
        ViewStep::V2d,
    )
}

/// `fit`.
pub fn fit(d: &mut Script) -> ScriptStep {
    view_command(d, &["fit"], ViewStep::Fit)
}

/// `2dfit`.
pub fn fit2d(d: &mut Script) -> ScriptStep {
    view_command(d, &["2dfit"], ViewStep::Fit2d)
}

/// `zoom z`.
pub fn zoom(d: &mut Script, factor: f64) -> ScriptStep {
    let text = g17(factor);
    view_command(d, &["zoom", &text], ViewStep::Zoom(factor))
}

/// `clear`.
pub fn clear(d: &mut Script) -> ScriptStep {
    d.hide_3d();
    view_command(d, &["clear"], ViewStep::Clear)
}

/// `clear names` (`erase names`): prints the names it erased.
pub fn erase(d: &mut Script, names: &[&str]) -> ScriptStep {
    let args: Vec<&str> = std::iter::once("clear")
        .chain(names.iter().copied())
        .collect();
    d.command(&args, |d| {
        let erased: String = names
            .iter()
            .filter(|name| d.hide(name))
            .map(|name| format!("{name} "))
            .collect();
        Ok(Reply::ok(erased))
    })?;
    d.view(ViewStep::Erase(names));
    Ok(())
}

/// `donly names`: prints the names it displays. When one of the names is not a
/// variable (`donly result*` gets no glob expansion), it returns before erasing anything.
pub fn donly(d: &mut Script, names: &[&str]) -> ScriptStep {
    let args: Vec<&str> = std::iter::once("donly")
        .chain(names.iter().copied())
        .collect();
    if !names.iter().all(|name| d.isdraw(name)) {
        d.command(&args, |_| Ok(Reply::ok("")))?;
        return Ok(());
    }
    d.command(&args, |d| {
        d.hide_all();
        let shown: String = names
            .iter()
            .filter(|name| d.show(name))
            .map(|name| format!("{name} "))
            .collect();
        Ok(Reply::ok(shown))
    })?;
    d.view(ViewStep::Donly(names));
    Ok(())
}

/// `don pattern*`: `donly` of the variables the glob matches.
pub fn don(d: &mut Script, prefix: &str) -> ScriptStep {
    let names = d.directory(prefix);
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    donly(d, &names)
}

/// `disp pattern*`: `display` of the variables the glob matches.
pub fn disp(d: &mut Script, prefix: &str) -> ScriptStep {
    let names = d.directory(prefix);
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    display(d, &names)
}

/// `display names`: prints the names it displays (those that were hidden).
pub fn display(d: &mut Script, names: &[&str]) -> ScriptStep {
    let args: Vec<&str> = std::iter::once("display")
        .chain(names.iter().copied())
        .collect();
    d.command(&args, |d| {
        let shown: String = names
            .iter()
            .filter(|name| d.show(name))
            .map(|name| format!("{name} "))
            .collect();
        Ok(Reply::ok(shown))
    })?;
    d.view(ViewStep::Display(names));
    Ok(())
}

/// `clpoles name`.
pub fn clpoles(d: &mut Script, name: &str) -> ScriptStep {
    view_command(d, &["clpoles", name], ViewStep::ClearPoles(name))
}

/// `clknots name`.
pub fn clknots(d: &mut Script, name: &str) -> ScriptStep {
    view_command(d, &["clknots", name], ViewStep::ClearKnots(name))
}

/// `xwd image`: the capture of view 1.
pub fn xwd(d: &mut Script, image: &str) -> ScriptStep {
    let path = format!("{image}.png");
    view_command(d, &["xwd", &path], ViewStep::Xwd(image))
}

/// `checkview -screenshot -2d -path image`.
pub fn checkview_screenshot(d: &mut Script, image: &str) -> ScriptStep {
    d.puts(&format!("checkview -screenshot -2d -path {image}.png"));
    xwd(d, image)
}

/// `checkview -display shape -2d -path image`: `smallview`, `clear`, `donly shape` when
/// it is defined, `fit` and `xwd`.
pub fn checkview_display(d: &mut Script, shape: &str, image: &str) -> ScriptStep {
    d.puts(&format!("checkview -display {shape} -2d -path {image}.png"));
    isdraw(d, shape)?;
    smallview(d, "AXON")?;
    clear(d)?;
    if isdraw(d, shape)? {
        donly(d, &[shape])?;
    }
    fit(d)?;
    xwd(d, image)
}
