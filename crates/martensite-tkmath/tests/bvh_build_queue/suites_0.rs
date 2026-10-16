// =============================================================================
// BvhBuildQueue Basic Tests
// =============================================================================

#[test]
fn hierarchy_build_queue_default_constructor() {
    let mut aQueue = BvhBuildQueue::new();

    assert_eq!(aQueue.size(), 0);
    assert!(!aQueue.has_busy_threads());
}

#[test]
fn hierarchy_build_queue_enqueue_single() {
    let mut aQueue = BvhBuildQueue::new();

    aQueue.enqueue(42);

    assert_eq!(aQueue.size(), 1);
}

#[test]
fn hierarchy_build_queue_enqueue_multiple() {
    let mut aQueue = BvhBuildQueue::new();

    aQueue.enqueue(1);
    aQueue.enqueue(2);
    aQueue.enqueue(3);

    assert_eq!(aQueue.size(), 3);
}

#[test]
fn hierarchy_build_queue_fetch_from_empty_queue() {
    let mut aQueue = BvhBuildQueue::new();
    let mut wasBusy = false;
    let mut aResult = aQueue.fetch(&mut wasBusy);

    assert_eq!(aResult, -1);
    assert!(!wasBusy);
    assert!(!aQueue.has_busy_threads());
}

#[test]
fn hierarchy_build_queue_fetch_single_item() {
    let mut aQueue = BvhBuildQueue::new();
    aQueue.enqueue(42);

    let mut wasBusy = false;
    let mut aResult = aQueue.fetch(&mut wasBusy);

    assert_eq!(aResult, 42);
    assert!(wasBusy);
    assert!(aQueue.has_busy_threads()); // Thread became busy
    assert_eq!(aQueue.size(), 0);
}

#[test]
fn hierarchy_build_queue_fetch_fifo_order() {
    let mut aQueue = BvhBuildQueue::new();

    aQueue.enqueue(10);
    aQueue.enqueue(20);
    aQueue.enqueue(30);

    let mut wasBusy = false;

    assert_eq!(aQueue.fetch(&mut wasBusy), 10);
    assert!(wasBusy);

    assert_eq!(aQueue.fetch(&mut wasBusy), 20);
    assert!(wasBusy);

    assert_eq!(aQueue.fetch(&mut wasBusy), 30);
    assert!(wasBusy);

    assert_eq!(aQueue.fetch(&mut wasBusy), -1);
    assert!(!wasBusy);
}

#[test]
fn hierarchy_build_queue_thread_count_tracking() {
    let mut aQueue = BvhBuildQueue::new();

    aQueue.enqueue(1);
    aQueue.enqueue(2);

    assert!(!aQueue.has_busy_threads());

    let mut wasBusy = false;

    // First fetch - thread becomes busy
    aQueue.fetch(&mut wasBusy);
    assert!(wasBusy);
    assert!(aQueue.has_busy_threads());

    // Second fetch while already busy
    aQueue.fetch(&mut wasBusy);
    assert!(wasBusy);
    assert!(aQueue.has_busy_threads());

    // Fetch from empty queue while busy
    aQueue.fetch(&mut wasBusy);
    assert!(!wasBusy);
    assert!(!aQueue.has_busy_threads());
}

#[test]
fn hierarchy_build_queue_alternating_enqueue_fetch() {
    let mut aQueue = BvhBuildQueue::new();

    aQueue.enqueue(1);
    assert_eq!(aQueue.size(), 1);

    let mut wasBusy = false;
    assert_eq!(aQueue.fetch(&mut wasBusy), 1);
    assert_eq!(aQueue.size(), 0);

    aQueue.enqueue(2);
    assert_eq!(aQueue.size(), 1);

    assert_eq!(aQueue.fetch(&mut wasBusy), 2);
    assert_eq!(aQueue.size(), 0);
}

#[test]
fn hierarchy_build_queue_large_queue() {
    let mut aQueue = BvhBuildQueue::new();

    let aCount: i32 = 1000;
    for i in 0..aCount {
        aQueue.enqueue(i);
    }

    assert_eq!(aQueue.size(), aCount);

    let mut wasBusy = false;
    for i in 0..aCount {
        assert_eq!(aQueue.fetch(&mut wasBusy), i);
    }

    assert_eq!(aQueue.size(), 0);
}

#[test]
fn hierarchy_build_queue_negative_values() {
    let mut aQueue = BvhBuildQueue::new();

    aQueue.enqueue(-1);
    aQueue.enqueue(-100);
    aQueue.enqueue(0);

    let mut wasBusy = false;

    assert_eq!(aQueue.fetch(&mut wasBusy), -1);
    assert_eq!(aQueue.fetch(&mut wasBusy), -100);
    assert_eq!(aQueue.fetch(&mut wasBusy), 0);
}
