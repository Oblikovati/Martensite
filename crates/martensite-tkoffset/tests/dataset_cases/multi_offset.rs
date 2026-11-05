//! `perform_offset_multi_with_ref`: an offset with values per face, its counts and masses compared
//! with the references the script gives, one line each.
use crate::script::OffsetScript;
use crate::steps::{MultiOffset, Offset};
use crate::support::{
    checkshape::{checkshape, checkshape_named},
    nbshapes::{Counting, nbshapes},
    properties::{Prop, printed_mass, props},
    tolerance_check::checkmaxtol,
};
use martensite_tkernel::{standard::Failure, storage::format_general};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The relative deviation `compare_prop_values` allows.
const DEVIATION: f64 = 1.0e-2;

fn compare_prop(name: &str, value: f64, reference: f64) -> bool {
    let off = (reference != 0. && ((reference - value) / reference).abs() > DEVIATION)
        || (value == 0. && reference != 0.);
    if off {
        let (value, reference) = (format_general(value, 6), format_general(reference, 15));
        println!("Error: The {name} of result shape is {value}, expected {reference}");
    } else {
        println!("OK: The {name} of result shape is as expected");
    }
    !off
}

fn compare_count(entity: &str, value: usize, reference: usize) -> bool {
    if value != reference {
        println!(
            "Error: number of {entity} entities in the result shape is {value}, expected {reference}"
        );
    } else {
        println!("OK: number of {entity} entities in the result shape is as expected");
    }
    value == reference
}

/// `perform_offset_multi`: true when the result exists and passes `checkshape`.
fn perform(script: &mut OffsetScript<'_>, step: &MultiOffset) -> Result<bool, Failure> {
    let source = script.required(step.shape)?;
    let copied = crate::topology_steps::copied(&source)?;
    script.set_shape("sx", copied);
    crate::offset_steps::apply(script, &Offset::Parameter(1.0e-7, true, 'i', true))?;
    crate::offset_steps::apply(script, &Offset::Load("sx", step.value, &[]))?;
    if !step.faces.is_empty() {
        let parts = crate::wire_end::exploded(script, "sx", ShapeEnum::Face);
        for (id, value) in step.faces.iter().zip(step.face_values) {
            let face = martensite_tkbrep::topods::face(&parts[*id as usize - 1].1)?;
            script.commands.offset.set_offset_on_face(&face, *value);
        }
    }
    crate::offset_steps::apply(script, &Offset::Perform(step.result))?;
    let Some(result) = script.session().shape(step.result).cloned() else {
        return Ok(false);
    };
    checkshape(&result);
    let unified: &'static str = Box::leak(format!("{}_unif", step.result).into_boxed_str());
    crate::topology_steps::unify_into(script, unified, step.result)?;
    Ok(true)
}

/// Runs the procedure; `run(script, &step)`.
pub fn run(script: &mut OffsetScript<'_>, step: &MultiOffset) -> Result<(), Failure> {
    if !perform(script, step)? {
        checkshape_named(script.session(), step.result);
        println!(
            "Error: operation with offset value {} has failed",
            step.value
        );
        return Ok(());
    }
    let counted = if step.unified {
        format!("{}_unif", step.result)
    } else {
        step.result.to_owned()
    };
    let counts = nbshapes(script.session(), &counted, Counting::Located);
    let result = script.shape(step.result);
    let (mut good, mut area, mut volume) = (true, 0., 0.);
    if let [area_ref, volume_ref, wires, faces] = *step.refs {
        area = printed_mass(props(Prop::Area, &result, None));
        volume = printed_mass(props(Prop::Volume, &result, None));
        good &= compare_prop("area", area, area_ref);
        good &= compare_prop("volume", volume, volume_ref);
        good &= compare_count("wire", counts[2], wires as usize);
        good &= compare_count("face", counts[3], faces as usize);
    }
    good &= compare_count("shell", counts[4], 1);
    good &= compare_count("solid", counts[5], 1);
    let status = if good { "OK" } else { "KO" };
    let (area_text, volume_text) = (format_general(area, 6), format_general(volume, 6));
    if !good {
        println!(
            "Error: operation with offset value {} produced incorrect result",
            step.value
        );
    }
    println!(
        "Offset value {} - {status}: area - {area_text}; volume - {volume_text}; wires - {}; faces - {} ({area_text} {volume_text} {} {})",
        step.value, counts[2], counts[3], counts[2], counts[3]
    );
    checkmaxtol(script.session(), step.result, Some("1.e-6"), None);
    Ok(())
}
