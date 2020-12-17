use std::collections::HashMap;

use crate::common::read_lines;

struct Bornes3D {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

struct Bornes4D {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
    w_min: isize,
    w_max: isize,
}

fn parse_input() -> Vec<Vec<bool>> {
    read_lines("./inputs/day17")
        .iter()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect()
}

fn cycle_v1(
    p_dim: &HashMap<(isize, isize, isize), bool>,
    bornes: &Bornes3D,
) -> (HashMap<(isize, isize, isize), bool>, Bornes3D) {
    let mut new_dim: HashMap<(isize, isize, isize), bool> = HashMap::new();
    let mut new_b = Bornes3D {
        x_min: (bornes.x_max - bornes.x_min) / 2,
        x_max: (bornes.x_max - bornes.x_min) / 2,
        y_min: (bornes.y_max - bornes.y_min) / 2,
        y_max: (bornes.y_max - bornes.y_min) / 2,
        z_min: (bornes.z_max - bornes.z_min) / 2,
        z_max: (bornes.z_max - bornes.z_min) / 2,
    };
    for x in bornes.x_min - 1..=bornes.x_max + 1 {
        for y in bornes.y_min - 1..=bornes.y_max + 1 {
            for z in bornes.z_min - 1..=bornes.z_max + 1 {
                let mut act_ngh = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        for k in -1..=1 {
                            if (i, j, k) == (0, 0, 0) {
                                continue;
                            }
                            if *p_dim.get(&(x + i, y + j, z + k)).unwrap_or(&false) {
                                act_ngh += 1;
                            }
                        }
                    }
                }
                let state = *p_dim.get(&(x, y, z)).unwrap_or(&false);
                let new_state =
                    (state && vec![2, 3].contains(&act_ngh)) || (!state && act_ngh == 3);
                if new_state {
                    new_b.x_min = new_b.x_min.min(x);
                    new_b.x_max = new_b.x_max.max(x);
                    new_b.y_min = new_b.y_min.min(y);
                    new_b.y_max = new_b.y_max.max(y);
                    new_b.z_min = new_b.z_min.min(z);
                    new_b.z_max = new_b.z_max.max(z);
                }
                new_dim.insert((x, y, z), new_state);
            }
        }
    }
    (new_dim, new_b)
}

fn cycle_v2(
    p_dim: &HashMap<(isize, isize, isize, isize), bool>,
    bornes: &Bornes4D,
) -> (HashMap<(isize, isize, isize, isize), bool>, Bornes4D) {
    let mut new_dim: HashMap<(isize, isize, isize, isize), bool> = HashMap::new();
    let mut new_b = Bornes4D {
        x_min: (bornes.x_max - bornes.x_min) / 2,
        x_max: (bornes.x_max - bornes.x_min) / 2,
        y_min: (bornes.y_max - bornes.y_min) / 2,
        y_max: (bornes.y_max - bornes.y_min) / 2,
        z_min: (bornes.z_max - bornes.z_min) / 2,
        z_max: (bornes.z_max - bornes.z_min) / 2,
        w_min: (bornes.w_max - bornes.w_min) / 2,
        w_max: (bornes.w_max - bornes.w_min) / 2,
    };
    for w in bornes.w_min - 1..=bornes.w_max + 1 {
        for x in bornes.x_min - 1..=bornes.x_max + 1 {
            for y in bornes.y_min - 1..=bornes.y_max + 1 {
                for z in bornes.z_min - 1..=bornes.z_max + 1 {
                    let mut act_ngh = 0;
                    for i in -1..=1 {
                        for j in -1..=1 {
                            for k in -1..=1 {
                                for l in -1..=1 {
                                    if (i, j, k, l) == (0, 0, 0, 0) {
                                        continue;
                                    }
                                    if *p_dim.get(&(x + i, y + j, z + k, w + l)).unwrap_or(&false) {
                                        act_ngh += 1;
                                    }
                                }
                            }
                        }
                    }
                    let state = *p_dim.get(&(x, y, z, w)).unwrap_or(&false);
                    let new_state =
                        (state && vec![2, 3].contains(&act_ngh)) || (!state && act_ngh == 3);
                    if new_state {
                        new_b.x_min = new_b.x_min.min(x);
                        new_b.x_max = new_b.x_max.max(x);
                        new_b.y_min = new_b.y_min.min(y);
                        new_b.y_max = new_b.y_max.max(y);
                        new_b.z_min = new_b.z_min.min(z);
                        new_b.z_max = new_b.z_max.max(z);
                        new_b.w_min = new_b.w_min.min(w);
                        new_b.w_max = new_b.w_max.max(w);
                    }
                    new_dim.insert((x, y, z, w), new_state);
                }
            }
        }
    }
    (new_dim, new_b)
}

#[allow(dead_code)]
fn print_dim(p_dim: &HashMap<(isize, isize, isize), bool>, bornes: &Bornes3D) {
    for z in bornes.z_min..=bornes.z_max {
        println!("z={}", z);
        for x in bornes.x_min..=bornes.x_max {
            for y in bornes.y_min..=bornes.y_max {
                match *p_dim.get(&(x, y, z)).unwrap_or(&false) {
                    true => print!("#"),
                    false => print!("."),
                }
            }
            println!("");
        }
        println!("");
    }
    println!("");
}

pub fn puzzle1() -> usize {
    let init = parse_input();
    let mut pocket_dim: HashMap<(isize, isize, isize), bool> = HashMap::new();
    let mut bornes = Bornes3D {
        x_min: 0,
        x_max: init.len() as isize - 1,
        y_min: 0,
        y_max: init.len() as isize - 1,
        z_min: 0,
        z_max: 0,
    };
    for (x, l) in init.iter().enumerate() {
        for (y, c) in l.iter().enumerate() {
            pocket_dim.insert((x as isize, y as isize, 0), *c);
        }
    }
    // println!("Before any cycles:\n");
    // print_dim(&pocket_dim, &bornes);
    for _i in 1..=6 {
        let (n_pocket_dim, n_bornes) = cycle_v1(&pocket_dim, &bornes);
        pocket_dim = n_pocket_dim;
        bornes = n_bornes;
        // println!("After {} cycle\n", i);
        // print_dim(&pocket_dim, &bornes);
    }
    return pocket_dim.values().filter(|s| **s).count();
}

pub fn puzzle2() -> usize {
    let init = parse_input();
    let mut pocket_dim: HashMap<(isize, isize, isize, isize), bool> = HashMap::new();
    let mut bornes = Bornes4D {
        x_min: 0,
        x_max: init.len() as isize - 1,
        y_min: 0,
        y_max: init.len() as isize - 1,
        z_min: 0,
        z_max: 0,
        w_min: 0,
        w_max: 0,
    };
    for (x, l) in init.iter().enumerate() {
        for (y, c) in l.iter().enumerate() {
            pocket_dim.insert((x as isize, y as isize, 0, 0), *c);
        }
    }
    // println!("Before any cycles:\n");
    // print_dim(&pocket_dim, &bornes);
    for _i in 1..=6 {
        let (n_pocket_dim, n_bornes) = cycle_v2(&pocket_dim, &bornes);
        pocket_dim = n_pocket_dim;
        bornes = n_bornes;
        // println!("After {} cycle\n", i);
        // print_dim(&pocket_dim, &bornes);
    }
    return pocket_dim.values().filter(|s| **s).count();
}
