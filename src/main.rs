pub struct Config {
    pub max_x: i32,
    pub max_y: i32,
    pub iterations: i32,
}

pub struct Board {
    max_x: usize,
    max_y: usize,
    data: Vec<bool>,
}

pub struct Ant {
    x: i32,
    y: i32,
    dir: i8,
    max_x: i32,
    max_y: i32,
}

impl Ant {
    fn new(x: i32, y: i32, max_x: i32, max_y: i32) -> Self {
        Self { x, y, dir: 0, max_x, max_y }
    }

    fn move_ant(&mut self, dirty: bool) {
        if dirty {
            // jeśli znajduje się na polu czarnym to obraca się w lewo (o kąt prosty), 
            // zmienia kolor pola na biały i przechodzi na następną komórkę;
            match self.dir {
                0 => {
                    self.x -= 1;
                    self.dir = 3;
                }
                1 => {
                    self.y -= 1;
                    self.dir = 0;
                }
                2 => {
                    self.x += 1;
                    self.dir = 1;
                }
                3 => {
                    self.y += 1;
                    self.dir = 2;
                }
                _ => {}
            }
        } else {
            match self.dir {
                0 => {
                    self.x += 1;
                    self.dir = 1;
                }
                1 => {
                    self.y += 1;
                    self.dir = 2;
                }
                2 => {
                    self.x -= 1;
                    self.dir = 3;
                }
                3 => {
                    self.y -= 1;
                    self.dir = 0;
                }
                _ => {}
            }
        }
        if self.x > self.max_x - 1 {
            self.x = 0;
        }
        if self.x < 0 {
            self.x = self.max_x - 1;
        }
        if self.y > self.max_y - 1 {
            self.y = 0;
        }
        if self.y < 0 {
            self.y = self.max_y - 1;
        }
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

    fn draw(&self) {
        for i in 0..self.max_x {
            for j in 0..self.max_y {
                at_xy(i as i32 * 2, j as i32, if self.get(i, j) { "██" } else { "░░" });
            }
        }
    }
}

fn main() {
    let cfg = setup();

//     let (mut rl, thread) = raylib::init()
//     .size(640, 480)
//     .title("Hello, World")
//     .build();

//     while !rl.window_should_close() {
//         let mut d = rl.begin_drawing(&thread);

//         d.clear_background(Color::WHITE);
//         d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
// }

    println!("rozmiar: {}x{}", cfg.max_x, cfg.max_y);
    let mut board = Board::new(cfg.max_x as usize, cfg.max_y as usize);
    let mut ant = Ant::new(cfg.max_x / 2, cfg.max_y / 2, cfg.max_x, cfg.max_y);
    board.draw();
    for _i in 0..cfg.iterations {
        let index = ant.y * cfg.max_x + ant.x;
        board.set(ant.x as usize, ant.y as usize, !board.get(ant.x as usize, ant.y as usize));
        ant.move_ant(board.data[index as usize]);
        at_xy(ant.x * 2, ant.y, if board.get(ant.x as usize, ant.y as usize) { "██" } else { "░░" });
    }
}

fn setup() -> Config {
    let rows = std::env::var("LINES");
    let cols = std::env::var("COLUMNS");
    let iter = std::env::var("ITERATIONS");
    let mut r = rows.unwrap_or("25".to_string()).parse().unwrap_or(25);
    let mut c = cols.unwrap_or("80".to_string()).parse().unwrap_or(80);
    let mut i = iter.unwrap_or("100000".to_string()).parse().unwrap_or(100000);

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

fn at_xy(x: i32, y: i32, text: &str) {
    print!("\x1b[{};{}H{}", y + 1, x + 1, text);
}
