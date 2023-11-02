use rand::prelude::*;

#[derive(Clone, Copy)]
pub struct Stone {
    weight: f64
}

impl Stone {
    pub fn weigh(stones: &[Stone]) -> u32 {
        stones.iter()
            .map(|s| s.weight)
            .sum::<f64>()
            .floor() as u32
    }

    pub fn reveal_weight(stone: &Stone) -> f64 { // Meant to be clunky to discourage use
        stone.weight
    }

    pub fn gen_stones(n: usize) -> Vec<Stone> {
        let mut rng = rand::thread_rng();
        (0..n).map(|_| { Stone {weight: rng.gen()} })
        .collect()
    }

    #[allow(dead_code)]
    pub fn gen_stones_sorted(n: usize) -> Vec<Stone> {
        let mut stones = Stone::gen_stones(n);
        stones.sort_by(|a,b| a.weight.total_cmp(&b.weight));
        stones
    }
}
