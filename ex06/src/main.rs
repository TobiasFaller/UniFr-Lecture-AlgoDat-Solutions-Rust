mod priority_queue;

use priority_queue::PriorityQueue;

fn main() {
	let mut pq: PriorityQueue<i64, &str> = PriorityQueue::new();
	
	let item1 = pq.insert(1, "Airforce One");
	let item2 = pq.insert(45, "Bermuda Triangle Blues (Flight 45)");
	let item3 = pq.insert(666, "Flight 666");
	let item4 = pq.insert(80, "Another flight 80");
	let item5 = pq.insert(777, "Another flight 777");
	let item6 = pq.insert(35, "Another flight 35");
	
	pq.change_key(&item1, 500);
	pq.change_key(&item3, 500);
	pq.change_key(&item4, 500);
	
	while pq.size() > 0 {
		let (key, value) = pq.pop().unwrap();
		println!("{}: {}", key, value);
	}
}