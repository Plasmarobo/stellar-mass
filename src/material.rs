use specs::prelude::*;

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
