#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Peak {
    Low,
    High,
}

pub struct PeaksIter<I, F> {
    source: I,
    signal: F,
    threshold: f64,
    influence: f64,
    window: Vec<f64>,
}

pub trait PeaksDetector<I> where I: Iterator {
    fn peaks<F>(self, lag: usize, threshold: f64, influence: f64, signal: F) -> PeaksIter<I, F>
        where F: FnMut(&I::Item) -> f64;
}

impl<I> PeaksDetector<I> for I where I: Iterator {
    fn peaks<F>(self, lag: usize, threshold: f64, influence: f64, signal: F) -> PeaksIter<I, F>
        where F: FnMut(&I::Item) -> f64
    {
        PeaksIter {
            source: self,
            signal,
            threshold,
            influence,
            window: Vec::with_capacity(lag),
        }
    }
}

impl<I, F> Iterator for PeaksIter<I, F> where I: Iterator, F: FnMut(&I::Item) -> f64 {
    type Item = (I::Item, Peak);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.source.next() {
                let value = (self.signal)(&item);
                if self.window.len() < self.window.capacity() {
                    self.window.push(value);
                } else {
                    let (mean, stddev) = self.stats();
                    if (value - mean).abs() > (self.threshold * stddev) {
                        let peak = if value > mean { Peak::High } else { Peak::Low };
                        let next_value =
                            (value * self.influence) + ((1. - self.influence) * self.window.last().cloned().unwrap());
                        self.window.remove(0);
                        self.window.push(next_value);
                        return Some((item, peak));
                    } else {
                        self.window.remove(0);
                        self.window.push(value);
                    }
                }
            } else {
                return None;
            }
        }
    }
}

impl<I, F> PeaksIter<I, F> {
    fn stats(&self) -> (f64, f64) {
        let (mut mean, mut stddev) = (0., 0.);
        if !self.window.is_empty() {
            for &v in self.window.iter() {
                mean += v;
            }
            mean /= self.window.len() as f64;
            let sq_sum = self.window
                .iter()
                .fold(0., |a, v| a + ((v - mean) * (v - mean)));
            stddev = (sq_sum / self.window.len() as f64).sqrt();
        }
        (mean, stddev)
    }
}

#[cfg(test)]
mod tests {
    use super::{Peak, PeaksDetector};

    #[test]
    fn sample_data() {
        let input = vec![
            1.0, 1.0, 1.1, 1.0, 0.9, 1.0, 1.0, 1.1, 1.0, 0.9, 1.0, 1.1, 1.0, 1.0, 0.9, 1.0, 1.0, 1.1, 1.0,
            1.0, 1.0, 1.0, 1.1, 0.9, 1.0, 1.1, 1.0, 1.0, 0.9, 1.0, 1.1, 1.0, 1.0, 1.1, 1.0, 0.8, 0.9, 1.0,
            1.2, 0.9, 1.0, 1.0, 1.1, 1.2, 1.0, 1.5, 1.0, 3.0, 2.0, 5.0, 3.0, 2.0, 1.0, 1.0, 1.0, 0.9, 1.0,
            1.0, 3.0, 2.6, 4.0, 3.0, 3.2, 2.0, 1.0, 1.0, 0.8, 4.0, 4.0, 2.0, 2.5, 1.0, 1.0, 1.0
        ];
        let output: Vec<_> = input
            .into_iter()
            .enumerate()
            .peaks(30, 5.0, 0.0, |e| e.1)
            .map(|((i, _), p)| (i, p))
            .collect();
        assert_eq!(output, vec![
            (45, Peak::High), (47, Peak::High), (48, Peak::High), (49, Peak::High),
            (50, Peak::High), (51, Peak::High), (58, Peak::High), (59, Peak::High),
            (60, Peak::High), (61, Peak::High), (62, Peak::High), (63, Peak::High),
            (67, Peak::High), (68, Peak::High), (69, Peak::High), (70, Peak::High),
        ]);
    }
}
