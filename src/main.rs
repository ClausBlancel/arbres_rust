#![allow(unused)]
use itertools::Itertools;

const ALPHABET_SIZE: usize = 256;

#[derive(Debug, Clone)]
pub struct TreeNode {
    lvl: u32,
    edge_table: Vec<(char, Edge)>,
    children: Vec<Edge>,
    index: Option<u32>
}

impl TreeNode {
    fn new(lvl: u32, index: Option<u32>) -> TreeNode {
        return TreeNode { 
            lvl,
            edge_table: vec![],
            children: vec![],
            index
        }
    }

    fn add_child(&mut self, label: String, label_length: u32, target_node: TreeNode) {
        self.children.push(Edge::new(label, label_length, target_node));
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    label: String,
    label_length: u32,
    target_node: TreeNode
}

impl Edge {
    fn new(label: String, label_length: u32, target_node: TreeNode) -> Edge {
        Edge {
            label,
            label_length,
            target_node
        }
    }
}

pub fn rank_string(string: String) -> (Vec<(char, char)>, String) {
    let mut pairs: Vec<(char,char)> = vec![];

    for i in 0..string.len()/2 {
        pairs.push((string.chars().nth(2*i).unwrap(), string.chars().nth(2*i + 1).unwrap()));
    }
    
    let mut sorted_pairs = pairs.clone();
    sorted_pairs.sort();
    let sorted_pairs = sorted_pairs.into_iter().unique().collect::<Vec<(char,char)>>();

    let mut ranked_string: String = "".to_string();
    for element in pairs {
        ranked_string += sorted_pairs.iter().position(|&x| x == element).unwrap().to_string().as_str();
    }

    return (sorted_pairs, ranked_string + "$")
}

pub fn insert_suffix(root: TreeNode, suffix: String, suffix_index: u32, lvl: u32) -> TreeNode {
    let mut current_node = root;
    let mut current_char = suffix.chars().nth(0).unwrap();
    let mut found = false;
    let mut i = 0;

    if current_char != '$' {
        for element in current_node.edge_table.iter() {
            if element.0 == current_char {
                current_node.edge_table.push((current_char, Edge::new(current_char.to_string(), 1, insert_suffix(element.1.target_node.clone(), suffix[1..].to_string(), suffix_index, lvl + 1))));
                current_node.edge_table.remove(i);
                found = true;
                break;
            }
            i += 1;
        }

        if !found {
            current_node.edge_table.push((current_char, Edge::new(current_char.to_string(), 1, insert_suffix(TreeNode::new(lvl + 1, None), suffix[1..].to_string(), suffix_index, lvl + 1))));
        }
    }

    return current_node;

    // while current_char != '$' {
        
    //     for element in current_node.edge_table.iter() {
    //         if element.0 == current_char {
    //             let current_node = element.1.target_node.clone();
    //             i += 1;
    //             found = true;
    //             break;
    //         }
    //     }

    //     if !found {
    //         current_node.edge_table.push((current_char, Edge::new(current_char.to_string(), 1, TreeNode::new(lvl, None))));
    //     }

    //     current_char = suffix.chars().nth(i).unwrap();
    //     lvl += 1;
    //     found = false;
    // }

    // current_node.edge_table.push((current_char, Edge::new(current_char.to_string(), 1, TreeNode::new(lvl, Some(suffix_index)))));
}

pub fn main() {
    let test_string = "121112212221$".to_string();
    let mut root = TreeNode::new(0, None);

    let (sorted_pairs, ranked_string) = rank_string(test_string);

    println!("{:?} | {:?}", sorted_pairs, ranked_string);

    for i in 0..ranked_string.len() {
        root = insert_suffix(root.clone(), ranked_string[i..].to_string(), i as u32, 0);
    }

    print!("{:?}", root);
}
