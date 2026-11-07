//! The location of entities in a model: the parents they depend on, and the result of the
//! transformations along them.
use super::{
    IgesDataGeneralModule, IgesDataIgesEntity, IgesDataIgesModel, IgesDataProtocolOps,
    IgesDataSingleParentEntity, IgesDataTransfEntity, iges_entity_of, iges_live,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, RuntimeTyped, SharedState, StandardTransient, exceptions},
};
use martensite_tkmath::gp::{
    Axis3, Direction3, GeneralTransform3, Point3, Transform3, Vector3, Xyz,
};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceGeneralLib, InterfaceModelEntities, InterfaceProtocol,
};

const TYPE_FOR_ASSOC: i32 = 402;

struct ToolLocationFields {
    theprec: f64,
    therefs: Vec<i32>,
    theassocs: Vec<i32>,
}

/// The dependences of the entities of a model, computed once.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataToolLocation;
/// use martensite_tkmath::gp::{GeneralTransform3, Transform3};
/// let mut result = Transform3::default();
/// assert!(IgesDataToolLocation::convert_location(1e-5, &GeneralTransform3::default(), &mut result, 1.0).unwrap());
/// ```
pub struct IgesDataToolLocation {
    transient: StandardTransient,
    themodel: Handle<IgesDataIgesModel>,
    thelib: SharedState<InterfaceGeneralLib>,
    state: SharedState<ToolLocationFields>,
}
impl_transient!(
    IgesDataToolLocation,
    "IgesDataToolLocation",
    StandardTransient,
    transient
);

