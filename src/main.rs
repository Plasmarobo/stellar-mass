extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate specs;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, GlGraphics, GlyphCache, TextureSettings};
use piston::{RenderArgs};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::{WindowSettings};
use specs::prelude::*;
use specs::WorldExt;

mod renderer;
use renderer::{Renderer, RenderData, LocalRenderArgs, DrawData, RenderDataInit};

mod material;
use material::MaterialInit;

mod physnode;
use physnode::{PhysNodeInit, DeltaTime};

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut shared_world = World::new();
    shared_world.insert(DeltaTime(0.0));
    shared_world.insert(LocalRenderArgs::default());
    RenderDataInit(&mut shared_world);
    MaterialInit(&mut shared_world);
    PhysNodeInit(&mut shared_world);
    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("stellar-mass", [512, 512])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Test entities
    shared_world.create_entity().with(RenderData::default()).build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Renderer::new(opengl), "renderer", &[])
        .build();
    dispatcher.setup(&mut shared_world);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.render_args() {
            let mut ra = shared_world.write_resource::<LocalRenderArgs>();
            ra.0 = args;
        }

        if let Some(args) = e.update_args() {
            {
                let mut delta = shared_world.write_resource::<DeltaTime>();
                *delta = DeltaTime(args.dt);
            }
            dispatcher.dispatch(&mut shared_world);
            shared_world.maintain();
        }
    }
}
