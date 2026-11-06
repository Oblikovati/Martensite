//! The queries of `StepDataField`. A rank out of the list bounds is the recorded
//! an out of range failure.
use super::field::{StringList, TransientGrid, TransientList, array_failure};
use super::{StepDataField, StepDataFieldKind as Kind, StepDataLogical, StepDataSelectMember};
use martensite_tkernel::{
    ncollection::{HArray1, HArray2},
    standard::{Failure, Handle, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};

fn list_item(
    any: &Handle<StandardTransient>,
    n1: i32,
) -> Result<Option<Handle<StandardTransient>>, Failure> {
    let list = any.cast::<TransientList>();
    let Some(live) = list.get() else {
        return Ok(None);
    };
    let array = live.array1();
    Ok(Some(array.value(n1).map_err(array_failure)?.clone()))
}

fn grid_item(
    any: &Handle<StandardTransient>,
    n1: i32,
    n2: i32,
) -> Result<Option<Handle<StandardTransient>>, Failure> {
    let grid = any.cast::<TransientGrid>();
    let Some(live) = grid.get() else {
        return Ok(None);
    };
    let array = live.array2();
    Ok(Some(array.value(n1, n2).map_err(array_failure)?.clone()))
}

fn text_of(text: &HAsciiString) -> AsciiString {
    text.string().clone()
}

impl StepDataField {
    fn is_list(&self) -> bool {
        self.thekind.arity_bits() == Kind::LIST.to_int()
    }
    fn is_grid(&self) -> bool {
        self.thekind.arity_bits() == Kind::LIST2.to_int()
    }
    /// `is_set(n1 = 1, n2 = 1)`.
    pub fn is_set(&self, n1: i32, n2: i32) -> Result<bool, Failure> {
        if self.thekind == Kind::UNDEFINED {
            return Ok(false);
        }
        if self.thekind == Kind::SELECT {
            return Ok(self
                .theany
                .cast::<dyn StepDataSelectMember>()
                .get()
                .is_some_and(|m| m.kind() != 0));
        }
        if self.is_list() {
            if let Some(item) = list_item(&self.theany, n1)? {
                return Ok(item.is_some());
            }
            if let Some(list) = self.theany.cast::<StringList>().get() {
                let array = list.array1();
                return Ok(array.value(n1).map_err(array_failure)?.is_some());
            }
        }
        if self.is_grid()
            && let Some(item) = grid_item(&self.theany, n1, n2)?
        {
            return Ok(item.is_some());
        }
        Ok(true)
    }
    /// `item_kind(n1 = 1, n2 = 1)`: the true kind of an item of an `any` list.
    pub fn item_kind(&self, n1: i32, n2: i32) -> Result<Kind, Failure> {
        if self.thekind.arity_bits() == 0 {
            return Ok(self.kind(true));
        }
        let kind = self.thekind.type_of();
        if kind != Kind::ANY {
            return Ok(kind);
        }
        let found = if self.is_list() {
            list_item(&self.theany, n1)?
        } else if self.is_grid() {
            grid_item(&self.theany, n1, n2)?
        } else {
            Some(Handle::default())
        };
        let Some(item) = found else {
            return Ok(kind);
        };
        if item.is_null() {
            return Ok(Kind::UNDEFINED);
        }
        if item.cast::<HAsciiString>().is_some() {
            return Ok(Kind::STRING);
        }
        let member = item.cast::<dyn StepDataSelectMember>();
        Ok(member
            .get()
            .map_or(Kind::ENTITY, |m| Kind::from_int(m.kind())))
    }
    /// `Kind(type = true)`: only the type, else the complete kind.
    pub fn kind(&self, type_only: bool) -> Kind {
        if !type_only {
            return self.thekind;
        }
        if let Some(member) = self.select_member().get() {
            return Kind::from_int(member.kind()).type_of();
        }
        self.thekind.type_of()
    }
    /// `arity`.
    pub fn arity(&self) -> i32 {
        self.thekind.arity()
    }
    /// `Length(index = 1)`.
    pub fn length(&self, index: i32) -> i32 {
        if self.is_list() {
            return self.theint;
        }
        if self.is_grid() {
            return if index == 2 {
                self.thereal as i32
            } else {
                self.theint
            };
        }
        0
    }
    /// `lower(index = 1)`.
    pub fn lower(&self, index: i32) -> i32 {
        if self.is_list() {
            return self.list_lower();
        }
        if self.is_grid() {
            return self.grid_lower(index);
        }
        0
    }
    fn list_lower(&self) -> i32 {
        if let Some(list) = self.theany.cast::<HArray1<i32>>().get() {
            return list.array1().lower();
        }
        if let Some(list) = self.theany.cast::<HArray1<f64>>().get() {
            return list.array1().lower();
        }
        if let Some(list) = self.theany.cast::<StringList>().get() {
            return list.array1().lower();
        }
        if let Some(list) = self.theany.cast::<TransientList>().get() {
            return list.array1().lower();
        }
        0
    }
    fn grid_lower(&self, index: i32) -> i32 {
        let bounds = if let Some(grid) = self.theany.cast::<HArray2<i32>>().get() {
            let grid = grid.array2();
            (grid.lower_row(), grid.lower_col())
        } else if let Some(grid) = self.theany.cast::<HArray2<f64>>().get() {
            let grid = grid.array2();
            (grid.lower_row(), grid.lower_col())
        } else if let Some(grid) = self.theany.cast::<TransientGrid>().get() {
            let grid = grid.array2();
            (grid.lower_row(), grid.lower_col())
        } else {
            return 0;
        };
        match index {
            1 => bounds.0,
            2 => bounds.1,
            _ => 0,
        }
    }
    /// `int`.
    pub fn int(&self) -> i32 {
        self.theint
    }
    /// The member at a rank of a list or a double list, null when it is not one.
    fn member_at(&self, n1: i32, n2: i32) -> Result<Handle<dyn StepDataSelectMember>, Failure> {
        let item = if self.is_list() {
            list_item(&self.theany, n1)?
        } else {
            grid_item(&self.theany, n1, n2)?
        };
        Ok(item.map_or_else(Handle::default, |item| item.cast()))
    }
    /// `integer(n1 = 1, n2 = 1)`.
    pub fn integer(&self, n1: i32, n2: i32) -> Result<i32, Failure> {
        if self.thekind.arity_bits() == 0 {
            let member = self.select_member();
            return Ok(member.get().map_or(self.theint, |m| m.int()));
        }
        if self.is_list()
            && let Some(list) = self.theany.cast::<HArray1<i32>>().get()
        {
            return list.array1().value(n1).copied().map_err(array_failure);
        }
        if self.is_grid()
            && let Some(grid) = self.theany.cast::<HArray2<i32>>().get()
        {
            return grid.array2().value(n1, n2).copied().map_err(array_failure);
        }
        if !self.is_list() && !self.is_grid() {
            return Ok(0);
        }
        Ok(self.member_at(n1, n2)?.get().map_or(0, |m| m.int()))
    }
    /// `boolean(n1 = 1, n2 = 1)`.
    pub fn boolean(&self, n1: i32, n2: i32) -> Result<bool, Failure> {
        Ok(self.integer(n1, n2)? > 0)
    }
    /// `logical(n1 = 1, n2 = 1)`.
    pub fn logical(&self, n1: i32, n2: i32) -> Result<StepDataLogical, Failure> {
        Ok(StepDataLogical::from_int(self.integer(n1, n2)?))
    }
    /// `real` of a scalar field (the member's when a select).
    pub fn scalar_real(&self) -> f64 {
        let member = self.select_member();
        member.get().map_or(self.thereal, |m| m.real())
    }
    /// `real(n1 = 1, n2 = 1)`. The member of a double list of
    /// transients gives its `int`.
    pub fn real(&self, n1: i32, n2: i32) -> Result<f64, Failure> {
        if self.thekind.arity_bits() == 0 {
            return Ok(self.scalar_real());
        }
        if self.is_list() {
            if let Some(list) = self.theany.cast::<HArray1<f64>>().get() {
                return list.array1().value(n1).copied().map_err(array_failure);
            }
            return Ok(self.member_at(n1, n2)?.get().map_or(0.0, |m| m.real()));
        }
        if self.is_grid() {
            if let Some(grid) = self.theany.cast::<HArray2<f64>>().get() {
                return grid.array2().value(n1, n2).copied().map_err(array_failure);
            }
            let member = self.member_at(n1, n2)?;
            return Ok(member.get().map_or(0.0, |m| f64::from(m.int())));
        }
        Ok(0.0)
    }
    /// `string` of a scalar field.
    pub fn scalar_string(&self) -> AsciiString {
        if self.thekind == Kind::STRING || self.thekind == Kind::ENUM {
            let text = self.theany.cast::<HAsciiString>();
            return text.get().map(text_of).unwrap_or_default();
        }
        let member = self.select_member();
        member.get().map(|m| m.string()).unwrap_or_default()
    }
    /// `string(n1 = 1, n2 = 1)`.
    pub fn string(&self, n1: i32, n2: i32) -> Result<AsciiString, Failure> {
        let scalar_kinds = [Kind::STRING, Kind::ENUM, Kind::SELECT];
        if scalar_kinds.contains(&self.thekind) {
            let select_without_member =
                self.thekind == Kind::SELECT && self.select_member().is_null();
            if !select_without_member {
                return Ok(self.scalar_string());
            }
        }
        if self.is_list()
            && let Some(list) = self.theany.cast::<StringList>().get()
        {
            let array = list.array1();
            let text = array.value(n1).map_err(array_failure)?;
            return Ok(text.get().map(text_of).unwrap_or_default());
        }
        let item = if self.is_list() {
            list_item(&self.theany, n1)?
        } else if self.is_grid() {
            grid_item(&self.theany, n1, n2)?
        } else {
            None
        };
        let Some(item) = item else {
            return Ok(AsciiString::default());
        };
        if let Some(text) = item.cast::<HAsciiString>().get() {
            return Ok(text_of(text));
        }
        let member = item.cast::<dyn StepDataSelectMember>();
        Ok(member.get().map(|m| m.string()).unwrap_or_default())
    }
    /// `Enum(n1 = 1, n2 = 1)`.
    pub fn enum_value(&self, n1: i32, n2: i32) -> Result<i32, Failure> {
        self.integer(n1, n2)
    }
    /// `enum_text(n1 = 1, n2 = 1)`.
    pub fn enum_text(&self, n1: i32, n2: i32) -> Result<AsciiString, Failure> {
        self.string(n1, n2)
    }
    /// `entity(n1 = 1, n2 = 1)`: null for a member or a string of a list.
    pub fn entity(&self, n1: i32, n2: i32) -> Result<Handle<StandardTransient>, Failure> {
        if self.thekind.arity_bits() == 0 {
            if self.thekind == Kind::ENTITY {
                return Ok(self.theany.clone());
            }
            return Ok(Handle::default());
        }
        let item = if self.is_list() {
            list_item(&self.theany, n1)?
        } else if self.is_grid() {
            grid_item(&self.theany, n1, n2)?
        } else {
            None
        };
        let Some(item) = item else {
            return Ok(Handle::default());
        };
        let immediate = item.cast::<dyn StepDataSelectMember>().is_some()
            || item.cast::<HAsciiString>().is_some();
        Ok(if immediate { Handle::default() } else { item })
    }
    /// `Transient`.
    pub fn transient(&self) -> Handle<StandardTransient> {
        self.theany.clone()
    }
}
