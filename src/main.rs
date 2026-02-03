use rand::Rng;

#[derive(Debug)]
struct City {
    city: String,
    population: u64,
}

#[derive(Debug)]
struct Item {
    name: String,
}
#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Iterator for Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item> {
    items.into_iter().filter(|i| i.name == product).collect()
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
    // println!("{:#?}", cities);

    let add = |x: i32| -> i32 { x + 1 };

    let add_v2 = |x| x + 1;
    add_v2(2);

    let example = |x| x;
    let string = example(String::from("my String"));

    // let vec = vec![1,2,3,4];

    // for val in vec.iter(){
    //     println!("{}",val)
    // }

    // let vec2 = vec![1, 2, 3];
    // let mut iter = (&vec2).into_iter();

    // while let Some(v) = iter.next() {
    //     println!("{}", v)
    // }

    let mut vec: Vec<Item> = Vec::new();
    vec.push(Item {
        name: "Miani".to_string(),
    });
    vec.push(Item {
        name: "GUini".to_string(),
    });
    vec.push(Item {
        name: "Lanini".to_string(),
    });
    vec.push(Item {
        name: "Rueini".to_string(),
    });

    let checked = check_inventory(vec, "Lanini".to_string());
    println!("{:?}", checked);


    let range = Range{
        start: 0,
        end: 10
    };

    let vec: Vec<u32> = range.filter(|x| x %2 == 0).collect();
    println!("{:?}",vec)


    // for r in range {
    //     println!("{}",r);
    // }



}
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;

//     // many default methods
// }
