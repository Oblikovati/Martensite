//! Borrow the actual member storage shared by the four IGES group classes.
use super::{
    IgesBasicGroup, IgesBasicGroupWithoutBackP, IgesBasicOrderedGroup,
    IgesBasicOrderedGroupWithoutBackP,
};
use crate::iges_data::IgesDataEntityKind;

pub(crate) trait IgesBasicGroupSource: IgesDataEntityKind {
    fn group_base(&self) -> &IgesBasicGroup;
}

impl IgesBasicGroupSource for IgesBasicGroup {
    fn group_base(&self) -> &IgesBasicGroup {
        self
    }
}

macro_rules! impl_derived_group_source {
    ($($class:ty),+ $(,)?) => {
        $(impl IgesBasicGroupSource for $class {
            fn group_base(&self) -> &IgesBasicGroup {
                &self.parent
            }
        })+
    };
}

impl_derived_group_source!(
    IgesBasicGroupWithoutBackP,
    IgesBasicOrderedGroup,
    IgesBasicOrderedGroupWithoutBackP,
);

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::standard::{Handle, project_interface};

    #[test]
    fn each_group_class_exposes_its_original_members() {
        let groups: [Handle<dyn IgesDataEntityKind>; 4] = [
            Handle::new(IgesBasicGroup::new()).into_upcast(),
            Handle::new(IgesBasicGroupWithoutBackP::new()).into_upcast(),
            Handle::new(IgesBasicOrderedGroup::new()).into_upcast(),
            Handle::new(IgesBasicOrderedGroupWithoutBackP::new()).into_upcast(),
        ];
        for group in groups {
            let source =
                project_interface::<dyn IgesBasicGroupSource>(group.get().unwrap()).unwrap();
            source.group_base().set_nb(2).unwrap();
            assert_eq!(source.group_base().nb_entities(), 2);
        }
    }
}
