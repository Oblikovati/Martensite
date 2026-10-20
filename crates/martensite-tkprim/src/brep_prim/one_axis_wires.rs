//! The wires of `BRepPrim_OneAxis`, each built once.
use super::BRepPrimOneAxis;
use super::one_axis::{
    WAXISEND, WAXISSTART, WBOTTOM, WEND, WLATERAL, WLATERALEND, WLATERALSTART, WSTART, WTOP,
    are_heights_equal, is_height_inverted, one_axis_domain_error,
};
use martensite_tkbrep::topods::{Edge, Wire};
use martensite_tkernel::standard::Failure;

/// One edge of a wire: the member that builds it and whether it is added forward.
type WireEdge<T> = (fn(&mut T) -> Result<Edge, Failure>, bool);

/// The built wire in slot `slot`, or `None` when it is not built yet.
fn built_wire<T: BRepPrimOneAxis + ?Sized>(prim: &T, slot: usize) -> Option<Wire> {
    let base = prim.one_axis_base();
    base.wires_built[slot].then(|| base.my_wires[slot].clone())
}

/// Makes the wire of `edges` in order, completes it and stores it in slot `slot`.
fn make_wire<T: BRepPrimOneAxis + ?Sized>(
    prim: &mut T,
    slot: usize,
    edges: &[WireEdge<T>],
) -> Result<Wire, Failure> {
    let mut wire = empty_wire(prim);
    add_wire_edges(prim, &mut wire, edges)?;
    finish_wire(prim, slot, wire)
}

/// Appends each requested edge immediately, preserving callback and failure order.
fn add_wire_edges<T: BRepPrimOneAxis + ?Sized>(
    prim: &mut T,
    wire: &mut Wire,
    edges: &[WireEdge<T>],
) -> Result<(), Failure> {
    let builder = prim.one_axis_base().my_builder;
    for (edge, direct) in edges {
        let edge = edge(prim)?;
        builder.add_wire_edge(wire, &edge, *direct)?;
    }
    Ok(())
}

/// Creates an empty wire for incremental edge construction.
fn empty_wire<T: BRepPrimOneAxis + ?Sized>(prim: &T) -> Wire {
    let mut wire = Wire::default();
    prim.one_axis_base().my_builder.make_wire(&mut wire);
    wire
}

/// Publishes a cached wire only after successful completion.
fn finish_wire<T: BRepPrimOneAxis + ?Sized>(
    prim: &mut T,
    slot: usize,
    wire: Wire,
) -> Result<Wire, Failure> {
    prim.one_axis_base().my_builder.complete_wire(&wire)?;
    let base = prim.one_axis_base_mut();
    base.my_wires[slot] = wire.clone();
    base.wires_built[slot] = true;
    Ok(wire)
}

/// `LateralWire()`: the top edge (reversed, when VMax is finite), the end edge, the
/// bottom edge (when VMin is finite) and the start edge (reversed).
pub(crate) fn lateral_wire<T: BRepPrimOneAxis + ?Sized>(prim: &mut T) -> Result<Wire, Failure> {
    if let Some(wire) = built_wire(prim, WLATERAL) {
        return Ok(wire);
    }
    let builder = prim.one_axis_base().my_builder;
    let mut wire = empty_wire(prim);
    if !prim.vmax_infinite() {
        builder.add_wire_edge(&mut wire, &prim.top_edge()?, false)?;
    }
    builder.add_wire_edge(&mut wire, &prim.end_edge()?, true)?;
    if !prim.vmin_infinite() {
        builder.add_wire_edge(&mut wire, &prim.bottom_edge()?, true)?;
    }
    builder.add_wire_edge(&mut wire, &prim.start_edge()?, false)?;
    finish_wire(prim, WLATERAL, wire)
}

/// `LateralStartWire()`: the start edge alone, reversed.
pub(crate) fn lateral_start_wire<T: BRepPrimOneAxis + ?Sized>(
    prim: &mut T,
) -> Result<Wire, Failure> {
    if let Some(wire) = built_wire(prim, WLATERALSTART) {
        return Ok(wire);
    }
    make_wire(prim, WLATERALSTART, &[(T::start_edge, false)])
}

