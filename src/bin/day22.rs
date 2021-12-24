use std::{cmp, collections::HashSet, fs};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Cube {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

impl Cube {
    fn new(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> Self {
        Cube {
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
        }
    }
}

struct Command(bool, Cube);

fn parse_coord_range(l: &str, coord: char) -> (i64, i64) {
    let (x1, x2) = l
        .split(&format!("{}=", coord))
        .nth(1)
        .unwrap()
        .split(",")
        .next()
        .unwrap()
        .split_once("..")
        .unwrap();
    let x1 = x1.parse().unwrap();
    let x2 = x2.parse().unwrap();
    if x1 > x2 {
        (x2, x1)
    } else {
        (x1, x2)
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Voxel(i64, i64, i64);

fn a(commands: &Vec<Command>) -> usize {
    let mut on_voxels = HashSet::new();
    for Command(is_on, cube) in commands.iter() {
        for x in cmp::max(-50, cube.x1)..=cmp::min(50, cube.x2) {
            for y in cmp::max(-50, cube.y1)..=cmp::min(50, cube.y2) {
                for z in cmp::max(-50, cube.z1)..=cmp::min(50, cube.z2) {
                    if *is_on {
                        on_voxels.insert(Voxel(x, y, z));
                    } else {
                        on_voxels.remove(&Voxel(x, y, z));
                    }
                }
            }
        }
    }
    on_voxels.len()
}

fn b(commands: &Vec<Command>) -> u64 {
    let mut xs = HashSet::new();
    let mut ys = HashSet::new();
    let mut zs = HashSet::new();
    let commands: Vec<Command> = commands
        .iter()
        .map(|Command(b, c)| {
            Command(
                *b,
                Cube::new(c.x1, c.x2 + 1, c.y1, c.y2 + 1, c.z1, c.z2 + 1),
            )
        })
        .collect();
    for Command(_, cube) in commands.iter() {
        xs.insert(cube.x1);
        xs.insert(cube.x2);
        ys.insert(cube.y1);
        ys.insert(cube.y2);
        zs.insert(cube.z1);
        zs.insert(cube.z2);
    }
    let mut grid = vec![vec![vec![0 as u8; zs.len()]; ys.len()]; xs.len()];
    let mut sizes = vec![vec![vec![0 as u64; zs.len()]; ys.len()]; xs.len()];
    let mut xs: Vec<i64> = xs.into_iter().collect();
    let mut ys: Vec<i64> = ys.into_iter().collect();
    let mut zs: Vec<i64> = zs.into_iter().collect();
    xs.sort();
    ys.sort();
    zs.sort();
    for (i, vx) in xs.iter().enumerate() {
        let x_size = if i < xs.len() - 1 { xs[i + 1] - *vx } else { 0 };
        for (j, vy) in ys.iter().enumerate() {
            let y_size = if j < ys.len() - 1 { ys[j + 1] - *vy } else { 0 };
            for (k, vz) in zs.iter().enumerate() {
                let z_size = if k < zs.len() - 1 { zs[k + 1] - *vz } else { 0 };
                sizes[i][j][k] = x_size as u64 * y_size as u64 * z_size as u64;
            }
        }
    }
    for Command(is_on, cube) in commands.iter() {
        for x in xs.binary_search(&cube.x1).unwrap()..xs.binary_search(&cube.x2).unwrap() {
            for y in ys.binary_search(&cube.y1).unwrap()..ys.binary_search(&cube.y2).unwrap() {
                for z in zs.binary_search(&cube.z1).unwrap()..zs.binary_search(&cube.z2).unwrap() {
                    grid[x][y][z] = if *is_on { 1 } else { 0 };
                }
            }
        }
    }
    let mut result: u64 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for k in 0..grid[i][j].len() {
                result += grid[i][j][k] as u64 * sizes[i][j][k];
            }
        }
    }
    result
}

fn main() {
    let contents = fs::read_to_string("inputs/day22.txt").unwrap();
    let commands = contents
        .split("\n")
        .map(|l| {
            let is_on = l.starts_with("on");
            let (x1, x2) = parse_coord_range(l, 'x');
            let (y1, y2) = parse_coord_range(l, 'y');
            let (z1, z2) = parse_coord_range(l, 'z');
            let cube = Cube::new(x1, x2, y1, y2, z1, z2);
            Command(is_on, cube)
        })
        .collect();
    println!("Part 1: {}", a(&commands));
    println!("Part 2: {}", b(&commands));
}
