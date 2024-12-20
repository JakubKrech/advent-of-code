use crate::utilities::get_input_lines;
use std::collections::HashMap;

// https://adventofcode.com/2024/day/20

const DAY_STRING : &str = "day_20";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let y_size : usize = input.len();
    let x_size : usize = input[0].len();

    let mut start_y : usize = 0;
    let mut start_x : usize = 0;

    let mut end_y : usize = 0;
    let mut end_x : usize = 0;

    let mut grid_raw : Vec<char> = vec!['.'; y_size * x_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(y_size).collect();
    let grid = grid_base.as_mut_slice();

    for yy in 0..input.len() {
        for xx in 0..input[0].len() {
            grid[yy][xx] = input[yy].chars().nth(xx).expect("Failed to fill grid");

            if grid[yy][xx] == 'S' {
                start_y = yy;
                start_x = xx;
            }

            if grid[yy][xx] == 'E' {
                end_y = yy;
                end_x = xx;
            }
        }
    }
    grid[end_y][end_x] = '.';

    let mut cost_grid_raw : Vec<usize> = vec![0; y_size * x_size];
    let mut cost_grid_base : Vec<_> = cost_grid_raw.as_mut_slice().chunks_mut(y_size).collect();
    let cost_grid = cost_grid_base.as_mut_slice();

    let mut path_nodes : Vec<(usize, usize)> = vec![(start_y, start_x)];

    let mut cur_y = start_y;
    let mut cur_x = start_x;
    let mut cur_cost = 1;

    while cur_y != end_y || cur_x != end_x {
        // ^
        if grid[cur_y - 1][cur_x] == '.' && !path_nodes.contains(&(cur_y - 1, cur_x)) {
            cur_y -= 1;
        }
        // >
        else if grid[cur_y][cur_x + 1] == '.' && !path_nodes.contains(&(cur_y, cur_x + 1)) {
            cur_x += 1;
        }
        // v
        else if grid[cur_y + 1][cur_x] == '.' && !path_nodes.contains(&(cur_y + 1, cur_x)) {
            cur_y += 1;
        }
        // <
        else if grid[cur_y][cur_x - 1] == '.' && !path_nodes.contains(&(cur_y, cur_x - 1)) {
            cur_x -= 1;
        }

        path_nodes.push((cur_y, cur_x));
        cost_grid[cur_y][cur_x] = cur_cost;
        cur_cost += 1;
    }

    let mut cheat_count : HashMap<usize, usize> = HashMap::new();

    for node in path_nodes {

        let curr_node_cost : usize = cost_grid[node.0][node.1];

        // ^^
        if node.0 >= 2 && grid[node.0 - 1][node.1] == '#' && grid[node.0 - 2][node.1] == '.' {
            if cost_grid[node.0 - 2][node.1] > curr_node_cost {
                let cost_diff = cost_grid[node.0 - 2][node.1] - curr_node_cost - 2;
                *cheat_count.entry(cost_diff).or_insert(0) += 1;
            }
        }

        // >>
        if node.1 + 2 < x_size && grid[node.0][node.1 + 1] == '#' && grid[node.0][node.1 + 2] == '.' {
            if cost_grid[node.0][node.1 + 2] > curr_node_cost {
                let cost_diff = cost_grid[node.0][node.1 + 2] - curr_node_cost - 2;
                *cheat_count.entry(cost_diff).or_insert(0) += 1;
            }
        }

        // vv
        if node.0 + 2 < y_size && grid[node.0 + 1][node.1] == '#' && grid[node.0 + 2][node.1] == '.' {
            if cost_grid[node.0 + 2][node.1] > curr_node_cost {
                let cost_diff = cost_grid[node.0 + 2][node.1] - curr_node_cost - 2;
                *cheat_count.entry(cost_diff).or_insert(0) += 1;
            }
        }

        // <<
        if node.1 >= 2 && grid[node.0][node.1 - 1] == '#' && grid[node.0][node.1 - 2] == '.' {
            if cost_grid[node.0][node.1 - 2] > curr_node_cost {
                let cost_diff = cost_grid[node.0][node.1 - 2] - curr_node_cost - 2;
                *cheat_count.entry(cost_diff).or_insert(0) += 1;
            }
        }
    }

    let mut cheats_worth_at_least_100 : usize = 0;

    for x in cheat_count {
        if x.0 >= 100 {
            cheats_worth_at_least_100 += x.1;
        }
    }

    return cheats_worth_at_least_100.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let y_size : usize = input.len();
    let x_size : usize = input[0].len();

    let mut start_y : usize = 0;
    let mut start_x : usize = 0;

    let mut end_y : usize = 0;
    let mut end_x : usize = 0;

    let mut grid_raw : Vec<char> = vec!['.'; y_size * x_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(y_size).collect();
    let grid = grid_base.as_mut_slice();

    for yy in 0..input.len() {
        for xx in 0..input[0].len() {
            grid[yy][xx] = input[yy].chars().nth(xx).expect("Failed to fill grid");

            if grid[yy][xx] == 'S' {
                start_y = yy;
                start_x = xx;
            }

            if grid[yy][xx] == 'E' {
                end_y = yy;
                end_x = xx;
            }
        }
    }
    grid[end_y][end_x] = '.';

    let mut cost_grid_raw : Vec<usize> = vec![0; y_size * x_size];
    let mut cost_grid_base : Vec<_> = cost_grid_raw.as_mut_slice().chunks_mut(y_size).collect();
    let cost_grid = cost_grid_base.as_mut_slice();

    let mut path_nodes : Vec<(usize, usize)> = vec![(start_y, start_x)];

    let mut cur_y = start_y;
    let mut cur_x = start_x;
    let mut cur_cost = 1;

    while cur_y != end_y || cur_x != end_x {

        // ^
        if grid[cur_y - 1][cur_x] == '.' && !path_nodes.contains(&(cur_y - 1, cur_x)) {
            cur_y -= 1;
        }
        // >
        else if grid[cur_y][cur_x + 1] == '.' && !path_nodes.contains(&(cur_y, cur_x + 1)) {
            cur_x += 1;
        }
        // v
        else if grid[cur_y + 1][cur_x] == '.' && !path_nodes.contains(&(cur_y + 1, cur_x)) {
            cur_y += 1;
        }
        // <
        else if grid[cur_y][cur_x - 1] == '.' && !path_nodes.contains(&(cur_y, cur_x - 1)) {
            cur_x -= 1;
        }

        path_nodes.push((cur_y, cur_x));
        cost_grid[cur_y][cur_x] = cur_cost;
        cur_cost += 1;
    }

    let mut cheat_count : HashMap<usize, usize> = HashMap::new();

    for i in 0..path_nodes .len() {
        for j in (i + 1)..path_nodes .len() {
            let node1 = path_nodes[i];
            let node2 = path_nodes[j];

            let distance : usize = node2.0.abs_diff(node1.0) + node2.1.abs_diff(node1.1);

            if distance > 20 {
                continue;
            }

            let cost_diff = cost_grid[node2.0][node2.1] - cost_grid[node1.0][node1.1] - distance;
            *cheat_count.entry(cost_diff).or_insert(0) += 1;
        }
    }

    let mut cheats_worth_at_least_100 : usize = 0;

    for x in cheat_count {
        if x.0 >= 100 {
            cheats_worth_at_least_100 += x.1;
        }
    }

    return cheats_worth_at_least_100.to_string();
}
