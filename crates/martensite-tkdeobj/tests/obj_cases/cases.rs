//! The scripts, in modules named after their group and grid.
use crate::checks::{
    case_file, checknbshapes, checktrinfo, children, new_document, one_shape, read_obj, read_shape,
    read_single_face, trinfo, write_obj,
};
use crate::scene::ViewScene;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkservice::graphic3d::{Graphic3dCameraProjection, Graphic3dTypeOfShadingModel};
use std::path::Path;

/// `tests/de_mesh/obj_read/end`: the explored document under Phong and PBR, both projections.
fn show_explored(scene: &mut ViewScene, document: &Handle<TDocStdDocument>, case: &str) {
    scene.clear();
    scene.xdisplay("D", document, true);
    scene.axo();
    scene.fit();
    scene.shading(Graphic3dTypeOfShadingModel::Phong);
    scene.light_intensity(0, 2.5);
    scene.light_intensity(1, 0.3);
    scene.camera(Graphic3dCameraProjection::Orthographic);
    scene.dump(case);
    scene.dump(&format!("{case}_ortho_phong"));
    scene.camera(Graphic3dCameraProjection::Perspective);
    scene.dump(&format!("{case}_persp_phong"));
    scene.shading(Graphic3dTypeOfShadingModel::Pbr);
    scene.camera(Graphic3dCameraProjection::Orthographic);
    scene.dump(&format!("{case}_ortho_pbr"));
    scene.camera(Graphic3dCameraProjection::Perspective);
    scene.dump(&format!("{case}_persp_pbr"));
}

fn show_raw(scene_case: &'static str, shape: &Shape, case: &str) -> ViewScene {
    let mut scene = ViewScene::open(scene_case);
    scene.display_shape("mlf", shape);
    scene.axo();
    scene.fit();
    scene.dump(&format!("{case}_raw"));
    scene
}

fn new_scene_explored(scene_case: &'static str, document: &Handle<TDocStdDocument>) {
    let mut scene = ViewScene::open(scene_case);
    show_explored(&mut scene, document, scene_case);
    scene.close();
}

fn write_lines(path: &Path, text: &str, line_end: &str) {
    let converted = format!("{text}\n").replace('\n', line_end);
    std::fs::write(path, converted).unwrap();
}

pub mod de_mesh_obj_read {
    use super::{
        Handle, TDocStdDocument, ViewScene, case_file, checknbshapes, checktrinfo, children,
        new_scene_explored, one_shape, read_obj, read_single_face, show_explored, show_raw,
        write_lines,
    };
    use crate::obj_case;

    const PRISM_MTL: &str = "newmtl ObjMat1\nNs 96.07\nKa 0.00 0.00 0.00\nKd 0.64 0.64 0.64\nKs 0.50 0.50 0.50\nNi 1.00\nd  1.00";
    const PRISM_OBJ: &str = "mtllib A1_material.mtl\ng Group1\nusemtl ObjMat1\nv 0 0 0\nv 2 0 0\nv 2 1 0\nv 1 2 0\nv 0 1 0\nv 0 0 2\nv 2 0 2\nv 2 1 2\nv 1 2 2\nv 0 1 2\nf 5 4 3 2 1\nf 7 8 9 10 6\nf 10 9 4 5\nf 9 8 3 4\nf 6 10 5 1\nf 2 3 8 7\nf 1 2 7 6";
    const EMPTY_NORMALS: &str = "g Group1\nv 0 0 0\nv 2 0 0\nv 2 1 0\nv 1 2 0\nv 0 1 0\nv 0 0 2\nv 2 0 2\nv 2 1 2\nv 1 2 2\nv 0 1 2\nf 5// 4// 3// 2// 1//\nf 7// 8// 9// 10// 6//\nf 10// 9// 4// 5//\nf 9// 8// 3// 4//\nf 6// 10// 5// 1//\nf 2// 3// 8// 7//\nf 1// 2// 7// 6//";
    const USEMTL_MTL: &str =
        "newmtl Red\nKd 1.0 0.0 0.0\nnewmtl Green\nKd 0.0 1.0 0.0\nnewmtl Blue\nKd 0.0 0.0 1.0";
    const USEMTL_OBJ: &str = "\nmtllib usemtl_material.mtl\nv 0 0 0\nv 2 0 0\nv 2 1 0\nv 1 2 0\nv 0 1 0\nv 0 0 2\nv 2 0 2\nv 2 1 2\nv 1 2 2\nv 0 1 2\ng Red\nusemtl Red\nf 5 4 3 2 1\ng Gren\nusemtl Green\nf 7 8 9 10 6\ng Blue\nusemtl Blue\nf 10 9 4 5\ng Gray\nusemtl\nf 9 8 3 4\nf 6 10 5 1\nf 2 3 8 7\nf 1 2 7 6";

