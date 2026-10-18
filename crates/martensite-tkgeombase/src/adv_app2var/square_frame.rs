//! Caller integer identities remain distinct from square callback-local controls.
#[path = "square_status.rs"]
mod status;
use super::approx_f2var::SampledSquare;
use super::scalar_incidence::{ScalarIncidence, scalar_incidence};
use martensite_tkernel::standard::Failure;
/// An original caller integer of square sampling; `AdvApp2VarSquareIntegerField::URootCount`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarSquareIntegerField {
    /// Number of coordinates in each function value.
    Dimension,
    /// Original U sampling count.
    URootCount,
    /// Original V sampling count.
    VRootCount,
    /// Caller preference, separate from the mutable callback-local preference.
    FavoriteIso,
}
use AdvApp2VarSquareIntegerField as Field;
const FIELDS: [Field; 4] = [
    Field::Dimension,
    Field::URootCount,
    Field::VRootCount,
    Field::FavoriteIso,
];
#[derive(Clone, Copy)]
enum FrameSquareField {
    Caller(Field),
    Status,
}
impl std::fmt::Debug for FrameSquareField {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Caller(field) => std::fmt::Debug::fmt(field, formatter),
            Self::Status => formatter.write_str("ErrorCode"),
        }
    }
}
/// Equal indices identify one initialized caller integer;
/// `AdvApp2VarSquareIntegerBindings { dimension:0, root_counts:[1,0], favorite_iso:2 }`.
#[derive(Clone, Copy, Debug)]
pub struct AdvApp2VarSquareIntegerBindings {
    /// Dimension's owned slot.
    pub dimension: usize,
    /// Owned U and V root-count slots.
    pub root_counts: [usize; 2],
    /// Original caller preference slot.
    pub favorite_iso: usize,
}
impl AdvApp2VarSquareIntegerBindings {
    fn slot(self, field: Field) -> usize {
        match field {
            Field::Dimension => self.dimension,
            Field::URootCount => self.root_counts[0],
            Field::VRootCount => self.root_counts[1],
            Field::FavoriteIso => self.favorite_iso,
        }
    }
}
/// Mutable operation-local caller integers, preserving shared slot identities.
/// `AdvApp2VarSquareOperationFrame::new(vec![3,3,2], bindings)?`.
pub struct AdvApp2VarSquareOperationFrame {
    values: Vec<i32>,
    bindings: AdvApp2VarSquareIntegerBindings,
    incidence: Vec<ScalarIncidence<FrameSquareField>>,
}
impl AdvApp2VarSquareOperationFrame {
    /// Validate owned indices once; `AdvApp2VarSquareOperationFrame::new(vec![3,3,2], bindings)?`.
    pub fn new(
        values: Vec<i32>,
        bindings: AdvApp2VarSquareIntegerBindings,
    ) -> Result<Self, Failure> {
        let incidence = scalar_incidence(
            FIELDS.map(|field| (FrameSquareField::Caller(field), bindings.slot(field))),
            values.len(),
            "AdvApp2VarSquareOperationFrame",
        )?;
        Ok(Self {
            values,
            bindings,
            incidence,
        })
    }
    /// Inspect an original caller integer; `frame.integer(AdvApp2VarSquareIntegerField::Dimension)`.
    pub fn integer(&self, field: Field) -> i32 {
        self.values[self.bindings.slot(field)]
    }
    pub(super) fn set_integer(&mut self, field: Field, value: i32) {
        self.values[self.bindings.slot(field)] = value;
    }
    pub(super) fn seed_request(&mut self, request: &SampledSquare<'_>) -> Result<(), Failure> {
        for group in &self.incidence {
            self.values[group.slot] =
                consistent_square_input(group, request, self.values[group.slot])?;
        }
        Ok(())
    }
    pub(super) fn root_counts(&self) -> [i32; 2] {
        [Field::URootCount, Field::VRootCount].map(|field| self.integer(field))
    }
    pub(super) fn write_request(&self, request: &mut SampledSquare<'_>) {
        request.ndimen = self.integer(Field::Dimension);
        [request.nbpntu, request.nbpntv] = self.root_counts();
        request.isofav = self.integer(Field::FavoriteIso);
    }
}
fn consistent_square_input(
    group: &ScalarIncidence<FrameSquareField>,
    request: &SampledSquare<'_>,
    entry_value: i32,
) -> Result<i32, Failure> {
    let initial = square_input(request, group.fields[0], entry_value);
    for field in &group.fields[1..] {
        let value = square_input(request, *field, entry_value);
        if value != initial {
            return Err(Failure::with_message(
                "Standard_DomainError",
                &format!(
                    "AdvApp2VarSquareOperationFrame: slot={} incident {field:?} value={value}; expected shared initial value={initial}",
                    group.slot
                ),
            ));
        }
    }
    Ok(initial)
}
fn square_input(request: &SampledSquare<'_>, field: FrameSquareField, entry_value: i32) -> i32 {
    let FrameSquareField::Caller(field) = field else {
        return entry_value;
    };
    match field {
        Field::Dimension => request.ndimen,
        Field::URootCount => request.nbpntu,
        Field::VRootCount => request.nbpntv,
        Field::FavoriteIso => request.isofav,
    }
}
