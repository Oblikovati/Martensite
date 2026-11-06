use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn compact_v4_shader_interfaces_fit_the_builtin_inclusive_location_cap() {
    let geometry = include_bytes!(
        "../../shaders/world-clip-grid12-v2.triangle-grid-v4-compact-passthrough.geom.spv"
    );
    let fragment = include_bytes!(
        "../../shaders/world-clip-grid12-v2.triangle-grid-v4-compact.unsnapped-v3.lighting.clipped.frag.spv"
    );
    assert_compact_interface(geometry, 3);
    assert_compact_interface(fragment, 1);
    assert_component_locations_do_not_overlap(geometry, 3);
    assert_component_locations_do_not_overlap(fragment, 1);
    assert_eq!(maximum_location_end(geometry, 3), 27);
    assert_eq!(maximum_location_end(fragment, 1), 27);
    assert_eq!(builtin_components(geometry, 3), 8);
    assert_eq!(user_components(geometry, 3), 108);
    assert_eq!(required_limits_for(true, true).total_output_components, 348);
    assert_eq!(geometry_output_component_limit(), 120);
}

fn assert_compact_interface(binary: &[u8], storage: u32) {
    let decorations = interface_decorations(binary, storage);
    assert!(decorations.iter().all(|(location, _)| *location <= 27));
    for required in [(13, 0), (14, 0), (10, 3), (11, 3), (12, 3), (22, 3)] {
        assert!(
            decorations.contains(&required),
            "missing {required:?}: {decorations:?}"
        );
    }
}

fn assert_component_locations_do_not_overlap(binary: &[u8], storage: u32) {
    let reflected = reflect_types(&spirv_words(binary), storage);
    assert!(component_locations(&reflected).is_ok());
}

fn component_locations(
    reflected: &InterfaceTypes,
) -> Result<BTreeMap<(u32, u32), u32>, (u32, u32)> {
    let mut occupied = BTreeMap::new();
    for (id, pointer) in &reflected.variables {
        let Some((location, component)) = reflected.decorations.get(id) else {
            continue;
        };
        reserve_type_components(
            reflected.pointers[pointer],
            *location,
            *component,
            id,
            reflected,
            &mut occupied,
        )?;
    }
    Ok(occupied)
}

fn reserve_type_components(
    type_id: u32,
    location: u32,
    component: u32,
    variable: &u32,
    reflected: &InterfaceTypes,
    occupied: &mut BTreeMap<(u32, u32), u32>,
) -> Result<(), (u32, u32)> {
    if let Some(array) = reflected.arrays.get(&type_id) {
        return reserve_array_locations(location, array, variable, reflected, occupied);
    }
    reserve_vector_components(type_id, location, component, variable, reflected, occupied)
}

fn reserve_array_locations(
    location: u32,
    array: &(u32, u32),
    variable: &u32,
    reflected: &InterfaceTypes,
    occupied: &mut BTreeMap<(u32, u32), u32>,
) -> Result<(), (u32, u32)> {
    let (element, count) = *array;
    let span = location_span(element, 0, reflected);
    for index in 0..reflected.constants[&count] {
        reserve_array_element(location + index * span, span, variable, occupied)?;
    }
    Ok(())
}

fn reserve_array_element(
    location: u32,
    span: u32,
    variable: &u32,
    occupied: &mut BTreeMap<(u32, u32), u32>,
) -> Result<(), (u32, u32)> {
    for offset in 0..span {
        reserve_full_location(location + offset, variable, occupied)?;
    }
    Ok(())
}

fn reserve_full_location(
    location: u32,
    variable: &u32,
    occupied: &mut BTreeMap<(u32, u32), u32>,
) -> Result<(), (u32, u32)> {
    for component in 0..4 {
        reserve_component(location, component, variable, occupied)?;
    }
    Ok(())
}

fn reserve_vector_components(
    type_id: u32,
    location: u32,
    component: u32,
    variable: &u32,
    reflected: &InterfaceTypes,
    occupied: &mut BTreeMap<(u32, u32), u32>,
) -> Result<(), (u32, u32)> {
    let components = reflected.vectors.get(&type_id).copied().unwrap_or(1);
    for offset in 0..components {
        let slot = (
            location + (component + offset) / 4,
            (component + offset) % 4,
        );
        reserve_component(slot.0, slot.1, variable, occupied)?;
    }
    Ok(())
}