    fn emptynorm_script() {
        println!("========");
        println!("Data Exchange, fix reading facets with empty normals like 'f 1// 2// 3//'");
        println!("Test reads small generated file.");
        println!("========");
        let file = case_file("emptynorm.obj");
        write_lines(&file, EMPTY_NORMALS, "\n");
        let mlf = read_single_face(&file);
        checknbshapes("mlf", &mlf, 1, None);
        checktrinfo(&mlf, 16, 10);
        let document = read_obj(&file);
        std::fs::remove_file(&file).unwrap();
        let mut scene = show_raw("emptynorm", &mlf, "emptynorm");
        show_again(&mut scene, &document, "emptynorm");
        scene.close();
    }
    obj_case!(
        emptynorm,
        "tests/de_mesh/obj_read/emptynorm",
        emptynorm_script
    );

    fn show_again(scene: &mut ViewScene, document: &Handle<TDocStdDocument>, case: &str) {
        scene.clear();
        scene.activate_again();
        show_explored(scene, document, case);
    }

    fn multiline_script() {
        println!("========");
        println!(
            "Data Exchange, reading multiline file with more than one line separator leads to memory corruption"
        );
        println!("========");
        let dummy = "dummy".repeat(50);
        let text = include_str!("../data/multiline.obj.template")
            .replace("__DUMMY__", &format!("\\\n{dummy}"))
            .replace("__SPLIT__", "\\\n");
        let file = case_file("multiline.obj");
        std::fs::write(&file, text).unwrap();
        let document = read_obj(&file);
        let shape = one_shape(&document, "s");
        checknbshapes("s", &shape, 1, Some(1));
        checktrinfo(&shape, 16, 10);
        new_scene_explored("multiline", &document);
    }
    obj_case!(
        multiline,
        "tests/de_mesh/obj_read/multiline",
        multiline_script
    );

    fn prism_script() {
        println!("========");
        println!("Data Exchange - implement import of mesh data from files in OBJ format");
        println!("Reading small generated file.");
        println!("========");
        write_lines(&case_file("prism_material.mtl"), PRISM_MTL, "\r\n");
        println!("#======================================================================");
        println!("# Ascii file, CRLF");
        println!("#======================================================================");
        let dos = case_file("prism_one_ascii_dos.obj");
        write_lines(&dos, PRISM_OBJ, "\r\n");
        let mcrlf = read_single_face(&dos);
        checknbshapes("mcrlf", &mcrlf, 1, None);
        checktrinfo(&mcrlf, 16, 10);
        println!("#======================================================================");
        println!("# Ascii file with single facet, LF");
        println!("#======================================================================");
        let unix = case_file("prism_one_ascii_unix.obj");
        write_lines(&unix, PRISM_OBJ, "\n");
        let mlf = read_single_face(&unix);
        checknbshapes("mlf", &mlf, 1, None);
        checktrinfo(&mlf, 16, 10);
        let mut scene = show_raw("prism", &mlf, "prism");
        let document = read_obj(&unix);
        show_again(&mut scene, &document, "prism");
        scene.close();
    }
    obj_case!(prism, "tests/de_mesh/obj_read/prism", prism_script);

