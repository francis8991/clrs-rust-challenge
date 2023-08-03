impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];

        dp[0] = 0;

        for coin in coins {
            for i in coin as usize..dp.len() {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }
        if dp[amount as usize] == amount + 1 {
            -1
        } else {
            dp[amount as usize]
        }
    }
}
