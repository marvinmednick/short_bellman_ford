static mut max_out_level : u32= 0;
static mut max_in_level : u32 = 0;

struct DFS {
    graph:  DirectedGraph,
	explored:  HashMap::<u32,bool>,
	pub finished_order:  Vec::<u32>,
	pub start_search:  HashMap::<u32,Vec::<u32>>,
	top_search_cnts:  HashMap::<u32, usize>,

}

impl DFS {

	pub fn new() -> DFS {
		DFS {
				explored:  HashMap::<u32,bool>::new(),
				finished_order:  Vec::<u32>::new(),
				start_search : HashMap::<u32,Vec::<u32>>::new(),
				top_search_cnts : HashMap::<u32,usize>::new(),
		}
	}
	pub fn add_search_entry(&mut self, vertex: u32, count: usize) {

			self.top_search_cnts.insert(vertex,count);
			let mut removed = None;
			if self.top_search_cnts.len() > 10 {
				let top_search_iter = self.top_search_cnts.iter();
				let mut top_search_count_vec : Vec::<(u32, usize)> = top_search_iter.map(|(k,v)| (*k, *v)).collect();
				top_search_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
				removed = top_search_count_vec.pop();
			}
			if let Some(entry) = removed {
				self.top_search_cnts.remove(&entry.0);
				
			}
			
	}

	pub fn dfs_outgoing(&mut self, vertex_id:  u32, start_vertex: u32, level: u32) {
			
//			let spacer = (0..level*5).map(|_| " ").collect::<String>();
			unsafe {
			if level > max_out_level {
				max_out_level = level;
//					println!("reached level {}", max_out_level);
			}
			}
			
			// Set current node to explored
			self.explored.insert(vertex_id,true);

			let mut cur_len : usize = 0;
		
			{
				let group_list = self.start_search.entry(start_vertex).or_insert(Vec::<u32>::new());
				group_list.push(vertex_id);
				cur_len = group_list.len();
			}
			self.add_search_entry(start_vertex,cur_len);

			
			let next_v : Vertex;

			if let Some(vertex) = self.vertex_map.get(&vertex_id) {

				next_v = vertex.clone();
			}

			else {
				panic!("invalid vertex");
			}

			// Search through each edge
			for edge in next_v.outgoing.keys() {
				let next_vertex = edge.clone();
				if !self.explored.contains_key(&edge) {
					self.dfs_outgoing(next_vertex,start_vertex,level+1);
				}
				else {
			//		println!("{}Vertex {} is already explored",spacer,edge);
				}
			}
			// so add it to the finished list
			self.finished_order.push(vertex_id);
	}

	pub fn dfs_incoming(&mut self, vertex_id:  u32, start_vertex: u32, level: u32) {
			
//			let spacer = (0..level*5).map(|_| " ").collect::<String>();
			unsafe {
			if level > max_in_level {
				max_in_level = level;
//				println!("reached level {}", max_in_level);
			}
			}
			
			// Set current node to explored
			self.explored.insert(vertex_id,true);

			let group_list = self.start_search.entry(start_vertex).or_insert(Vec::<u32>::new());
			group_list.push(vertex_id);
			let cur_len = group_list.len();
			self.add_search_entry(start_vertex,cur_len);

			let next_v : Vertex;

			if let Some(vertex) = self.vertex_map.get(&vertex_id) {

				next_v = vertex.clone();
			}

			else {
				panic!("invalid vertex");
			}

			// Search through each edge
			for edge in next_v.incoming.keys() {
				let next_vertex = edge.clone();
				if !self.explored.contains_key(&edge) {
					self.dfs_incoming(next_vertex,start_vertex,level+1);
				}
				else {
			//		println!("{}Vertex {} is already explored",spacer,edge);
				}
			}
			// so add it to the finished list
			self.finished_order.push(vertex_id);
	}

	pub fn dfs_loop_incoming(&mut self, list: &Vec<u32>) {

//		println!("Looping on incoming DFS");
		self.finished_order = Vec::<u32>::new();
		self.start_search = HashMap::<u32,Vec::<u32>>::new();
		self.explored = HashMap::<u32,bool>::new();
		self.top_search_cnts = HashMap::<u32,usize>::new();

		let mut _count : usize = 0;
		for v in list {
/*			if _count % 1000000 == 0 {
				print!("*");
				io::stdout().flush().unwrap();
			} */
			let vertex = v.clone();
//			println!("Looping on {}",vertex);
			if !self.explored.contains_key(&vertex) {
				self.dfs_incoming(vertex,vertex,0);
			}
			_count += 1;
		}
	}

	pub fn dfs_loop_outgoing(&mut self, list: &Vec<u32>) {
//		println!("Looping on outgoing DFS");
		self.finished_order = Vec::<u32>::new();
		self.start_search = HashMap::<u32,Vec::<u32>>::new();
		self.explored = HashMap::<u32,bool>::new();
		self.top_search_cnts = HashMap::<u32,usize>::new();

		let mut _count : usize = 0;
		for v in list {
/*			if _count % 1000000 == 0 {
				print!("#");
				io::stdout().flush().unwrap();
			} */
			let vertex = v.clone();
//			println!("Looping on {}",vertex);
			if !self.explored.contains_key(&vertex) {
				self.dfs_outgoing(vertex,vertex,0);
			}
		}
	}

}
