use specs::prelude::*;

use crate::{renderer::RenderData, physnode::PhysNode};

#[derive(Debug)]
struct Material
{
    //hot_transition: ref Option<Material>,
    //cold_transition: ref Option<Material>,
    hardness: i32,
    viscocity: i32,
    malleability: i32,
    conductivity: i32,
    insulation: i32,
    radioactivity: i32,
    corrosivity: i32,
    volatility: i32,
    aesthetic: i32,
    mass: i32,
    combustion_threshold: i32,
    combustion_rate: i32,
    energy: i32
}

impl Component for Material
{
    type Storage = VecStorage<Self>;
}

pub fn MaterialInit(world: &mut World)
{
    world.register::<Material>();
}

struct MaterialSimulation
{
    sim_clock: f64,
    sim_rate: f64,
}

impl<'a> System<'a> for MaterialSimulation
{
    type SystemData = (ReadStorage<'a, Material>, ReadStorage<'a, RenderData>, ReadStorage<'a, PhysNode>);

    fn run(&mut self, data: Self::SystemData)
    {
        let (mats, rds, phys) = data;
        // Perform material sim updates
        for (mat, rd, phy) in (&mats, &rds, phys.maybe()).join()
        {
            
        } 
        // Perform any updates to material + renderables
    }
}