fn reserve_component(
    location: u32,
    component: u32,
    variable: &u32,
    occupied: &mut BTreeMap<(u32, u32), u32>,
) -> Result<(), (u32, u32)> {
    let slot = (location, component);
    if occupied.insert(slot, *variable).is_some() {
        return Err(slot);
    }
    Ok(())
}

#[test]
fn array_location_padding_cannot_be_reused_by_a_component_lane() {
    let mut array = InterfaceTypes::default();
    array.variables.extend([(1, 100), (2, 101)]);
    array.pointers.extend([(100, 200), (101, 201)]);
    array.arrays.insert(200, (202, 203));
    array.vectors.insert(202, 3);
    array.constants.insert(203, 3);
    array.decorations.extend([(1, (10, 0)), (2, (10, 3))]);
    assert_eq!(component_locations(&array), Err((10, 3)));

    let mut split = InterfaceTypes::default();
    split.variables.extend([(1, 100), (2, 101)]);
    split.pointers.extend([(100, 202), (101, 201)]);
    split.vectors.insert(202, 3);
    split.decorations.extend([(1, (10, 0)), (2, (10, 3))]);
    assert!(component_locations(&split).is_ok());
}

fn interface_decorations(binary: &[u8], storage: u32) -> BTreeSet<(u32, u32)> {
    let words = spirv_words(binary);
    let (variables, decorations) = parse_interface(&words, storage);
    variables
        .iter()
        .filter_map(|id| decorations.get(id).copied())
        .collect()
}

fn parse_interface(words: &[u32], storage: u32) -> (BTreeSet<u32>, BTreeMap<u32, (u32, u32)>) {
    let mut variables = BTreeSet::new();
    let mut decorations = BTreeMap::<u32, (u32, u32)>::new();
    parse_instructions(words, storage, &mut variables, &mut decorations);
    (variables, decorations)
}

fn parse_instructions(
    words: &[u32],
    storage: u32,
    variables: &mut BTreeSet<u32>,
    decorations: &mut BTreeMap<u32, (u32, u32)>,
) {
    let mut cursor = 5;
    while cursor < words.len() {
        let count = (words[cursor] >> 16) as usize;
        let opcode = words[cursor] & 0xffff;
        let instruction = &words[cursor..cursor + count];
        parse_variable(instruction, opcode, storage, variables);
        parse_decoration(instruction, opcode, decorations);
        cursor += count;
    }
}

fn parse_variable(instruction: &[u32], opcode: u32, storage: u32, variables: &mut BTreeSet<u32>) {
    if opcode == 59 && instruction[3] == storage {
        variables.insert(instruction[2]);
    }
}

fn parse_decoration(instruction: &[u32], opcode: u32, decorations: &mut BTreeMap<u32, (u32, u32)>) {
    if opcode != 71 || !matches!(instruction[2], 30 | 31) {
        return;
    }
    let entry = decorations.entry(instruction[1]).or_insert((u32::MAX, 0));
    if instruction[2] == 30 {
        entry.0 = instruction[3];
    } else {
        entry.1 = instruction[3];
    }
}

fn spirv_words(binary: &[u8]) -> Vec<u32> {
    assert_eq!(binary.len() % 4, 0);
    binary
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| u32::from_le_bytes(*word))
        .collect()
}

fn maximum_location_end(binary: &[u8], storage: u32) -> u32 {
    let reflected = reflect_types(&spirv_words(binary), storage);
    reflected
        .variables
        .iter()
        .filter_map(|(id, pointer)| {
            let (location, component) = reflected.decorations.get(id)?;
            let pointee = reflected.pointers.get(pointer)?;
            Some(location + location_span(*pointee, *component, &reflected))
        })
        .max()
        .unwrap()
        - 1
}

#[derive(Default)]
struct InterfaceTypes {
    variables: BTreeMap<u32, u32>,
    decorations: BTreeMap<u32, (u32, u32)>,
    pointers: BTreeMap<u32, u32>,
    arrays: BTreeMap<u32, (u32, u32)>,
    vectors: BTreeMap<u32, u32>,
    constants: BTreeMap<u32, u32>,
    structs: BTreeMap<u32, Vec<u32>>,
    builtins: BTreeMap<u32, u32>,
    member_builtins: BTreeSet<(u32, usize)>,
}

