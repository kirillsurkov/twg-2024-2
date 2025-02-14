use std::time::Duration;

use bevy::{animation::RepeatAnimation, prelude::*, utils::hashbrown::HashMap};

use crate::hero::HeroId;

use super::LocalSchedule;

pub struct ComplexAnimPlayerPlugin;

impl Plugin for ComplexAnimPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(LocalSchedule, play);
    }
}

#[derive(Clone)]
pub struct ComplexAnimPart {
    pub name: String,
    pub repeat: u32,
    pub speed: f32,
    pub wait: Duration,
}

#[derive(Clone)]
pub struct Showoff {
    parts: Vec<ComplexAnimPart>,
    current_part: usize,
    timer: f32,
}

impl Showoff {
    pub fn new(parts: Vec<ComplexAnimPart>) -> Self {
        Self {
            parts,
            current_part: 0,
            timer: 0.0,
        }
    }
}

#[derive(Debug)]
pub enum State {
    Stop,
    Idle,
    Attack(f32),
    Win,
    Lose,
    Showoff(Duration),
}

pub const SHOWOFF_LAZY: State = State::Showoff(Duration::from_millis(5000));
pub const SHOWOFF_IMMEDIATE: State = State::Showoff(Duration::from_millis(0));

#[derive(Component)]
pub struct Animations {
    by_name: HashMap<String, Handle<AnimationClip>>,
    current: String,
}

impl Animations {
    pub fn new(by_name: HashMap<String, Handle<AnimationClip>>) -> Self {
        Self {
            by_name,
            current: String::new(),
        }
    }

    pub fn current(&self) -> String {
        self.current.clone()
    }
}

#[derive(Component)]
pub struct ComplexAnimPlayer {
    anim_player: Entity,
    state_changed: bool,
    state: State,
    replay: bool,
    idle: Option<String>,
    attack: Option<(String, u32)>,
    win: Option<String>,
    lose: Option<String>,
    showoffs: Vec<Showoff>,
    current_showoff: Option<Showoff>,
    timer: f32,
}

impl ComplexAnimPlayer {
    pub fn new(anim_player: Entity) -> Self {
        Self {
            anim_player,
            state_changed: true,
            state: State::Stop,
            replay: false,
            idle: None,
            attack: None,
            win: None,
            lose: None,
            showoffs: vec![],
            current_showoff: None,
            timer: 0.0,
        }
    }

    pub fn with_idle(mut self, track: &str) -> Self {
        self.idle = Some(track.to_string());
        self
    }

    pub fn with_attack(mut self, attack: &str, frames: u32) -> Self {
        self.attack = Some((attack.to_string(), frames));
        self
    }

    pub fn with_win(mut self, track: &str) -> Self {
        self.win = Some(track.to_string());
        self
    }

    pub fn with_lose(mut self, track: &str) -> Self {
        self.lose = Some(track.to_string());
        self
    }

    pub fn with_showoff(mut self, showoff: Showoff) -> Self {
        self.showoffs.push(showoff);
        self
    }

    pub fn play(&mut self, state_changed: bool, state: State) {
        self.state_changed = state_changed;
        self.state = state;
    }

    pub fn replay(&mut self) {
        self.replay = true;
    }
}

fn play(
    mut query: Query<(&mut ComplexAnimPlayer, &mut Animations)>,
    mut anim_players: Query<&mut AnimationPlayer>,
    time: Res<Time>,
) {
    const TRANSITION: Duration = Duration::from_millis(250);

    for (mut player, mut animations) in query.iter_mut() {
        let mut anim_player = anim_players.get_mut(player.anim_player).unwrap();

        if player.replay {
            player.replay = false;
            anim_player.replay();
        }

        match player.state {
            State::Stop => anim_player.pause(),
            _ => anim_player.resume(),
        }

        match player.state {
            State::Stop => {}
            State::Idle => match &player.idle {
                Some(idle) => {
                    animations.current = idle.clone();
                    anim_player
                        .play_with_transition(animations.by_name[idle].clone_weak(), TRANSITION)
                        .repeat();
                }
                None => {}
            },
            State::Attack(speed) => match &player.attack {
                Some((attack, frames)) => {
                    let duration = *frames as f32 / 24.0;
                    animations.current = attack.clone();
                    anim_player
                        .play_with_transition(animations.by_name[attack].clone_weak(), TRANSITION)
                        .set_speed(duration * speed);
                }
                None => {}
            },
            State::Win => match &player.win {
                Some(win) => {
                    animations.current = win.clone();
                    anim_player
                        .play_with_transition(animations.by_name[win].clone_weak(), TRANSITION)
                        .repeat();
                }
                None => {}
            },
            State::Lose => match &player.lose {
                Some(lose) => {
                    animations.current = lose.clone();
                    anim_player
                        .play_with_transition(animations.by_name[lose].clone_weak(), TRANSITION);
                }
                None => {}
            },
            State::Showoff(interval) => {
                if player.state_changed {
                    player.current_showoff = None;
                }
                let current_showoff = match player.current_showoff.as_mut() {
                    Some(showoff) => showoff,
                    None => {
                        if player.timer >= interval.as_secs_f32() {
                            player.timer = 0.0;
                            anim_player.replay();
                            player.current_showoff = Some(player.showoffs[0].clone());
                            player.current_showoff.as_mut().unwrap()
                        } else {
                            match &player.idle {
                                Some(idle) => {
                                    animations.current = idle.clone();
                                    anim_player
                                        .play_with_transition(
                                            animations.by_name[idle].clone_weak(),
                                            TRANSITION,
                                        )
                                        .repeat();
                                }
                                None => {}
                            }
                            player.timer += time.delta_seconds();
                            continue;
                        }
                    }
                };

                let part = &current_showoff.parts[current_showoff.current_part];
                animations.current = part.name.clone();
                let anim = animations.by_name.get(&part.name).unwrap();
                anim_player
                    .play_with_transition(anim.clone_weak(), TRANSITION)
                    .set_speed(part.speed)
                    .set_repeat(RepeatAnimation::Count(part.repeat));

                if anim_player.is_finished() {
                    if current_showoff.timer >= part.wait.as_secs_f32() {
                        anim_player.play(anim.clone_weak()).set_speed(-0.5).repeat();
                        current_showoff.timer = 0.0;
                        current_showoff.current_part += 1;
                    } else {
                        current_showoff.timer += time.delta_seconds();
                    }
                }

                if current_showoff.current_part >= current_showoff.parts.len() {
                    player.current_showoff = None;
                }
            }
        }
    }
}
