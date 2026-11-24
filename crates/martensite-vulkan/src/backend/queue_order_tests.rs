use super::*;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn a_turn_without_earlier_tickets_is_taken_at_once() {
    let order = Arc::new(QueueOrder::default());
    drop(order.next_turn());
    drop(order.next_turn());
    let tickets = order.tickets.lock().unwrap();
    assert_eq!((tickets.issued, tickets.served), (2, 2));
}

#[test]
fn a_later_host_turn_waits_for_a_reserved_turn_on_another_thread() {
    let order = Arc::new(QueueOrder::default());
    let reserved = order.reserve();
    let (sender, events) = mpsc::channel();
    let host = {
        let order = Arc::clone(&order);
        let sender = sender.clone();
        std::thread::spawn(move || {
            let _turn = order.next_turn();
            sender.send("host").unwrap();
        })
    };
    std::thread::sleep(Duration::from_millis(50));
    let turn = reserved.enter();
    sender.send("reserved").unwrap();
    drop(turn);
    host.join().unwrap();
    assert_eq!(
        events.iter().take(2).collect::<Vec<_>>(),
        ["reserved", "host"]
    );
}

#[test]
fn an_unused_reservation_releases_its_turn_when_dropped() {
    let order = Arc::new(QueueOrder::default());
    let reserved = order.reserve();
    let later = {
        let order = Arc::clone(&order);
        std::thread::spawn(move || drop(order.next_turn()))
    };
    drop(reserved);
    later.join().unwrap();
    assert_eq!(order.tickets.lock().unwrap().served, 2);
}