impl IgesDataToolLocation {
    /// The dependences of a model, by the modules of a protocol.
    ///
    /// # Errors
    /// A failure of a module or of a single parent entity.
    pub fn new(
        amodel: &Handle<IgesDataIgesModel>,
        protocol: &Handle<dyn IgesDataProtocolOps>,
    ) -> Result<Self, Failure> {
        let places = iges_live(amodel, "tool location")?.nb_entities().max(0) as usize + 1;
        let general: Handle<dyn InterfaceProtocol> = protocol.cast();
        let tool = Self {
            transient: StandardTransient::default(),
            themodel: amodel.clone(),
            thelib: SharedState::new(InterfaceGeneralLib::new(&general)),
            state: SharedState::new(ToolLocationFields {
                theprec: 1.0e-5,
                therefs: vec![0; places],
                theassocs: vec![0; places],
            }),
        };
        tool.load()?;
        Ok(tool)
    }
    fn number_of<T: ?Sized + RuntimeTyped>(&self, ent: &Handle<T>) -> i32 {
        self.themodel
            .get()
            .map_or(0, |model| model.number(&ent.cast()))
    }
    /// Computes the dependences: of every entity but the transformations and the associativities
    /// other than the single parents.
    ///
    /// # Errors
    /// A failure of a module or of a single parent entity.
    pub fn load(&self) -> Result<(), Failure> {
        let model = iges_live(&self.themodel, "tool location")?;
        for i in 1..=model.nb_entities() {
            let ent = model.entity(i);
            if ent.is_kind(<dyn IgesDataTransfEntity>::static_type()) {
                continue;
            }
            let assoc = ent.cast::<dyn IgesDataSingleParentEntity>();
            if let Some(link) = assoc.get() {
                let parent = link.single_parent();
                for j in 1..=link.nb_children()? {
                    self.set_parent_assoc(&parent, &link.child(j)?);
                }
                continue;
            }
            if ent
                .get()
                .is_some_and(|live| live.type_number() == TYPE_FOR_ASSOC)
            {
                continue;
            }
            self.set_own_as_dependent(&ent)?;
        }
        Ok(())
    }
    /// Sets the precision of [`Self::analyse_location`].
    pub fn set_precision(&self, prec: f64) {
        self.state.write().theprec = prec;
    }
    /// Records `parent` as referring to `child`; a second parent makes the location ambiguous.
    pub fn set_reference(
        &self,
        parent: &Handle<IgesDataIgesEntity>,
        child: &Handle<IgesDataIgesEntity>,
    ) {
        self.record(parent, child, false);
    }
    /// Records `parent` as the parent of `child` by an associativity.
    pub fn set_parent_assoc(
        &self,
        parent: &Handle<IgesDataIgesEntity>,
        child: &Handle<IgesDataIgesEntity>,
    ) {
        self.record(parent, child, true);
    }
    fn record(
        &self,
        parent: &Handle<IgesDataIgesEntity>,
        child: &Handle<IgesDataIgesEntity>,
        assoc: bool,
    ) {
        let (np, nc) = (self.number_of(parent), self.number_of(child));
        if np == 0 || nc == 0 {
            return;
        }
        let mut state = self.state.write();
        let list = if assoc {
            &mut state.theassocs
        } else {
            &mut state.therefs
        };
        let slot = &mut list[nc as usize];
        *slot = if *slot > 0 { -1 } else { np };
    }
    /// Forgets the dependences of an entity.
    pub fn reset_dependences(&self, child: &Handle<IgesDataIgesEntity>) {
        let nc = self.number_of(child);
        if nc == 0 {
            return;
        }
        let mut state = self.state.write();
        state.therefs[nc as usize] = 0;
        state.theassocs[nc as usize] = 0;
    }
    /// Records the entities the own parameters of `ent` refer to as depending on it.
    ///
    /// # Errors
    /// A failure of the module of the entity.
    pub fn set_own_as_dependent(&self, ent: &Handle<IgesDataIgesEntity>) -> Result<(), Failure> {
        let selected = self.thelib.read().select(&ent.cast());
        let Some((gmodule, cn)) = selected else {
            return Ok(());
        };
        let module = gmodule.cast::<dyn IgesDataGeneralModule>();
        let mut list = InterfaceEntityIterator::new();
        iges_live(&module, "tool location")?.own_shared_case(cn, ent, &mut list)?;
        list.start();
        while list.more() {
            self.set_reference(ent, &iges_entity_of(&list.value()?));
            list.next();
        }
        Ok(())
    }
    /// Whether an entity is a transformation.
    pub fn is_transf(&self, ent: &Handle<IgesDataIgesEntity>) -> bool {
        ent.is_kind(<dyn IgesDataTransfEntity>::static_type())
    }
    /// Whether an entity is an associativity.
    pub fn is_associativity(&self, ent: &Handle<IgesDataIgesEntity>) -> bool {
        ent.get()
            .is_some_and(|live| live.type_number() == TYPE_FOR_ASSOC)
    }
    /// Whether an entity has a transformation of its own.
    pub fn has_transf(&self, ent: &Handle<IgesDataIgesEntity>) -> bool {
        ent.get().is_some_and(IgesDataIgesEntity::has_transf)
    }
    /// The location an entity gives by itself.
    ///
    /// # Errors
    /// A failure of the transformation.
    pub fn explicit_location(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
    ) -> Result<GeneralTransform3, Failure> {
        iges_live(ent, "tool location")?.location()
    }
    /// The rank of the reference and of the associativity parent of an entity.
    fn links(&self, num: i32) -> (i32, i32) {
        let state = self.state.read();
        (state.therefs[num as usize], state.theassocs[num as usize])
    }
    /// Whether an entity has several parents, of one way or both.
    pub fn is_ambiguous(&self, ent: &Handle<IgesDataIgesEntity>) -> bool {
        let num = self.number_of(ent);
        if num == 0 {
            return false;
        }
        let (reference, assoc) = self.links(num);
        reference < 0 || assoc < 0 || (reference != 0 && assoc != 0)
    }
    fn unambiguous_links(&self, num: i32, member: &str) -> Result<(i32, i32), Failure> {
        let (reference, assoc) = self.links(num);
        if reference < 0 || assoc < 0 || (reference != 0 && assoc != 0) {
            return Err(exceptions::domain_error(&format!(
                "IGES tool location: {member}: entity {num} has parents ({reference}, {assoc}); expected one"
            )));
        }
        Ok((reference, assoc))
    }
    /// Whether an entity has a parent.
    ///
    /// # Errors
    /// A domain failure when the entity is ambiguous.
    pub fn has_parent(&self, ent: &Handle<IgesDataIgesEntity>) -> Result<bool, Failure> {
        let num = self.number_of(ent);
        if num == 0 {
            return Ok(false);
        }
        let (reference, assoc) = self.unambiguous_links(num, "HasParent")?;
        Ok(reference != 0 || assoc != 0)
    }
    /// The parent of an entity, null when none.
    ///
    /// # Errors
    /// A domain failure when the entity is ambiguous.
    pub fn parent(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let num = self.number_of(ent);
        if num == 0 {
            return Ok(Handle::default());
        }
        let (reference, assoc) = self.unambiguous_links(num, "Parent")?;
        let rank = if assoc != 0 { assoc } else { reference };
        if rank == 0 {
            return Ok(Handle::default());
        }
        Ok(iges_live(&self.themodel, "tool location")?.entity(rank))
    }
    /// Whether the parent of an entity comes from an associativity.
    ///
    /// # Errors
    /// A domain failure when the entity is ambiguous.
    pub fn has_parent_by_associativity(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
    ) -> Result<bool, Failure> {
        let num = self.number_of(ent);
        if num == 0 {
            return Ok(false);
        }
        Ok(self.unambiguous_links(num, "HasParentByAssociativity")?.1 != 0)
    }
    /// The location the parents of an entity give, the identity without one.
    ///
    /// # Errors
    /// A domain failure when an entity is ambiguous, a failure of a transformation.
    pub fn parent_location(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
    ) -> Result<GeneralTransform3, Failure> {
        let parent = self.parent(ent)?;
        if parent.is_null() {
            return Ok(GeneralTransform3::default());
        }
        self.effective_location(&parent)
    }
    /// The location of an entity combined with the one of its parents.
    ///
    /// # Errors
    /// A domain failure when an entity is ambiguous, a failure of a transformation.
    pub fn effective_location(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
    ) -> Result<GeneralTransform3, Failure> {
        let mut locat = self.explicit_location(ent)?;
        locat.pre_multiply(self.parent_location(ent)?);
        Ok(locat)
    }
    /// [`Self::convert_location`] at the precision of this tool, without changing the unit.
    ///
    /// # Errors
    /// A failure of the construction of the transformation.
    pub fn analyse_location(
        &self,
        loc: &GeneralTransform3,
        result: &mut Transform3,
    ) -> Result<bool, Failure> {
        let prec = self.state.read().theprec;
        Self::convert_location(prec, loc, result, 1.0)
    }
    /// Converts a general transformation to a similarity, when it is one at the precision `prec`;
    /// the translation is scaled by `unit`. False when it is not one.
    ///
    /// # Errors
    /// A failure of the construction of the transformation.
    pub fn convert_location(
        prec: f64,
        loc: &GeneralTransform3,
        result: &mut Transform3,
        unit: f64,
    ) -> Result<bool, Failure> {
        *result = Transform3::default();
        let row = |index: i32| -> Result<Xyz, Failure> {
            Ok(Xyz::new(
                loc.value(index, 1)?,
                loc.value(index, 2)?,
                loc.value(index, 3)?,
            ))
        };
        let (v1, v2, v3) = (row(1)?, row(2)?, row(3)?);
        let (m1, m2, m3) = (v1.modulus(), v2.modulus(), v3.modulus());
        if m1 < prec || m2 < prec || m3 < prec {
            return Ok(false);
        }
        let mm = (m1 + m2 + m3) / 3.0;
        if (m1 - mm).abs() > prec * mm || (m2 - mm).abs() > prec * mm || (m3 - mm).abs() > prec * mm
        {
            return Ok(false);
        }
        let (v1, v2, v3) = (v1 / m1, v2 / m2, v3 / m3);
        if v1.dot(v2).abs() > prec || v2.dot(v3).abs() > prec || v3.dot(v1).abs() > prec {
            return Ok(false);
        }
        if (mm - 1.0).abs() > prec {
            *result = Transform3::scaling(Point3::new(0.0, 0.0, 0.0), mm)?;
        }
        let mut tp = loc.translation_part();
        if unit != 1.0 {
            tp *= unit;
        }
        if tp.x() != 0.0 || tp.y() != 0.0 || tp.z() != 0.0 {
            result.set_translation_part(Vector3::from(tp));
        }
        let identity = Xyz::new(1.0, 0.0, 0.0) == v1
            && Xyz::new(0.0, 1.0, 0.0) == v2
            && Xyz::new(0.0, 0.0, 1.0) == v3;
        if !identity {
            *result *= rotation_of(v1, v2, v3)?;
        }
        Ok(true)
    }
}

/// The transformation that gives the coordinates in the frame of three orthonormal rows.
fn rotation_of(v1: Xyz, v2: Xyz, v3: Xyz) -> Result<Transform3, Failure> {
    let d1 = Direction3::new(v1.x(), v1.y(), v1.z())?;
    let d3 = Direction3::new(v3.x(), v3.y(), v3.z())?;
    let mut axes = Axis3::new(Point3::new(0.0, 0.0, 0.0), d3, d1)?;
    if v3.crossed(v1).dot(v2) < 0.0 {
        axes.y_reverse();
    }
    let mut transf = Transform3::default();
    transf.set_transformation_to(axes);
    Ok(transf)
}
