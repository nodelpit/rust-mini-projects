#[derive(Debug)]
struct Fighter {
    name: String,
    attack_power: u8,
    current_hp: u16,
    max_hp: u16,
}

#[derive(Debug)]
enum Action {
    Attack,
    Defend,
    Heal,
    Quit,
}

#[derive(Debug)]
enum ActionError {
    InvalidInput,
}

fn parse_action(input: &str) -> Result<Action, ActionError> {
    match input.trim().to_lowercase().as_str() {
        "attack" => Ok(Action::Attack),
        "defend" => Ok(Action::Defend),
        "heal" => Ok(Action::Heal),
        "quit" => Ok(Action::Quit),
        _ => Err(ActionError::InvalidInput),
    }
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

    println!("{:?}", parse_action("attack"));
    println!("{:?}", parse_action("attack  "));
    println!("{:?}", parse_action("Attack"));
    println!("{:?}", parse_action("Attzc zzk"));
    println!("{:?}", parse_action("zd"));
    println!("{:?}", parse_action(""));
}
