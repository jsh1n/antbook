fn main(){
    // argument setting
    let a: Vec<u32> = vec![1, 4, 5, 6, 14, 15, 32, 41, 1, 123241, 414, 1111111, 9999999];
    let n: usize = 13;
    println!("max length: {}", solve(a, n));
}

use std::cmp;

fn solve(a: Vec<u32>, n: usize) -> u32 {
    if n != a.len() {
        panic!("n is not equal to the number of element of a!");
    }
    let mut iter: u32 = 0;
    let mut ans: u32 = 0;
    for i in 0..n-2 {
        for j in i+1..n-1 {
            for k in j+1..n {
                let mut len = a[i] + a[j] + a[k];
                let mut max_a = cmp::max(cmp::max(a[i], a[j]), a[k]);
                println!("{} {} {}", i,j,k);
                println!("len:{}, max_a:{}, {} {} {}", len, max_a, a[i], a[j], a[k]);
                if max_a < len - max_a {
                    println!("can be triangle");
                    ans = cmp::max(ans, len) as u32;
                }
                iter = iter + 1;
                println!("----------------------------------------------------");
            }
        }
    }
    println!("number of iteration: {}", iter);
    return ans;
}

#[test]
fn answer_check () {
    assert_eq!(solve(vec![2, 3, 4, 5, 10], 5), 12);
    assert_eq!(solve(vec![4, 5, 10, 20], 4), 0);
}