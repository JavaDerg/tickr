#[cfg(test)]
mod tests;

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Ticker {
    start: Instant,
    last: Instant,
    duration: Duration
}

impl Ticker {
    pub fn new(period: Duration) -> Self {
        let last = Instant::now();
        Self {
            start: last.clone(),
            last,
            duration: period
        }
    }

    pub fn tick(&mut self) -> Option<Duration> {
        match self.last.elapsed() >= self.duration {
            true => {
                self.last = Instant::now();
                Some(self.start.elapsed())
            },
            false => None
        }
    }
}

impl Iterator for Ticker {
    type Item = Option<Duration>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tick())
    }
}
