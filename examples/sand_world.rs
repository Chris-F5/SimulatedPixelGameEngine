use ggez::{
    event::{self, EventHandler},
    graphics,
    graphics::Image,
    nalgebra::{Point2, Vector2},
    Context, ContextBuilder, GameResult,
};
use shred::{DispatcherBuilder, SystemData, World};
use spge::{
    components::cell_components::{CellColor, TestComp},
    storage::cell_storage::{CellStorage, MaskedCellStorage, WriteCellStorage},
    systems::SandSystem,
    CHUNK_SIZE,
};

fn main() {
    // INIT world
    let mut world = World::empty();

    let cell_colors: MaskedCellStorage<CellColor> = Default::default();
    let test_cells: MaskedCellStorage<TestComp> = Default::default();

    world.insert(cell_colors);
    world.insert(test_cells);

    {
        let sand_col = CellColor {
            r: 224,
            g: 188,
            b: 27,
            a: 255,
        };
        let mut colors = WriteCellStorage::<CellColor>::fetch(&world);
        colors.insert(5, 5, sand_col);
        colors.insert(4, 1, sand_col);
        colors.insert(4, 5, sand_col);
        colors.insert(10, 10, sand_col);
    }
    let update_dispatcher: shred::Dispatcher<'static, 'static> = DispatcherBuilder::new()
        .with(SandSystem, "sand", &[])
        .build();

    // Make window and run event loop
    let (mut ctx, mut event_loop) = ContextBuilder::new("spge_game", "Chris Lang Games")
        .window_setup(ggez::conf::WindowSetup::default().title("Sand World Example"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions((CHUNK_SIZE * 20) as f32, (CHUNK_SIZE * 20) as f32),
        )
        .build()
        .expect("error creating ggez context!");

    let mut game = Game::new(world, update_dispatcher);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Game<'a> {
    world: shred::World,
    update_dispatcher: shred::Dispatcher<'a, 'a>,
    renderer: Renderer,
}

impl<'a> Game<'a> {
    pub fn new(world: shred::World, update_dispatcher: shred::Dispatcher<'a, 'a>) -> Game<'a> {
        Game::<'a> {
            world: world,
            update_dispatcher: update_dispatcher,
            renderer: Renderer::new(),
        }
    }
}

impl<'a> EventHandler for Game<'a> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update_dispatcher.dispatch(&mut self.world);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::clear(ctx, ggez::graphics::WHITE);
        self.renderer.render(ctx, &mut self.world);
        ggez::graphics::present(ctx)
    }
}

pub struct Renderer;
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }

    pub fn render(&mut self, ctx: &mut Context, world: &mut World) {
        let cell_colors = world.fetch::<MaskedCellStorage<CellColor>>();
        let cell_colors = CellStorage::new(cell_colors);

        let colors = cell_colors.data.inner.cells;
        let mut rgba_colors: [u8; (CHUNK_SIZE * CHUNK_SIZE * 4) as usize] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        for i in 0..(CHUNK_SIZE * CHUNK_SIZE) as usize {
            rgba_colors[i * 4] = colors[i].r;
            rgba_colors[i * 4 + 1] = colors[i].g;
            rgba_colors[i * 4 + 2] = colors[i].b;
            rgba_colors[i * 4 + 3] = colors[i].a;
        }

        let mut cells_image =
            Image::from_rgba8(ctx, CHUNK_SIZE as u16, CHUNK_SIZE as u16, &rgba_colors[..]).unwrap();

        cells_image.set_filter(graphics::FilterMode::Nearest);

        graphics::draw(
            ctx,
            &cells_image,
            graphics::DrawParam::new()
                .dest(Point2::new(0.0, (CHUNK_SIZE * 20) as f32))
                .scale(Vector2::new(20.0, -20.0)),
        )
        .unwrap();
    }
}
