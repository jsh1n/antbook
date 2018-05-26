fn main() {
    //argument
    let a: &[isize] = &[1, 2, 4, 7];
    let k: isize = 13;
    println!("{}", solve(a, k));
}

fn solve(a: &[isize], k: isize) -> bool {
    if depth_first_search(a, k, 0, 0) {
        return true;
    }else {
        return false;
    }
}

fn depth_first_search(a: &[isize], k: isize, sum: isize, i: usize) -> bool {
    if i == a.len() {
        return sum == k;
    }
    if depth_first_search(a, k, sum, i + 1) {
        return true;
    }
    if depth_first_search(a, k, sum + a[i], i + 1) {
        return true;
    }
    return false;
}

#[test]
fn check_answer() {
    assert_eq!(solve(&[1, 2, 4, 7], 13), true);
    assert_eq!(solve(&[1, 2, 4, 7], 15), false);
}