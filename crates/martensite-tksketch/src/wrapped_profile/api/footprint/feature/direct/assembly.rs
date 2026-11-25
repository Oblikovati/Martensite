//! Assemble a frustum and its retained tool images in one unpublished native owner.
use super::*;

pub(super) struct Member {
    faces: Vec<Shape>,
    cut_cap: Shape,
    outer: Shape,
    counters: Vec<Shape>,
}

impl Member {
    pub(super) fn capture(
        attempt: Attempt,
        region: SurfaceRegionId,
        native: &DetachedWrappedBodies,
    ) -> FeatureChecked<Self> {
        let source = WrappedEntitySource::Region(region);
        let solid = named(
            attempt,
            native,
            WrappedHistoryKey {
                source,
                role: HistoryRole::Solid,
            },
        )?;
        let cap = named(
            attempt,
            native,
            WrappedHistoryKey {
                source,
                role: HistoryRole::FirstDepthFace,
            },
        )?;
        let faces = occurrences(attempt, solid, ShapeEnum::Face)?;
        let cut_cap = faces
            .iter()
            .find(|face| face.is_same(cap))
            .cloned()
            .ok_or_else(|| refused(attempt, "zero-depth cap in its tool member"))?;
        let cap = attempt.native(ASSEMBLY, "zero-depth face", topods::face(&cut_cap))?;
        let mut wires = direct_children(attempt, &cap, ShapeEnum::Wire)?;
        let outer = attempt.native(ASSEMBLY, "outer zero-depth wire", outer_wire(&cap))?;
        let outer_position = wires
            .iter()
            .position(|wire| wire.is_same(&outer))
            .ok_or_else(|| refused(attempt, "explicit outer zero-depth wire"))?;
        let outer = wires.remove(outer_position);
        Ok(Self {
            faces,
            cut_cap,
            outer,
            counters: wires,
        })
    }
}

pub(super) struct Frustum<'a> {
    shell: topods::Shell,
    pub(super) remainder: Face,
    images: Vec<Shape>,
    installed: Vec<seams::InstalledSeam<'a>>,
}

impl<'a> Frustum<'a> {
    pub(super) fn admit(attempt: Attempt, cone: &Face, stock: &Shape) -> FeatureChecked<Self> {
        let faces = occurrences(attempt, stock, ShapeEnum::Face)?;
        let [band] = direct_children(attempt, cone, ShapeEnum::Wire)?
            .try_into()
            .map_err(|_| refused(attempt, "one original cone wire"))?;
        let ends: Vec<_> = faces.iter().filter(|face| !face.is_same(cone)).collect();
        if faces.len() != 3 || ends.len() != 2 || cone.orientation() != Orientation::Forward {
            return Err(refused(
                attempt,
                "one forward cone and two planar end faces",
            ));
        }
        let builder = Builder;
        let mut shell = topods::Shell::default();
        builder.make_shell(&mut shell);
        for face in ends {
            let face = attempt.native(ASSEMBLY, "frustum end face", topods::face(face))?;
            let (geometry, _) = attempt.native(
                ASSEMBLY,
                "frustum end carrier",
                BRepTool::surface_with_location(&face),
            )?;
            if geometry.downcast::<GeomPlane>().is_null() {
                return Err(refused(attempt, "planar frustum end carriers"));
            }
            attempt.native(ASSEMBLY, "retain end", builder.add(&mut shell, &face))?;
        }
        let mut remainder = empty_face(attempt, cone)?;
        attempt.native(
            ASSEMBLY,
            "keep original cone seam and rims",
            builder.add(&mut remainder, &band),
        )?;
        Ok(Self {
            shell,
            remainder,
            images: Vec::new(),
            installed: Vec::new(),
        })
    }

    pub(super) fn attach(
        &mut self,
        attempt: Attempt,
        member: Member,
        sewing: &'a seams::Carrier,
        cancelled: &mut CancelPoll<'_>,
    ) -> FeatureChecked<()> {
        let builder = Builder;
        let outer = orient(&member.outer, attempt.kind);
        self.installed.extend(sew_wire(
            attempt,
            sewing,
            &outer,
            &mut self.remainder,
            cancelled,
        )?);
        for counter in &member.counters {
            let mut island = empty_face(attempt, &self.remainder)?;
            self.installed.extend(sew_wire(
                attempt,
                sewing,
                &orient(counter, attempt.kind),
                &mut island,
                cancelled,
            )?);
            attempt.native(
                ASSEMBLY,
                "retain counter island",
                builder.add(&mut self.shell, &island),
            )?;
            self.images.push(island.to_shape());
        }
        for face in member
            .faces
            .iter()
            .filter(|face| !face.is_same(&member.cut_cap))
        {
            let face = orient(face, attempt.kind);
            attempt.native(
                ASSEMBLY,
                "retain tool face",
                builder.add(&mut self.shell, &face),
            )?;
        }
        Ok(())
    }

