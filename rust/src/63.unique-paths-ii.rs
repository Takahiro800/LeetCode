use crate::Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let height = obstacle_grid.len();
        let width = obstacle_grid[0].len();

        let mut path_count: Vec<i32> = vec![0; width];
        if obstacle_grid[0][0] == 0 {
            path_count[0] = 1;
        }

        for i in 0..height {
            for j in 0..width {
                if obstacle_grid[i][j] == 1 {
                    path_count[j] = 0;
                } else if j > 0 {
                    path_count[j] += path_count[j - 1];
                }
            }
        }

        path_count[width - 1]
    }
}
