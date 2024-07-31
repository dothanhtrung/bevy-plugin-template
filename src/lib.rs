// Copyright 202x Your Name <your email>

//! ### Plugin
//! doc goes here

use bevy::app::{App, Plugin, Update};
#[cfg(feature = "state")]
use bevy::prelude::{in_state, States};

macro_rules! plugin_systems {
    ( ) => {
        (system1, system2)
    };
}

#[cfg(feature = "state")]
#[derive(Default)]
pub struct YourPlugin<T>
where
    T: States,
{
    /// List of game state that this plugin will run in
    pub states: Option<Vec<T>>,
}

#[cfg(feature = "state")]
impl<T> Plugin for YourPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        if let Some(states) = &self.states {
            for state in states {
                app.add_systems(Update, plugin_systems!().run_if(in_state(state.clone())));
            }
        } else {
            app.add_systems(Update, plugin_systems!());
        }
    }
}

#[cfg(feature = "state")]
impl<T> YourPlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states: Some(states) }
    }
}

/// Use this if you don't care to state and want this plugin's systems run all the time.
#[derive(Default)]
pub struct YourPluginNoState;

impl Plugin for YourPluginNoState {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, plugin_systems!());
    }
}

fn system1() {}

fn system2() {}
