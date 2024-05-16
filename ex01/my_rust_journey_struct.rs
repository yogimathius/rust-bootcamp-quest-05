#[derive(Debug)]
struct NationalPark {
    name: String,
}

#[derive(Debug)]
struct City {
    name: String,
}

#[derive(Debug)]
struct State {
    name: String,
    population: u32,
    capital: City,
    national_park: Vec<NationalPark>,
}

fn main() {
    let washington = State {
        name: String::from("Washington"),
        population: 7_615_000,
        capital: City {
            name: String::from("Olympia"),
        },
        national_park: vec![
            NationalPark {
                name: String::from("Olympic"),
            },
            NationalPark {
                name: String::from("North Cascade"),
            },
            NationalPark {
                name: String::from("Rainier"),
            },
        ],
    };

    println!("{:?}", washington);
}
