pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut m = 0i32;
        let rmaxs: Vec<i32> = prices
            .iter()
            .rev()
            .map(|p| {
                let old = m;
                if *p > m {
                    m = *p;
                }
                old
            })
            .collect();

        let mut m = *prices.last().unwrap();
        let rmins: Vec<i32> = prices
            .iter()
            .rev()
            .map(|p| {
                if *p <= m {
                    m = *p;
                }
                m
            })
            .collect();

        let mut mp = 0;
        for i in 0..prices.len() - 1 {
            let p = rmaxs[i] - rmins[i];
            if p > mp {
                mp = p;
            }
        }
        mp
    }
}
