use std::collections::{HashSet, VecDeque};

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let height = grid.len();
        let width = grid[0].len();

        let mut ans = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..height {
            for j in 0..width {
                if grid[i][j] != 1 || visited.contains(&(i, j)) {
                    continue;
                }

                let mut area = 0;
                let mut iland = VecDeque::new();
                iland.push_back((i, j));

                while let Some((y, x)) = iland.pop_front() {
                    if visited.contains(&(y, x)) {
                        continue;
                    }

                    area += 1;
                    visited.insert((y, x));

                    let directions = [(0, 1), (1, 0), (!0, 0), (0, !0)];
                    for &(dy, dx) in directions.iter() {
                        let x = x.wrapping_add(dx);
                        let y = y.wrapping_add(dy);

                        if x < width && y < height && grid[y][x] == 1 {
                            iland.push_back((y, x));
                        }
                    }
                }

                ans = ans.max(area);
            }
        }
        ans
    }
}
