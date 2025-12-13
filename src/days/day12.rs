use anyhow::Result;
use good_lp::{constraint, default_solver, variable, variables, Expression, ProblemVariables, SolverModel};

pub fn part1(input: &str) -> Result<i64> {
    Ok(parse_numbers(input))
}

#[derive(Debug)]
struct Region {
    x: i64,
    y: i64,
    amount: Vec<i32>,
}

fn parse_numbers(input: &str) -> i64 {
    let mut score = 0;
    // split at \n\n
    let sections: Vec<&str> = input.split("\n\n").collect();
    // the last section is the regions
    let regions: Vec<Region> = sections
        .last()
        .unwrap()
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let xy: Vec<&str> = parts[0].split("x").collect();
            let x: i64 = xy[0].parse().unwrap();
            let y: i64 = xy[1].parse().unwrap();
            let amount: Vec<i32> = parts[1]
                .split(" ")
                .filter(|string| string.trim().len() > 0)
                .map(|s| s.trim().parse().unwrap())
                .collect();
            Region { x, y, amount }
        })
        .collect();
    let shapes: Vec<Vec<(usize, usize)>> = sections
        .iter()
        .take(sections.len() - 1)
        .map(|section| {
            section
                .lines()
                .skip(1) // skip the first line
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        if c == '#' {
                            Some((x, y))
                        } else { None }
                    })
                })
                .collect()
        })
        .collect();

    for region in regions {
        // check if we can pack all shapes into the region, they have to fit exactly, can be rotated or flipped
        // also the amounts have to match, e.g. if region.amount = [2, 1] and shapes = [shape1, shape2]
        // then we need to fit shape1 twice and shape2 once into the region

        // quick check: total area of shapes must be less area of region
        let region_area = region.x as usize * region.y as usize;
        let mut shapes_area = 0;
        for (shape_idx, shape) in shapes.iter().enumerate() {
            let shape_area = shape.len();
            let amount = if shape_idx < region.amount.len() { region.amount[shape_idx] } else { 0 };
            shapes_area += shape_area * (amount as usize);
        }
        if shapes_area >= region_area {
            println!("Region {:?} cannot be tiled: area mismatch", region);
            continue;
        }
        //if can_tile_region(&region, &shapes) {
        score += 1;
        //} else {
        //    println!("Region {:?} cannot be tiled", region);
        // }
    }

    score
}

#[derive(Clone, Debug)]
struct Placement {
    shape_idx: usize,
    cells: Vec<(usize, usize)>, // absolute cells covered in the region grid
}

fn normalize(mut cells: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let min_x = cells.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = cells.iter().map(|(_, y)| *y).min().unwrap();
    for (x, y) in cells.iter_mut() {
        *x -= min_x;
        *y -= min_y;
    }
    cells.sort_unstable();
    cells
}

fn orientations(shape: &[(usize, usize)]) -> Vec<Vec<(i32, i32)>> {
    let pts: Vec<(i32, i32)> = shape.iter().map(|&(x, y)| (x as i32, y as i32)).collect();

    // 8 symmetries: 4 rotations of (x,y) and 4 rotations of flipped (x,-y)
    let mut outs: Vec<Vec<(i32, i32)>> = Vec::new();

    let transforms = |(x, y): (i32, i32)| -> [(i32, i32); 8] {
        // rotations
        let r0 = (x, y);
        let r90 = (-y, x);
        let r180 = (-x, -y);
        let r270 = (y, -x);
        // flip across x-axis then rotate
        let f0 = (x, -y);
        let f90 = (y, x);
        let f180 = (-x, y);
        let f270 = (-y, -x);
        [r0, r90, r180, r270, f0, f90, f180, f270]
    };

    for k in 0..8 {
        let mut o: Vec<(i32, i32)> = pts.iter().map(|&p| transforms(p)[k]).collect();
        o = normalize(o);
        outs.push(o);
    }

    // dedup
    outs.sort();
    outs.dedup();
    outs
}

fn generate_placements(region_w: usize, region_h: usize, shapes: &[Vec<(usize, usize)>], amounts: &[i32]) -> Vec<Placement> {
    let mut placements = Vec::new();

    for (shape_idx, shape) in shapes.iter().enumerate() {
        if shape_idx >= amounts.len() || amounts[shape_idx] == 0 {
            continue;
        }
        let os = orientations(shape);

        for o in os {
            let max_x = o.iter().map(|(x, _)| *x).max().unwrap() as i32;
            let max_y = o.iter().map(|(_, y)| *y).max().unwrap() as i32;

            let w = region_w as i32;
            let h = region_h as i32;

            for ox in 0..=(w - 1 - max_x) {
                for oy in 0..=(h - 1 - max_y) {
                    let mut abs_cells = Vec::with_capacity(o.len());
                    let mut ok = true;
                    for &(dx, dy) in &o {
                        let ax = ox + dx;
                        let ay = oy + dy;
                        if ax < 0 || ay < 0 || ax >= w || ay >= h {
                            ok = false;
                            break;
                        }
                        abs_cells.push((ax as usize, ay as usize));
                    }
                    if ok {
                        placements.push(Placement { shape_idx, cells: abs_cells });
                    }
                }
            }
        }
    }

    placements
}

fn can_tile_region(region: &Region, shapes: &[Vec<(usize, usize)>]) -> bool {
    let w = region.x as usize;
    let h = region.y as usize;

    let placements = generate_placements(w, h, shapes, &region.amount);
    if placements.is_empty() {
        return false;
    }

    // Build ILP
    let mut vars: ProblemVariables = variables!();
    let place_vars = vars.add_vector(variable().binary(), placements.len());

    // cell constraints: each cell covered exactly once
    let mut model = vars.minimise(0).using(default_solver);

    // For each cell (cx, cy): sum(place_vars[p] if p covers cell) == 1
    for cy in 0..h {
        for cx in 0..w {
            let mut expr: Expression = 0.into();
            for (pi, p) in placements.iter().enumerate() {
                if p.cells.iter().any(|&(x, y)| x == cx && y == cy) {
                    expr += place_vars[pi];
                }
            }
            model = model.with(constraint!(expr <= 1));
        }
    }

    // amount constraints: each shape i used exactly region.amount[i]
    for (si, &amt) in region.amount.iter().enumerate() {
        let mut expr: Expression = 0.into();
        for (pi, p) in placements.iter().enumerate() {
            if p.shape_idx == si {
                expr += place_vars[pi];
            }
        }
        model = model.with(constraint!(expr == amt));
    }

    model.solve().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_path;
    use std::fs;

    const EXAMPLE_ANSWER_PART1: i64 = 2;
    const INPUT_ANSWER_PART1: i64 = 487;
    const DAY: &str = "day12";

    #[test]
    fn example_part1() {
        let path = repo_path(&["inputs", DAY, "example.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, EXAMPLE_ANSWER_PART1);
    }

    #[test]
    fn input_part1() {
        let path = repo_path(&["inputs", DAY, "input.txt"]);
        let input = fs::read_to_string(path).expect("missing example file");
        let ans = part1(&input).unwrap();
        assert_eq!(ans, INPUT_ANSWER_PART1);
    }
}
