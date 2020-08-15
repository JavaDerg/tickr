#[cfg(test)]
mod tests;

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Ticker {
    last: Instant,
    duration: Duration
}

#[deprecated("Use zip instead")]
pub struct Tickerator<T: Iterator> {
    inner: T,
    ticker: Ticker
}

#[allow(deprecated)]
pub trait TickerIter<T: Iterator>  {
    fn ticker(self, duration: Duration) -> Tickerator<T>;
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

#[allow(deprecated)]
impl<T: Iterator> Tickerator<T> {
    pub fn new(iter: T, period: Duration) -> Self {
        Self {
            inner: iter,
            ticker: Ticker::new(period)
        }
    }
}

#[allow(deprecated)]
impl<T: Iterator> Iterator for Tickerator<T> {
    type Item = (T::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(item) => Some((item, self.ticker.tick())),
            None => None
        }
    }
}

#[allow(deprecated)]
impl<T: Iterator> TickerIter<T> for T {
    fn ticker(self, duration: Duration) -> Tickerator<T> {
        Tickerator::new(self, duration)
    }
}
