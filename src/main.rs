use raylib::prelude::*;

pub struct Config {
    pub max_x: i32,
    pub max_y: i32,
    pub iterations: i32,
}

pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

pub struct Board {
    max_x: usize,
    max_y: usize,
    data: Vec<bool>,
}

pub struct Ant {
    x: i32,
    y: i32,
    dir: Direction,
    max_x: i32,
    max_y: i32,
}

impl Ant {
    fn new(x: i32, y: i32, max_x: i32, max_y: i32) -> Self {
        Self { x, y, dir: Direction::UP, max_x, max_y }
    }

    fn move_ant(&mut self, dirty: bool) {
        if dirty {
            match self.dir {
                Direction::UP => { self.x -= 1; self.dir = Direction::LEFT; }
                Direction::LEFT => { self.y -= 1; self.dir = Direction::DOWN; }
                Direction::DOWN => { self.x += 1; self.dir = Direction::RIGHT; }
                Direction::RIGHT => { self.y += 1; self.dir = Direction::UP; }
            }
        } else {
            match self.dir {
                Direction::UP => { self.x += 1; self.dir = Direction::RIGHT; }
                Direction::RIGHT => { self.y -= 1; self.dir = Direction::DOWN; }
                Direction::DOWN => { self.x -= 1; self.dir = Direction::LEFT; }
                Direction::LEFT => { self.y += 1; self.dir = Direction::UP; }
            }
        }

        self.x = self.x.rem_euclid(self.max_x);
        self.y = self.y.rem_euclid(self.max_y);
    }
}

impl Board {
    fn new(x: usize, y: usize) -> Self {
        let mut v: Vec<bool> = Vec::new();
        for _ in 0..x * y {
            v.push(false);
        }
        // Self {maxx: x, maxy: y, data: Vec::with_capacity(x * y)}
        Self { max_x: x, max_y: y, data: v }
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        let index = y * self.max_x + x;
        self.data[index] = val
        //atxy(x as i32 + 1, y as i32 + 1, "$"); 
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let index = y * self.max_x + x;
        self.data[index]
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        for i in 0..self.max_x {
            for j in 0..self.max_y {
                let color = if self.get(i, j) { Color::BLACK } else { Color::WHITE };
                d.draw_rectangle(i as i32 * 10, j as i32 * 10, 10, 10, color);
            }
        }
    }
}

fn main() {
    let cfg = setup();

    let (mut rl, thread) = init()
        .size(800, 600)
        .title("Langton's Ant")
        .build();

    let mut board = Board::new(cfg.max_x as usize, cfg.max_y as usize);
    let mut ant = Ant::new(cfg.max_x / 2, cfg.max_y / 2, cfg.max_x, cfg.max_y);
    let mut iteration = 0;
    let mut simulation_running = true;

    while !rl.window_should_close() {
        if simulation_running {
            let index = ant.y * cfg.max_x + ant.x;
            board.set(ant.x as usize, ant.y as usize, !board.get(ant.x as usize, ant.y as usize));
            ant.move_ant(board.data[index as usize]);
            iteration += 1;
            if iteration >= cfg.iterations {
                simulation_running = false;
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        board.draw(&mut d);
        d.draw_rectangle(ant.x * 10, ant.y * 10, 10, 10, Color::RED);
        d.draw_text(&format!("Iteration: {}", iteration), 10, 10, 20, Color::BLACK);

        if !simulation_running {
            d.draw_text("Simulation Paused", 10, 40, 20, Color::RED);
        }
    }
}

fn setup() -> Config {
    let rows = std::env::var("LINES");
    let cols = std::env::var("COLUMNS");
    let iter = std::env::var("ITERATIONS");
    let mut r = rows.unwrap_or("60".to_string()).parse().unwrap_or(60);
    let mut c = cols.unwrap_or("80".to_string()).parse().unwrap_or(80);
    let mut i = iter.unwrap_or("1000".to_string()).parse().unwrap_or(1000);

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        c = args[1].parse().unwrap_or(c);
    }

    if args.len() > 2 {
        r = args[2].parse().unwrap_or(r);
    }

    if args.len() > 3 {
        i = args[3].parse().unwrap_or(i);
    }

    Config {
        max_x: c,
        max_y: r,
        iterations: i,
    }
}