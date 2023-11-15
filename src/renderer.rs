use piston_window::*;
use graphics::types::Vec2d;
use opengl_graphics::{Texture, OpenGL, GlGraphics, GlyphCache, TextureSettings};
use piston::RenderArgs;
use specs::prelude::*;
use std::{fmt, any::Any};

pub enum DrawData
{
    Color([f32; 4]),
    Texture(Box<Texture>),
    Text(String, [f32; 4]),
}

impl fmt::Debug for DrawData
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("DrawData");
        match self {
            DrawData::Color(color) => builder.field("color", &color),
            DrawData::Texture(_texture) => builder.field("texture", &"."),
            DrawData::Text(text, color) => {
                builder.field("text", text);
                builder.field("color", &format!("{},{},{},{}", color[0], color[1], color[2], color[3]))
            }
        };
        builder.finish()
    }
}

#[derive(Debug)]
pub struct RenderData
{
    pub pos: Vec2d<f64>,
    pub rot: f64,
    pub draw_data: DrawData
}

impl Component for RenderData
{
    type Storage = VecStorage<Self>;
}

impl RenderData
{
    pub fn new(pos: Vec2d<f64>, rot: f64, draw_data: DrawData) -> RenderData
    {
        RenderData { pos, rot, draw_data }
    }

    pub fn default() -> RenderData
    {
        RenderData { pos: [ 10.0, 10.0 ], rot: 0.0, draw_data: DrawData::Color([1.0, 0.0, 0.0, 1.0]) }
    }
}

pub struct LocalRenderArgs(pub RenderArgs);

impl Default for LocalRenderArgs {
    fn default() -> LocalRenderArgs
    {
        LocalRenderArgs { 0: RenderArgs { ext_dt: 0.0, window_size: [512.0, 512.0], draw_size: [512, 512] }}
    }
}


pub struct Renderer<'b>
{
    gl: GlGraphics,
    font: GlyphCache<'b>
}

impl<'a, 'b> System<'a> for Renderer<'b>
{
    type SystemData = (Read<'a, LocalRenderArgs>, ReadStorage<'a, RenderData>);

    fn run(&mut self, data: Self::SystemData)
    {
        use graphics::*;
        let (args, renderdata) = data;
        let square = rectangle::square(0.0, 0.0, 50.0);
        let args = args.0;
        let c = self.gl.draw_begin(args.viewport());
        // Clear the screen.
        clear([0.0,1.0,0.0,1.0], &mut self.gl);
        for rd in renderdata.join()
        {
            
                let transform = c
                                .transform
                                .trans(rd.pos[0], rd.pos[1])
                                .rot_rad(rd.rot)
                                .trans(-25.0, -25.0);

                match &rd.draw_data
                {
                    DrawData::Color(color) => {
                        rectangle(*color, square, transform, &mut self.gl);
                    },
                    DrawData::Texture(tex) => {
                        image(&(*(*tex)), transform, &mut self.gl);
                    },
                    DrawData::Text(txt, color) => {
                        text(*color, 32, txt, &mut self.font, transform, &mut self.gl).unwrap();
                }
            }
        }
        self.gl.draw_end();
    }
}


impl<'b> Renderer<'b>
{
    pub fn new(opengl: OpenGL) -> Renderer<'b>
    {
        Renderer {
            gl: GlGraphics::new(opengl),
            font: GlyphCache::new("FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap()
        }
    }
}

pub fn RenderDataInit(world: &mut World)
{
    world.register::<RenderData>();
}
