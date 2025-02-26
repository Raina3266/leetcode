use std::collections::HashSet;

pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    if grid[0][0] == 1 || grid[num_rows - 1][num_cols - 1] == 1 {
        return -1;
    }

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut visited = HashSet::new();
    visited.insert((0, 0));
}

fn dfs(grid: &mut Vec<Vec<i32>>, x: isize, y: isize, path_len: i32, shortest: &mut i32) {}
