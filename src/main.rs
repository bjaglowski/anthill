use raylib::prelude::*;
use std::{thread, time};
use rand::prelude::ThreadRng;
use rand::Rng;
use rand::seq::SliceRandom;

pub struct Config {
    pub max_x: usize,
    pub max_y: usize,
    pub iterations: usize,
    pub interval: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ItemType {
    Leaf, // blue
    Stick, // yellow
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Item {
    x: usize,
    y: usize,
    freeze_time_left: usize,
    item_type: ItemType,
}

pub struct Board {
    max_x: usize,
    max_y: usize,
    items: Vec<Item>,
    ants: Vec<Ant>,
}

pub struct Ant { // red
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    carrying_item: Option<Item>,
}

impl Item {
    fn new(x: usize, y: usize, freeze_time_left: usize, item_type: ItemType) -> Self {
        Self { x, y, freeze_time_left, item_type }
    }
}

impl Ant {
    fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> Self {
        Self { x, y, max_x, max_y, carrying_item: None }
    }

    fn get_neighbors(&self) -> Vec<(usize, usize)> {
        // get neighboring fields
        let x: i32 = self.x as i32;
        let y: i32 = self.y as i32;
        let max_x: i32 = self.max_x as i32;
        let max_y: i32 = self.max_y as i32;


        if self.y % 2 == 0 {
            vec![
                ((x - 1).rem_euclid(max_x) as usize, (y - 1).rem_euclid(max_y) as usize),
                (self.x, (y - 1).rem_euclid(max_y) as usize),
                ((self.x + 1).rem_euclid(self.max_x), self.y),
                (self.x, (self.y + 1).rem_euclid(self.max_y)),
                ((x - 1).rem_euclid(max_x) as usize, (self.y + 1).rem_euclid(self.max_y)),
                ((x - 1).rem_euclid(max_x) as usize, self.y),
            ]
        } else {
            vec![
                (self.x, (self.y - 1).rem_euclid(self.max_y)),
                ((self.x + 1).rem_euclid(self.max_x), (y - 1).rem_euclid(max_y) as usize),
                ((self.x + 1).rem_euclid(self.max_x), self.y),
                ((self.x + 1).rem_euclid(self.max_x), (self.y + 1).rem_euclid(self.max_y)),
                (self.x, (self.y + 1).rem_euclid(self.max_y)),
                ((x - 1).rem_euclid(max_x) as usize, self.y),
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

    fn calculate_center_x(&self, x: usize, y: usize, radius: f32) -> f32 {
        // https://www.redblobgames.com/grids/hexagons/
        x as f32 * 3.0_f32.sqrt() * radius + if y % 2 == 0 { radius } else { radius + 3.0_f32.sqrt() * radius / 2.0 }
    }

    fn calculate_center_y(&self, y: usize, radius: f32) -> f32 {
        // https://www.redblobgames.com/grids/hexagons/
        y as f32 * 3.0 / 2.0 * radius + radius
    }

    fn pick_or_leave(&mut self) {
        for ant in &mut self.ants {

            // is ant carrying something?
            if let Some(carrying_item) = ant.carrying_item {
                // if let Some(_) = self.items.iter().position(|item| item.x == ant.x && item.y == ant.y) {
                //     continue;
                // }

                // check if some similar items are nearby
                for (nx, ny) in ant.get_neighbors().into_iter() {
                    let similar_items_nearby_count = self.items.iter().filter(|item| item.x == nx && item.y == ny && carrying_item.item_type == item.item_type).count();

                    if similar_items_nearby_count > 0 {

                        // attach the item harder if more elements there are nearby
                        let freeze_time_left = 2_u32.pow(similar_items_nearby_count as u32) as usize;
                        self.items.push(Item::new(ant.x, ant.y, freeze_time_left, carrying_item.item_type));
                        ant.carrying_item = None;
                        break
                    }
                }
            } else {

                // is ant staying on some item?
                if let Some(index) = self.items.iter().position(|item| item.x == ant.x && item.y == ant.y) {
                    let mut item = self.items.remove(index);


                    if item.freeze_time_left == 0 {
                        // the item's attach is sufficiently weakened to pick
                        ant.carrying_item = Some(item);
                    } else {
                        // the item is attached too hard to pick, bite it
                        item.freeze_time_left = item.freeze_time_left - 1;
                        self.items.push(item);
                    }
                }
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle, radius: f32) {

        // draw fields
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let center_x = self.calculate_center_x(x, y, radius);
                let center_y = self.calculate_center_y(y, radius);

                draw_hexagon(d, center_x, center_y, radius * 0.95, Color::LIGHTGRAY);
            }
        }

        // draw items
        for item in &self.items {
            let item_x = self.calculate_center_x(item.x, item.y, radius);
            let item_y = self.calculate_center_y(item.y, radius);

            let color = match item.item_type {
                ItemType::Leaf => Color::BLUE,
                ItemType::Stick => Color::YELLOW,
            };

            draw_hexagon(d, item_x, item_y, radius * 0.95, color);
        }

        // draw ants
        for ant in &self.ants {
            let ant_x = self.calculate_center_x(ant.x, ant.y, radius);
            let ant_y = self.calculate_center_y(ant.y, radius);

            let mut color = Color::RED;
            if let Some(_) = &ant.carrying_item {
                color = Color::ORANGE; // orange ants are carrying some items
            }

            draw_hexagon(d, ant_x, ant_y, radius * 0.95, color);
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
        .title("Ants on hexagonal grid")
        .build();

    let mut board = Board::new(cfg.max_x, cfg.max_y);

    // add ants randomly
    for _ in 0..cfg.max_x * cfg.max_y / 30 {
        let rand_x = rng.gen_range(0..cfg.max_x);
        let rand_y = rng.gen_range(0..cfg.max_y);

        board.ants.push(Ant::new(rand_x, rand_y, cfg.max_x, cfg.max_y));
    }

    // add items randomly
    for iter in 0..cfg.max_x * cfg.max_y / 5 {
        let rand_x = rng.gen_range(0..cfg.max_x);
        let rand_y = rng.gen_range(0..cfg.max_y);

        let item_type = match iter % 2 {
            0 => ItemType::Stick,
            _ => ItemType::Leaf,
        };

        board.items.push(Item::new(rand_x, rand_y, 0, item_type));
    }

    let mut iteration = 0;
    let mut simulation_running = true;
    let radius_x = window_size_x as f32 / (cfg.max_x as f32 + 0.5) / 3.0_f32.sqrt();
    let radius_y = window_size_y as f32 / (cfg.max_y as f32 + 0.5) / (3.0 / 2.0);
    let millis_interval = time::Duration::from_millis(cfg.interval as u64);
    let radius = if radius_x > radius_y { radius_y } else { radius_x };

    while !rl.window_should_close() {
        if simulation_running {
            board.pick_or_leave();
            move_ant(&mut board, &mut rng);

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
            d.draw_text("Simulation paused", 10, 40, 20, Color::RED);
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

fn move_ant(board: &mut Board, mut rng: &mut ThreadRng) {
    for ant in &mut board.ants {
        // move ant by 5 fields randomly
        for _ in 0..5 {
            let mut neighbors = ant.get_neighbors().into_iter().collect::<Vec<_>>();
            neighbors.shuffle(&mut rng);


            for (nx, ny) in neighbors {
                // block move if ant which is carrying an item is trying to step on another item
                if let Some(_) = &ant.carrying_item {
                    if board.items.iter().any(|item| item.x == nx && item.y == ny) {
                        continue;
                    }
                }
                ant.x = nx;
                ant.y = ny;
                break;
            }
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
