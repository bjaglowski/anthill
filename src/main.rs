use raylib::prelude::*;
use std::{thread, time};
use rand::Rng;
use rand::seq::SliceRandom;

pub struct Config {
    pub max_x: i32,
    pub max_y: i32,
    pub iterations: i32,
    pub interval: i32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    UpLeft,
    UpRight,
    Right,
    DownRight,
    DownLeft,
    Left,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ItemType {
    Lisc,
    Kij,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Item {
    x: i32,
    y: i32,
    freeze_time_left: i32,
    item_type: ItemType,
}

pub struct Board {
    max_x: usize,
    max_y: usize,
    items: Vec<Item>,
    ants: Vec<Ant>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Ant {
    x: i32,
    y: i32,
    dir: Direction,
    max_x: i32,
    max_y: i32,
    carrying_item: Option<Item>,
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
    fn new(x: i32, y: i32, freeze_time_left: i32, item_type: ItemType) -> Self {
        Self { x, y , freeze_time_left, item_type}
    }
}

impl Ant {
    fn new(x: i32, y: i32, max_x: i32, max_y: i32) -> Self {
        Self { x, y, dir: Direction::UpLeft, max_x, max_y, carrying_item: None }
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

    fn get_neighbors(&self) -> Vec<(i32, i32)>{
        if self.y % 2 == 0 {
            vec![
                ((self.x - 1).rem_euclid(self.max_x), (self.y - 1).rem_euclid(self.max_y)),
                (self.x, (self.y - 1).rem_euclid(self.max_y)),
                ((self.x + 1).rem_euclid(self.max_x), self.y),
                (self.x, (self.y + 1).rem_euclid(self.max_y)),
                ((self.x - 1).rem_euclid(self.max_x), (self.y + 1).rem_euclid(self.max_y)),
                ((self.x - 1).rem_euclid(self.max_x), self.y),
            ]
        } else {
            vec![
                (self.x, (self.y - 1).rem_euclid(self.max_y)),
                ((self.x + 1).rem_euclid(self.max_x), (self.y - 1).rem_euclid(self.max_y)),
                ((self.x + 1).rem_euclid(self.max_x), self.y),
                ((self.x + 1).rem_euclid(self.max_x), (self.y + 1).rem_euclid(self.max_y)),
                (self.x, (self.y + 1).rem_euclid(self.max_y)),
                ((self.x - 1).rem_euclid(self.max_x), self.y),
            ]
        }
    }
}

impl Board {
    fn new(x: usize, y: usize) -> Self {
        let items: Vec<Item> = Vec::with_capacity(x * y);
        let ants: Vec<Ant> = Vec::with_capacity(x * y);
        Self { max_x: x, max_y: y, items, ants }
    }

    fn calculate_center_x(&self, x: i32, y: i32, radius: f32) -> f32{
        x as f32 * 3.0_f32.sqrt() * radius + if y % 2 == 0 { radius } else { radius + 3.0_f32.sqrt() * radius / 2.0 }
    }

    fn is_field_free(&self, x: i32, y: i32) -> bool{
        if self.items.iter().any(|item| item.x == x && item.y == y) {
            return false
        }
        if self.ants.iter().any(|ant| ant.x == x && ant.y == y) {
            return false
        }
        true
    }

    fn calculate_center_y(&self, y: i32, radius: f32) -> f32{
        y as f32 * 3.0/2.0 * radius + radius
    }

    fn pick_or_leave(&mut self) {

        for ant in &mut self.ants {
            if let Some(carrying_item) = ant.carrying_item {
                for (nx, ny) in ant.get_neighbors().into_iter() {
                    let similar_items_nearby_count = self.items.iter().filter(|item| item.x == nx && item.y == ny && carrying_item.item_type == item.item_type).count();
                    if similar_items_nearby_count > 0 {
                        if let Some(_) = self.items.iter().position(|item| item.x == ant.x && item.y == ant.y ) {
                            continue
                        } else {
                            self.items.push(Item::new(ant.x, ant.y, similar_items_nearby_count as i32 * 2, carrying_item.item_type));
                            ant.carrying_item = None;
                        }
                    }
                }
            } else {
                if let Some(index) = self.items.iter().position(|item| item.x == ant.x && item.y == ant.y) {
                    let mut item = self.items.remove(index);
                    if item.freeze_time_left == 0 {
                        ant.carrying_item = Some(item);
                    } else {
                        item.freeze_time_left = item.freeze_time_left - 1;
                        self.items.push(item);
                    }
                }
            }
        }
    }
    // fn move_ants(&mut self) {
    //
    //     let mut rng = rand::thread_rng();
    //
    //     for ant in &mut self.ants {
    //
    //         let num:i32 = rng.gen();
    //
    //         ant.move_ant(num % 2 == 0);
    //         ant.move_ant(num % 2 == 0);
    //         ant.move_ant(num % 2 == 0);
    //         ant.move_ant(num % 2 == 0);
    //
    //
    //         // let num:i32 = rng.gen();
    //         // let (x, y) = ant.get_neighbors()[num as usize % 6 ];
    //         // if self.is_field_free(x, y){
    //         //     ant.x = x;
    //         //     ant.y = y;
    //         // }
    //     }
    // }

    fn draw(&self, d: &mut RaylibDrawHandle, radius: f32) {
        for j in 0..self.max_y {
            for i in 0..self.max_x {
                // https://www.redblobgames.com/grids/hexagons/
                let x_offset = if j % 2 == 0 { radius } else { radius + 3.0_f32.sqrt() * radius / 2.0 };
                let center_x = (i as f32 * 3.0_f32.sqrt() * radius) + x_offset;
                let center_y = (j as f32 * 3.0/2.0 * radius) + radius;

                draw_hexagon(d, center_x, center_y, radius * 0.95, Color::LIGHTGRAY);
            }
        }

        for item in &self.items {
            let item_x = self.calculate_center_x(item.x, item.y, radius);
            let item_y = self.calculate_center_y(item.y, radius);

            let color = match item.item_type {
                ItemType::Lisc => Color::BLUE,
                ItemType::Kij => Color::YELLOW,
            };

            draw_hexagon(d, item_x, item_y, radius * 0.95, color);
        }

        for ant in &self.ants {
            let ant_x = self.calculate_center_x(ant.x, ant.y, radius);
            let ant_y = self.calculate_center_y(ant.y, radius);
            draw_hexagon(d, ant_x, ant_y, radius * 0.95, Color::RED);
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
    let mut rng = rand::thread_rng();

    let (mut rl, thread) = init()
        .size(window_size_x, window_size_y)
        .title("Ants on Hexagonal Grid")
        .build();

    let mut board = Board::new(cfg.max_x as usize, cfg.max_y as usize);

    for _ in 0..cfg.max_x*cfg.max_y/30 {
        let rand_x = rng.gen_range(0..cfg.max_x);
        let rand_y = rng.gen_range(0..cfg.max_y);
        board.ants.push(Ant::new(rand_x, rand_y, cfg.max_x, cfg.max_y));
    }

    for iter in 0..cfg.max_x*cfg.max_y/10 {
        let rand_x = rng.gen_range(0..cfg.max_x);
        let rand_y = rng.gen_range(0..cfg.max_y);

        let item_type = match iter % 2 {
            0 => ItemType::Kij,
            _ => ItemType::Lisc,
        };

        board.items.push(Item::new(rand_x, rand_y, 0, item_type));
    }

    let mut iteration = 0;
    let mut simulation_running = true;
    let radius_x = window_size_x as f32 / (cfg.max_x as f32 + 0.5) / 3.0_f32.sqrt();
    let radius_y = window_size_y as f32 / (cfg.max_y as f32 + 0.5) / (3.0/2.0);
    let millis_interval = time::Duration::from_millis(cfg.interval as u64);
    let radius = if radius_x > radius_y { radius_y } else { radius_x };

    while !rl.window_should_close() {
        if simulation_running {
            board.pick_or_leave();
            for ant in &mut board.ants {

                for _ in 0..5 {
                    let mut neigbors = ant.get_neighbors().into_iter().collect::<Vec<_>>();
                    neigbors.shuffle(&mut rng);


                    for (nx, ny) in neigbors {
                        if let Some(item) = &ant.carrying_item {
                            if board.items.iter().any(|item| item.x == nx && item.y == ny) {
                                continue
                            } else {
                                ant.x = nx;
                                ant.y = ny;
                                break
                            }

                        } else {
                            ant.x = nx;
                            ant.y = ny;
                            break
                        }
                    }
                }

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

        d.draw_text(&format!("Iteration: {}", iteration), 10, 10, 20, Color::BLACK);

        if !simulation_running {
            d.draw_text("Simulation Paused", 10, 40, 20, Color::RED);
        }
    }
}

fn setup() -> Config {
    let rows = std::env::var("LINES");
    let cols = std::env::var("COLUMNS");
    let iter = std::env::var("ANTS");
    let interval = std::env::var("INTERVAL");
    let mut r = rows.unwrap_or("20".to_string()).parse().unwrap_or(20);
    let mut c = cols.unwrap_or("20".to_string()).parse().unwrap_or(20);
    let mut i = iter.unwrap_or("1000".to_string()).parse().unwrap_or(1000);
    let mut t = interval.unwrap_or("0".to_string()).parse().unwrap_or(0);

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
