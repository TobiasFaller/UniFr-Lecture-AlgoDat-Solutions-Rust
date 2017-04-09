#![feature(box_syntax)]

extern crate time;

mod graph;

use graph::Graph;

use time::get_time;

fn main() {
	let file_name = "graphs/bawue_bayern.zip";
	
	println!("Reading graph from file {}", &file_name);
	
	let start_time = get_time();
	match Graph::read_graph_from_file(file_name) {
		Ok(graph) => {
			println!("Read graph with {} nodes and {} arcs in {} ms!", graph.num_nodes(), graph.num_arcs(),
					(get_time() - start_time).num_milliseconds()
				);
			
			let start_time = get_time();
			println!("Calculated lcc with size {} in {} ms!", &graph.compute_lcc().0,
					(get_time() - start_time).num_milliseconds());
		},
		Err(error) => {
			println!("{}", error);
		}
	}
}