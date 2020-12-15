use num_format::{Locale, ToFormattedString};

pub fn part1(input: String) {
    let split: Vec<&str> = input.split("\n").collect();
    let earliest: usize = split[0].parse().unwrap();
    let buses: Vec<usize> = split[1]
        .split(",")
        .filter_map(|bus| bus.parse().ok())
        .collect();

    let mut timestamp = earliest;

    loop {
        let mut found = false;
        for bus_id in &buses {
            if timestamp % bus_id == 0 {
                println!("{}", bus_id * (timestamp - earliest));
                found = true;
                break;
            }
        }

        if found {
            break;
        }

        timestamp += 1;
    }
}

pub fn part2(input: String) {
    let split: Vec<&str> = input.split("\n").collect();
    let earliest: usize = split[0].parse().unwrap();

    // let mut timestamp = earliest;

    let locale = Locale::en;
    let mut iter: Vec<(usize, usize)> = split[1]
        .split(",")
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|x| (x.0, x.1.parse().unwrap()))
        .collect();
    let len = iter.len();

    // Sort to get largest number first at index 0
    iter.sort_unstable_by_key(|x| x.1);
    iter.reverse();

    let biggest = iter[0];
    // let mut timestamp = earliest;
    let mut timestamp = 100_000_000_000_000;

    while (timestamp + biggest.0) % biggest.1 != 0 {
        timestamp += 1;
    }

    let mut found = 0;

    while found == 0 {
        // Print timestamp every 50,000,000 numbers checked
        if (timestamp + biggest.0) % (50_000_000 * biggest.1) == 0 {
            println!("{}", timestamp.to_formatted_string(&locale));
        }

        // We know the first element is a multiple of the timestamp
        for (i, bus_id) in &iter[1..] {
            if (timestamp + i) % bus_id == 0 {
                found += 1;
            } else {
                break;
            }
        }

        // Minus 1 because iterator skips first element
        if found == len - 1 {
            break;
        }

        timestamp += biggest.1;
        found = 0;
    }

    println!("{}", timestamp);
}

pub fn part2_slow(input: String) {
    let split: Vec<&str> = input.split("\n").collect();
    let earliest: usize = split[0].parse().unwrap();

    let buses: Vec<&str> = split[1].split(",").collect();

    // let mut timestamp = earliest;
    let mut timestamp = 100_000_000_000_000;

    let locale = Locale::en;
    let iter: Vec<(usize, usize)> = buses
        .into_iter()
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|x| (x.0, x.1.parse().unwrap()))
        .collect();
    let len = iter.len();
    let mut found = 0;

    while found == 0 {
        if timestamp % 500_000_000 == 0 {
            println!("{}", timestamp.to_formatted_string(&locale));
        }

        for (i, bus_id) in &iter {
            if (timestamp + i) % bus_id == 0 {
                found += 1;
            } else {
                break;
            }
        }

        if found == len {
            break;
        }

        timestamp += 1;
        found = 0;
    }

    println!("{}", timestamp);
}