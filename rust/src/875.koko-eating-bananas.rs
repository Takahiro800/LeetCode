impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        use std::cmp::Ordering;
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        let mut current_k = *piles.iter().max().unwrap();

        while left < right {
            let speed = (left + right) / 2;

            let sum_time = piles.iter().map(|&x| (x + speed - 1) / speed).sum::<i32>();

            match sum_time.cmp(&h) {
                Ordering::Equal => {
                    right = speed;
                    current_k = std::cmp::min(speed, current_k);
                }
                Ordering::Less => {
                    right = speed;
                    current_k = std::cmp::min(speed, current_k);
                }
                Ordering::Greater => left = &speed + 1,
            }
        }
        current_k
    }
}