fn reflect_types(words: &[u32], storage: u32) -> InterfaceTypes {
    let mut reflected = InterfaceTypes {
        variables: BTreeMap::new(),
        decorations: BTreeMap::new(),
        pointers: BTreeMap::new(),
        arrays: BTreeMap::new(),
        vectors: BTreeMap::new(),
        constants: BTreeMap::new(),
        structs: BTreeMap::new(),
        builtins: BTreeMap::new(),
        member_builtins: BTreeSet::new(),
    };
    reflect_instructions(words, storage, &mut reflected);
    reflected
}

fn reflect_instructions(words: &[u32], storage: u32, reflected: &mut InterfaceTypes) {
    let mut cursor = 5;
    while cursor < words.len() {
        let count = (words[cursor] >> 16) as usize;
        reflect_one(&words[cursor..cursor + count], storage, reflected);
        cursor += count;
    }
}

fn reflect_one(instruction: &[u32], storage: u32, reflected: &mut InterfaceTypes) {
    match instruction[0] & 0xffff {
        23 => {
            reflected.vectors.insert(instruction[1], instruction[3]);
        }
        28 => {
            reflected
                .arrays
                .insert(instruction[1], (instruction[2], instruction[3]));
        }
        30 => {
            reflected
                .structs
                .insert(instruction[1], instruction[2..].to_vec());
        }
        32 => {
            reflected.pointers.insert(instruction[1], instruction[3]);
        }
        43 => {
            reflected.constants.insert(instruction[2], instruction[3]);
        }
        59 if instruction[3] == storage => {
            reflected.variables.insert(instruction[2], instruction[1]);
        }
        71 if matches!(instruction[2], 30 | 31) => {
            reflect_location(instruction, reflected);
        }
        71 if instruction[2] == 11 => {
            reflected.builtins.insert(instruction[1], instruction[3]);
        }
        72 if instruction[3] == 11 => {
            reflected
                .member_builtins
                .insert((instruction[1], instruction[2] as usize));
        }
        _ => {}
    }
}

fn reflect_location(instruction: &[u32], reflected: &mut InterfaceTypes) {
    let entry = reflected
        .decorations
        .entry(instruction[1])
        .or_insert((u32::MAX, 0));
    if instruction[2] == 30 {
        entry.0 = instruction[3];
    } else {
        entry.1 = instruction[3];
    }
}

fn location_span(type_id: u32, component: u32, reflected: &InterfaceTypes) -> u32 {
    if let Some((element, count)) = reflected.arrays.get(&type_id) {
        return reflected.constants[count] * location_span(*element, 0, reflected);
    }
    let components = reflected.vectors.get(&type_id).copied().unwrap_or(1);
    (component + components).div_ceil(4)
}

fn builtin_components(binary: &[u8], storage: u32) -> u32 {
    let reflected = reflect_types(&spirv_words(binary), storage);
    reflected
        .variables
        .iter()
        .map(|(id, pointer)| variable_builtin_components(*id, *pointer, &reflected))
        .sum()
}

fn user_components(binary: &[u8], storage: u32) -> u32 {
    let reflected = reflect_types(&spirv_words(binary), storage);
    reflected
        .variables
        .iter()
        .filter(|(id, _)| reflected.decorations.contains_key(id))
        .map(|(_, pointer)| component_count(reflected.pointers[pointer], &reflected))
        .sum()
}

fn variable_builtin_components(id: u32, pointer: u32, reflected: &InterfaceTypes) -> u32 {
    let Some(pointee) = reflected.pointers.get(&pointer) else {
        return 0;
    };
    let direct = if reflected.builtins.contains_key(&id) {
        component_count(*pointee, reflected)
    } else {
        0
    };
    direct + member_builtin_components(*pointee, reflected)
}

fn member_builtin_components(type_id: u32, reflected: &InterfaceTypes) -> u32 {
    reflected
        .structs
        .get(&type_id)
        .into_iter()
        .flat_map(|members| members.iter().enumerate())
        .filter(|(index, _)| reflected.member_builtins.contains(&(type_id, *index)))
        .map(|(_, member)| component_count(*member, reflected))
        .sum()
}

fn component_count(type_id: u32, reflected: &InterfaceTypes) -> u32 {
    if let Some((element, count)) = reflected.arrays.get(&type_id) {
        return reflected.constants[count] * component_count(*element, reflected);
    }
    if let Some(components) = reflected.vectors.get(&type_id) {
        return *components;
    }
    reflected
        .structs
        .get(&type_id)
        .map(|members| {
            members
                .iter()
                .map(|member| component_count(*member, reflected))
                .sum()
        })
        .unwrap_or(1)
}
