# Smoothed z-score peaks detector

## Description

Rust implementation for [Smoothed z-score algorithm](https://stackoverflow.com/questions/22583391/peak-recognition-in-realtime-timeseries-data/22640362#22640362).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
smoothed_z_score = "0.1"
```

and this to your crate root:

```rust
extern crate smoothed_z_score;
```

## Example usage

Consider this dataset (from the original stackoverflow reply):

![sample dataset](https://i.stack.imgur.com/KdpF7.jpg)

```rust
    use smoothed_z_score::{Peak, PeaksDetector};

    fn main() {
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
```