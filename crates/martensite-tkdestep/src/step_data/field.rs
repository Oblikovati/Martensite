//! `StepDataField`: a generally defined field: no value, a single value of any kind, or
//! a list or double list. The queries are in `field_queries`, the list setters in
//! `field_items`.
use super::{
    StepDataFieldKind as Kind, StepDataLogical, StepDataSelectInt, StepDataSelectMember,
    StepDataSelectNamed, StepDataSelectReal,
};
use martensite_tkernel::{
    ncollection::{ArrayError, HArray1, HArray2},
    standard::{Failure, Handle, RuntimeTyped, StandardTransient},
    tcollection::HAsciiString,
};

/// `HArray1<Handle<`StandardTransient`>>`.
pub(super) type TransientList = HArray1<Handle<StandardTransient>>;
/// `HArray1<Handle<HAsciiString>>`.
pub(super) type StringList = HArray1<Handle<HAsciiString>>;
/// `HArray2<Handle<`StandardTransient`>>`.
pub(super) type TransientGrid = HArray2<Handle<StandardTransient>>;

/// An array failure as the exception of a field member.
pub(super) fn array_failure(error: ArrayError) -> Failure {
    error.into_failure("StepDataField")
}

/// `StepDataField`. `clone` is the copy constructor without `copy` (handled data are
/// shared); [`StepDataField::copy_from`] is the assignment.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataField, StepDataFieldKind};
/// let mut field = StepDataField::new();
/// field.set_integer(42);
/// assert_eq!(field.kind(true), StepDataFieldKind::INTEGER);
/// assert_eq!(field.integer(1, 1).unwrap(), 42);
/// ```
#[derive(Clone, Default)]
pub struct StepDataField {
    pub(super) thekind: Kind,
    pub(super) theint: i32,
    pub(super) thereal: f64,
    pub(super) theany: Handle<StandardTransient>,
}

