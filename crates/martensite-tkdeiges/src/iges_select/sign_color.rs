//! The signature of the color of an entity.
use super::signatures::iges_signature;
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_graph::IgesGraphColor;
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_select::{IFSelectSignature, IFSelectSignatureBase},
    interface::{
        InterfaceInterfaceModel, InterfaceModelEntities, InterfaceSignType,
        interface_sign_type_text,
    },
    moni_tool::MoniToolSignText,
};

/// The name of the signature of a mode.
fn color_name(mode: i32) -> &'static str {
    match mode {
        2 => "IGES Color Name",
        3 => "IGES Color R,G,B",
        4 => "IGES Color RED Value",
        5 => "IGES Color GREEN Value",
        6 => "IGES Color BLUE Value",
        _ => "IGES Color Number",
    }
}

/// The intensities of the standard color ranks 1 to 8, as `R,G,B`.
const STANDARD_RGB: [[i32; 3]; 8] = [
    [0, 0, 0],
    [100, 0, 0],
    [0, 100, 0],
    [0, 0, 100],
    [100, 100, 0],
    [100, 0, 100],
    [0, 100, 100],
    [100, 100, 100],
];

/// The names of the standard color ranks 1 to 8.
const STANDARD_NAMES: [&str; 8] = [
    "BLACK", "RED", "GREEN", "BLUE", "YELLOW", "MAGENTA", "CYAN", "WHITE",
];

iges_signature! {
    /// The color of an entity: 2 its name, 3 its intensities, 4 to 6 one of them, any other mode
    /// its number.
    ///
    /// ```
    /// use martensite_tkdeiges::{iges_data::IgesDataIgesEntity, iges_select::IgesSelectSignColor};
    /// use martensite_tkernel::standard::Handle;
    /// use martensite_tkxsbase::interface::InterfaceSignType;
    /// let entity = Handle::new(IgesDataIgesEntity::default());
    /// let color = Handle::default();
    /// entity.get().unwrap().init_color(&color, 2);
    /// assert_eq!(IgesSelectSignColor::new(2).value(&entity.cast(), &Handle::default()), "RED");
    /// ```
    IgesSelectSignColor, "IgesSelectSignColor", { themode: i32 }
}

impl IgesSelectSignColor {
    /// The signature of a mode.
    pub fn new(mode: i32) -> Self {
        let sign = Self {
            transient: StandardTransient::default(),
            base: IFSelectSignatureBase::new(color_name(mode)),
            themode: mode,
        };
        if (4..=6).contains(&mode) {
            sign.set_int_case(true, 0, true, 0);
        }
        sign
    }
}

/// The directory number of a color, 0 without model.
fn color_number(
    model: &Handle<dyn InterfaceInterfaceModel>,
    color: &Handle<StandardTransient>,
) -> i32 {
    model.get().map_or(0, |live| 2 * live.number(color) - 1)
}

/// The intensities of a color entity, -1 for those not there.
fn intensities(color: &Handle<StandardTransient>) -> [f64; 3] {
    let mut rgb = [-1.0; 3];
    if let Some(live) = color.cast::<IgesGraphColor>().get() {
        let [red, green, blue] = &mut rgb;
        live.rgb_intensity(red, green, blue);
    }
    rgb
}

/// An intensity as text, empty when it is not there.
fn intensity_text(value: f64) -> String {
    if value >= 0.0 {
        IFSelectSignatureBase::int_value(value as i32)
    } else {
        String::new()
    }
}

impl IgesSelectSignColor {
    fn number_value(
        rank: i32,
        model: &Handle<dyn InterfaceInterfaceModel>,
        color: &Handle<StandardTransient>,
    ) -> String {
        match rank {
            0 => "(none)".to_owned(),
            r if r > 0 => format!("S{}", IFSelectSignatureBase::int_value(r)),
            _ => format!(
                "D{}",
                IFSelectSignatureBase::int_value(color_number(model, color))
            ),
        }
    }
    fn name_value(
        rank: i32,
        model: &Handle<dyn InterfaceInterfaceModel>,
        color: &Handle<StandardTransient>,
    ) -> String {
        if rank == 0 {
            return "(none)".to_owned();
        }
        if rank > 0 {
            return STANDARD_NAMES
                .get(rank as usize - 1)
                .map_or("Unknown Number", |name| *name)
                .to_owned();
        }
        let name = color
            .cast::<IgesGraphColor>()
            .get()
            .map_or_else(Handle::default, IgesGraphColor::color_name);
        match name.get() {
            Some(text) => String::from_utf8_lossy(text.string().as_bytes()).into_owned(),
            None => format!(
                "D{}",
                IFSelectSignatureBase::int_value(color_number(model, color))
            ),
        }
    }
    /// The text of one intensity or of all; standard ranks are known, others come from the entity.
    fn intensity_value(mode: i32, rank: i32, color: &Handle<StandardTransient>) -> String {
        if rank == 0 {
            return String::new();
        }
        if rank > 0 {
            let Some(rgb) = STANDARD_RGB.get(rank as usize - 1) else {
                return String::new();
            };
            return match mode {
                3 => format!("{},{},{}", rgb[0], rgb[1], rgb[2]),
                _ => rgb[(mode - 4) as usize].to_string(),
            };
        }
        let rgb = intensities(color);
        match mode {
            3 => format!(
                "{},{},{}",
                intensity_text(rgb[0]),
                intensity_text(rgb[1]),
                intensity_text(rgb[2])
            ),
            _ => intensity_text(rgb[(mode - 4) as usize]),
        }
    }
}

impl InterfaceSignType for IgesSelectSignColor {
    fn value(
        &self,
        ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> String {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return String::new();
        };
        let rank = live.rank_color();
        let color = live.color().cast::<StandardTransient>();
        match self.themode {
            2 => Self::name_value(rank, model, &color),
            3..=6 => Self::intensity_value(self.themode, rank, &color),
            _ => Self::number_value(rank, model, &color),
        }
    }
}

impl IFSelectSignature for IgesSelectSignColor {
    fn signature_base(&self) -> &IFSelectSignatureBase {
        &self.base
    }
}
