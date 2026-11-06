//! The designation of the parameters one read of an entity's record covers.
use martensite_tkernel::standard::Failure;
use martensite_tkxsbase::interface::interface_error;

/// A run of `count` items of `item_size` parameters each, from a start parameter, and the term of
/// each item a read takes.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataParamCursor;
/// let cursor = IgesDataParamCursor::with_items(3, 2, 3);
/// assert_eq!((cursor.start(), cursor.limit(), cursor.count()), (3, 9, 2));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct IgesDataParamCursor {
    thestart: i32,
    thelimit: i32,
    thecount: i32,
    theisize: i32,
    theoffst: i32,
    thetsize: i32,
    theadv: bool,
}

impl IgesDataParamCursor {
    /// The one parameter `num`.
    pub fn new(num: i32) -> Self {
        Self::with_items(num, 1, 1)
    }
    /// `nb` items of `size` parameters from `num`.
    pub fn with_items(num: i32, nb: i32, size: i32) -> Self {
        Self {
            thestart: num,
            thelimit: num + nb * size,
            thecount: nb,
            theisize: size,
            theoffst: 0,
            thetsize: size,
            theadv: true,
        }
    }
    /// Takes the next term of an item, `size` parameters long; `autoadv` advances the reader only
    /// when this term ends the item.
    ///
    /// # Errors
    /// An interface failure when the term goes beyond the item.
    pub fn set_term(&mut self, size: i32, autoadv: bool) -> Result<(), Failure> {
        self.theoffst += self.thetsize;
        self.thetsize = size;
        if autoadv {
            self.theadv = self.theoffst + self.thetsize == self.theisize;
        }
        if self.theoffst + self.thetsize > self.theisize {
            return Err(interface_error(&format!(
                "IGES param cursor: term of size {size} at offset {} overpasses the item size {}",
                self.theoffst, self.theisize
            )));
        }
        Ok(())
    }
    /// Takes the next term as one parameter.
    ///
    /// # Errors
    /// An interface failure when the term goes beyond the item.
    pub fn set_one(&mut self, autoadv: bool) -> Result<(), Failure> {
        self.set_term(1, autoadv)
    }
    /// Takes the next term as two coordinates.
    ///
    /// # Errors
    /// An interface failure when the term goes beyond the item.
    pub fn set_xy(&mut self, autoadv: bool) -> Result<(), Failure> {
        self.set_term(2, autoadv)
    }
    /// Takes the next term as three coordinates.
    ///
    /// # Errors
    /// An interface failure when the term goes beyond the item.
    pub fn set_xyz(&mut self, autoadv: bool) -> Result<(), Failure> {
        self.set_term(3, autoadv)
    }
    /// Sets whether the read advances the reader's current number.
    pub fn set_advance(&mut self, advance: bool) {
        self.theadv = advance;
    }
    /// The first parameter.
    pub fn start(&self) -> i32 {
        self.thestart
    }
    /// The parameter after the last one.
    pub fn limit(&self) -> i32 {
        self.thelimit
    }
    /// The number of items.
    pub fn count(&self) -> i32 {
        self.thecount
    }
    /// The size of an item.
    pub fn item_size(&self) -> i32 {
        self.theisize
    }
    /// The size of the current term.
    pub fn term_size(&self) -> i32 {
        self.thetsize
    }
    /// The offset of the current term in an item.
    pub fn offset(&self) -> i32 {
        self.theoffst
    }
    /// Whether the read advances the reader's current number.
    pub fn advance(&self) -> bool {
        self.theadv
    }
}

impl From<i32> for IgesDataParamCursor {
    fn from(num: i32) -> Self {
        Self::new(num)
    }
}
