use std::collections::HashMap;

use super::read_strs;

fn read_fish(path: &str) -> HashMap<u8, usize> {
    let lines = read_strs(path);
    // split first line on comma

    let mut fishes = HashMap::new();
    let timers = lines[0].split(',');
    for timer in timers {
        // increase the number of fish with this timer
        let time = timer.parse::<u8>().unwrap();
        let fish = fishes.entry(time).or_insert(0);
        *fish += 1;
    }

    fishes
}

fn print_fish(fishes: &HashMap<u8, usize>, label: &str) {
    // sum all the values in the hashmap
    let sum: usize = fishes.values().sum();
    println!("{}: ==> total {}", label, sum);
}

fn progress_fishes(fishes: &mut HashMap<u8, usize>, time: u32) {
    // print day and fish list
    print_fish(fishes, "Initial state");

    if time == 0 {
        return;
    }

    for t in 0..time {
        // new school
        let mut new_school = HashMap::new();

        let mut birthing_fish = 0;

        // give birth to new fish
        if fishes.contains_key(&0) {
            birthing_fish = *fishes.get(&0).unwrap();
        }

        // birhting is done, loop through the rest of the fishes and lower their timers
        for i in 1..9 {
            let fish = fishes.entry(i).or_insert(0);
            // these fish will be older in the next cycle
            new_school.insert(i - 1, *fish);
        }

        // insert the birthed fish
        new_school.insert(8, birthing_fish);
        // add the resetted fish
        let reset_fish = new_school.entry(6).or_insert(0);
        *reset_fish += birthing_fish;

        *fishes = new_school;

        print_fish(fishes, format!("After {} days", t).as_str());
    }
}

pub fn day6() {
    let mut fishes = read_fish("input/day6.txt");
    progress_fishes(&mut fishes, 256);
    // show number of fish
    print_fish(&fishes, "Final population")
}
