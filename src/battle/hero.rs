use super::{
    ability::{Ability, Attack, Beam, FireCube, Halve, RegenMana, StarWars},
    card::CardBranch,
    effect::HasEffect,
};

#[derive(Debug, Clone)]
pub struct Hero {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub branches: Vec<CardBranch>,
    pub hp: f32,
    pub mana_regen: f32,
    pub attack: f32,
    pub attack_speed: f32,
    pub crit: f32,
    pub evasion: f32,
    pub abils: Vec<Box<dyn HasEffect>>,
}

pub fn nulch() -> Hero {
    Hero {
        id: "nulch",
        name: "Нульч",
        desc: "Организатор конкурса",
        branches: vec![CardBranch::Regen, CardBranch::Mana],
        hp: 1100.0,
        mana_regen: 10.0,
        attack: 25.0,
        attack_speed: 0.95,
        crit: 0.16,
        evasion: 0.15,
        abils: vec![
            Ability::<Attack>::new().into(),
            Ability::<RegenMana>::new().into(),
            Ability::<Beam<true, 150, true>>::new().into(),
            Ability::<Beam<false, 150, false>>::new().into(),
        ],
    }
}

pub fn rasp() -> Hero {
    Hero {
        id: "rasp",
        name: "Расп",
        desc: "Это твоя мамка, анон",
        branches: vec![CardBranch::Attack, CardBranch::Mana],
        hp: 1200.0,
        mana_regen: 10.0,
        attack: 22.0,
        attack_speed: 1.04,
        crit: 0.11,
        evasion: 0.18,
        abils: vec![
            Ability::<Attack>::new().into(),
            Ability::<RegenMana>::new().into(),
            Ability::<FireCube>::new().into(),
        ],
    }
}

pub fn dtyan() -> Hero {
    Hero {
        id: "dtyan",
        name: "Деревотян",
        desc: "Вырасти дерево, если сможешь!",
        branches: vec![CardBranch::Regen, CardBranch::Evasion],
        hp: 1400.0,
        mana_regen: 10.0,
        attack: 34.0,
        attack_speed: 0.75,
        crit: 0.20,
        evasion: 0.08,
        abils: vec![
            Ability::<Attack>::new().into(),
            Ability::<RegenMana>::new().into(),
            Ability::<Beam<true, 300>>::new().into(),
        ],
    }
}

pub fn dimas() -> Hero {
    Hero {
        id: "dimas",
        name: "Димасик",
        desc: "Организатор предыдущего конкурса",
        branches: vec![CardBranch::Attack, CardBranch::Crit],
        hp: 1350.0,
        mana_regen: 11.0,
        attack: 35.0,
        attack_speed: 0.71,
        crit: 0.17,
        evasion: 0.09,
        abils: vec![
            Ability::<Attack>::new().into(),
            Ability::<RegenMana>::new().into(),
            Ability::<StarWars>::new().into(),
        ],
    }
}

pub fn duck() -> Hero {
    Hero {
        id: "duck",
        name: "Утка",
        desc: "Умеет делиться пополам",
        branches: vec![CardBranch::Hp, CardBranch::Regen],
        hp: 1200.0,
        mana_regen: 10.0,
        attack: 24.0,
        attack_speed: 0.95,
        crit: 0.15,
        evasion: 0.14,
        abils: vec![
            Ability::<Attack>::new().into(),
            Ability::<RegenMana>::new().into(),
            Ability::<Halve>::new().into(),
        ],
    }
}

pub fn kisanya() -> Hero {
    Hero {
        id: "kisanya",
        name: "Кисаня",
        desc: "Обязательно пройдёт твою игру на стриме",
        branches: vec![CardBranch::Crit, CardBranch::Evasion],
        hp: 1100.0,
        mana_regen: 9.0,
        attack: 22.0,
        attack_speed: 1.11,
        crit: 0.15,
        evasion: 0.12,
        abils: vec![
            Ability::<Attack>::new().into(),
            Ability::<RegenMana>::new().into(),
            Ability::<Beam<false, 300>>::new().into(),
        ],
    }
}
