use std::collections::HashMap;
use std::fs;

fn main() {
    solve("data/input");
    //solve("data/test_input");
}

fn triangle_starts_with_char(triangle:&(&str,&str,&str),char_check:char) -> bool {
    triangle.0.chars().next().unwrap()==char_check || triangle.1.chars().next().unwrap()==char_check || triangle.2.chars().next().unwrap()==char_check
}



fn solve(data_path:&str)
{
    let content = fs::read_to_string(data_path).expect("Expected to read the file");
    let lines=content.split("\n").filter(|line|!line.is_empty());
    let mut connection_pairs:HashMap<(&str,&str),bool>=HashMap::new(); // Whether connection exists
    let mut connection_sets:HashMap<&str,Vec<&str>> =HashMap::new(); // All connections for a given node
    for line in lines {
        let mut pair_split=line.split("-");
        let (node_id_0,node_id_1) = (pair_split.next().unwrap(),pair_split.next().unwrap());
        connection_pairs.entry((node_id_0,node_id_1)).or_insert(true); // Forward
        connection_pairs.entry((node_id_1,node_id_0)).or_insert(true); // Backward
        connection_sets.entry(node_id_0).and_modify(|value|value.push(node_id_1)).or_insert(vec!{node_id_1}); // Add entry
        connection_sets.entry(node_id_1).and_modify(|value|value.push(node_id_0)).or_insert(vec!{node_id_0}); // Add entry
    }
    // Check connections
    let mut triangle_sets:HashMap<(&str,&str,&str),bool>=HashMap::new(); // Vector of array of 3 items, for comparison]
    for node_connection_pair in &connection_sets {
        for &second_node_connection_id in node_connection_pair.1 { // Cycle through every node connected to "root" node
            let second_node_connection_set=connection_sets.get(&second_node_connection_id).unwrap();
            for third_node_connection_id in second_node_connection_set {
                if connection_pairs.contains_key(&(*third_node_connection_id,*node_connection_pair.0)) {
                    // Has a connection back. insert if not a duplicate
                    //check_or_insert_triangle_set(&mut triangle_sets,(*node_connection_pair.0,second_node_connection_id,third_node_connection_id))
                    let mut sorted_nodes:Vec<&str>=vec!{*node_connection_pair.0,second_node_connection_id,third_node_connection_id};
                    sorted_nodes.sort();
                    let triangle_tuple=(sorted_nodes[0],sorted_nodes[1],sorted_nodes[2]);
                    if !triangle_sets.contains_key(&triangle_tuple)
                    {
                        triangle_sets.entry(triangle_tuple).or_insert(true);
                    }
                }
            }
        }
    }
/*
    let mut fully_connection_sets:Vec<(Vec<&str>)>=Vec::new();
    for node_connection_pair in &connection_sets {
        // The count will be the number of items in this set that are all connected to each other
        // eg in A,B,C,D for this to work we must have A-B,A-C,A-D, B-C,B-D,
        let mut all_node_ids=node_connection_pair.1.clone();
        all_node_ids.push(node_connection_pair.0); // Add original
        all_node_ids.sort(); // Sort alphabetically
        // We kinda wanna check every possible combo,
        let mut fully_connected_node_ids:Vec<&str>=Vec::new();
        for test_node_id in &all_node_ids { // For each node id on the set connected to A (A,B,C,D)
            for test_connected_node_id in &all_node_ids { // Check against each connected node (A,B,C,D).
                if test_node_id!=test_connected_node_id { // must be different!
                    if connection_pairs.contains_key(&(*test_node_id,*test_connected_node_id)) {  // If is also B is connected to C.
                        fully_connected_node_ids.push()
                    }
                }
            }
        }
    }
    */
    println!("triangle sets found {}",triangle_sets.len());
    let filtered_sets:Vec<&(&str,&str,&str)>=triangle_sets.iter().filter(|triangle_set|triangle_starts_with_char(triangle_set.0,'t')).map(|pair|pair.0).collect();
    println!("Pt1 - filtered_sets {}",filtered_sets.len());
}

