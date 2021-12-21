use std::fs;

fn solve(image: &Vec<Vec<bool>>, lut: &Vec<bool>, iters: usize) -> usize {
    let mut image = image.clone();
    for iteration in 0..iters {
        let mut new_image = vec![vec![false; image[0].len() + 2]; image.len() + 2];
        for y in -1..=image.len() as i32 {
            for x in -1..=image[0].len() as i32 {
                let mut result: usize = 0;
                let mut i: usize = 0;
                for wy in (y - 1..y + 2).rev() {
                    for wx in (x - 1..x + 2).rev() {
                        let in_bounds = wy >= 0
                            && wy < image.len() as i32
                            && wx >= 0
                            && wx < image[0].len() as i32;
                        if in_bounds {
                            result |= (image[wy as usize][wx as usize] as usize) << i;
                        } else if iteration % 2 == 1 {
                            result |= 1 << i;
                        }
                        i += 1;
                    }
                }
                new_image[(y + 1) as usize][(x + 1) as usize] = lut[result];
            }
        }
        image = new_image;
    }
    image.iter().map(|c| c.iter().filter(|c| **c).count()).sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day20.txt").unwrap();
    let (lut, image_str) = contents.split_once("\n\n").unwrap();
    let lut = lut.chars().map(|c| c == '#').collect();
    let sparse_image = image_str
        .split("\n")
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect();
    println!("Part 1: {}", solve(&sparse_image, &lut, 2));
    println!("Part 2: {}", solve(&sparse_image, &lut, 50));
}
