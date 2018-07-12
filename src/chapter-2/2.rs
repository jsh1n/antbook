// stack overflowする
fn flip_to1(col: usize, map: &mut [u8; 30], index: usize){
    if map[index] == 1 {
        map[index] = 0;
        // 再帰で隣接マスについてこれを呼び出す
        flip_to1(col, map, index + 1);
        flip_to1(col, map, index - 1);
        flip_to1(col, map, index - col);
        flip_to1(col, map, index + col);
    }
}

fn main() {
    //argument
    let row: usize = 6;
    let col: usize = 5;
    let mut map: [u8; 30] = [
        1,1,1,0,0,0,
        1,0,0,0,1,1,
        0,0,1,0,0,1,
        1,1,0,0,0,0,
        1,1,0,1,1,1];
    for i in 0..map.len() {
        flip_to1(col, &mut map, i);
    }
}
