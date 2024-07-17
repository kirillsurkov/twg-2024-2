use super::{
    card::{CardBranch, CardOps},
    hero::Hero,
};

#[derive(Debug, Clone)]
pub struct Player {
    pub hero: Hero,
    pub money: u32,
    pub attack: u32,
    pub hp: u32,
    pub cards: Vec<Box<dyn CardOps>>,
    pub cards_reserved: Vec<(bool, Box<dyn CardOps>)>,
}

impl Player {
    pub fn new(hero: Hero) -> Self {
        Self {
            hero,
            money: 300,
            attack: 3,
            hp: 50,
            cards: vec![],
            cards_reserved: vec![],
        }
    }

    pub fn use_reserved_card(&mut self, index: usize) {
        let Some((ref mut active, ref card)) = self.cards_reserved.get_mut(index) else {
            return;
        };
        if *active && self.money >= card.cost() {
            *active = false;
            // self.money -= card.cost();
            if let Some(card) = self.cards.iter_mut().find(|c| c.id() == card.id()) {
                card.set_level(card.level() + 1);
            } else {
                let mut card = card.clone();
                card.set_level(1);
                self.cards.push(card);
            }
        }
    }

    pub fn reserve_cards(&mut self, cards: Vec<Box<dyn CardOps>>) {
        self.cards_reserved = cards
            .into_iter()
            .map(|mut card| {
                if let Some(c) = self.cards.iter().find(|c| c.id() == card.id()) {
                    card = c.clone();
                } else {
                    card.set_level(0);
                }
                (true, card)
            })
            .collect();
    }

    pub fn branch_value(&self, branch: &CardBranch) -> u32 {
        let mut total = 0;
        for card in &self.cards {
            if card.branches().contains(branch) {
                total += card.level() as u32
            }
        }
        total
    }
}