    pub(super) fn finish(
        mut self,
        attempt: Attempt,
    ) -> FeatureChecked<(Shape, Vec<Shape>, Vec<seams::InstalledSeam<'a>>)> {
        let builder = Builder;
        attempt.native(
            ASSEMBLY,
            "retain main band",
            builder.add(&mut self.shell, &self.remainder),
        )?;
        attempt.native(
            ASSEMBLY,
            "closed attached shell",
            self.shell.set_closed(true),
        )?;
        let mut solid = topods::Solid::default();
        builder.make_solid(&mut solid);
        attempt.native(
            ASSEMBLY,
            "assemble attached solid",
            builder.add(&mut solid, &self.shell),
        )?;
        self.images.insert(0, self.remainder.to_shape());
        Ok((solid.to_shape(), self.images, self.installed))
    }
}

fn sew_wire<'a>(
    attempt: Attempt,
    sewing: &'a seams::Carrier,
    wire: &Shape,
    target: &mut Face,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<Vec<seams::InstalledSeam<'a>>> {
    let boundary = direct_children(attempt, wire, ShapeEnum::Edge)?;
    let certified = boundary
        .iter()
        .map(|edge| {
            attempt.poll(cancelled, ASSEMBLY)?;
            let edge = attempt.native(ASSEMBLY, "retained seam edge", topods::edge(edge))?;
            sewing.certify(attempt, edge, cancelled)
        })
        .collect::<FeatureChecked<Vec<_>>>()?;
    let installed = certified
        .into_iter()
        .map(|item| item.install(attempt, target))
        .collect::<FeatureChecked<Vec<_>>>()?;
    attempt.native(
        ASSEMBLY,
        "attach complete certified wire",
        Builder.add(target, wire),
    )?;
    Ok(installed)
}

fn named(
    attempt: Attempt,
    native: &DetachedWrappedBodies,
    key: WrappedHistoryKey,
) -> FeatureChecked<&Shape> {
    native.entities.get(&key).ok_or_else(|| {
        attempt.history(
            CylinderFeatureSource::Tool(key),
            "exact named input tool entity",
        )
    })
}

fn orient(shape: &Shape, kind: CylinderFeatureKind) -> Shape {
    match kind {
        CylinderFeatureKind::Boss => shape.clone(),
        CylinderFeatureKind::Pocket => shape.reversed(),
    }
}

fn empty_face(attempt: Attempt, face: &Face) -> FeatureChecked<Face> {
    let copy = attempt.native(ASSEMBLY, "copy cone carrier", face.empty_copied())?;
    attempt.native(ASSEMBLY, "replacement cone face", topods::face(&copy))
}

pub(super) fn direct_children(
    attempt: Attempt,
    root: &Shape,
    kind: ShapeEnum,
) -> FeatureChecked<Vec<Shape>> {
    let mut access = ShapeIterator::new(root, true, true);
    let mut shapes = Vec::new();
    while access.more() {
        let child = attempt.native(ASSEMBLY, "direct topology item", access.value())?;
        if attempt.native(ASSEMBLY, "direct topology kind", child.shape_type())? != kind {
            return Err(refused(attempt, "uniform direct topology children"));
        }
        shapes.push(child.to_shape());
        access.next();
    }
    Ok(shapes)
}

pub(super) fn occurrences(
    attempt: Attempt,
    root: &Shape,
    kind: ShapeEnum,
) -> FeatureChecked<Vec<Shape>> {
    let mut access = attempt.native(
        ASSEMBLY,
        "traverse input occurrences",
        Explorer::new(root, kind, ShapeEnum::Shape),
    )?;
    let mut shapes = Vec::new();
    while access.more() {
        shapes.push(access.current().to_shape());
        attempt.native(ASSEMBLY, "next input occurrence", access.next())?;
    }
    Ok(shapes)
}

pub(super) fn sort_stock_disks(attempt: Attempt, disks: [Face; 2]) -> FeatureChecked<[Face; 2]> {
    let axial = |face: &Face| -> FeatureChecked<f64> {
        let (shape, loc) = attempt.native(
            ASSEMBLY,
            "stock disk plane",
            BRepTool::surface_with_location(face),
        )?;
        if !loc.is_identity() || shape.downcast::<GeomPlane>().is_null() {
            return Err(refused(attempt, "world stock disk plane"));
        }
        Ok(shape.downcast::<GeomPlane>().pln().location().z())
    };
    let [a, b] = disks;
    if axial(&a)? < axial(&b)? {
        Ok([a, b])
    } else {
        Ok([b, a])
    }
}
