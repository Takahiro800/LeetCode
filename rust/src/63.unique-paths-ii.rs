use crate::Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let width = obstacle_grid[0].len();

        let mut path_count: Vec<i32> = vec![0; width];
        if obstacle_grid[0][0] == 0 {
            path_count[0] = 1;
        }

        for (i, row) in obstacle_grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    path_count[j] = 0;
                } else if j > 0 {
                    path_count[j] += path_count[j - 1];
                }
            }
        }

        path_count[width - 1]
    }
}
