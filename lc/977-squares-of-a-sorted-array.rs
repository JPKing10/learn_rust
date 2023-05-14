pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(nums.len());
    (0..nums.len()).for_each(|_| res.push(0));
    let mut l = 0;
    let mut h = nums.len() - 1;
    let mut p = nums.len() - 1;
    let mut i = 0;
    while l < h {
        if nums[l].abs() < nums[h].abs() {
            i = h;
            h -= 1;
        } else {
            i = l;
            l += 1;
        }
        res[p] = nums[i] * nums[i];
        p -= 1;
    }
    return res;
}

fn main() {
    println!("{:?}", sorted_squares(vec![-4,-1,0,3,10]))
}
