use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Ticker {
    last: Instant,
    duration: Duration
}

pub struct Tickerator<T: Iterator> {
    inner: T,
    ticker: Ticker
}

pub trait TickerIter<T: Iterator>  {
    fn ticker(self, duration: Duration);
}

impl Ticker {
    pub fn new(period: Duration) -> Self {
        Self {
            last: Instant::now(),
            duration: period
        }
    }

    pub fn tick(&mut self) -> bool {
        let state = self.last.elapsed() >= self.duration;
        if state {
            self.last = Instant::now();
        }
        state
    }
}

impl Iterator for Ticker {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tick())
    }
}

impl<T: Iterator> Tickerator<T> {
    pub fn new(iter: T, period: Duration) -> Self {
        Self {
            inner: iter,
            ticker: Ticker::new(period)
        }
    }
}

impl<T: Iterator> Iterator for Tickerator<T> {
    type Item = (T::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(item) => Some((item, self.ticker.tick())),
            None => None
        }
    }
}

impl<T: Iterator> TickerIter for T {
    fn ticker(self, duration: Duration) -> Tickerator<T> {
        Tickerator::new(self, duration)
    }
}
