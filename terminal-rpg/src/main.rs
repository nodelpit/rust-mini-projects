#[derive(Debug)]
struct Fighter {
    name: String,
    attack_power: u8,
    current_hp: u16,
    max_hp: u16,
}

impl Fighter {
    fn new(name: &str, attack_power: u8, current_hp: u16, max_hp: u16) -> Self {
        Self {
            name: name.to_string(),
            attack_power,
            current_hp,
            max_hp,
        }
    }
}

fn main() {
    let hero = Fighter::new("Rustler", 12, 100, 100);
    let monster = Fighter::new("Pythor", 15, 90, 90);

    println!("{:?}", hero);
    println!("{:?}", monster);
}