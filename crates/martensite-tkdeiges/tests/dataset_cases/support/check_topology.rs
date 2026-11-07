//! `the triangulation topology check`: the topology checks of the triangulations of a shape.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::{IndexedDataMapOfShapeListOfShape, IndexedMapOfShape};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Point2, Xyz};
use martensite_tkmath::poly::{Connect, mesh_purpose};
use std::collections::BTreeSet;

/// `ComputeArea(P1, P2, P3)` in 3D.
fn compute_area(p1: Xyz, p2: Xyz, p3: Xyz) -> f64 {
    0.5 * (p3 - p1).crossed(p2 - p1).modulus()
}

/// `ComputeArea(P1, P2, P3)` in 2D.
fn compute_area_2d(p1: Point2, p2: Point2, p3: Point2) -> f64 {
    let (ax, ay) = (p3.x() - p1.x(), p3.y() - p1.y());
    let (bx, by) = (p2.x() - p1.x(), p2.y() - p1.y());
    0.5 * (ax * by - ay * bx).abs()
}

/// `the triangulation topology check`: free links, cross face errors, asynchronous edges, free
/// nodes and null-area triangles of the triangulations of a shape;
/// `MeshTestCheckTopology::new(&shape).perform(&mut report)?`.
pub struct MeshTestCheckTopology {
    shape: Shape,
    /// `myMapFaceLinks`: the face number and the node pairs of its free links, in
    /// insertion order.
    map_face_links: Vec<(i32, Vec<i32>)>,
    errors: Vec<i32>,
    errors_val: Vec<f64>,
    async_edges: Vec<i32>,
    free_node_faces: Vec<i32>,
    free_node_nums: Vec<i32>,
    small_triangles_faces: Vec<i32>,
    small_triangles_triangles: Vec<i32>,
}

/// The node `index` of a polygon node array.
fn node_of(
    nodes: &martensite_tkernel::ncollection::Array1<'_, i32>,
    index: i32,
) -> Result<i32, Failure> {
    nodes
        .value(index)
        .copied()
        .map_err(|e| e.into_failure("Array1"))
}

/// One-based access of the sequences.
fn at<T: Copy>(values: &[T], index: i32) -> Result<T, Failure> {
    usize::try_from(index - 1)
        .ok()
        .and_then(|i| values.get(i).copied())
        .ok_or_else(|| Failure::out_of_range("Sequence::Value"))
}

impl MeshTestCheckTopology {
    /// The constructor; nothing is checked yet.
    pub fn new(shape: &Shape) -> Self {
        Self {
            shape: shape.clone(),
            map_face_links: Vec::new(),
            errors: Vec::new(),
            errors_val: Vec::new(),
            async_edges: Vec::new(),
            free_node_faces: Vec::new(),
            free_node_nums: Vec::new(),
            small_triangles_faces: Vec::new(),
            small_triangles_triangles: Vec::new(),
        }
    }

