use {input_macro::input, std::convert::TryInto};
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

fn get_direction_offset(direction: Direction) -> [(isize, isize); 13] {
    let base_offsets = [
        (2, -3),
        (-2, -3),
        (-1, -3),
        (1, -3),
        (0, -3),
        (-1, -2),
        (1, -2),
        (0, -2),
        (-1, -1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (1, 0),
    ];
    match direction {
        Direction::North => base_offsets,
        Direction::South => base_offsets.map(|(x, y)| (-x, -y)),
        Direction::East => base_offsets.map(|(x, y)| (-y, x)),
        Direction::West => base_offsets.map(|(x, y)| (y, -x)),
    }
}

fn get_direction(direction: Direction) -> (isize, isize) {
    match direction {
        Direction::North => (0, -1),
        Direction::East => (1, 0),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
    }
}

fn try_move_player(player: &Player, dx: isize, dy: isize) -> (usize, usize, Block) {
    let x = player.x.checked_add_signed(dx);
    let y = player.y.checked_add_signed(dy);
    match (x, y) {
        (Some(x), Some(y)) if x < 5 && y < 5 => (x, y, MAP[y][x]),
        _ => (0, 0, Block::Wall),
    }
}

fn rotate_left(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn rotate_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::West => Direction::North,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
    }
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
                let (dx, dy) = get_direction(player.direction);
                let (x, y, block) = try_move_player(&player, dx, dy);

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
                player.direction = rotate_left(player.direction);
                action = Action::Rotate;
            },
            "d" => {
                player.direction = rotate_right(player.direction);
                action = Action::Rotate;
            },
            _ => {},
        }
    }
}

fn render(player: &Player) {
    let offsets = get_direction_offset(player.direction);
    let mut frame = [[' '; 26]; 14];
    #[expect(unused_variables)]
    for (i, (dx, dy)) in offsets.into_iter().enumerate() {
        let (x, y, block) = try_move_player(player, dx, dy);

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

        let (dx, dy) = get_direction(player.direction);
        let (x, y, block) = try_move_player(&player, dx, dy);

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
