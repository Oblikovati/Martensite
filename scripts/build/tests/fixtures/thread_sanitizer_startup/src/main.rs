fn main() {
    eprintln!("{{\"sanitizer_startup\":\"main_entered\"}}");
    let worker = std::thread::spawn(|| 42u32);
    assert_eq!(worker.join().unwrap(), 42);
    println!("{{\"sanitizer_startup\":\"completed\",\"joined_value\":42}}");
}
