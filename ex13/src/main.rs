#![feature(box_syntax)]

extern crate time;

mod graph;

use graph::Graph;

use std::io::Write;
use std::u64;
use std::fs::File;
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
			println!();

			// --------------- LCC -----------------------------------------------------

			//calculate_lcc(&graph);
			println!();

			match calculate_distances(&mut graph) {
				Ok(_) => { },
				Err(error) => {
					println!("{}", error);
				}
			}
		},
		Err(error) => {
			println!("{}", error);
		}
	}
}

fn calculate_distances(graph: &mut Graph) -> std::io::Result<()> {
	let mut nuremberg_file = File::create("nuremberg.map")?;
	let mut longest_file = File::create("longest.map")?;

	nuremberg_file.write_all(b"[map]")?;
	longest_file.write_all(b"[map]")?;

	// ------------------------ Distance ------------------------------------

	let path = calculate_path(graph, u64::MAX);

	nuremberg_file.write_all(path.0.as_bytes())?;
	nuremberg_file.write_all(b" (blue|distance)[/map]\r\n[map]")?;

	longest_file.write_all(path.1.as_bytes())?;
	longest_file.write_all(b" (blue|distance)[/map]\r\n[map]")?;

	// ---------------- Time with 130 km/h ----------------------------------

	let path = calculate_path(graph, 130);

	nuremberg_file.write_all(path.0.as_bytes())?;
	nuremberg_file.write_all(b" (red|130kmh)[/map]\r\n[map]")?;

	longest_file.write_all(path.1.as_bytes())?;
	longest_file.write_all(b" (red|130kmh)[/map]\r\n[map]")?;

	// ---------------- Time with 100 km/h ----------------------------------

	let path = calculate_path(graph, 100);

	nuremberg_file.write_all(path.0.as_bytes())?;
	nuremberg_file.write_all(b" (green|100kmh)[/map]")?;

	longest_file.write_all(path.1.as_bytes())?;
	longest_file.write_all(b" (green|100kmh)[/map]")?;

	return Ok(());
}

fn calculate_lcc(graph: &Graph) {
	let start_time = get_time();
	let lcc = graph.compute_lcc();
	let time = (get_time() - start_time).num_milliseconds();
	println!("Calculated lcc with size {} in {} ms!", &lcc.0, time);
}

fn calculate_path(graph: &mut Graph, max_speed: u64) -> (String, String) {
	graph.reset();

	match max_speed {
		u64::MAX => {
			graph.set_arc_costs_to_distance();
		},
		_ => {
			graph.set_arc_costs_to_travel_time(max_speed);
		}
	}

	let (mut nuremberg, mut longest) = (String::default(), String::default());

	// Shortest / Fastest

	let start_time = get_time();
	graph.compute_shortest_paths(5508637);
	let path = graph.travel_to(4435496, max_speed);
	let time = (get_time() - start_time).num_milliseconds();

	match max_speed {
		u64::MAX => { println!("Shortest path:") }
		_ => { println!("Fastest path with {} km/h:", max_speed) }
	};
	println!("Distance {:.2} km, Time {}, Computation time {} ms",	path.0 / 1000.0,
		time_to_string(path.1), time);

	graph.generate_mapbb(&mut nuremberg, 4435496);

	// Longest

	let start_time = get_time();
	let furthest_node = graph.get_furthest_node();
	let path = graph.travel_to(furthest_node.1, max_speed);
	let time = (get_time() - start_time).num_milliseconds();

	match max_speed {
		u64::MAX => { println!("Longest path:") }
		_ => { println!("Longest path with {} km/h:", max_speed) }
	};
	println!("Distance {:.2} km, Time {}, Computation time {} ms",	path.0 / 1000.0,
		time_to_string(path.1), time);
	println!();

	graph.generate_mapbb(&mut longest, furthest_node.1);

	return (nuremberg, longest);
}

/// Convert time in hours to string format.
fn time_to_string(time: f64) -> String {
    let hours = (time / 3600.0) as u64;
    let minutes = ((time - (hours as f64) * 3600.0) / 60.0) as u64;
    format!("{} hour(s) {} minute(s)", hours, minutes)
}