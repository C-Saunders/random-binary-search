use std::collections::HashMap;
mod generators;

use generators::*;

struct TableEntry {
    random: i32,
    split: i32,
}

impl TableEntry {
    fn new() -> Self {
        TableEntry {
            random: 0,
            split: 0,
        }
    }
}

pub fn evaluate(samples: u32, upper_bound: u32) {
    let mut frequency_table = HashMap::with_capacity(upper_bound as usize);
    for _ in 0..samples {
        let split_result = get_from_binary_search(upper_bound);
        let split_counter = frequency_table
            .entry(split_result)
            .or_insert(TableEntry::new());
        split_counter.split += 1;

        let random_result = get_random(upper_bound);
        let random_counter = frequency_table
            .entry(random_result)
            .or_insert(TableEntry::new());
        random_counter.random += 1;
    }

    print_result_csv(&frequency_table);
}

fn print_result_csv(result: &HashMap<u32, TableEntry>) -> () {
    let mut keys = result.keys().collect::<Vec<&u32>>();
    keys.sort();

    println!("Value, Random, Split");
    for key in keys {
        let item = result.get(key).unwrap();
        println!(
            "{}, {}, {}",
            format!("{:width$}", key, width = 3),
            item.random,
            item.split
        );
    }
}
