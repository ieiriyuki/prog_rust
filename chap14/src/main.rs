struct City {
    name: String,
    population: i64,
    country: String,
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

use std::thread;
fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic)
    -> thread::JoinHandle<Vec<City>>
{
    // get_statisticは実装されていない
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

fn main() {
    let tokyo = City {name: "tokyo".to_string(), population: 10000000, country: "japan".to_string()};
    let newyork = City {name: "new york".to_string(), population: 20000000, country: "united states".to_string()};
    let mut cities = vec![tokyo, newyork];
    sort_cities(&mut cities);
    println!("Hello, world!");
}
