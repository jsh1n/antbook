fn main() {
    //argument
    static row: usize    = 6;
    static column: usize = 5;
    static map: &[usize] = &[WWW...
                          W...WW
                          ..W..W
                          WW....
                          WW.WWW]
}

fn dfs(x: usize, y: usize){
    //隣接するWを.に置き換える
    if map[y*column+x] == . {
        panic!("dfs starts in .");
    }
}