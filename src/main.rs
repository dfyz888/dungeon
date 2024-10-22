use input_macro::input;
mod exit;
mod wall;

#[derive(Clone, Copy)]
enum Block {
    Air,
    Wall,
    Exit,
}

static MAP: [[Block; 5]; 5] = {
    use Block::*;
    [
        [Air, Air, Air, Air, Wall],
        [Air, Wall, Air, Air, Exit],
        [Air, Air, Air, Air, Wall],
        [Wall, Air, Wall, Air, Air],
        [Air, Air, Exit, Air, Wall],
    ]
};

struct Player {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

enum Action {
    None,
    Walk,
    Wall,
    Rotate,
    Exit,
}

fn main() {
    let mut player = Player {
        x: 0,
        y: 0,
        direction: Direction::North,
    };
    let mut action = Action::None;

    loop {
        render(&player);

        if let Action::Wall = action {
            println!("Стена!");
            action = Action::None;
        }
        let input = input!("> ");
        match input.as_str() {
            "w" => {
                let (dx, dy) = match player.direction {
                    Direction::North => (0, -1),
                    Direction::East => (1, 0),
                    Direction::South => (0, 1),
                    Direction::West => (-1, 0),
                };

                let x = player.x.checked_add_signed(dx);
                let y = player.y.checked_add_signed(dy);
                let (x, y, block) = match (x, y) {
                    (Some(x), Some(y)) if x < 5 && y < 5 => (x, y, MAP[y][x]),
                    _ => (0, 0, Block::Wall),
                };

                match block {
                    Block::Air => {
                        player.x = x;
                        player.y = y;
                        action = Action::Walk;
                    },
                    Block::Wall => {
                        action = Action::Wall;
                    },
                    Block::Exit => {
                        action = Action::Exit;
                    },
                }
            },
            "a" => {
                player.direction = match player.direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                };

                action = Action::Rotate;
            },
            "d" => {
                player.direction = match player.direction {
                    Direction::North => Direction::East,
                    Direction::West => Direction::North,
                    Direction::South => Direction::West,
                    Direction::East => Direction::South,
                };
                action = Action::Rotate;
            },
            _ => {},
        }
    }
}

fn render(player: &Player) {
    let var_name = match player.direction {
        Direction::North => {
            [
                (0, -2, -3),
                (1, 2, -3),
                (2, -1, -3),
                (3, 0, -3),
                (4, 1, -3),
                (5, -1, -2),
                (6, 1, -2),
                (7, 0, -2),
                (8, -1, -1),
                (9, 1, -1),
                (10, 0, -1),
                (11, -1, 0),
                (12, 1, 0),
            ]
        },
        Direction::South => {
            [
                (0, 2, 3),
                (1, -2, 3),
                (2, 1, 3),
                (3, -1, 3),
                (4, 0, 3),
                (5, 1, 2),
                (6, -1, 2),
                (7, 0, 2),
                (8, 1, 1),
                (9, -1, 1),
                (10, 0, 1),
                (11, 1, 0),
                (12, -1, 0),
            ]
        },
        Direction::East => {
            [
                (0, 3, 2),
                (1, 3, -2),
                (2, 3, 1),
                (3, 3, -1),
                (4, 3, 0),
                (5, 2, 2),
                (6, 2, -2),
                (7, 2, 0),
                (8, 1, -1),
                (9, 1, 1),
                (10, 1, 0),
                (11, 0, -1),
                (12, 0, 1),
            ]
        },
        Direction::West => {
            [
                (0, -3, -2),
                (1, -3, 2),
                (2, -3, -1),
                (3, -3, 1),
                (4, -3, 0),
                (5, -2, -1),
                (6, -2, 1),
                (7, -2, 0),
                (8, -1, -1),
                (9, -1, 1),
                (10, -1, 0),
                (11, 0, -1),
                (12, 0, 1),
            ]
        },
    };
    let positions = var_name;
    let mut frame = [[' '; 26]; 14];
    for (i, dx, dy) in positions {
        let x = player.x.checked_add_signed(dx);
        let y = player.y.checked_add_signed(dy);
        let (_, _, block) = match (x, y) {
            (Some(x), Some(y)) if x < 5 && y < 5 => (x, y, MAP[y][x]),
            _ => (0, 0, Block::Wall),
        };
        let pattern = match block {
            Block::Air => continue,
            Block::Wall => wall::PATTERNS[i * 4 + player.direction as usize],
            Block::Exit => exit::PATTERNS[i * 4 + player.direction as usize],
        };
        for (i, line) in pattern.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char != '`' {
                    frame[i][j] = char;
                }
            }
        }
    }

    println!("\x1Bc");
    for row in frame.iter() {
        for &cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_initial_position() {
        let player = Player {
            x: 0,
            y: 0,
            direction: Direction::North,
        };
        assert_eq!(player.x, 0);
        assert_eq!(player.y, 0);
        assert_eq!(matches!(player.direction, Direction::North), true);
    }

    #[test]
    fn test_player_movement_north() {
        let mut player = Player {
            x: 2,
            y: 2,
            direction: Direction::North,
        };
        #[expect(unused_assignments)]
        let mut action = Action::None;
        let (dx, dy) = match player.direction {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };

        let x = player.x.checked_add_signed(dx);
        let y = player.y.checked_add_signed(dy);
        let (x, y, block) = match (x, y) {
            (Some(x), Some(y)) if x < 5 && y < 5 => (x, y, MAP[y][x]),
            _ => (0, 0, Block::Wall),
        };

        match block {
            Block::Air => {
                player.x = x;
                player.y = y;
                action = Action::Walk;
            },
            Block::Wall => {
                action = Action::Wall;
            },
            Block::Exit => {
                action = Action::Exit;
            },
        }

        assert_eq!(player.x, 2);
        assert_eq!(player.y, 1);
        assert_eq!(matches!(action, Action::Walk), true);
    }
}
