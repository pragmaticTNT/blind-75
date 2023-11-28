/*
 * @lc app=leetcode id=269 lang=rust
 *
 * [###] Alien Dictionary
 * 
    There is a new alien language which uses the latin alphabet. However, the order among letters are unknown to you. You receive a list of non-empty words from the dictionary, where words are sorted lexicographically by the rules of this new language. Derive the order of letters in this language.

    Example 1:

    Input:
    [
    "wrt",
    "wrf",
    "er",
    "ett",
    "rftt"
    ]

    Output: "wertf"

    If multiple solutions are possible, we will return a random one. 
 */
pub struct Solution;

// @lc code=start 
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Node {
    pub in_nodes: HashSet<usize>,
    pub out_nodes: HashSet<usize>,
}

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph: HashMap<usize, Node> = HashMap::new();

        // add all the characters to the graph
        for word in words.iter() {
            for c in word.chars() {
                let index = (c as u8 - b'a').into();
                graph.insert(index, 
                    Node {
                        in_nodes: HashSet::new(),
                        out_nodes: HashSet::new(),
                    });
            }
        }

        // add relationships between the characters
        for pair in words.windows(2) {
            let (before, after) = extract_order(&pair[0], &pair[1]);
            if let (Some(before), Some(after)) = (before, after) {
                graph.get_mut(&before).unwrap().out_nodes.insert(after);   
                graph.get_mut(&after).unwrap().in_nodes.insert(before);
            }
        }

        let mut toposort = vec![];
        let mut visited = HashSet::new();
        let mut queue = vec![];

        // find the sources for the topological sort
        for (index, node) in &graph {
            if node.in_nodes.len() == 0 {
                queue.push(index);
            }
        }
        queue.sort_by(|a, b| a.cmp(&b));
        let mut queue: VecDeque<_> = queue.into_iter().collect();

        while let Some(i) = queue.pop_front() {
            toposort.push(i);
            visited.insert(i);
            //(*graph.get_mut(i).unwrap()).out_nodes.sort_by(|a, b| a.cmp(&b));
            println!("{}, {:?}", i, graph[i].out_nodes);
            for nbour_index in &graph[i].out_nodes {
                if visited.contains(nbour_index) {
                    return String::new();
                }
                queue.push_back(nbour_index);
            }
        }
        
        toposort.iter().map(|&&u| (u as u8 + b'a') as char).collect()
    }
}

pub fn extract_order(word1: &String, word2: &String) -> (Option<usize>, Option<usize>) {
    for (b1, b2) in word1.chars().zip(word2.chars()) {
        if b1 != b2 {
            return (Some((b1 as u8 - b'a').into()), Some((b2 as u8 - b'a').into()));
        }
    }
    (None, None)
}
 // @lc code=end