    fn smooth_script() {
        println!("========");
        println!("Data Exchange, skip duplicating smooth groups");
        println!("========");
        let file = case_file("smooth.obj");
        std::fs::write(&file, include_str!("../data/smooth.obj")).unwrap();
        let document = read_obj(&file);
        let shape = one_shape(&document, "s");
        checknbshapes("s", &shape, 3, Some(2));
        checktrinfo(&shape, 306, 205);
        new_scene_explored("smooth", &document);
    }
    obj_case!(smooth, "tests/de_mesh/obj_read/smooth", smooth_script);

    fn usemtl_script() {
        println!("========");
        println!("Data Exchange - the reader ignores usemtl with empty name");
        println!("========");
        write_lines(&case_file("usemtl_material.mtl"), USEMTL_MTL, "\n");
        let file = case_file("usemtl.obj");
        write_lines(&file, USEMTL_OBJ, "\n");
        let document = read_obj(&file);
        let shape = one_shape(&document, "s");
        let parts = children(&shape);
        let main = document.get().unwrap().main().unwrap();
        let materials =
            martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool::vis_material_tool(&main).unwrap();
        let last = materials
            .get()
            .unwrap()
            .get_material_of_shape(&parts[3])
            .unwrap();
        assert!(
            last.get().is_none_or(|material| material.is_empty()),
            "EMPTY material is expected for last group"
        );
        println!("EMPTY");
        new_scene_explored("usemtl", &document);
    }
    obj_case!(usemtl, "tests/de_mesh/obj_read/usemtl", usemtl_script);
}

pub mod de_mesh_obj_write {
    use super::{
        Handle, Path, TDocStdDocument, ViewScene, case_file, checknbshapes, new_document,
        one_shape, read_obj, write_obj,
    };
    use crate::obj_case;
    use martensite_tkdestep::stepcaf_control::STEPCAFControlReader;
    use martensite_tkernel::message::ProgressRange;
    use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;

    fn read_step_document(path: &Path) -> Handle<TDocStdDocument> {
        let document = new_document();
        let mut reader = STEPCAFControlReader::new().unwrap();
        reader.read_file(path.to_str().unwrap()).unwrap();
        assert!(
            reader
                .transfer(&document, &ProgressRange::default())
                .unwrap()
        );
        document
    }

    fn as1_script() {
        println!("========");
        println!("Data Exchange - add the document writer tool for wavefront OBJ file");
        println!("Write as1 STEP model into OBJ file");
        println!("========");
        let input = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../external/martensite-test-data/dataset-7.9.0/step/as1-oc-214-mat.stp")
            .canonicalize()
            .unwrap();
        println!(" File STEP to read : {}", input.display());
        let first = read_step_document(&input);
        println!("Document saved with name D1");
        let ss = one_shape(&first, "ss");
        println!("Incremental Mesh, multi-threading OFF");
        BRepMeshIncrementalMesh::with_shape(&ss, 1.0, false, 0.5, false).unwrap();
        println!("Meshing statuses: NoError");
        let file = case_file("as1_tmp.obj");
        write_obj(&first, &file);
        let document = read_obj(&file);
        let shape = one_shape(&document, "s");
        checknbshapes("s", &shape, 18, Some(2));
        let mut scene = ViewScene::open("as1");
        scene.xdisplay("D", &document, false);
        scene.axo();
        scene.fit();
        scene.dump("as1");
        scene.close();
        println!("Deleting temporary file {}", file.display());
        println!(
            "Deleting temporary file {}",
            file.with_extension("mtl").display()
        );
    }
    obj_case!(as1, "tests/de_mesh/obj_write/as1", as1_script);
}

