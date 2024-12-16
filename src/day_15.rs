use aoc2024::read_lines_partitioned;
use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/** https://adventofcode.com/2024/day/15 */
fn main() {
    let start = std::time::Instant::now();

    let (mut warehouse, robot_start_pos, moves) = parse_puzzle_input();
    let mut warehouse_2 = widen_warehouse(&warehouse);
    let robot_start_pos_2 = (robot_start_pos.0, robot_start_pos.1 * 2);

    move_robot(&mut warehouse, robot_start_pos, &moves);
    move_robot(&mut warehouse_2, robot_start_pos_2, &moves);

    println!("P1: {}", sum_gps(&warehouse));
    println!("P2: {}", sum_gps(&warehouse_2));
    println!("\nTook {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Obj {
    None,
    Wall,
    Box,
    BoxL,
    BoxR,
}

#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }

    pub fn flipped(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

type Warehouse = Vec<Vec<Obj>>;
type Move = (Dir, i32);
type Pos = (usize, usize);

fn move_robot(warehouse: &mut Warehouse, robot_start_pos: Pos, moves: &[Move]) {
    let mut robot_pos = robot_start_pos;
    for (d, n) in moves {
        for _ in 0..*n {
            if let Some((y, x)) = try_push(warehouse, robot_pos, d) {
                robot_pos = (y, x);
            }
        }
    }
}

fn try_push(warehouse: &mut Warehouse, robot_pos: Pos, dir: &Dir) -> Option<Pos> {
    let new_robot_pos = moved_pos(&robot_pos, dir);
    match warehouse[new_robot_pos.0][new_robot_pos.1] {
        Obj::None => Some(new_robot_pos),
        Obj::Wall => None,
        Obj::Box => try_push_box(warehouse, new_robot_pos, dir),
        Obj::BoxL => try_push_box_lr(warehouse, new_robot_pos, dir),
        Obj::BoxR => try_push_box_lr(warehouse, new_robot_pos, dir),
    }
}

fn try_push_box(warehouse: &mut Warehouse, start: Pos, dir: &Dir) -> Option<Pos> {
    search_first_none(warehouse, start, dir).and_then(|none_pos| {
        swap_objs(warehouse, &none_pos, &start);
        Some(start)
    })
}

fn try_push_box_lr(warehouse: &mut Warehouse, start: Pos, dir: &Dir) -> Option<Pos> {
    if dir == &Dir::Left || dir == &Dir::Right {
        search_first_none(warehouse, start, dir).and_then(|none_pos| {
            let mut swap_pos = none_pos;
            while swap_pos != start {
                let next_swap_pos = moved_pos(&swap_pos, &dir.flipped());
                swap_objs(warehouse, &swap_pos, &next_swap_pos);
                swap_pos = next_swap_pos;
            }
            Some(start)
        })
    } else {
        wide_up_down(warehouse, start, dir).and_then(|to_move| {
            to_move
                .into_iter()
                .sorted_by(|a, b| {
                    if dir == &Dir::Up {
                        a.0.cmp(&b.0)
                    } else {
                        b.0.cmp(&a.0)
                    }
                })
                .for_each(|p| swap_objs(warehouse, &p, &moved_pos(&p, dir)));
            Some(start)
        })
    }
}

fn moved_pos(p: &Pos, d: &Dir) -> Pos {
    (
        (p.0 as i32 + d.delta().0) as usize,
        (p.1 as i32 + d.delta().1) as usize,
    )
}

fn swap_objs(warehouse: &mut Warehouse, p0: &Pos, p1: &Pos) {
    let tmp = warehouse[p0.0][p0.1];
    warehouse[p0.0][p0.1] = warehouse[p1.0][p1.1];
    warehouse[p1.0][p1.1] = tmp;
}

fn search_first_none(warehouse: &Warehouse, start: Pos, dir: &Dir) -> Option<Pos> {
    let mut search_pos = start;
    while warehouse[search_pos.0][search_pos.1] != Obj::None {
        if warehouse[search_pos.0][search_pos.1] == Obj::Wall {
            return None;
        }
        search_pos = moved_pos(&search_pos, dir);
    }
    Some(search_pos)
}

fn wide_up_down(warehouse: &mut Warehouse, start: Pos, dir: &Dir) -> Option<Vec<Pos>> {
    let mut to_move = Vec::<Pos>::new();

    let mut checked = HashSet::<Pos>::new();
    let mut stack = Vec::<Pos>::from([start]);
    while !stack.is_empty() {
        let p = stack.pop().unwrap();
        if !checked.insert(p) {
            continue;
        }

        let move_space = moved_pos(&p, dir);
        if warehouse[move_space.0][move_space.1] == Obj::Wall {
            return None;
        } else if warehouse[move_space.0][move_space.1] != Obj::None {
            stack.push(move_space);
        }
        to_move.push(p);

        match warehouse[p.0][p.1] {
            Obj::BoxL => stack.push((p.0, p.1 + 1)),
            Obj::BoxR => stack.push((p.0, p.1 - 1)),
            _ => {}
        }
    }

    Some(to_move)
}

fn sum_gps(warehouse: &Warehouse) -> usize {
    warehouse
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, o)| match o {
                Obj::None => 0,
                Obj::Wall => 0,
                Obj::Box => 100 * y + x,
                Obj::BoxL => 100 * y + x,
                Obj::BoxR => 0,
            })
        })
        .sum()
}