    /// `perform(di)`: the polygons shared by two faces, then each triangulation.
    pub fn perform(&mut self, report: &mut String) -> Result<(), Failure> {
        let mut map_f = IndexedMapOfShape::default();
        let mut map_ef = IndexedDataMapOfShapeListOfShape::default();
        topexp::map_shapes_of_type(&self.shape, ShapeEnum::Face, &mut map_f)?;
        topexp::map_shapes_and_ancestors(
            &self.shape,
            ShapeEnum::Edge,
            ShapeEnum::Face,
            &mut map_ef,
        )?;

        // check polygons
        for ie in 1..=map_ef.extent() {
            let edge = topods::edge(map_ef.find_key(ie)?)?;
            let faces: Vec<Shape> = map_ef.find_from_index(ie)?.iter().cloned().collect();
            if faces.len() < 2 {
                continue;
            }
            // get polygon on first face
            let face1 = topods::face(&faces[0])?;
            let (t1, loc1) = BRepTool::triangulation(&face1, mesh_purpose::NONE)?;
            let poly1 = BRepTool::polygon_on_triangulation(&edge, &t1, &loc1)?;
            let (Some(p1), Some(tri1)) = (poly1.get(), t1.get()) else {
                continue;
            };
            let nodes1 = p1.nodes();
            // cycle on other polygons
            for face in &faces[1..] {
                let face2 = topods::face(face)?;
                let (t2, loc2) = BRepTool::triangulation(&face2, mesh_purpose::NONE)?;
                let poly2 = BRepTool::polygon_on_triangulation(&edge, &t2, &loc2)?;
                let (Some(p2), Some(tri2)) = (poly2.get(), t2.get()) else {
                    continue;
                };
                let nodes2 = p2.nodes();
                // check equality of polygons lengths
                if nodes2.length() != nodes1.length() {
                    self.async_edges.push(i32::try_from(ie).unwrap_or(i32::MAX));
                    break;
                }
                // check distances between corresponding points
                let mut sq_defle = BRepTool::edge_tolerance(&edge)?;
                sq_defle *= sq_defle;
                let i_f1 = map_f.find_index(face1.as_ref());
                let i_f2 = map_f.find_index(face2.as_ref());
                let trsf1 = face1.location().transformation();
                let trsf2 = face2.location().transformation();
                for (i2, i1) in (nodes2.lower()..).zip(nodes1.lower()..=nodes1.upper()) {
                    let a = tri1.node(node_of(&nodes1, i1)?)?.transformed(trsf1);
                    let b = tri2.node(node_of(&nodes2, i2)?)?.transformed(trsf2);
                    let sq_dist = a.square_distance(&b);
                    if sq_dist > sq_defle {
                        self.errors.push(i32::try_from(i_f1).unwrap_or(0));
                        self.errors.push(i1);
                        self.errors.push(i32::try_from(i_f2).unwrap_or(0));
                        self.errors.push(i2);
                        self.errors_val.push(sq_dist.sqrt());
                    }
                }
            }
        }

        // check triangulations
        for i_f in 1..=map_f.extent() {
            let face_index = i32::try_from(i_f).unwrap_or(i32::MAX);
            let face = topods::face(map_f.find_key(i_f)?)?;
            let (t, loc) = BRepTool::triangulation(&face, mesh_purpose::NONE)?;
            let Some(tri) = t.get() else {
                report.push_str(&format!("face {face_index} has no triangulation\n"));
                continue;
            };
            let trsf = loc.transformation();

            // remember boundary nodes
            let mut map_bnd_nodes = BTreeSet::new();
            let mut ex = Explorer::new(face.as_ref(), ShapeEnum::Edge, ShapeEnum::Shape)?;
            while ex.more() {
                let edge = topods::edge(ex.current())?;
                let poly = BRepTool::polygon_on_triangulation(&edge, &t, &loc)?;
                if let Some(p) = poly.get() {
                    let nodes = p.nodes();
                    for i in nodes.lower()..=nodes.upper() {
                        map_bnd_nodes.insert(node_of(&nodes, i)?);
                    }
                }
                ex.next()?;
            }

            let mut used_nodes = BTreeSet::new();

            // check of free links and nodes
            let conn = Connect::new(&t)?;
            let nb_tri = tri.nb_triangles();
            for i in 1..=nb_tri {
                let (n0, n1, n2) = tri.triangle(i)?.get();
                let n = [n0, n1, n2];
                used_nodes.extend(n);
                let pts = [
                    tri.node(n[0])?.transformed(trsf),
                    tri.node(n[1])?.transformed(trsf),
                    tri.node(n[2])?.transformed(trsf),
                ];
                let area = compute_area(pts[0].xyz(), pts[1].xyz(), pts[2].xyz());
                if area < precision::square_confusion() {
                    self.small_triangles_faces.push(face_index);
                    self.small_triangles_triangles.push(i);
                } else if tri.has_uv_nodes() {
                    let area =
                        compute_area_2d(tri.uv_node(n[0])?, tri.uv_node(n[1])?, tri.uv_node(n[2])?);
                    if area < precision::square_p_confusion() {
                        self.small_triangles_faces.push(face_index);
                        self.small_triangles_triangles.push(i);
                    }
                }

                let (t0, t1, t2) = conn.triangles(i)?;
                let adjacent = [t0, t1, t2];
                for j in 0..3 {
                    if adjacent[j] == 0 {
                        // free link found
                        let k = (j + 1) % 3; // the following node of the edge
                        let (a, b) = (n[j], n[k]);
                        // skip if it is on boundary
                        if map_bnd_nodes.contains(&a) && map_bnd_nodes.contains(&b) {
                            continue;
                        }
                        let position = match self
                            .map_face_links
                            .iter()
                            .position(|(f, _)| *f == face_index)
                        {
                            Some(p) => p,
                            None => {
                                self.map_face_links.push((face_index, Vec::new()));
                                self.map_face_links.len() - 1
                            }
                        };
                        self.map_face_links[position].1.extend([a, b]);
                    }
                }
            }

            // check of free nodes
            for k in 1..=tri.nb_nodes() {
                if !used_nodes.contains(&k) {
                    self.free_node_faces.push(face_index);
                    self.free_node_nums.push(k);
                }
            }
        }
        Ok(())
    }

