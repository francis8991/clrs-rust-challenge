impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 1 || n == 0 {
            return n;
        }

        let mut d1 = 0;
        let mut d2 = 1;
        let mut plus = 0;

        for n in 2..=n {
            plus = d1 + d2;
            d1 = d2;
            d2 = plus;
        }
        plus
    }
}
