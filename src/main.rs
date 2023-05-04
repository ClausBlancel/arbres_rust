#![allow(unused)]
use itertools::Itertools;

#[derive(Debug)]
pub struct TreeNode {
    lvl: u8,
    children: Vec<Edge>,
    index: Option<u32>
}

#[derive(Debug)]
pub struct Edge {
    label: String,
    label_length: u32,
    target_node: TreeNode
}

fn rank_string(string: String) {
    let mut pairs: Vec<(char,char)> = vec![];

    for i in 0..string.len()/2 {
        pairs.push((string.chars().nth(2*i).unwrap(), string.chars().nth(2*i + 1).unwrap()));
    }
    
    let mut sorted_pairs = pairs.clone();
    sorted_pairs.sort();
    let sorted_pairs = sorted_pairs.into_iter().unique().collect::<Vec<(char,char)>>();

    let mut res: String = "".to_string();
    for element in pairs {
        res += sorted_pairs.iter().position(|&x| x == element).unwrap().to_string().as_str();
    }

    println!("{}", res);
}



fn main() {
    let test_string = "121112212221".to_string();

    rank_string(test_string);
}
