use std::marker::PhantomData;

use bevy::{gltf::Gltf, prelude::*};

#[derive(Resource)]
pub struct Model<T> {
    pub handle: Handle<Gltf>,
    _pd: PhantomData<T>,
}

impl<T> Model<T> {
    pub fn new(gltf: Handle<Gltf>) -> Self {
        Self {
            handle: gltf,
            _pd: PhantomData::default(),
        }
    }
}
