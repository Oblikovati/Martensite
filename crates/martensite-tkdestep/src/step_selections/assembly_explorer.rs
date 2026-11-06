//! `STEPSelectionsAssemblyExplorer`: the assembly trees of a graph.
use super::assembly_links::STEPSelectionsAssemblyLinks;
use super::{STEPSelectionsAssemblyComponent, STEPSelectionsAssemblyLink, entities_of};
use crate::step_basic::StepBasicProductDefinition;
use crate::step_entity::step_live;
use crate::step_repr::{
    StepReprMappedItem, StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape,
    StepReprPropertyDefinition,
};
use crate::step_shape::{
    StepShapeContextDependentShapeRepresentation, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::interface::InterfaceGraph;
use std::io::{self, Write};

const WHERE: &str = "STEPSelectionsAssemblyExplorer";

/// Assembly explorer.
pub struct STEPSelectionsAssemblyExplorer {
    my_graph: InterfaceGraph,
    my_roots: Vec<Handle<STEPSelectionsAssemblyComponent>>,
    /// `myMap`: keyed by the shape_definition_representation, in the order they were added.
    my_map: Vec<(
        Handle<StepShapeShapeDefinitionRepresentation>,
        Handle<STEPSelectionsAssemblyComponent>,
    )>,
}

/// The text of a label; empty for a null one.
fn text_of(label: &Handle<martensite_tkernel::tcollection::HAsciiString>) -> String {
    label.get().map_or_else(String::new, |live| {
        String::from_utf8_lossy(live.string().as_bytes()).into_owned()
    })
}

/// `get_product_name`.
fn product_name(sdr: &Handle<StepShapeShapeDefinitionRepresentation>) -> Result<String, Failure> {
    let definition = step_live(sdr, WHERE)?.definition().property_definition();
    let Some(live) = definition.get() else {
        return Ok(String::new());
    };
    let product_definition = live.definition().product_definition();
    let Some(live_pd) = product_definition.get() else {
        return Ok(String::new());
    };
    let formation = live_pd.formation();
    let Some(live_formation) = formation.get() else {
        return Ok(String::new());
    };
    let product = live_formation.of_product();
    let name = step_live(&product, WHERE)?.name();
    let text = step_live(&name, WHERE)?.string().clone();
    Ok(String::from_utf8_lossy(text.as_bytes()).into_owned())
}

impl STEPSelectionsAssemblyExplorer {
    /// `STEPSelectionsAssemblyExplorer(G)`.
    pub fn new(graph: &InterfaceGraph) -> Result<Self, Failure> {
        let mut made = Self {
            my_graph: graph.clone(),
            my_roots: Vec::new(),
            my_map: Vec::new(),
        };
        made.init(graph)?;
        Ok(made)
    }
    /// `find_item_with_nauo(nauo)`: the context_dependent_shape_representation or the mapped_item of
    /// the occurrence; null when none.
    pub fn find_item_with_nauo(
        &self,
        nauo: &Handle<StepReprNextAssemblyUsageOccurrence>,
    ) -> Result<Handle<StandardTransient>, Failure> {
        let pds = entities_of(self.my_graph.sharings(&nauo.cast())?)?
            .into_iter()
            .find(|ent| ent.cast::<StepReprProductDefinitionShape>().is_some());
        let Some(pds) = pds else {
            return Ok(Handle::default());
        };
        let mut sdr = Handle::<StepShapeShapeDefinitionRepresentation>::default();
        for sharing in entities_of(self.my_graph.sharings(&pds)?)? {
            if sharing
                .cast::<StepShapeContextDependentShapeRepresentation>()
                .is_some()
            {
                return Ok(sharing);
            }
            sdr = sharing.cast();
            if sdr.is_some() {
                break;
            }
        }
        let Some(live) = sdr.get() else {
            return Ok(Handle::default());
        };
        let rep: Handle<StepShapeShapeRepresentation> = live.used_representation().cast();
        let Some(live_rep) = rep.get() else {
            return Ok(Handle::default());
        };
        for rank in 1..=live_rep.nb_items() {
            let item = live_rep.items_value(rank)?;
            if item.cast::<StepReprMappedItem>().is_some() {
                return Ok(item.cast());
            }
        }
        Ok(Handle::default())
    }
    /// `find_sdr_with_product(product)`.
    pub fn find_sdr_with_product(
        &self,
        product: &Handle<StepBasicProductDefinition>,
    ) -> Result<Handle<StepShapeShapeDefinitionRepresentation>, Failure> {
        for sharing in entities_of(self.my_graph.sharings(&product.cast())?)? {
            if sharing.cast::<StepReprPropertyDefinition>().is_null() {
                continue;
            }
            for sub in entities_of(self.my_graph.sharings(&sharing)?)? {
                let sdr: Handle<StepShapeShapeDefinitionRepresentation> = sub.cast();
                if sdr.is_some() {
                    return Ok(sdr);
                }
            }
        }
        Ok(Handle::default())
    }
    /// The component of a sub-part: the known one, else a new one with its tree filled.
    fn component_of(
        &mut self,
        sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
    ) -> Result<Handle<STEPSelectionsAssemblyComponent>, Failure> {
        if let Some((_, known)) = self.my_map.iter().find(|(key, _)| key == sdr) {
            return Ok(known.clone());
        }
        let sublist: Handle<STEPSelectionsAssemblyLinks> = Handle::new(HSequenceDefault::default());
        let component = Handle::new(STEPSelectionsAssemblyComponent::new(sdr, &sublist));
        self.fill_list_with_graph(&component)?;
        self.my_map.push((sdr.clone(), component.clone()));
        Ok(component)
    }
    /// `fill_list_with_graph(cmp)`.
    pub fn fill_list_with_graph(
        &mut self,
        component: &Handle<STEPSelectionsAssemblyComponent>,
    ) -> Result<(), Failure> {
        let live = step_live(component, WHERE)?;
        let sdr = live.get_sdr();
        let list = live.get_list();
        let pds: Handle<StepReprProductDefinitionShape> = step_live(&sdr, WHERE)?
            .definition()
            .property_definition()
            .cast();
        let Some(live_pds) = pds.get() else {
            return Ok(());
        };
        let pd = live_pds.definition().product_definition();
        if pd.is_null() {
            return Ok(());
        }
        for sharing in entities_of(self.my_graph.sharings(&pd.cast())?)? {
            let nauo: Handle<StepReprNextAssemblyUsageOccurrence> = sharing.cast();
            let Some(live_nauo) = nauo.get() else {
                continue;
            };
            if pd != live_nauo.relating_product_definition() {
                continue;
            }
            let link = Handle::new(STEPSelectionsAssemblyLink::default());
            link.set_nauo(&nauo);
            link.set_item(&self.find_item_with_nauo(&nauo)?);
            let related = live_nauo.related_product_definition();
            if related.is_null() {
                continue;
            }
            let sub_sdr = self.find_sdr_with_product(&related)?;
            if sub_sdr.is_null() {
                continue;
            }
            link.set_component(&self.component_of(&sub_sdr)?);
            step_live(&list, WHERE)?.append(link);
        }
        Ok(())
    }
    /// `init(G)`.
    pub fn init(&mut self, graph: &InterfaceGraph) -> Result<(), Failure> {
        self.my_graph = graph.clone();
        self.my_roots.clear();
        self.my_map.clear();
        for root in entities_of(self.my_graph.root_entities()?)? {
            let sdr: Handle<StepShapeShapeDefinitionRepresentation> = root.cast();
            if sdr.is_null() {
                continue;
            }
            let component = Handle::new(STEPSelectionsAssemblyComponent::default());
            component.set_sdr(&sdr);
            component.set_list(&Handle::new(HSequenceDefault::default()));
            self.fill_list_with_graph(&component)?;
            self.my_roots.push(component);
        }
        Ok(())
    }
    /// `nb_assemblies`.
    pub fn nb_assemblies(&self) -> i32 {
        self.my_roots.len() as i32
    }
    /// `Root(rank)`.
    pub fn root(&self, rank: i32) -> Option<Handle<STEPSelectionsAssemblyComponent>> {
        usize::try_from(rank - 1)
            .ok()
            .and_then(|index| self.my_roots.get(index).cloned())
    }
    /// `print_sub_assembly`.
    fn print_sub_assembly(
        &self,
        os: &mut dyn Write,
        component: &Handle<STEPSelectionsAssemblyComponent>,
        level: usize,
    ) -> io::Result<()> {
        let to_io = |failure: Failure| {
            io::Error::other(String::from_utf8_lossy(failure.what()).into_owned())
        };
        let model = self.my_graph.model();
        let live = step_live(component, WHERE).map_err(to_io)?;
        let sdr = live.get_sdr();
        let label = model.string_label(&sdr.cast());
        let name = product_name(&sdr).map_err(to_io)?;
        writeln!(os, "SDR: {}\tProduct: {name}", text_of(&label))?;
        let links = live.get_list();
        let links: Vec<_> = links
            .get()
            .map(|l| l.sequence(|items| items.iter().cloned().collect()))
            .unwrap_or_default();
        for link in links {
            let live_link = step_live(&link, WHERE).map_err(to_io)?;
            let tabs = "\t".repeat(level + 1);
            let nauo = model.string_label(&live_link.get_nauo().cast());
            write!(os, "{tabs}NAUO :{};\t", text_of(&nauo))?;
            let item = live_link.get_item();
            if item.cast::<StepReprMappedItem>().is_some() {
                write!(os, "MI ")?;
            } else if item
                .cast::<StepShapeContextDependentShapeRepresentation>()
                .is_some()
            {
                write!(os, "CDSR ")?;
            } else {
                write!(os, "UNKNOWN LINK!!!")?;
            }
            self.print_sub_assembly(os, &live_link.get_component(), level + 1)?;
        }
        Ok(())
    }
    /// `dump(os)`.
    pub fn dump(&self, os: &mut dyn Write) -> io::Result<()> {
        for (index, root) in self.my_roots.iter().enumerate() {
            writeln!(os, "Assembly N: {}\n", index + 1)?;
            self.print_sub_assembly(os, root, 0)?;
        }
        Ok(())
    }
}

type HSequenceDefault = STEPSelectionsAssemblyLinks;
