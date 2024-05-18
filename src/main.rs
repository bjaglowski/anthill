use raylib::prelude::*;
use std::{thread, time};

pub struct Config {
    pub max_x: i32,
    pub max_y: i32,
    pub iterations: i32,
    pub interval: i32,
}

pub enum Direction {
    UpLeft,
    UpRight,
    Right,
    DownRight,
    DownLeft,
    Left,
}

pub struct Item {
    x: i32,
    y: i32,
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
    item: Vec<Item>,
}

impl Direction {

    fn rotate_left(&self) -> Self {
        match self {
            Direction::UpLeft => Direction::Left,
            Direction::Left => Direction::DownLeft,
            Direction::DownLeft => Direction::DownRight,
            Direction::DownRight => Direction::Right,
            Direction::Right => Direction::UpRight,
            Direction::UpRight => Direction::UpLeft,
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::UpLeft => Direction::UpRight,
            Direction::UpRight => Direction::Right,
            Direction::Right => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
            Direction::DownLeft => Direction::Left,
            Direction::Left => Direction::UpLeft,
        }
    }
}

impl Item {
    fn new(x: i32, y: i32, ) -> Self {
        Self { x, y}
    }
}

impl Ant {
    fn new(x: i32, y: i32, max_x: i32, max_y: i32) -> Self {
        Self { x, y, dir: Direction::UpLeft, max_x, max_y, item: Vec::with_capacity(1)}
    }

    fn move_ant(&mut self, dirty: bool) {

        if dirty {
            self.dir = self.dir.rotate_left();
        } else {
            self.dir = self.dir.rotate_right();
        }

        if self.y % 2 == 0 {
            match self.dir {
                Direction::UpLeft => { self.x -= 1; self.y -= 1; }
                Direction::UpRight => { self.y -= 1; }
                Direction::Right => { self.x += 1; }
                Direction::DownRight => { self.y += 1; }
                Direction::DownLeft => { self.x -= 1; self.y += 1; }
                Direction::Left => { self.x -= 1; }
            }
        } else {
            match self.dir {
                Direction::UpLeft => { self.y -= 1; }
                Direction::UpRight => { self.x += 1; self.y -= 1; }
                Direction::Right => { self.x += 1; }
                Direction::DownRight => { self.x += 1; self.y += 1; }
                Direction::DownLeft => { self.y += 1; }
                Direction::Left => { self.x -= 1; }
            }
        }

        self.x = self.x.rem_euclid(self.max_x);
        self.y = self.y.rem_euclid(self.max_y);
    }
}

impl Board {
    fn new(x: usize, y: usize) -> Self {
        let mut v: Vec<bool> = Vec::with_capacity(x * y);
        for _ in 0..x * y {
            v.push(false);
        }
        Self { max_x: x, max_y: y, data: v }
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        let index = y * self.max_x + x;
        self.data[index] = val;
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let index = y * self.max_x + x;
        self.data[index]
    }

    fn draw(&self, d: &mut RaylibDrawHandle, radius: f32) {
        for j in 0..self.max_y {
            for i in 0..self.max_x {
                // https://www.redblobgames.com/grids/hexagons/
                let x_offset = if j % 2 == 0 { radius } else { radius + 3.0_f32.sqrt() * radius / 2.0 };
                let color = if self.get(i, j) { Color::BLACK } else { Color::LIGHTGRAY };
                let center_x = (i as f32 * 3.0_f32.sqrt() * radius) + x_offset;
                let center_y = (j as f32 * 3.0/2.0 * radius) + radius;

                draw_hexagon(d, center_x, center_y, radius * 0.95, Color::LIGHTGRAY);
            }
        }
    }
}

fn draw_hexagon(d: &mut RaylibDrawHandle, x: f32, y: f32, radius: f32, color: Color) {
    let mut points = [Vector2::zero(); 6];
    for i in 0..6 {
        // https://www.redblobgames.com/grids/hexagons/
        let angle = std::f32::consts::PI / 3.0 * (i as f32);
        points[i] = Vector2::new(x + radius * angle.cos(), y + radius * angle.sin());
    }
    d.draw_poly(Vector2::new(x, y), 6, radius, 0.0, color);
}

fn main() {
    let cfg = setup();

    let window_size_x = 800;
    let window_size_y = 600;

    let (mut rl, thread) = init()
        .size(window_size_x, window_size_y)
        .title("Ants on Hexagonal Grid")
        .build();

    let mut board = Board::new(cfg.max_x as usize, cfg.max_y as usize);

    let mut ants: Vec<Ant> = Vec::with_capacity(5);
    ants.push(Ant::new(cfg.max_x / 2, cfg.max_y / 2, cfg.max_x, cfg.max_y));
    ants.push(Ant::new(cfg.max_x / 4, cfg.max_y / 4, cfg.max_x, cfg.max_y));
    ants.push(Ant::new(cfg.max_x / 3, cfg.max_y / 3, cfg.max_x, cfg.max_y));

    let mut iteration = 0;
    let mut simulation_running = true;
    let radius_x = window_size_x as f32 / (cfg.max_x as f32 + 0.5) / 3.0_f32.sqrt();
    let radius_y = window_size_y as f32 / (cfg.max_y as f32 + 0.5) / (3.0/2.0);
    let millis_interval = time::Duration::from_millis(cfg.interval as u64);

    let radius = if radius_x > radius_y {radius_y} else {radius_x};

    while !rl.window_should_close() {
        if simulation_running {
            for mut ant in &mut ants {
                board.set(ant.x as usize, ant.y as usize, !board.get(ant.x as usize, ant.y as usize));
                ant.move_ant(board.get(ant.x as usize, ant.y as usize));
            }

            iteration += 1;
            if iteration >= cfg.iterations {
                simulation_running = false;
            }
        }

        thread::sleep(millis_interval);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        board.draw(&mut d, radius);

        // https://www.redblobgames.com/grids/hexagons/
        for mut ant in &mut ants {
            let ant_x = (ant.x as f32 * 3.0_f32.sqrt() * radius) + if ant.y % 2 == 0 { radius } else { radius + 3.0_f32.sqrt() * radius / 2.0 };
            let ant_y = (ant.y as f32 * 3.0/2.0 * radius) + radius;
            draw_hexagon(&mut d, ant_x, ant_y, radius, Color::RED);
        }

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
    let itvl = std::env::var("INTERVAL");
    let mut r = rows.unwrap_or("20".to_string()).parse().unwrap_or(20);
    let mut c = cols.unwrap_or("20".to_string()).parse().unwrap_or(20);
    let mut i = iter.unwrap_or("1000".to_string()).parse().unwrap_or(1000);
    let mut t = itvl.unwrap_or("0".to_string()).parse().unwrap_or(0);

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

    if args.len() > 4 {
        t = args[4].parse().unwrap_or(i);
    }

    Config {
        max_x: c,
        max_y: r,
        iterations: i,
        interval: t,
    }
}