impl StepDataField {
    /// `StepDataField()`: no value defined.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepDataField(other, copy)`.
    pub fn from_other(other: &Self, copy: bool) -> Result<Self, Failure> {
        let mut field = other.clone();
        if copy {
            field.copy_from(other)?;
        }
        Ok(field)
    }
    /// `copy_from(other)`: strings, selects and lists are copied, entities shared. A
    /// `SelectNamed` copies the name and the field of the source.
    pub fn copy_from(&mut self, other: &Self) -> Result<(), Failure> {
        self.thekind = other.thekind;
        self.theint = other.theint;
        self.thereal = other.thereal;
        self.theany = other.theany.clone();
        if self.thekind == Kind::STRING || self.thekind == Kind::ENUM {
            let text = self.theany.cast::<HAsciiString>();
            if let Some(live) = text.get() {
                self.theany = Handle::new(HAsciiString::copied(live)).cast();
            }
            return Ok(());
        }
        if self.thekind == Kind::SELECT {
            self.copy_select()?;
            return Ok(());
        }
        if self.thekind.arity_bits() == Kind::LIST.to_int() {
            self.copy_list()?;
        }
        Ok(())
    }
    fn copy_select(&mut self) -> Result<(), Failure> {
        let real = self.theany.cast::<StepDataSelectReal>();
        if let Some(live) = real.get() {
            let copy = StepDataSelectReal::default();
            copy.set_real(live.real());
            self.theany = Handle::new(copy).cast();
            return Ok(());
        }
        let int = self.theany.cast::<StepDataSelectInt>();
        if let Some(live) = int.get() {
            let copy = StepDataSelectInt::default();
            copy.set_kind(live.kind());
            copy.set_int(live.int());
            self.theany = Handle::new(copy).cast();
            return Ok(());
        }
        let named = self.theany.cast::<StepDataSelectNamed>();
        if let Some(live) = named.get() {
            let copy = StepDataSelectNamed::default();
            if live.has_name() {
                copy.set_name(&live.name());
            }
            let source = live.field();
            copy.change_field(|field| field.copy_from(&source))?;
            self.theany = Handle::new(copy).cast();
        }
        Ok(())
    }
    fn copy_list(&mut self) -> Result<(), Failure> {
        if let Some(copy) = copied_array::<i32>(&self.theany)? {
            self.theany = copy;
            return Ok(());
        }
        if let Some(copy) = copied_array::<f64>(&self.theany)? {
            self.theany = copy;
            return Ok(());
        }
        let strings = self.theany.cast::<StringList>();
        if let Some(live) = strings.get() {
            let source = live.array1();
            let copy =
                StringList::with_bounds(source.lower(), source.upper()).map_err(array_failure)?;
            for index in source.lower()..=source.upper() {
                let text = source.value(index).map_err(array_failure)?;
                if let Some(text) = text.get() {
                    copy.change_array1()
                        .set_value(index, Handle::new(HAsciiString::copied(text)))
                        .map_err(array_failure)?;
                }
            }
            drop(source);
            self.theany = Handle::new(copy).cast();
            return Ok(());
        }
        if let Some(copy) = copied_array::<Handle<StandardTransient>>(&self.theany)? {
            self.theany = copy;
        }
        Ok(())
    }
    /// `clear(kind = Undefined)`.
    pub fn clear(&mut self, kind: Kind) {
        self.thekind = kind;
        self.theint = 0;
        self.thereal = 0.0;
        self.theany.nullify();
    }
    /// `clear(int)`: the encoded kind.
    pub fn clear_int(&mut self, kind: i32) {
        self.clear(Kind::from_int(kind));
    }
    /// `set_derived`.
    pub fn set_derived(&mut self) {
        self.clear(Kind::DERIVED);
    }
    /// The select member, when the field is a select holding one.
    pub(super) fn select_member(&self) -> Handle<dyn StepDataSelectMember> {
        if self.thekind == Kind::SELECT {
            return self.theany.cast();
        }
        Handle::default()
    }
    /// `set_int(val)`: does not change the kind.
    pub fn set_int(&mut self, val: i32) {
        if let Some(member) = self.select_member().get() {
            member.set_integer(val);
            return;
        }
        if matches!(
            self.thekind,
            Kind::INTEGER | Kind::BOOLEAN | Kind::LOGICAL | Kind::ENUM
        ) {
            self.theint = val;
        }
    }
    /// `set_integer(val = 0)`.
    pub fn set_integer(&mut self, val: i32) {
        if let Some(member) = self.select_member().get() {
            member.set_integer(val);
            return;
        }
        self.clear(Kind::INTEGER);
        self.theint = val;
    }
    /// `set_boolean(val = false)`.
    pub fn set_boolean(&mut self, val: bool) {
        if let Some(member) = self.select_member().get() {
            member.set_boolean(val);
            return;
        }
        self.clear(Kind::BOOLEAN);
        self.theint = i32::from(val);
    }
    /// `set_logical(val = false)`.
    pub fn set_logical(&mut self, val: StepDataLogical) {
        if let Some(member) = self.select_member().get() {
            member.set_logical(val);
            return;
        }
        self.clear(Kind::LOGICAL);
        self.theint = val as i32;
    }
    /// `set_real(val = 0.0)`.
    pub fn set_real(&mut self, val: f64) {
        if let Some(member) = self.select_member().get() {
            member.set_real(val);
            return;
        }
        self.clear(Kind::REAL);
        self.thereal = val;
    }
    /// `set_string(val = "")`: does not redefine an `Enum` kind.
    pub fn set_string(&mut self, val: &[u8]) {
        if let Some(member) = self.select_member().get() {
            member.set_string(val);
            return;
        }
        if self.thekind != Kind::ENUM {
            self.clear(Kind::STRING);
        }
        self.theany = Handle::new(HAsciiString::new(val)).cast();
    }
    /// `set_enum(val = -1, text = "")`.
    pub fn set_enum(&mut self, val: i32, text: &[u8]) {
        self.clear(Kind::ENUM);
        self.set_int(val);
        if !text.is_empty() {
            self.set_string(text);
        }
    }
    /// `set_select_member(val)`: ignored when null.
    pub fn set_select_member(&mut self, val: &Handle<dyn StepDataSelectMember>) {
        if val.is_null() {
            return;
        }
        self.clear(Kind::SELECT);
        self.theany = val.cast();
    }
    /// `set_entity(val)`; a null `val` is the `set_entity()` predeclaration.
    pub fn set_entity(&mut self, val: &Handle<StandardTransient>) {
        self.clear(Kind::ENTITY);
        self.theany = val.clone();
    }
    /// `set_list(size, first = 1)`: declares a list typed by the last set.
    pub fn set_list(&mut self, size: i32, first: i32) -> Result<(), Failure> {
        self.theint = size;
        self.thereal = 0.0;
        let last = first + size - 1;
        self.theany = match self.thekind {
            Kind::INTEGER | Kind::BOOLEAN | Kind::LOGICAL => new_list::<i32>(first, last)?,
            Kind::REAL => new_list::<f64>(first, last)?,
            Kind::ENUM | Kind::STRING => new_list::<Handle<HAsciiString>>(first, last)?,
            _ => new_list::<Handle<StandardTransient>>(first, last)?,
        };
        if self.thekind == Kind::UNDEFINED {
            self.thekind = Kind::ANY;
        }
        self.thekind = self.thekind.combine(Kind::LIST);
        Ok(())
    }
    /// `set_list2(siz1, siz2, f1 = 1, f2 = 1)`. The handle is cleared
    /// before the select kind is read, so a select list is always of transients.
    pub fn set_list2(&mut self, siz1: i32, siz2: i32, f1: i32, f2: i32) -> Result<(), Failure> {
        self.theint = siz1;
        self.thereal = f64::from(siz2);
        let (r2, c2) = (f1 + siz1 - 1, f2 + siz2 - 1);
        self.theany = match self.thekind {
            Kind::INTEGER | Kind::BOOLEAN | Kind::LOGICAL => new_grid::<i32>(f1, r2, f2, c2)?,
            Kind::REAL => new_grid::<f64>(f1, r2, f2, c2)?,
            _ => new_grid::<Handle<StandardTransient>>(f1, r2, f2, c2)?,
        };
        if self.thekind == Kind::UNDEFINED {
            self.thekind = Kind::ANY;
        }
        self.thekind = self.thekind.combine(Kind::LIST2);
        Ok(())
    }
    /// `Set(val)`: a string, a member, an array (its size recorded), else an entity.
    pub fn set(&mut self, val: &Handle<StandardTransient>) {
        let kind = self.thekind;
        self.clear(Kind::UNDEFINED);
        self.theany = val.clone();
        if val.is_null() {
            return;
        }
        if val.cast::<HAsciiString>().is_some() {
            self.thekind = Kind::STRING;
            return;
        }
        if val.cast::<dyn StepDataSelectMember>().is_some() {
            self.thekind = Kind::SELECT;
            return;
        }
        if !self.set_list_value(kind, val) {
            self.set_grid_value(kind, val);
        }
    }
    fn set_list_value(&mut self, kind: Kind, val: &Handle<StandardTransient>) -> bool {
        let chosen = |or: Kind| if kind == Kind::UNDEFINED { or } else { kind };
        let (element, length) = if let Some(list) = val.cast::<HArray1<i32>>().get() {
            (chosen(Kind::INTEGER), list.array1().len())
        } else if let Some(list) = val.cast::<HArray1<f64>>().get() {
            (Kind::REAL, list.array1().len())
        } else if let Some(list) = val.cast::<StringList>().get() {
            (Kind::STRING, list.array1().len())
        } else if let Some(list) = val.cast::<TransientList>().get() {
            (chosen(Kind::ANY), list.array1().len())
        } else {
            return false;
        };
        self.thekind = element.combine(Kind::LIST);
        self.theint = length as i32;
        true
    }
    fn set_grid_value(&mut self, kind: Kind, val: &Handle<StandardTransient>) {
        let chosen = |or: Kind| if kind == Kind::UNDEFINED { or } else { kind };
        let (element, rows, columns) = if let Some(grid) = val.cast::<HArray2<i32>>().get() {
            let grid = grid.array2();
            (chosen(Kind::INTEGER), grid.nb_rows(), grid.nb_columns())
        } else if let Some(grid) = val.cast::<HArray2<f64>>().get() {
            let grid = grid.array2();
            (Kind::REAL, grid.nb_rows(), grid.nb_columns())
        } else if let Some(grid) = val.cast::<TransientGrid>().get() {
            let grid = grid.array2();
            (chosen(Kind::ANY), grid.nb_rows(), grid.nb_columns())
        } else {
            return;
        };
        self.thekind = element.combine(Kind::LIST2);
        self.theint = rows as i32;
        self.thereal = columns as f64;
    }
}

