impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut track = vec![];
        let mut used = vec![false; len];
        let mut res = vec![];

        fn backtrack(
            nums: &Vec<i32>,
            len: usize,
            used: &mut Vec<bool>,
            track: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if len == track.len() {
                res.push(track.to_vec());
                return;
            }
            for i in 0..len {
                if let Some(true) = used.get(i) {
                    continue;
                }

                track.push(nums[i]);
                used[i] = true;
                backtrack(nums, len, used, track, res);
                track.pop();
                used[i] = false;
            }
        }
        backtrack(&nums, len, &mut used, &mut track, &mut res);

        res
    }
}
