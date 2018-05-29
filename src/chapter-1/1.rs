fn main() {
    // argument
    let n: usize = 3;
    let m: i32 = 9;
    let k: Vec<i32> = vec![1, 3, 5];

    // execution
    println!("{}", solve(n, m, k));
}
#[warn(dead_code)]

fn binary_search(slice: &mut [i32], search_value: i32) -> bool {
    let mut l: usize = 0;
    let mut r: usize = slice.len();

    slice.sort();
    println!("{:?}, {}", slice, search_value);
    while r - l >= 1 {
        let i: usize = (l + r) / 2;
        if slice[i] == search_value {
            return true;
        } else if slice[i] < search_value {
            l = i + 1;
        } else {
            r = i;
        }
        println!("r: {}, l: {}", r, l);
    }
    return false;
}

fn solve(n: usize, m: i32, k: Vec<i32>) -> bool {
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..k.len() {
        for j in 0..k.len() {
            vec.push(k[i] + k[j]);
        }
    }
    let slice = &mut vec[..];
    for a in 0..n {
        for b in 0..n {
            println!("{}, {}", k[a], k[b]);
            if binary_search(slice, m - k[a] - k[b]) {
                return true;
            }
        }
    }
    return false;
}

#[test]
fn check_answer() {
    assert_eq!(solve(3, 10, vec![1, 3, 5]), true);
    assert_eq!(solve(3, 9, vec![1, 3, 5]), false);
}
