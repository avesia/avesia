use wasm_bindgen::prelude::*;
use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

#[wasm_bindgen]
pub fn start(element_id: String) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(element_id),
                ..default()
            }),
            ..default()
        }))
        .add_system(hello_world)
        .run();
}
