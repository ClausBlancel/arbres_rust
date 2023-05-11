use std::fs::File;
use std::io::Write;

use itertools::Itertools;

// Tree Node structure
#[derive(Debug, Clone)]
pub struct TreeNode {
    lvl: u32,
    edge_table: Vec<Edge>,
    index: Option<u32>
}

impl TreeNode {
    fn new(lvl: u32, index: Option<u32>) -> TreeNode {
        return TreeNode { 
            lvl,
            edge_table: vec![],
            index
        }
    }
}

// Edge structure that is labeled with the part of the suffix it links to
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

// Takes a string as an argument and map its pairs into a vector
// Returns a tuple containing the vector of pairs and the ranked string, which is the string of the indexes of the pairs in the vector
pub fn rank_string(string: String) -> (Vec<(char, char)>, String) {
    let mut pairs: Vec<(char,char)> = vec![];

    // Map the string into pairs
    for i in 0..string.len()/2 {
        pairs.push((string.chars().nth(2*i).unwrap(), string.chars().nth(2*i + 1).unwrap()));
    }
    
    // Sort the pairs and remove duplicates
    let mut sorted_pairs = pairs.clone();
    sorted_pairs.sort();
    let sorted_pairs = sorted_pairs.into_iter().unique().collect::<Vec<(char,char)>>();

    // Create the ranked string
    let mut ranked_string: String = "".to_string();
    for element in pairs {
        ranked_string += sorted_pairs.iter().position(|&x| x == element).unwrap().to_string().as_str();
    }

    return (sorted_pairs, ranked_string + "$")
}

// Takes a tree node and a suffix as arguments and insert the suffix in the tree
pub fn insert_suffix(root: TreeNode, suffix: String, suffix_index: u32, lvl: u32) -> TreeNode {
    let mut current_node = root;
    let current_char = suffix.chars().nth(0).unwrap();
    let mut found = false;
    let mut i = 0;

    // Checks if we are at the end of the suffix
    if current_char != '$' {

        // If not we explore every edge of the current node to see if there is one that starts with the current char
        for element in current_node.edge_table.iter() {

            // If there is one we replace it with a new edge that will have an updated target node
            if element.label.chars().next().unwrap() == current_char {
                current_node.edge_table.push(Edge::new(current_char.to_string(), 1, insert_suffix(element.target_node.clone(), suffix[1..].to_string(), suffix_index, lvl + 1)));
                current_node.edge_table.remove(i);
                found = true;
                break;
            }
            i += 1;
        }

        // If we didn't find an edge that starts with the current char we create a new one
        if !found {
            current_node.edge_table.push(Edge::new(current_char.to_string(), 1, insert_suffix(TreeNode::new(lvl + 1, None), suffix[1..].to_string(), suffix_index, lvl + 1)));
        }
    } else {
        // If we are at the end of the suffix we create the ending edge and node that contains the index of the suffix
        current_node.edge_table.push(Edge::new(current_char.to_string(), 1, TreeNode::new(lvl + 1, Some(suffix_index))));
    }

    // We return the updated node for recursive calls
    return current_node;
}

// Takes a tree node as an argument and contract the tree
// Recursively calls itself until the tree is fully contracted
pub fn contract_trie(node: TreeNode) -> (TreeNode, String, bool) {
    let mut current_node = node;

    // We check how many edges the current node has
    if current_node.edge_table.len() > 1 {
        let mut end: bool;
    
        // If there is more than one we can't contract the child nodes with the one we are currently on
        // So we make recursive calls on the child nodes
        for i in 0..current_node.edge_table.len() {

            end = false;

            // end will become true once we reach a node that has zero children
            while !end {
                let child: TreeNode;
                let child_label: String;
                (child, child_label, end) = contract_trie(current_node.edge_table[i].target_node.clone());

                // We update our current node with the results of the recursive call
                current_node.edge_table[i].label += child_label.as_str();
                current_node.edge_table[i].label_length += child_label.len() as u32;
                current_node.edge_table[i].target_node = child;
            }
        }

        // We return the updated node with end = true to indicate to the parent node that it can't contract further on this part of the tree
        return (current_node, "".to_string(), true);

    // If our current node has only one child edge, the parent node can contract
    } else if current_node.edge_table.len() == 1 {
        // We gather the label of the edge
        let to_merge = current_node.edge_table[0].label.clone();

        // We return the child node and the label of the edge to the parent node
        // When the parent node will update itself with the results of the recursive call it will contract the edge with the child node
        return (current_node.edge_table[0].target_node.clone(), to_merge, false);

    // If our current node has no child edge, we return the node with end = true to indicate to the parent node that it can't contract further on this part of the tree
    } else {
        return (current_node, "".to_string(), true);
    }
}

// Takes a string as an argument and make a suffix tree out of it
pub fn make_tree(string: String) -> TreeNode {
    let mut root = TreeNode::new(0, None);

    // We insert every suffix of the string in the tree by looping over the string
    for i in 0..string.len() {
        root = insert_suffix(root, string[i..].to_string(), i as u32, 0);
    }

    // Updated tree
    return root;
}

// Writes a suffix tree in debug format in a file
pub fn write_tree_in_file(root: TreeNode, filename: String) {
    let mut file = File::create(filename).unwrap();

    file.write_all(format!("{:?}", root).as_bytes()).unwrap();
}

pub fn main() {
    // let test_string = "121112212221$".to_string();
    let small_string = "aba$".to_string();
    // let mut root = TreeNode::new(0, None);

    // let (sorted_pairs, ranked_string) = rank_string(test_string);

    // println!("{:?} | {:?}", sorted_pairs, ranked_string);

    // for i in 0..ranked_string.len() {
    //     root = insert_suffix(root, ranked_string[i..].to_string(), i as u32, 0);
    // }

    // root = contract_trie(root).0;

    let root = make_tree(small_string);

    write_tree_in_file(root.clone(), "tree.txt".to_string());
    
    println!("{:?}", root);

    let root = contract_trie(root).0;
    
    write_tree_in_file(root.clone(), "compact_tree.txt".to_string());

    print!("{:?}", root);
}