fn widen_warehouse(warehouse: &Warehouse) -> Warehouse {
    warehouse
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|o| match o {
                    Obj::None => vec![Obj::None, Obj::None],
                    Obj::Wall => vec![Obj::Wall, Obj::Wall],
                    Obj::Box => vec![Obj::BoxL, Obj::BoxR],
                    Obj::BoxL => panic!("Can't widen widened warehouse"),
                    Obj::BoxR => panic!("Can't widen widened warehouse"),
                })
                .collect()
        })
        .collect()
}

fn parse_puzzle_input() -> (Warehouse, Pos, Vec<Move>) {
    let mut warehouse = Warehouse::new();
    let mut moves = Vec::new();
    let mut robot_start_pos = (0, 0);

    let mut push_move = |c: char, n: i32| {
        let m = match c {
            '^' => Some(Dir::Up),
            'v' => Some(Dir::Down),
            '<' => Some(Dir::Left),
            '>' => Some(Dir::Right),
            _ => None,
        };
        if let Some(m) = m {
            moves.push((m, n));
        }
    };

    read_lines_partitioned(
        "input/day_15.txt",
        |line| {
            warehouse.push(
                line.char_indices()
                    .map(|(x, c)| match c {
                        '.' => Obj::None,
                        '#' => Obj::Wall,
                        'O' => Obj::Box,
                        '@' => {
                            robot_start_pos = (warehouse.len(), x);
                            Obj::None
                        }
                        _ => unreachable!(),
                    })
                    .collect(),
            );
        },
        |line| {
            let mut last_c = '\0';
            let mut c_count = 0;
            for c in line.chars() {
                if c != last_c {
                    push_move(last_c, c_count);

                    last_c = c;
                    c_count = 0;
                }
                c_count += 1;
            }
            push_move(last_c, c_count);
        },
    )
    .expect("Failed to open input file");

    (warehouse, robot_start_pos, moves)
}

#[allow(dead_code)]
impl Obj {
    pub fn char(&self) -> char {
        match self {
            Obj::None => '.',
            Obj::Wall => '#',
            Obj::Box => 'O',
            Obj::BoxL => '[',
            Obj::BoxR => ']',
        }
    }

    pub fn color_spec(&self) -> ColorSpec {
        match self {
            Obj::None => ColorSpec::new()
                .set_fg(Some(Color::White))
                .set_dimmed(true)
                .to_owned(),
            Obj::Wall => ColorSpec::new().set_fg(Some(Color::White)).to_owned(),
            Obj::Box => ColorSpec::new().set_fg(Some(Color::Green)).to_owned(),
            Obj::BoxL => ColorSpec::new().set_fg(Some(Color::Magenta)).to_owned(),
            Obj::BoxR => ColorSpec::new().set_fg(Some(Color::Magenta)).to_owned(),
        }
    }
}

#[allow(dead_code)]
fn print_warehouse(warehouse: &Warehouse, robot_pos: Pos) -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for (y, row) in warehouse.iter().enumerate() {
        for (x, o) in row.iter().enumerate() {
            if (y, x) == robot_pos {
                stdout.set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::White))
                        .set_intense(true),
                )?;
                write!(&mut stdout, "@")?;
            } else {
                stdout.set_color(&o.color_spec())?;
                write!(&mut stdout, "{}", o.char())?;
            }
        }
        writeln!(&mut stdout)?;
    }
    stdout.reset()?;
    Ok(())
}