/// `LateralEndWire()`: the end edge alone.
pub(crate) fn lateral_end_wire<T: BRepPrimOneAxis + ?Sized>(prim: &mut T) -> Result<Wire, Failure> {
    if let Some(wire) = built_wire(prim, WLATERALEND) {
        return Ok(wire);
    }
    make_wire(prim, WLATERALEND, &[(T::end_edge, true)])
}

/// The ordered rim and optional side edges of a cap wire.
struct CapWire<T: ?Sized> {
    slot: usize,
    member: &'static str,
    exists: fn(&T) -> Result<bool, Failure>,
    missing: &'static str,
    rim: WireEdge<T>,
    sides: [WireEdge<T>; 2],
}

/// Builds the rim before querying whether optional side edges are required.
fn cap_wire<T: BRepPrimOneAxis + ?Sized>(prim: &mut T, cap: CapWire<T>) -> Result<Wire, Failure> {
    if let Some(wire) = built_wire(prim, cap.slot) {
        return Ok(wire);
    }
    if !(cap.exists)(prim)? {
        return Err(one_axis_domain_error(cap.member, cap.missing));
    }
    let mut wire = empty_wire(prim);
    add_wire_edges(prim, &mut wire, &[cap.rim])?;
    if prim.has_sides() {
        add_wire_edges(prim, &mut wire, &cap.sides)?;
    }
    finish_wire(prim, cap.slot, wire)
}

/// `TopWire()`: the top edge and, with sides, the start top edge and the end top edge
/// (reversed); Standard_DomainError without a top.
pub(crate) fn top_wire<T: BRepPrimOneAxis + ?Sized>(prim: &mut T) -> Result<Wire, Failure> {
    cap_wire(
        prim,
        CapWire {
            slot: WTOP,
            member: "TopWire",
            exists: T::has_top,
            missing: "no top",
            rim: (T::top_edge, true),
            sides: [(T::start_top_edge, true), (T::end_top_edge, false)],
        },
    )
}

/// `BottomWire()`: the bottom edge (reversed) and, with sides, the end bottom edge and
/// the start bottom edge (reversed); Standard_DomainError without a bottom.
pub(crate) fn bottom_wire<T: BRepPrimOneAxis + ?Sized>(prim: &mut T) -> Result<Wire, Failure> {
    cap_wire(
        prim,
        CapWire {
            slot: WBOTTOM,
            member: "BottomWire",
            exists: T::has_bottom,
            missing: "no bottom",
            rim: (T::bottom_edge, false),
            sides: [(T::end_bottom_edge, true), (T::start_bottom_edge, false)],
        },
    )
}

/// Whether a side wire holds the axis edge: an open meridian with distinct heights and
/// a finite end. Reuse the height comparison made before constructing the first cap
/// edge, as `BRepPrim_OneAxis` requires.
fn side_has_axis<T: BRepPrimOneAxis + ?Sized>(
    prim: &T,
    heights_equal: bool,
) -> Result<bool, Failure> {
    Ok(!prim.meridian_closed()?
        && !heights_equal
        && (!prim.vmax_infinite() || !prim.vmin_infinite()))
}

/// A cap predicate and its edge; the edge is built immediately after a true query.
type CapEdge<T> = (fn(&T) -> Result<bool, Failure>, WireEdge<T>);

/// The source-ordered first cap, axis, second cap and meridian edges of a side wire.
struct SideWire<T: ?Sized> {
    slot: usize,
    member: &'static str,
    first: CapEdge<T>,
    last: CapEdge<T>,
    meridian: WireEdge<T>,
    flip_axis: bool,
}

/// Queries one cap and appends its edge before any later branch is evaluated.
fn add_cap_edge<T: BRepPrimOneAxis + ?Sized>(
    prim: &mut T,
    wire: &mut Wire,
    cap: CapEdge<T>,
) -> Result<(), Failure> {
    let (exists, edge) = cap;
    if exists(prim)? {
        add_wire_edges(prim, wire, &[edge])?;
    }
    Ok(())
}

