use graphics::types::Vec2d;
use specs::prelude::*;

use crate::renderer::RenderData;

#[derive(Default)]
pub struct DeltaTime(pub f64);

#[derive(Debug)]
pub struct PhysNode
{
    vel: Vec2d<f64>,
    acc: Vec2d<f64>,
    z: u8
}

impl Component for PhysNode
{
    type Storage = DenseVecStorage<Self>;
}

struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (Read<'a, DeltaTime>,
                       WriteStorage<'a, PhysNode>,
                       WriteStorage<'a, RenderData>);

    fn run(&mut self, data: Self::SystemData)
    {
        let (delta, mut phys, mut rd) = data;

        let delta = delta.0;

        for (phys, rd) in (&mut phys, &mut rd).join() {
            rd.pos[0] += phys.vel[0] * delta;
            rd.pos[1] += phys.vel[1] * delta;
            phys.vel[0] += phys.acc[0] * delta;
            phys.vel[1] += phys.acc[1] * delta;
        }
    }
}

pub fn PhysNodeInit(world: &mut World)
{
    world.register::<PhysNode>();
}
