use std::f64::consts::PI;
use std::io::{self, Write};

const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;
const SCREEN_WIDTH: usize = 120;
const SCREEN_HEIGHT: usize = 30;
const FOV: f64 = PI / 4.0;
const MAX_DEPTH: f64 = 16.0;

const MAP: [&str; MAP_HEIGHT] = [
    "################",
    "#..............#",
    "#..###.........#", //Выход - E, спавн - P, # - стена
    "#..............#",
    "#....####......#",
    "#..............#",
    "#...####.......#",
    "#..............#",
    "####...........#",
    "#..............#",
    "#.......########",
    "#..............#",
    "#..............#",
    "#.......########",
    "#..........E..P#",
    "################",
];

struct Player {
    x: f64,
    y: f64,
    angle: f64,
}

impl Player {
    fn new(x: f64, y: f64, angle: f64) -> Self {
        Self { x, y, angle }
    }

    fn move_player(&mut self, direction: char, map: &Vec<Vec<char>>) {
        let move_speed = 0.1;
        let rot_speed = 0.1;

        match direction {
            'w' => {
                let new_x = self.x + self.angle.cos() * move_speed;
                let new_y = self.y + self.angle.sin() * move_speed;
                if map[new_y as usize][new_x as usize] != '#' {
                    self.x = new_x;
                    self.y = new_y;
                }
            }
            's' => {
                let new_x = self.x - self.angle.cos() * move_speed;
                let new_y = self.y - self.angle.sin() * move_speed;
                if map[new_y as usize][new_x as usize] != '#' {
                    self.x = new_x;
                    self.y = new_y;
                }
            }
            'a' => self.angle -= rot_speed,
            'd' => self.angle += rot_speed,
            _ => {}
        }
    }
}

fn load_map() -> (Vec<Vec<char>>, Player) {
    let mut player_x = 0.0;
    let mut player_y = 0.0;

    let map: Vec<Vec<char>> = MAP.iter().enumerate().map(|(y, row)| {
        row.chars().enumerate().map(|(x, c)| {
            if c == 'P' {
                player_x = x as f64;
                player_y = y as f64;
                '.'
            } else {
                c
            }
        }).collect()
    }).collect();

    let player = Player::new(player_x, player_y, 0.0);
    (map, player)
}

fn render_screen(player: &Player, map: &Vec<Vec<char>>) {
    let mut screen = vec![vec![' '; SCREEN_HEIGHT]; SCREEN_WIDTH];

    for x in 0..SCREEN_WIDTH {
        let ray_angle = player.angle - FOV / 2.0 + (x as f64 / SCREEN_WIDTH as f64) * FOV;

        let mut distance_to_wall = 0.0;
        let mut hit_wall = false;

        let eye_x = ray_angle.cos();
        let eye_y = ray_angle.sin();

        while !hit_wall && distance_to_wall < MAX_DEPTH {
            distance_to_wall += 0.1;

            let test_x = (player.x + eye_x * distance_to_wall).round() as isize;
            let test_y = (player.y + eye_y * distance_to_wall).round() as isize;

            if test_x < 0 || test_x >= MAP_WIDTH as isize || test_y < 0 || test_y >= MAP_HEIGHT as isize {
                hit_wall = true;
                distance_to_wall = MAX_DEPTH;
            } else if map[test_y as usize][test_x as usize] == '#' {
                hit_wall = true;
            }
        }

        let ceiling = (SCREEN_HEIGHT as f64 / 2.0) - SCREEN_HEIGHT as f64 / distance_to_wall;
        let floor = SCREEN_HEIGHT as f64 - ceiling;

        for y in 0..SCREEN_HEIGHT {
            if y as f64 <= ceiling {
                screen[x][y] = ' '; 
            } else if y as f64 > ceiling && y as f64 <= floor {
                screen[x][y] = '|'; 
            } else {
                screen[x][y] = '.'; 
            }
        }
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            print!("{}", screen[x][y]);
        }
        println!();
    }
}

fn check_victory(player: &Player, map: &Vec<Vec<char>>) -> bool {
    let player_x = player.x.round() as usize;
    let player_y = player.y.round() as usize;
    map[player_y][player_x] == 'E'
}

fn main() {
    let (map, mut player) = load_map();

    loop {
        render_screen(&player, &map);

        if check_victory(&player, &map) {
            println!("Поздравляем! Вы прошли лабиринт!");
            break;
        }

        let mut input = String::new();
        print!("Введите команду (w/a/s/d): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(direction) = input.chars().next() {
            player.move_player(direction, &map);
        }
    }
}
