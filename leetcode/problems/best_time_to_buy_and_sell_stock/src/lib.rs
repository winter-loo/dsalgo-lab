pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0), |(mut min_price, mut mp), &p| {
                mp = mp.max(p - min_price);
                min_price = p.min(min_price);
                (min_price, mp)
            })
            .1
    }

    pub fn max_profit_procedure(prices: Vec<i32>) -> i32 {
        let (mut min_price, mut mp) = (i32::MAX, 0);
        for p in prices {
            mp = mp.max(p - min_price);
            min_price = min_price.min(p);
        }
        mp
    }

    pub fn max_profit_precomputation(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut m = 0i32;
        let mut rmaxs = vec![0; prices.len()];
        for i in (0..prices.len()).rev() {
            rmaxs[i] = m;
            m = m.max(prices[i]);
        }
        // println!("rmaxs={rmaxs:?}");

        let mut mp = 0;
        for i in 0..prices.len() - 1 {
            let p = rmaxs[i] - prices[i];
            if p > mp {
                mp = p;
            }
        }
        mp
    }
}