fn new_list<T: Default + Clone + Send + Sync + 'static>(
    first: i32,
    last: i32,
) -> Result<Handle<StandardTransient>, Failure>
where
    HArray1<T>: RuntimeTyped,
{
    let list = HArray1::<T>::with_bounds(first, last).map_err(array_failure)?;
    Ok(Handle::new(list).cast())
}

fn new_grid<T: Default + Clone + Send + Sync + 'static>(
    r1: i32,
    r2: i32,
    c1: i32,
    c2: i32,
) -> Result<Handle<StandardTransient>, Failure>
where
    HArray2<T>: RuntimeTyped,
{
    let grid = HArray2::<T>::with_bounds(r1, r2, c1, c2).map_err(array_failure)?;
    Ok(Handle::new(grid).cast())
}

/// A new array with the bounds and values of `any`, when it is an `HArray1<T>`.
fn copied_array<T: Default + Clone + Send + Sync + 'static>(
    any: &Handle<StandardTransient>,
) -> Result<Option<Handle<StandardTransient>>, Failure> {
    let list = any.cast::<HArray1<T>>();
    let Some(live) = list.get() else {
        return Ok(None);
    };
    let source = live.array1();
    let copy = HArray1::<T>::with_bounds(source.lower(), source.upper()).map_err(array_failure)?;
    for index in source.lower()..=source.upper() {
        let value = source.value(index).map_err(array_failure)?.clone();
        copy.change_array1()
            .set_value(index, value)
            .map_err(array_failure)?;
    }
    Ok(Some(Handle::new(copy).cast()))
}
