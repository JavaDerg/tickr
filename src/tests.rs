use crate::Ticker;
use std::time::Duration;

#[test]
fn test1_1sec_thread_sleep() {
    let dur = Duration::new(1, 0);
    let mut ticker = Ticker::new(dur.clone());
    assert!(ticker.tick().is_none());
    std::thread::sleep(dur);
    assert!(ticker.tick().is_some());
    assert!(ticker.tick().is_none());
}