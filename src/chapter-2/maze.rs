use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main(){
    let f = File::open("file/map.txt").expect("Fail to open file");
    let f = BufReader::new(f);

    let mut vector: Vec<Vec<char>> = vec![];
    for line in f.lines() {
        let line: String = line.expect("");
        let mut vec: Vec<char> = vec![];
        for char in line.as_str().chars() {
            vec.push(char);
        }
        vector.push(vec);
    }
    let mut flag = false;
    println!("{}", DFS(&mut vector, &mut flag));
}

fn DFS(map: &mut Vec<Vec<char>>, flag: &mut bool) -> bool {
    fn recursive(map: &mut Vec<Vec<char>>, i: usize, j: usize, flag: &mut bool) {
        match map[i][j] {
            '.' => {
                map[i][j] = '#';
                recursive(map, i+1, j, flag);
                recursive(map, i, j+1, flag);
                recursive(map, i-1, j, flag);
                recursive(map, i, j-1, flag);
            },
            'G' => {
                *flag = true;
            },
            '#' => {},
            _ => {
                println!("Unknown tile:{}", &map[i][j]);
                map[i][j] = '#';
            },
        }
    }
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                println!("DFS started");
                map[i][j] = '.';
                recursive(map, i, j, flag); 
            }
        }
    }
    return *flag;
}
