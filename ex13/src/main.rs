#![feature(box_syntax)]

extern crate time;

mod graph;

use graph::Graph;

use std::u64;
use time::get_time;

const DEFAULT_FILE: &str = "graphs/bawue_bayern.zip";

fn main() {
	let file_name = DEFAULT_FILE;
	
	println!("Reading graph from file {}", &file_name);
	
	let start_time = get_time();
	match Graph::read_graph_from_file(file_name) {
		Ok(mut graph) => {
			println!("Read graph with {} nodes and {} arcs in {} ms!", &graph.num_nodes(),
					&graph.num_arcs(), (get_time() - start_time).num_milliseconds());
			
			// --------------- LCC -----------------------------------------------------
			
			calculate_lcc(&graph);
			
			// --------------- Distance ------------------------------------------------
			
			calculate_path(&mut graph, u64::MAX);
			
			// --------------- Time with 130 km/h ---------------------------------------
			
			calculate_path(&mut graph, 130);
			
			// --------------- Time with 100 km/h ---------------------------------------
			
			calculate_path(&mut graph, 100);
		},
		Err(error) => {
			println!("{}", error);
		}
	}
}

fn calculate_lcc(graph: &Graph) {
	let start_time = get_time();
	let lcc = graph.compute_lcc();
	let time = (get_time() - start_time).num_milliseconds();
	println!("Calculated lcc with size {} in {} ms!", &lcc.0, time);
}

fn calculate_path(graph: &mut Graph, max_speed: u64) {
	match max_speed {
		u64::MAX => { },
		_ => {
			graph.set_arc_costs_to_travel_time(max_speed);
			graph.reset();
		}
	}
	
	// Shortest / Fastest
	
	let start_time = get_time();
	graph.compute_shortest_paths(5508637);
	let path = graph.travel_to(4435496, max_speed);
	let time = (get_time() - start_time).num_milliseconds();
	
	match max_speed {
		u64::MAX => { println!("Shortest path:") }
		_ => { println!("Fastest path with {} km/h:", max_speed) }
	};
	println!("Distance {} km, Time {}, Computation time {} ms",	path.0,
		time_to_string(path.1), time);
	
	// Longest
	
	let start_time = get_time();
	let furthest_node = graph.get_furthest_node();
	let path = graph.travel_to(furthest_node.1, max_speed);
	let time = (get_time() - start_time).num_milliseconds();
	
	match max_speed {
		u64::MAX => { println!("Longest path:") }
		_ => { println!("Longest path with {} km/h:", max_speed) }
	};
	println!("Distance {} km, Time {}, Computation time {} ms",	path.0,
		time_to_string(path.1), time);
}

/// Convert time in hours to string format.
fn time_to_string(time: f64) -> String {
    let hours = time as u64;
    let minutes = ((time - hours as f64) * 60.0) as u64;
    format!("{} hour(s) {} minute(s)", hours, minutes)
}