pub mod bugs_moddata_3 {
    use super::{Graphic3dTypeOfShadingModel, Shape, ViewScene, case_file, read_shape, trinfo};
    use crate::cad_model::cad_model;
    use crate::obj_case;
    use crate::wavefront::wavefront;
    use martensite_tkbrep::brep_tools;
    use martensite_tkernel::tcollection::AsciiString;

    fn banner() {
        println!("==========");
        println!("Stored normals of triangulation-only faces");
        println!("==========");
    }

    /// Every script up to the file read back; what differs is how the shape is stored after it.
    fn common_part(name: &'static str) -> (Shape, ViewScene) {
        banner();
        let (res, scene) = cad_model(name);
        trinfo(&res);
        let base = case_file(&format!("{name}.obj"));
        wavefront(&res, &base);
        let read = read_shape(&base);
        let (triangles, nodes, _) = crate::checks::mesh_totals(&read);
        assert_eq!((triangles, nodes), (1228, 1030), "the mesh read back");
        (read, scene)
    }

    fn show_stored(scene: &mut ViewScene, shape: &Shape, name: &str) {
        scene.clear();
        scene.reopen_as_v1();
        scene.bottom();
        scene.display_shape("b", shape);
        scene.fit();
        scene.shading(Graphic3dTypeOfShadingModel::Phong);
        scene.dump(name);
    }

    /// The storage formats of these scripts are not in this kernel: the script stops before them, with no
    /// view of a shape that was not stored and restored.
    fn up_to_storage(name: &'static str) {
        let (_, scene) = common_part(name);
        scene.close();
    }

    fn bug31136_1_script() {
        up_to_storage("bug31136_1");
    }
    obj_case!(
        bug31136_1,
        "tests/bugs/moddata_3/bug31136_1",
        bug31136_1_script
    );

    fn bug31136_2_script() {
        let (read, mut scene) = common_part("bug31136_2");
        let file = case_file("bug31136_2_brep.brep");
        let path = AsciiString::new(file.to_str().unwrap());
        println!("n");
        assert!(brep_tools::write_file(&read, &path, &Default::default()).unwrap());
        let mut restored = Shape::default();
        let builder = martensite_tkbrep::brep::BRepBuilder::default();
        assert!(
            brep_tools::read_file(&mut restored, &path, &builder, &Default::default()).unwrap()
        );
        let (triangles, nodes, _) = crate::checks::mesh_totals(&restored);
        assert_eq!((triangles, nodes), (1228, 1030), "the mesh restored");
        show_stored(&mut scene, &restored, "bug31136_2_brep");
        scene.close();
    }
    obj_case!(
        bug31136_2,
        "tests/bugs/moddata_3/bug31136_2",
        bug31136_2_script
    );

    fn bug31136_3_script() {
        up_to_storage("bug31136_3");
    }
    obj_case!(
        bug31136_3,
        "tests/bugs/moddata_3/bug31136_3",
        bug31136_3_script
    );

    fn bug31136_4_script() {
        up_to_storage("bug31136_4");
    }
    obj_case!(
        bug31136_4,
        "tests/bugs/moddata_3/bug31136_4",
        bug31136_4_script
    );
}

pub mod v3d_materials {
    use super::{Path, ViewScene, case_file, read_obj};
    use crate::obj_case;
    use martensite_tkernel::quantity::Color;
    use martensite_tkservice::graphic3d::{Graphic3dMaterialAspect, Graphic3dNameOfMaterial};
    use std::fmt::Write as _;

    const BOX_VERTS: [[f32; 3]; 8] = [
        [1.0, -1.0, -1.0],
        [1.0, -1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, -1.0, -1.0],
        [1.0, 1.0, -1.0],
        [1.0, 1.0, 1.0],
        [-1.0, 1.0, 1.0],
        [-1.0, 1.0, -1.0],
    ];
    const BOX_QUADS: [[i32; 4]; 6] = [
        [1, 2, 3, 4],
        [5, 8, 7, 6],
        [1, 5, 6, 2],
        [2, 6, 7, 3],
        [3, 7, 8, 4],
        [5, 1, 4, 8],
    ];

