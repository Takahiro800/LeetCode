impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering;
        let m = matrix[0].len();
        let n = matrix.len();

        let mut left = 0;
        let mut right = m * n;

        while left < right {
            let mid = (left + right) / 2;
            let num = matrix[mid / m][mid % m];

            match num.cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
            }
        }
        false
    }
}
