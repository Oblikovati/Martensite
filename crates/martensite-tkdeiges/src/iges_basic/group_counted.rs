//! A group made with room for its entities.
use super::IgesBasicGroup;
use crate::iges_data::iges_new_array1;
use martensite_tkernel::standard::Failure;

impl IgesBasicGroup {
    /// A group with room for `nb` entities, all null; no room when `nb` is not positive.
    ///
    /// # Errors
    /// A failure when the array of `nb` entities can't be made.
    pub fn with_nb(nb: i32) -> Result<Self, Failure> {
        let group = Self::default();
        if nb > 0 {
            group.state.write().entities = iges_new_array1(1, nb)?;
        }
        Ok(group)
    }
}

#[cfg(test)]
mod tests {
    use super::IgesBasicGroup;

    #[test]
    fn room_is_made_only_for_a_positive_count() {
        assert_eq!(IgesBasicGroup::with_nb(3).expect("room").nb_entities(), 3);
        assert_eq!(IgesBasicGroup::with_nb(0).expect("room").nb_entities(), 0);
    }

    #[test]
    fn a_counted_group_is_an_unordered_group() {
        let group = IgesBasicGroup::with_nb(1).expect("room");
        assert_eq!((group.type_number(), group.form_number()), (402, 1));
    }
}
