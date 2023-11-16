extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate specs;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, GlGraphics, GlyphCache, TextureSettings};
use piston::{RenderArgs, MouseScrollEvent, MouseRelativeEvent, Button, MouseButton};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent, PressEvent, ReleaseEvent};
use piston::window::{WindowSettings};
use specs::prelude::*;
use specs::WorldExt;

mod renderer;
use renderer::{Renderer, RenderData, LocalRenderArgs, DrawData, RenderDataInit};

mod material;
use material::MaterialInit;

mod physnode;
use physnode::{PhysNodeInit, DeltaTime};

mod camera2d;
use camera2d::Camera2d;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut shared_world = World::new();
    shared_world.insert(DeltaTime(0.0));
    shared_world.insert(LocalRenderArgs::default());
    shared_world.insert(Camera2d::default());
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
    shared_world.create_entity().with(RenderData::new([25.0, 25.0], 0.0, DrawData::Color([0.0, 0.0, 1.0, 1.0]))).build();
    shared_world.create_entity().with(RenderData::new([30.0, 30.0], 0.0, DrawData::Text("Hello World".to_string(), [1.0, 1.0, 1.0, 1.0]))).build();

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(Renderer::new(opengl))
        .build_async(&mut shared_world);
    let mut dragging = false;
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.mouse_scroll_args()
        {
            let mut cam = shared_world.write_resource::<Camera2d>();
            cam.zoom += args[1] / 3.0;
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            if button == MouseButton::Left
            {
                dragging = true;
            }
        }

        if let Some(Button::Mouse(button)) = e.release_args() {
            if button == MouseButton::Left
            {
                dragging = false;
            }
        }

        if let Some(args) = e.mouse_relative_args() {
            let mut cam = shared_world.write_resource::<Camera2d>();
            if (dragging)
            {
                cam.pos[0] += args[0];
                cam.pos[1] += args[1];
            }
        }

        if let Some(args) = e.render_args() {
            let mut ra = shared_world.write_resource::<LocalRenderArgs>();
            ra.0 = args;
        }

        if let Some(args) = e.update_args() {
            {
                let mut delta = shared_world.write_resource::<DeltaTime>();
                *delta = DeltaTime(args.dt);
            }
            shared_world.maintain();
        }
    }
}