    /// `%g` of a value, six significant digits.
    fn g6(value: f64) -> String {
        if value == 0.0 {
            return "0".to_owned();
        }
        let exponent = martensite_cmath::log10(value.abs()).floor() as i32;
        if !(-5..6).contains(&exponent) {
            return format!("{value:e}");
        }
        let decimals = (5 - exponent).max(0) as usize;
        let fixed = format!("{value:.decimals$}");
        if fixed.contains('.') {
            fixed.trim_end_matches('0').trim_end_matches('.').to_owned()
        } else {
            fixed
        }
    }

    fn srgb(color: Color) -> String {
        Color::linear_to_srgb_vec3_f32(color.rgb())
            .map(|channel| g6(f64::from(channel)))
            .join(" ")
    }

    fn write_material(mtl: &mut String, name: &str, material: &Graphic3dMaterialAspect) {
        let _ = writeln!(mtl, "newmtl {name}");
        let _ = writeln!(mtl, "Ka {}", srgb(material.ambient_color()));
        let _ = writeln!(mtl, "Kd {}", srgb(material.diffuse_color()));
        let _ = writeln!(mtl, "Ks {}", srgb(material.specular_color()));
        let _ = writeln!(mtl, "Ns {}", g6(f64::from(material.shininess()) * 1000.0));
        if material.transparency() >= 0.0001 {
            let _ = writeln!(mtl, "Tr {}", g6(f64::from(material.transparency())));
        }
        mtl.push('\n');
    }

    fn write_box(obj: &mut String, name: &str, index: i32, x: i32, y: i32) {
        let _ = writeln!(obj, "g {name}\nusemtl {name}");
        for [vx, vy, vz] in BOX_VERTS {
            let values = [vx + 3.0 * x as f32, vy - 3.0 * y as f32, vz].map(|v| g6(f64::from(v)));
            let _ = writeln!(obj, "v {}", values.join(" "));
        }
        obj.push_str("s off\n");
        for quad in BOX_QUADS {
            let base = 8 * index;
            let values = quad.map(|corner| (corner + base).to_string());
            let _ = writeln!(obj, "f {}", values.join(" "));
        }
        obj.push('\n');
    }

    /// `vlistmaterials file.obj`: one box per standard material, six in a row.
    fn write_material_table(obj_path: &Path) {
        let mtl_path = obj_path.with_extension("mtl");
        let mut obj = format!(
            "mtllib {}\n",
            mtl_path.file_name().unwrap().to_str().unwrap()
        );
        let mut mtl = String::new();
        let count = Graphic3dNameOfMaterial::Default as usize;
        for (index, name) in Graphic3dNameOfMaterial::ALL[..count].iter().enumerate() {
            let material = Graphic3dMaterialAspect::new(*name);
            let text = String::from_utf8_lossy(material.string_name().as_bytes()).into_owned();
            write_material(&mut mtl, &text, &material);
            let index = index as i32;
            write_box(&mut obj, &text, index, index % 6, index / 6);
        }
        std::fs::write(obj_path, obj).unwrap();
        std::fs::write(mtl_path, mtl).unwrap();
    }

    fn materials_script() {
        println!("========");
        println!("A table of the standard colors and materials");
        println!("Generate a table of predefined Materials (named materials)");
        println!("========");
        let mut scene = ViewScene::open("materials");
        scene.axo();
        println!("<a href=materials_table.html>Material Comparison Table</a>");
        let file = case_file("materials.obj");
        write_material_table(&file);
        let document = read_obj(&file);
        scene.xdisplay("D", &document, true);
        scene.fit();
        scene.dump("materials");
        scene.close();
    }
    obj_case!(materials, "tests/v3d/materials/materials", materials_script);
}
