fn main(){
    // argument setting
    let a: Vec<u32> = vec![1, 4, 5, 6, 14, 15];
    println!("max length:{:?}", solve(a));
}

use std::cmp;

fn solve(a: Vec<u32>) -> u32 {
    let mut ans: u32 = 0;
    let n = a.len() as usize;
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                let mut len = a[i] + a[j] + a[k];
                let mut max_a = cmp::max(cmp::max(a[i], a[j]), a[k]);
                println!("{} {} {}", i,j,k);
                println!("len:{}, max_a:{}, {} {} {}", len, max_a, a[i], a[j], a[k]);
                if max_a < len - max_a {
                    println!("can trianglize");
                    ans = cmp::max(ans, len) as u32;
                }
                println!("----------------------------------------------------");
            }
        }
    }
    return ans;
}

#[test]
fn ans_check () {
    assert_eq!(solve(vec![2, 3, 4, 5, 10]), 12);
    assert_eq!(solve(vec![4, 5, 10, 20]), 0);
}