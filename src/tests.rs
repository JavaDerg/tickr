use crate::Ticker;
use std::time::Duration;

#[test]
fn test1_1sec_thread_sleep() {
    let dur = Duration::new(1, 0);
    let mut ticker = Ticker::new(dur.clone());
    assert!(!ticker.tick());
    std::thread::sleep(dur);
    assert!(ticker.tick());
    assert!(!ticker.tick());
}