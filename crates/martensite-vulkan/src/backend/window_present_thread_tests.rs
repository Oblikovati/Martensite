use super::*;

#[test]
fn a_job_that_unwinds_still_finishes_its_receipt_with_an_error() {
    let receipt = Arc::new(PresentReceipt::default());
    drop(ReceiptWriter(Some(Arc::clone(&receipt))));
    let (chain, failed) = receipt.take_chain();
    assert!(chain.is_none() && failed);
    let (commands, outcome) = receipt.take_commands();
    assert!(commands.is_none());
    assert!(
        outcome
            .unwrap_err()
            .to_string()
            .contains("presentation thread")
    );
}

#[test]
fn a_deferred_error_is_reported_once_and_the_failure_stays_visible() {
    let receipt = PresentReceipt::default();
    receipt.finish(PresentedFrame {
        finished: true,
        failed: true,
        outcome: Some(Err(stopped_thread())),
        ..Default::default()
    });
    assert!(receipt.take_commands().1.is_err());
    assert!(receipt.take_commands().1.is_ok());
    assert!(receipt.take_chain().1);
}

#[test]
fn a_waiter_on_another_thread_wakes_when_the_job_finishes() {
    let receipt = Arc::new(PresentReceipt::default());
    let waiter = {
        let receipt = Arc::clone(&receipt);
        std::thread::spawn(move || receipt.take_chain().1)
    };
    std::thread::sleep(std::time::Duration::from_millis(20));
    receipt.finish(PresentedFrame {
        finished: true,
        ..Default::default()
    });
    assert!(!waiter.join().unwrap());
}
