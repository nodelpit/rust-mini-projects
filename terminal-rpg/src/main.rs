#[allow(dead_code)]
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

    fn take_damage(&mut self, amount: u16) {
        self.current_hp = self.current_hp.saturating_sub(amount);
    }

    fn heal(&mut self, amount: u16) {
        self.current_hp = (self.current_hp + amount).min(self.max_hp);
    }

    fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
}

fn apply_action(hero: &mut Fighter, monster: &mut Fighter, action: Action) -> String {
    match action {
        Action::Attack => {
            monster.take_damage(hero.attack_power.into());
            "You attacked a monster".to_string()
        }

        Action::Defend => {
            // TODO ticket 6
            "You defended yourself of the monster".to_string()
        }

        Action::Heal => {
            hero.heal(15);
            "You healed yourself".to_string()
        }

        Action::Quit => "See you in another hell..".to_string(),
    }
}

fn main() {
    let mut hero = Fighter::new("Rustler", 12, 100, 100);
    let mut monster = Fighter::new("Pythor", 15, 90, 90);

    println!("{:?}", hero);
    println!("{:?}", monster);

    println!("\n");

    println!("{:?}", parse_action("attack"));
    println!("{:?}", parse_action("attack  "));
    println!("{:?}", parse_action("Attack"));
    // println!("{:?}", parse_action("Attzc zzk"));
    // println!("{:?}", parse_action("zd"));
    // println!("{:?}", parse_action(""));

    println!("\n");

    println!(
        "{:?}",
        apply_action(&mut hero, &mut monster, Action::Attack)
    );

    println!("He has left {:?}HP", monster.current_hp);

    println!("\n");
    monster.take_damage(hero.attack_power.into());

    println!("Hero encore en vie ? -> {:?}", hero.is_alive());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_take_damage_normal() {
        let mut fighter = Fighter::new("Test", 10, 100, 100);
        fighter.take_damage(30);
        assert_eq!(fighter.current_hp, 70);
    }

    #[test]
    fn test_take_damage_clamp() {
        let mut fighter = Fighter::new("Test", 10, 10, 100);
        fighter.take_damage(30);
        assert_eq!(fighter.current_hp, 0);
    }

    #[test]
    fn test_heal_normal() {
        let mut fighter = Fighter::new("Test", 10, 10, 100);
        fighter.heal(20);
        assert_eq!(fighter.current_hp, 30);
    }

    #[test]
    fn test_heal_clamp() {
        let mut fighter = Fighter::new("Test", 10, 90, 100);
        fighter.heal(20);
        assert_eq!(fighter.current_hp, 100);
    }

    #[test]
    fn test_apply_action_attack() {
        let mut hero = Fighter::new("Rustler", 12, 100, 100);
        let mut monster = Fighter::new("Pythor", 15, 90, 90);

        apply_action(&mut hero, &mut monster, Action::Attack);

        assert_eq!(monster.current_hp, 78);
    }
}
