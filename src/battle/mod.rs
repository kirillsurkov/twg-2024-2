mod ability;
mod card;
mod effect;
mod modifier;

use ability::{Ability, Attack};
use effect::{Data, HasEffect};
use modifier::Modifier;

pub struct Hero {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub hp: f32,
    pub mana_regen: f32,
    pub attack: f32,
    pub attack_speed: f32,
    pub crit: f32,
    pub evasion: f32,
    pub abilities: Vec<Box<dyn HasEffect + Send + Sync>>,
}

pub fn nulch() -> Hero {
    Hero {
        id: "nulch",
        name: "Нульч",
        desc: "Организатор конкурса",
        hp: 1100.0,
        mana_regen: 10.0,
        attack: 25.0,
        attack_speed: 0.95,
        crit: 0.16,
        evasion: 0.15,
        abilities: vec![Ability::<Attack>::new().into()],
    }
}

pub fn rasp() -> Hero {
    Hero {
        id: "rasp",
        name: "Расп",
        desc: "Это твоя мамка, анон",
        hp: 1200.0,
        mana_regen: 10.0,
        attack: 22.0,
        attack_speed: 1.04,
        crit: 0.11,
        evasion: 0.18,
        abilities: vec![Ability::<Attack>::new().into()],
    }
}

pub fn dtyan() -> Hero {
    Hero {
        id: "dtyan",
        name: "Деревотян",
        desc: "Вырасти дерево, если сможешь!",
        hp: 1400.0,
        mana_regen: 10.0,
        attack: 34.0,
        attack_speed: 0.75,
        crit: 0.20,
        evasion: 0.08,
        abilities: vec![Ability::<Attack>::new().into()],
    }
}

pub fn dimas() -> Hero {
    Hero {
        id: "dimas",
        name: "Димасик",
        desc: "Организатор предыдущего конкурса",
        hp: 1350.0,
        mana_regen: 11.0,
        attack: 35.0,
        attack_speed: 0.71,
        crit: 0.17,
        evasion: 0.09,
        abilities: vec![Ability::<Attack>::new().into()],
    }
}

pub fn duck() -> Hero {
    Hero {
        id: "duck",
        name: "Утка",
        desc: "Умеет делиться пополам",
        hp: 1200.0,
        mana_regen: 10.0,
        attack: 24.0,
        attack_speed: 0.95,
        crit: 0.15,
        evasion: 0.14,
        abilities: vec![Ability::<Attack>::new().into()],
    }
}

pub fn kisanya() -> Hero {
    Hero {
        id: "kisanya",
        name: "Кисаня",
        desc: "Обязательно пройдёт твою игру на стриме",
        hp: 1100.0,
        mana_regen: 9.0,
        attack: 22.0,
        attack_speed: 1.11,
        crit: 0.15,
        evasion: 0.12,
        abilities: vec![Ability::<Attack>::new().into()],
    }
}

struct Player {
    hero: Hero,
    cards: Vec<Box<dyn HasEffect>>,
}

struct Fighter {
    hp: f32,
    attack: f32,
    attack_speed: f32,
}

enum EffectOwner {
    Fighter1,
    Fighter2,
}

#[test]
fn battle() {
    let player1 = Player {
        cards: vec![],
        hero: nulch(),
    };
    let player2 = Player {
        cards: vec![],
        hero: rasp(),
    };

    let mut fighter1 = Fighter {
        hp: player1.hero.hp,
        attack: player1.hero.attack,
        attack_speed: 1.0 / player1.hero.attack_speed,
    };
    let mut fighter2 = Fighter {
        hp: player2.hero.hp,
        attack: player2.hero.attack,
        attack_speed: 1.0 / player2.hero.attack_speed,
    };

    let mut effects = vec![]
        .into_iter()
        .chain(
            player1
                .hero
                .abilities
                .iter()
                .map(|a| (a.effect(), EffectOwner::Fighter1)),
        )
        .chain(
            player1
                .cards
                .iter()
                .map(|c| (c.effect(), EffectOwner::Fighter1)),
        )
        .chain(
            player2
                .hero
                .abilities
                .iter()
                .map(|a| (a.effect(), EffectOwner::Fighter2)),
        )
        .chain(
            player2
                .cards
                .iter()
                .map(|c| (c.effect(), EffectOwner::Fighter2)),
        )
        .collect::<Vec<_>>();

    let duration = 2;
    let fps = 100;
    let delta = 1.0 / fps as f32;

    for time in 0..duration * fps {
        let time = time as f32 / fps as f32;
        for (effect, owner) in effects.iter_mut() {
            let (myself, enemy) = match owner {
                EffectOwner::Fighter1 => (&mut fighter1, &mut fighter2),
                EffectOwner::Fighter2 => (&mut fighter2, &mut fighter1),
            };

            let modifiers = effect.update(Data {
                delta,
                myself,
                enemy,
            });

            if modifiers.len() > 0 {
                println!("{time}: {modifiers:?}");
            }

            for modifier in modifiers {
                match modifier {
                    Modifier::ChangeAttack(val) => {
                        myself.attack += val;
                    }
                    Modifier::Damage(val) => {
                        enemy.hp -= val;
                    }
                }
            }
        }
    }
}
