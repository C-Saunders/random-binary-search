use clap::{App, Arg};
use random_binary_search::evaluate;

fn main() {
    let matches = App::new("Random Binary Search Simulator")
        .version("0.1.0")
        .author("Charlie S. <charlieasaunders@gmail.com>")
        .about(
            "Generates data from a random binary search and uniform random values for comparison. Prints CSV.",
        )
        .arg(
            Arg::with_name("samples")
                .short("s")
                .help("Number of samples to take.")
                .default_value("10000000")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max_value")
                .short("m")
                .help("Generate values between 1 and this value.")
                .default_value("100")
                .takes_value(true),
        )
        .get_matches();

    evaluate(
        matches.value_of("samples").unwrap().parse::<u32>().unwrap(),
        matches
            .value_of("max_value")
            .unwrap()
            .parse::<u32>()
            .unwrap(),
    )
}
