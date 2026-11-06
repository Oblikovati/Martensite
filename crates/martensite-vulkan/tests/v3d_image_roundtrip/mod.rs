//! File, stream and bounded-memory image roundtrips retain the complete QA construction.
mod codec;
mod presentation;
use crate::dataset_shape::locate_dataset_file;
use crate::dump_diff::{DiffOptions, diff_dumps};
use crate::session_run::{SessionRun, banner};
use std::error::Error;
use std::path::PathBuf;
type Outcome = Result<(), Box<dyn Error>>;
const IMAGES: [[&str; 2]; 3] = [
    ["bug31956_1.png", "bug31956_2.png"],
    ["bug31956_test_1.png", "bug31956_test_2.png"],
    ["bug31956_test_1s.png", "bug31956_test_2s.png"],
];

const IMAGE_BANNER: &[&str] = &[
    "============",
    "0031956: Visualization - provide Image_AlienPixMap::Save() writing into a memory buffer instead of a file",
    "============",
    "",
];

/// Preserve all three QA paths, six images and four comparisons; `image_roundtrips()?`.
pub(crate) fn image_roundtrips() -> Outcome {
    banner(IMAGE_BANNER);
    let inputs = [
        locate_dataset_file("chataignier.gif")?,
        locate_dataset_file("hatch_1.png")?,
    ];
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_standard_projection(martensite_tkv3d::v3d::V3dTypeOfOrientation::ZUP_TOP)?;
    roundtrip_phase(&mut run, &inputs, codec::ImagePath::File, IMAGES[0])?;
    run.session().clear()?;
    roundtrip_phase(&mut run, &inputs, codec::ImagePath::Buffer, IMAGES[1])?;
    run.session().clear()?;
    roundtrip_phase(&mut run, &inputs, codec::ImagePath::Stream, IMAGES[2])?;
    compare_roundtrips()?;
    run.finish()
}

fn roundtrip_phase(
    run: &mut SessionRun,
    inputs: &[PathBuf; 2],
    path: codec::ImagePath,
    images: [&'static str; 2],
) -> Outcome {
    for ((input, image), temporary) in inputs
        .iter()
        .zip(images)
        .zip(["bug31956_temp_1.gif", "bug31956_temp_2.png"])
    {
        let decoded = codec::decoded_roundtrip(input, path, temporary)?;
        presentation::display_decoded(run.session(), decoded)?;
        run.session().fit()?;
        run.dump(image)?;
    }
    Ok(())
}

fn compare_roundtrips() -> Outcome {
    let options = DiffOptions {
        tolerance: 0.1,
        black_white: false,
        border_filter: false,
        diff_image: None,
    };
    for (images, label) in [(IMAGES[1], ""), (IMAGES[2], " (using stream)")] {
        for (index, image) in images.into_iter().enumerate() {
            if diff_dumps(image, IMAGES[0][index], &options)? != 0 {
                let ordinal = if index == 0 { "1st" } else { "2nd" };
                println!("Error difference in {ordinal} image{label}");
            }
        }
    }
    Ok(())
}
