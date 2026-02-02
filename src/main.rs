use rand::Rng;

#[derive(Debug)]
struct City {
    city: String,
    population: u64,
}

fn sort_pop(cities: &mut Vec<City>) {
    // We can use a closure here
    // cities.sort_by_key(|c| c.population);

    cities.sort_by(|a, b| {
        // First, compare populations descending
        let res = b.population.cmp(&a.population);
        if res == std::cmp::Ordering::Equal {
            // If populations are equal, sort by name ascending
            a.city.cmp(&b.city)
        } else {
            res
        }
    });
}

fn main() {
    let mut rng = rand::rng();

    // Changed to 'let mut' so we can sort it later
    let mut cities = vec![
        City {
            city: "A".to_string(),
            population: rng.random_range(0..=100),
        },
        City {
            city: "B".to_string(),
            population: rng.random_range(0..=100),
        },
        City {
            city: "C".to_string(),
            population: rng.random_range(0..=100),
        },
        City {
            city: "D".to_string(),
            population: rng.random_range(0..=100),
        },
        City {
            city: "E".to_string(),
            population: rng.random_range(0..=100),
        },
        City {
            city: "F".to_string(),
            population: rng.random_range(0..=100),
        },
    ];
    

    // Sort the populated list
    sort_pop(&mut cities);

    // Print the list we actually put data into
    println!("{:#?}", cities);


    let add = |x: i32| -> i32{
        x + 1
    };

    let add_v2 = |x| x + 1;
    add_v2(2);

    let example = |x| x;
    let string = example(String::from("my String"));
    




}
