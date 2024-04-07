pub struct GameEngine {
    is_running: bool,
    world: World,
    renderer: Renderer,
}

impl GameEngine {
    fn new() -> Self {
        Self {
            is_running: true,
            world: World::new(),
            renderer: Renderer::new()
        }
    }

    fn run(&mut self) {
        while self.is_running {
            self.handle_events();
            self.update();
            self.render();
            self.check_exit_conditions();
        }
    }

    fn handle_events(&mut self) {
        // handles inputs here (e.g. mouse, keyboard)
    }

    fn update(&mut self) {
        self.world.update();
    }

    fn render(&self) {
        self.renderer.render(&self.world);
    }

    fn check_exit_conditions(&mut self) {
        // should game continue or exit?
    }
}