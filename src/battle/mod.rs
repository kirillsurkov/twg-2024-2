mod ability;
mod card;
mod effect;
mod fight;
pub mod hero;
mod modifier;
pub mod player;

use fight::{Fight, FightCapture};
use player::Player;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub struct Battle {
    players: Vec<Player>,
}

#[derive(Debug)]
pub struct RoundCapture {
    player1: &'static str,
    player2: &'static str,
    fight_capture: FightCapture,
}

impl Battle {
    pub fn new(players: Vec<Player>) -> Self {
        Self { players }
    }

    pub fn round(&mut self) -> Vec<RoundCapture> {
        let mut players = self.players.iter_mut().collect::<Vec<_>>();
        players.shuffle(&mut thread_rng());
        players
            .chunks_mut(2)
            .into_iter()
            .map(|pair| {
                if let [p1, p2] = pair {
                    RoundCapture {
                        player1: p1.hero.id,
                        player2: p2.hero.id,
                        fight_capture: Fight::new(p1, p2).run(),
                    }
                } else {
                    todo!("skipping");
                }
            })
            .collect()
    }
}

#[test]
fn battle() {
    use hero::{dimas, dtyan, duck, kisanya, nulch, rasp};
    let mut battle = Battle {
        players: vec![
            Player::new(nulch()),
            Player::new(rasp()),
            Player::new(dtyan()),
            Player::new(dimas()),
            Player::new(duck()),
            Player::new(kisanya()),
        ],
    };

    println!("{:#?}", battle.round());
}
