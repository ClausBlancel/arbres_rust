use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::fmt::Display;

use itertools::Itertools;
use regex::Regex;

// Tree Node structure
#[derive(Debug, Clone)]
pub struct TreeNode {
    edge_table: Vec<Edge>,
    index: Option<u32>
}

impl TreeNode {
    fn new(index: Option<u32>) -> TreeNode {
        return TreeNode { 
            edge_table: vec![],
            index
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = format!("{:?}\n", self.index);
        for element in self.edge_table.iter() {
            string += format!("{}\n", element).as_str();
        }
        write!(f, "{}", string)
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

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "└─{}──{}", self.label, self.target_node)
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

    return (sorted_pairs, ranked_string + "~")
}

// Takes a tree node and a suffix as arguments and insert the suffix in the tree
pub fn insert_suffix(root: TreeNode, suffix: String, suffix_index: u32) -> TreeNode {
    let mut current_node = root;
    let current_char = suffix.chars().nth(0).unwrap();
    let mut found = false;
    let mut i = 0;

    // Checks if we are at the end of the suffix
    if current_char != '~' {

        // If not we explore every edge of the current node to see if there is one that starts with the current char
        for element in current_node.edge_table.iter() {

            // If there is one we replace it with a new edge that will have an updated target node
            if element.label.chars().next().unwrap() == current_char {
                current_node.edge_table.push(Edge::new(current_char.to_string(), 1, insert_suffix(element.target_node.clone(), suffix[1..].to_string(), suffix_index)));
                current_node.edge_table.remove(i);
                found = true;
                break;
            }
            i += 1;
        }

        // If we didn't find an edge that starts with the current char we create a new one
        if !found {
            current_node.edge_table.push(Edge::new(current_char.to_string(), 1, insert_suffix(TreeNode::new(None), suffix[1..].to_string(), suffix_index)));
        }
    } else {
        // If we are at the end of the suffix we create the ending edge and node that contains the index of the suffix
        current_node.edge_table.push(Edge::new(current_char.to_string(), 1, TreeNode::new(Some(suffix_index))));
    }

    // We sort the edges by their label to make sure the tree is in alphabetical order
    current_node.edge_table.sort_by(|a, b| a.label.cmp(&b.label));

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
    let mut root = TreeNode::new(None);

    // We insert every suffix of the string in the tree by looping over the string
    for i in 0..string.len() {
        root = insert_suffix(root, string[i..].to_string(), (2 * (i + 1) - 1) as u32);
    }

    // Updated tree
    return root;
}

// Writes a suffix tree in debug format in a file
pub fn write_tree_in_file(root: TreeNode, filename: String) {
    let mut file = File::create(filename).unwrap();

    file.write_all(format!("{:?}", root).as_bytes()).unwrap();
}

// Reads a file and returns a vector of strings that needs to be turned into a tree
pub fn read_file(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"^\S[0-9]*$").unwrap();
    let mut lines_to_tree: Vec<String> = Vec::new();

    // We loop over every line of the file and check if it matches the regex
    for line in reader.lines() {
        let line = line?;

        // If it does we add it to the vector
        if re.is_match(&line) {
            lines_to_tree.push(line);
        }
    }

    // We return the vector
    Ok(lines_to_tree)
}

pub fn main() {
    // In the strings vector we have every string that needs to be turned into a tree
    let strings = read_file("data.txt").unwrap();
    let mut i = 0;

    // We loop over every string and turn it into a tree
    for element in strings {
        let (_pairs, ranked_string) = rank_string(element + "~");
        let tree = make_tree(ranked_string);
        let (contracted_tree, _, _) = contract_trie(tree); 

        // We write the tree in a file
        write_tree_in_file(contracted_tree, format!("tree{}.txt", i));
        i += 1;
    }
    
}