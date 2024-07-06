use std::time::Duration;

use bevy::{animation::RepeatAnimation, prelude::*, utils::hashbrown::HashMap};

pub struct ComplexAnimPlayerPlugin;

impl Plugin for ComplexAnimPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play);
    }
}

#[derive(Clone)]
pub struct ComplexAnimPart {
    pub name: String,
    pub repeat: RepeatAnimation,
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

pub enum State {
    Idle,
    Showoff,
}

#[derive(Component)]
pub struct Animations {
    pub by_name: HashMap<String, Handle<AnimationClip>>,
}

#[derive(Component)]
pub struct ComplexAnimPlayer {
    anim_player: Entity,
    state: State,
    idle: Option<String>,
    showoffs: Vec<Showoff>,
    current_showoff: Option<Showoff>,
}

impl ComplexAnimPlayer {
    pub fn new(anim_player: Entity) -> Self {
        Self {
            anim_player,
            state: State::Idle,
            idle: None,
            showoffs: vec![],
            current_showoff: None,
        }
    }

    pub fn with_idle(mut self, track: &str) -> Self {
        self.idle = Some(track.to_string());
        self
    }

    pub fn with_showoff(mut self, showoff: Showoff) -> Self {
        self.showoffs.push(showoff);
        self
    }

    pub fn play(&mut self, state: State) {
        self.state = state;
    }
}

fn play(
    mut query: Query<(&mut ComplexAnimPlayer, &Animations)>,
    mut anim_players: Query<&mut AnimationPlayer>,
    time: Res<Time>,
) {
    const TRANSITION: Duration = Duration::from_millis(250);

    for (mut player, animations) in query.iter_mut() {
        let mut anim_player = anim_players.get_mut(player.anim_player).unwrap();
        match player.state {
            State::Idle => match &player.idle {
                Some(idle) => {
                    anim_player
                        .play_with_transition(animations.by_name[idle].clone_weak(), TRANSITION)
                        .repeat();
                }
                None => {}
            },
            State::Showoff => {
                let current_showoff = match player.current_showoff.as_mut() {
                    Some(showoff) => showoff,
                    None => {
                        player.current_showoff = Some(player.showoffs[0].clone());
                        player.current_showoff.as_mut().unwrap()
                    }
                };

                let part = &current_showoff.parts[current_showoff.current_part];
                let anim = animations.by_name.get(&part.name).unwrap();
                anim_player
                    .play_with_transition(anim.clone_weak(), TRANSITION)
                    .set_speed(part.speed)
                    .set_repeat(part.repeat);

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
