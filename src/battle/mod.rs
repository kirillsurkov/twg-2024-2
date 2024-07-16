pub mod ability;
mod card;
mod effect;
pub mod fight;
pub mod hero;
pub mod modifier;
pub mod player;

use fight::{Fight, FightCapture, Owner};
use player::Player;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub struct Battle {
    pub players: Vec<Player>,
    next_players: Vec<Player>,
}

#[derive(Debug)]
pub struct RoundCapture {
    pub player1: &'static str,
    pub player2: &'static str,
    pub winner: Owner,
    pub fight_capture: FightCapture,
}

impl Battle {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            players: players,
            next_players: vec![],
        }
    }

    pub fn round(&mut self) -> Vec<RoundCapture> {
        self.next_players = self.players.clone();

        let mut players = self.next_players.iter_mut().collect::<Vec<_>>();
        players.shuffle(&mut thread_rng());

        let rounds = players
            .chunks_mut(2)
            .into_iter()
            .map(|pair| {
                if let [p1, p2] = pair {
                    let (winner, fight_capture) = Fight::new(p1, p2).run();
                    RoundCapture {
                        player1: p1.hero.id,
                        player2: p2.hero.id,
                        winner,
                        fight_capture,
                    }
                } else {
                    todo!("skipping");
                }
            })
            .collect::<Vec<_>>();

        rounds
    }

    pub fn apply(&mut self) {
        self.players = self.next_players.clone();
        self.next_players.clear();
    }
}

#[test]
fn battle() {
    use hero::{dimas, dtyan, duck, kisanya, nulch, rasp};
    let mut battle = Battle::new(vec![
        Player::new(nulch()),
        Player::new(rasp()),
        Player::new(dtyan()),
        Player::new(dimas()),
        Player::new(duck()),
        Player::new(kisanya()),
    ]);

    println!("{:#?}", battle.round());
}
