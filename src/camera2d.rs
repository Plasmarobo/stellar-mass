use graphics::types::Vec2d;

pub struct Camera2d
{
    pub pos: Vec2d,
    pub zoom: f64,
    pub lod: u8
}

impl Camera2d
{
    pub fn new(pos: Vec2d, zoom: f64, lod: u8) -> Camera2d
    {
        Camera2d
        {
            pos,
            zoom,
            lod
        }
    }

    pub fn pos(&self) -> Vec2d
    {
        [self.pos[0] * 2.0 * self.zoom, self.pos[1] * 2.0 * self.zoom]
    }
}

impl Default for Camera2d
{
    fn default() -> Camera2d
    {
        Camera2d { pos: [1.0, 1.0], zoom: 1.0, lod: 1 }
    }
}
