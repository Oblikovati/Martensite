//! The exact public dataset members (release 7.9.0) the primitive regression cases
//! restore.
//!
//! Taken from the dataset archive of release 7.9.0 (archive SHA-256
//! a92ed91c3271c299287c1c404bb9454d463251094bfc848acc44aee60e6a026c). One row per member: file name, directory under
//! `dataset-7.9.0/` of the `external/martensite-test-data` submodule, size in bytes and
//! SHA-256. A row with `path=` names a file outside the dataset folder: bug_gh469.brep is
//! the unchanged `cutCompound2.shape` of an issue attachment
//! (`github-issues/`, see its SOURCES.json).

/// `name directory bytes sha256` per pinned member, sorted by name.
pub const PINNED_MEMBERS: &str = "\
OCC1243-1.brep brep 1817 247d47663038c851f7412f38ba58d3351cd7af6d021ce0667f2840d76decc195
OCC1243-2.brep brep 3935 9a830cd09c12d00460c691906dc1a22b3e1e65f791640e235485eb11b00efc4e
OCC23165-curve.rle brep 4716 9621513c446338566d2d8ed2b8722e18514ba2f739f88f5a23afa86ab6eacd37
OCC23165-edge1.brep brep 8851 92dac378e675d118363daa5ed85cd1522250e3af17610304dfe4de18e61a2d17
OCC26016-revolve-crash.brep brep 845 d0d1ae0e5fcd3ab7fdfa38bc85ff8451980c6fb0089b6c02596bac6193fda2e3
OCC26387-Paraboloid.brep brep 42019 20d1e960ebf0b9e8382d61b3aa7c096022155559ff7ad0c068935a7f5b277807
OCC26406-Paraboloid.brep brep 38478 19000e03d6a1108b81a000306a5f79ffb8705000e6107b993729947ac6ac40bd
OCC615.brep brep 82800 d6329e221c2e1b5462cbac3129f2f89605c64d100fcb7178c7bf6ffbeed7bdc2
buc60684a.brep brep 2270 f267c2f6c0481d6129c5ddb136c35b4c79d05ee7f1a4a2ffe9cecbaad769ba78
bug23849_segment_2.brep brep 1121215 ac34f3c750ec345397e68c9e54a5acbc03d909757e2565444455baa37e6e8378
bug25631_fbx.brep brep 70186 8e5338df28b77dd5e936d2eb4d402705cf9e2ae92db2ad88c0eedefceef2a418
bug26516_tmp.brep brep 782 c87b1c644683ee800f95885304669b8945fcf395ce7d97559a5c0c84e0b3d26b
bug26560_planarspline.brep brep 758 be31bd5388c13d0ecec8b5ce82b53549b1e68b5acad0c7c9e13df6ad8f98176f
bug28003_Surf_Bezier.brep brep 1813 ee121192be7865bbf74fd3d68ea5f571ce6f243c322803c5063e341cf288db5a
bug28402_hugeassembly.brep brep 46366 6dda75ac4cfb78b0865edf23760254eb31983b469110fad517907462d0e2084d
bug31995.brep brep 9179 f80f4d7e3add11b13fd8e27fc84149f662ffb24417e03bc253e1b88420dbc160
bug61037.brep brep 29750 a3f8a0399f33a3be067b3e53de34931f863eedc048e01cc81dc0bc81e9fbdc3d
bug_gh469.brep github-issues 418891 222f67f07136a5dc1475ef9f23acfce3e3840d1ab47b8452b834dcdcf878861c path=github-issues/OCCT-469/cutCompound2.shape";
