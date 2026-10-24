//! #197, #269: Windows checkout must preserve snapshot and healing-resource bytes.
use super::TemporaryGitFixture;
use sha2::{Digest, Sha256};

struct FrozenSnapshot {
    path: &'static str,
    bytes: &'static [u8],
    sha256: &'static str,
}

const SNAPSHOTS: &[FrozenSnapshot] = &[
    FrozenSnapshot {
        path: "crates/martensite-tkg3d/tests/fixtures/bezier-initialization.txt",
        bytes: include_bytes!(
            "../../../crates/martensite-tkg3d/tests/fixtures/bezier-initialization.txt"
        ),
        sha256: "a519eed7d67f5bef040c6ec993993b368fc1145e7fd41574bb00ea9d2aa37832",
    },
    FrozenSnapshot {
        path: "crates/martensite-tkg3d/tests/fixtures/surface-grid-reinitialization.txt",
        bytes: include_bytes!(
            "../../../crates/martensite-tkg3d/tests/fixtures/surface-grid-reinitialization.txt"
        ),
        sha256: "ec3633463a322e2a845850c3a3f4697301cb03dd048e205cab56ef47b28c228c",
    },
    FrozenSnapshot {
        path: "crates/martensite-tkg3d/tests/fixtures/geometry-failures.txt",
        bytes: include_bytes!(
            "../../../crates/martensite-tkg3d/tests/fixtures/geometry-failures.txt"
        ),
        sha256: "e05d70c04ef37c2a5f74e8a7c30cd2fe8ab875ee7e745957ade6b43864beff4f",
    },
    FrozenSnapshot {
        path: "crates/martensite-tkg3d/tests/fixtures/surface-cache-lifecycle.txt",
        bytes: include_bytes!(
            "../../../crates/martensite-tkg3d/tests/fixtures/surface-cache-lifecycle.txt"
        ),
        sha256: "c31d5877fa95b59e19af995592116521a369b12954bec01eb45369584e74c36d",
    },
    FrozenSnapshot {
        path: "crates/martensite-tkg3d/tests/fixtures/surface-copy-isolation.txt",
        bytes: include_bytes!(
            "../../../crates/martensite-tkg3d/tests/fixtures/surface-copy-isolation.txt"
        ),
        sha256: "7a47186880fd2dda736b079fcaabd654c26193b76ccb36a0ec8e94f902b5534e",
    },
    FrozenSnapshot {
        path: "crates/martensite-tkshhealing/tests/dataset_cases/data/heal/MDTV",
        bytes: include_bytes!(
            "../../../crates/martensite-tkshhealing/tests/dataset_cases/data/heal/MDTV"
        ),
        sha256: "e92aaecfec8fc0a118e4ad6bc6ce14a0e8bb1b73d8a0c46c083b7bcf268b1e8f",
    },
    FrozenSnapshot {
        path: "crates/martensite-tkshhealing/tests/dataset_cases/data/heal/data",
        bytes: include_bytes!(
            "../../../crates/martensite-tkshhealing/tests/dataset_cases/data/heal/data"
        ),
        sha256: "e465a659f75312a8ddd490d9e8ad4076c057dbccc7c5e327fa542153ae7d05e4",
    },
];

#[test]
fn real_autocrlf_checkout_preserves_frozen_fixture_bytes() {
    let fixture = snapshot_repository();
    fixture.run(&["config", "core.autocrlf", "true"]);
    for snapshot in SNAPSHOTS {
        let path = fixture.directory.path().join(snapshot.path);
        std::fs::remove_file(&path).unwrap();
        fixture.run(&["checkout-index", "--force", "--", snapshot.path]);
        assert_eq!(
            std::fs::read(path).unwrap(),
            snapshot.bytes,
            "{}",
            snapshot.path
        );
    }
}

fn snapshot_repository() -> TemporaryGitFixture {
    let fixture = TemporaryGitFixture::new();
    fixture.write(".gitattributes", include_str!("../../../.gitattributes"));
    fixture.write(".fixture-global-attributes", "");
    fixture.run(&[
        "config",
        "core.attributesFile",
        ".fixture-global-attributes",
    ]);
    fixture.run(&["add", "--", ".gitattributes"]);
    for snapshot in SNAPSHOTS {
        stage_snapshot(&fixture, snapshot);
    }
    fixture
}

fn stage_snapshot(fixture: &TemporaryGitFixture, snapshot: &FrozenSnapshot) {
    // The independent digest catches conversion before include_bytes! on Windows too.
    assert_eq!(
        format!("{:x}", Sha256::digest(snapshot.bytes)),
        snapshot.sha256
    );
    let path = fixture.directory.path().join(snapshot.path);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, snapshot.bytes).unwrap();
    fixture.run(&["add", "--", snapshot.path]);
}
