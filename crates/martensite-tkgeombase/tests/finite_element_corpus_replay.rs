//! Fixed original process partitions preserve the reference-criterion cache contract.
#[path = "support/construction_corpus/mod.rs"]
mod construction_corpus;
#[path = "support/fem_corpus/mod.rs"]
mod fem_corpus;

#[test]
fn all_reference_matrix_shards_match_frozen_answers() {
    if let Some(shard) = fem_corpus::child_shard().unwrap() {
        let receipt = fem_corpus::replay_shard(shard);
        print!("\n{}", receipt.protocol());
        return;
    }
    let process = fem_corpus::CurrentTestProcess::new(std::env::current_exe().unwrap());
    let first = fem_corpus::run_original_shards(&process).unwrap();
    let second = fem_corpus::run_original_shards(&process).unwrap();
    assert_eq!(
        first, second,
        "Every byte of all 64 fresh-child outputs must repeat"
    );
    assert_eq!(
        first.iter().map(|receipt| receipt.rows).sum::<usize>(),
        1230
    );
    println!("FEM_FIXED_PARTITION_PASS:64:1230:2");
}