    /// `NbFacesWithFL()`.
    pub fn nb_faces_with_fl(&self) -> i32 {
        i32::try_from(self.map_face_links.len()).unwrap_or(i32::MAX)
    }

    fn face_links(&self, index: i32) -> Result<&(i32, Vec<i32>), Failure> {
        usize::try_from(index - 1)
            .ok()
            .and_then(|i| self.map_face_links.get(i))
            .ok_or_else(|| Failure::out_of_range("IndexedDataMap::FindKey"))
    }

    /// `GetFaceNumWithFL(theIndex)`.
    pub fn get_face_num_with_fl(&self, index: i32) -> Result<i32, Failure> {
        Ok(self.face_links(index)?.0)
    }

    /// `NbFreeLinks(theIndex)`.
    pub fn nb_free_links(&self, index: i32) -> Result<i32, Failure> {
        Ok(i32::try_from(self.face_links(index)?.1.len() / 2).unwrap_or(i32::MAX))
    }

    /// `GetFreeLink(theFaceIndex, theLinkIndex, theNode1, theNode2)`.
    pub fn get_free_link(&self, face_index: i32, link_index: i32) -> Result<(i32, i32), Failure> {
        let seq = &self.face_links(face_index)?.1;
        let ind = (link_index - 1) * 2 + 1;
        Ok((at(seq, ind)?, at(seq, ind + 1)?))
    }

    /// `NbCrossFaceErrors()`.
    pub fn nb_cross_face_errors(&self) -> i32 {
        i32::try_from(self.errors_val.len()).unwrap_or(i32::MAX)
    }

    /// `GetCrossFaceError(theIndex, ...)`: face 1, node 1, face 2, node 2 and the
    /// distance.
    pub fn get_cross_face_error(&self, index: i32) -> Result<(i32, i32, i32, i32, f64), Failure> {
        let ind = (index - 1) * 4 + 1;
        Ok((
            at(&self.errors, ind)?,
            at(&self.errors, ind + 1)?,
            at(&self.errors, ind + 2)?,
            at(&self.errors, ind + 3)?,
            at(&self.errors_val, index)?,
        ))
    }

    /// `NbAsyncEdges()`.
    pub fn nb_async_edges(&self) -> i32 {
        i32::try_from(self.async_edges.len()).unwrap_or(i32::MAX)
    }

    /// `GetAsyncEdgeNum(theIndex)`.
    pub fn get_async_edge_num(&self, index: i32) -> Result<i32, Failure> {
        at(&self.async_edges, index)
    }

    /// `NbFreeNodes()`.
    pub fn nb_free_nodes(&self) -> i32 {
        i32::try_from(self.free_node_faces.len()).unwrap_or(i32::MAX)
    }

    /// `GetFreeNodeNum(theIndex, theFaceNum, theNodeNum)`.
    pub fn get_free_node_num(&self, index: i32) -> Result<(i32, i32), Failure> {
        Ok((
            at(&self.free_node_faces, index)?,
            at(&self.free_node_nums, index)?,
        ))
    }

    /// `NbSmallTriangles()`.
    pub fn nb_small_triangles(&self) -> i32 {
        i32::try_from(self.small_triangles_faces.len()).unwrap_or(i32::MAX)
    }

    /// `GetSmallTriangle(theIndex, theFaceNum, theNodeNum)`.
    pub fn get_small_triangle(&self, index: i32) -> Result<(i32, i32), Failure> {
        Ok((
            at(&self.small_triangles_faces, index)?,
            at(&self.small_triangles_triangles, index)?,
        ))
    }
}