/// Completes each side edge in order; both height comparisons precede the first cap.
fn build_side_wire<T: BRepPrimOneAxis + ?Sized>(
    prim: &mut T,
    side: SideWire<T>,
) -> Result<Wire, Failure> {
    let mut wire = empty_wire(prim);
    let inverted = is_height_inverted(prim)?;
    let equal = are_heights_equal(prim)?;
    add_cap_edge(prim, &mut wire, side.first)?;
    if side_has_axis(prim, equal)? {
        add_wire_edges(
            prim,
            &mut wire,
            &[(T::axis_edge, inverted ^ side.flip_axis)],
        )?;
    }
    add_cap_edge(prim, &mut wire, side.last)?;
    add_wire_edges(prim, &mut wire, &[side.meridian])?;
    finish_wire(prim, side.slot, wire)
}

/// Checks cached state and the source guard before allocating a side wire.
fn side_wire<T: BRepPrimOneAxis + ?Sized>(
    prim: &mut T,
    side: SideWire<T>,
) -> Result<Wire, Failure> {
    if let Some(wire) = built_wire(prim, side.slot) {
        return Ok(wire);
    }
    if !prim.has_sides() {
        return Err(one_axis_domain_error(side.member, "no sides"));
    }
    build_side_wire(prim, side)
}

/// `StartWire()`: the start bottom edge, the axis edge (reversed unless the heights
/// are inverted), the start top edge (reversed) and the start edge, each when it
/// exists; Standard_DomainError without sides.
pub(crate) fn start_wire<T: BRepPrimOneAxis + ?Sized>(prim: &mut T) -> Result<Wire, Failure> {
    side_wire(
        prim,
        SideWire {
            slot: WSTART,
            member: "StartWire",
            flip_axis: false,
            first: (T::has_bottom, (T::start_bottom_edge, true)),
            last: (T::has_top, (T::start_top_edge, false)),
            meridian: (T::start_edge, true),
        },
    )
}

/// `EndWire()`: the end top edge, the axis edge (forward unless the heights are
/// inverted), the end bottom edge (reversed) and the end edge (reversed), each when it
/// exists; Standard_DomainError without sides.
pub(crate) fn end_wire<T: BRepPrimOneAxis + ?Sized>(prim: &mut T) -> Result<Wire, Failure> {
    side_wire(
        prim,
        SideWire {
            slot: WEND,
            member: "EndWire",
            flip_axis: true,
            first: (T::has_top, (T::end_top_edge, true)),
            last: (T::has_bottom, (T::end_bottom_edge, false)),
            meridian: (T::end_edge, false),
        },
    )
}

/// The checks of the axis wires: sides, an infinite range at both ends and an open
/// meridian, else Standard_DomainError.
fn check_axis_wire<T: BRepPrimOneAxis + ?Sized>(prim: &T, member: &str) -> Result<(), Failure> {
    if !prim.has_sides() {
        return Err(one_axis_domain_error(member, "no sides"));
    }
    if !prim.vmax_infinite() || !prim.vmin_infinite() {
        return Err(one_axis_domain_error(member, "not infinite"));
    }
    if prim.meridian_closed()? {
        return Err(one_axis_domain_error(member, "meridian closed"));
    }
    Ok(())
}

/// `AxisStartWire()`: the axis edge alone, reversed; Standard_DomainError without
/// sides, with a finite end or a closed meridian.
pub(crate) fn axis_start_wire<T: BRepPrimOneAxis + ?Sized>(prim: &mut T) -> Result<Wire, Failure> {
    if let Some(wire) = built_wire(prim, WAXISSTART) {
        return Ok(wire);
    }
    check_axis_wire(prim, "AxisStartWire")?;
    make_wire(prim, WAXISSTART, &[(T::axis_edge, false)])
}

/// `AxisEndWire()`: the axis edge alone; Standard_DomainError without sides, with a
/// finite end or a closed meridian.
pub(crate) fn axis_end_wire<T: BRepPrimOneAxis + ?Sized>(prim: &mut T) -> Result<Wire, Failure> {
    if let Some(wire) = built_wire(prim, WAXISEND) {
        return Ok(wire);
    }
    check_axis_wire(prim, "AxisEndWire")?;
    make_wire(prim, WAXISEND, &[(T::axis_edge, true)])
}
