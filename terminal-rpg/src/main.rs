use std::io::stdin;

#[allow(dead_code)]
#[derive(Debug)]
struct Fighter {
    name: String,
    attack_power: u8,
    current_hp: u16,
    max_hp: u16,
    is_defending: bool,
    heal_charges: u8,
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
    fn new(
        name: &str,
        attack_power: u8,
        current_hp: u16,
        max_hp: u16,
        is_defending: bool,
        heal_charges: u8,
    ) -> Self {
        Self {
            name: name.to_string(),
            attack_power,
            current_hp,
            max_hp,
            is_defending,
            heal_charges,
        }
    }

    fn take_damage(&mut self, amount: u16) {
        if self.is_defending {
            self.current_hp = self.current_hp.saturating_sub(amount / 2);
        } else {
            self.current_hp = self.current_hp.saturating_sub(amount);
        }

        self.is_defending = false
    }

    fn heal(&mut self, amount: u16) -> Result<(), String> {
        if self.heal_charges > 0 {
            self.heal_charges -= 1;
            self.current_hp = (self.current_hp + amount).min(self.max_hp);
            Ok(())
        } else {
            Err("No heal remaining".to_string())
        }
    }

    fn defend(&mut self) {
        self.is_defending = true;
    }

    fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
}

fn apply_action(hero: &mut Fighter, monster: &mut Fighter, action: Action) -> String {
    match action {
        Action::Attack => {
            monster.take_damage(hero.attack_power.into());
            "You attacked a monster !".to_string()
        }

        Action::Defend => {
            hero.defend();
            "You defended yourself of the monster".to_string()
        }

        Action::Heal => match hero.heal(20) {
            Ok(()) => "You healed yourself".to_string(),
            Err(msg) => msg,
        },

        Action::Quit => "See you in another hell..".to_string(),
    }
}

fn main() {
    let mut hero = Fighter::new("Rustler", 12, 100, 100, false, 3);
    let mut monster = Fighter::new("Pythor", 15, 90, 90, false, 0);

    println!("\n");
    println!("=========== TERMINAL-RPG ===========");

    loop {
        println!("What action do you want to perform");

        let mut input = String::new();

        let _ = stdin().read_line(&mut input).expect("Failed to read input");
        let parsed_result = parse_action(&input);
        println!("\n");

        match parsed_result {
            Ok(action) => match action {
                Action::Quit => break,
                other_action => {
                    println!("{}", apply_action(&mut hero, &mut monster, other_action));
                    println!("Monster remaining HP: {}", monster.current_hp);
                    println!("\n");

                    if monster.is_alive() {
                        hero.take_damage(monster.attack_power.into());
                        println!("A monster attacked you !");
                        println!("Your remaining HP: {}", hero.current_hp);
                        println!("===========\n");
                    }
                }
            },
            Err(_) => println!("Unknow action, please try again."),
        }

        if hero.current_hp == 0 {
            println!("GAME OVER");
            break;
        } else if monster.current_hp == 0 {
            println!("YOU WIN BABY");
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_take_damage_normal() {
        let mut fighter = Fighter::new("Test", 10, 100, 100, false, 3);
        fighter.take_damage(30);
        assert_eq!(fighter.current_hp, 70);
    }

    #[test]
    fn test_take_damage_clamp() {
        let mut fighter = Fighter::new("Test", 10, 10, 100, false, 3);
        fighter.take_damage(30);
        assert_eq!(fighter.current_hp, 0);
    }

    #[test]
    fn test_take_damage_with_defending() {
        let mut fighter = Fighter::new("Test", 10, 100, 100, false, 3);
        fighter.defend();
        fighter.take_damage(30);
        assert_eq!(fighter.current_hp, 85);
    }

    #[test]
    fn test_heal_normal() {
        let mut fighter = Fighter::new("Test", 10, 10, 100, false, 3);
        fighter.heal(20);
        assert_eq!(fighter.current_hp, 30);
    }

    #[test]
    fn test_heal_clamp() {
        let mut fighter = Fighter::new("Test", 10, 90, 100, false, 3);
        fighter.heal(20);
        assert_eq!(fighter.current_hp, 100);
    }

    #[test]
    fn test_heal_with_0_charges() {
        let mut fighter = Fighter::new("Test", 10, 90, 100, false, 0);
        fighter.heal(20);
        assert_eq!(fighter.current_hp, 90);
    }

    #[test]
    fn test_apply_action_attack() {
        let mut hero = Fighter::new("Rustler", 12, 100, 100, false, 3);
        let mut monster = Fighter::new("Pythor", 15, 90, 90, false, 3);

        apply_action(&mut hero, &mut monster, Action::Attack);

        assert_eq!(monster.current_hp, 78);
    }
}
