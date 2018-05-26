fn main() {
    // argument
    let n: usize = 3;
    let m: u32 = 10;
    let k: Vec<u32> = vec![1, 3, 5];

    // execution
    println!("{}", solve(n, m, k));
}
#[warn(dead_code)]

fn binary_search(mut v: Vec<u32>, search_value: u32) -> bool {
    let mut l: usize = 0;
    let mut r: usize = v.len();

    v.sort();
    while r - l >= 1 {
        let i: usize = (l + r) / 2;
        if v[i] == search_value {
            return true;
        } else if v[i] < search_value {
            l = i + 1;
        } else {
            r = i;
        }
    }
    return false;
}

fn solve(n: usize, m: u32, k: Vec<u32>) -> bool {
    let mut vec: Vec<u32> = Vec::new();
    for i in 0..k.len() {
        for j in 0..k.len() {
            vec.push(k[i] * k[j]);
        }
    }
    for a in 0..n {
        for b in 0..n {
            if binary_search(vec, m - k[a] - k[b]) {
                return true;
            }
        }
    }
    return false;
}
