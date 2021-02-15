use math::round;
use rand::Rng;

pub fn get_from_binary_search(upper_bound: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let mut lower = 1;
    let mut upper = upper_bound;
    while lower != upper {
        let coin_flip = rng.gen_bool(0.50);
        let halfway_point = lower + round::stochastic((upper - lower) as f64 / 2.0, 0) as u32;
        if coin_flip {
            upper = halfway_point;
        } else {
            lower = halfway_point;
        }
    }

    upper
}

pub fn get_random(upper_bound: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=upper_bound)
}
