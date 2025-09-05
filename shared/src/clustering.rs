use crate::core::{Amplitude,usize_dif,usize_subtract};
use std::cmp::{min,max};
/*
clustering algorithm for voxels.

storing bounding Cuboid for cluster gives acceptible performance most of the time.
slow performance with big clusters. with big bounding Cuboid. because have to check many points.
splitting clusters into regions to help with this.




*/

pub const GRID_SIZE: usize = 10;


#[derive(Debug,Copy,Clone)]
pub struct XYZV {
	pub x:usize,
	pub y: usize,
	pub z:usize,
	pub v: Amplitude,
}






#[derive(Debug,Clone,Copy)]
struct Cuboid{
	min_x:usize,
	min_y:usize,
	min_z:usize,
	max_x:usize,
	max_y:usize,
	max_z:usize,
}

impl Cuboid{
	
	fn buffered(&self, x:usize, y:usize ,z:usize) -> Cuboid{
		Cuboid{
			min_x:usize_subtract(self.min_x,x)
			,min_y: usize_subtract(self.min_y,y)
			,min_z: usize_subtract(self.min_z,z)
			,max_x:self.max_x+x
			,max_y:self.max_y+y
			,max_z:self.max_z+z
		}
	}
	
	
	fn contains(&self , x: usize , y: usize, z: usize) -> bool{
		self.min_x<=x && x<= self.max_x 
		&& self.min_y<=y && y<=self.max_y 
		&& self.min_z<=z && z<=self.max_z
	}
	
	//extend to include point
	fn include_point(&mut self , x:usize , y:usize , z:usize){
		
		if x > self.max_x{
			self.max_x = x;
		}
		
		if x < self.min_x{
			self.min_x = x;
		}
		
		if y > self.max_y{
			self.max_y = y;
		}
		
		if y < self.min_y{
			self.min_y = y;
		}
		
		if z > self.max_z{
			self.max_z = z;
		}
		
		if z < self.min_z{
			self.min_z = z;
		}
		
	}
		
	
	
	fn from_point(x:usize,y:usize,z:usize) -> Cuboid{
		return Cuboid{min_x:x, max_x:x, min_y:y, max_y:y, min_z:z, max_z:z};

	}
	
	
	fn merged(&self, other: &Cuboid) -> Cuboid{
		Cuboid{
			max_x:max(self.max_x,other.max_x),
			max_y:max(self.max_y,other.max_y),
			max_z:max(self.max_z,other.max_z),
			min_x:min(self.min_x,other.min_x),		
			min_y:min(self.min_y,other.min_y),
			min_z:min(self.min_z,other.min_z),
		}
	}
}



/*grid index should work for this.
simpler than other types and no complicated balancing*/
#[derive(Debug,Clone)]
pub struct Region{
	pub key: (usize,usize,usize),
	pub data_bounds: Cuboid,//bounds of data rather than region itself.
	pub data: Vec<XYZV>,
}

impl Region{
	
	//key from x,y,z coord of point within region
	pub fn key_from_point(x:usize, y:usize,z:usize) -> (usize,usize,usize) {
		(x/GRID_SIZE, y/GRID_SIZE, z/GRID_SIZE)
	}
	
	
	//key for region to right of key
	pub fn right(key:(usize,usize,usize)) -> (usize,usize,usize){
		(key.0+1, key.1, key.2)
	}
	
	pub fn min_x(key:(usize,usize,usize)) -> usize{
		key.0*GRID_SIZE
	}
	
	pub fn min_y(key:(usize,usize,usize)) -> usize{
		key.1*GRID_SIZE
	}
	
	//key for region above of key
	pub fn top(key:(usize,usize,usize)) -> (usize,usize,usize){
		(key.0, key.1+1, key.2)
	}
	
		//key for region above of key
	pub fn top_right(key:(usize,usize,usize)) -> (usize,usize,usize){
		(key.0+1, key.1+1, key.2)
	}
	
	
	
	fn is_adjacent(&self , point:&XYZV , x_gap:usize , y_gap:usize , z_gap:usize) -> bool{
		//check bounding Cuboid
		if self.data_bounds.buffered(x_gap,y_gap,z_gap).contains(point.x,point.y,point.z){
			for p in self.data.iter(){
				if usize_dif(p.x,point.x) <= x_gap && usize_dif(p.y,point.y) <= y_gap && usize_dif(p.z,point.z) <= z_gap{
					return true;
				}
			}
		}
		return false;
	}
	
	
	fn from_point(p:XYZV) -> Region{
		let mut points = Vec::with_capacity(GRID_SIZE*GRID_SIZE*GRID_SIZE/10);
		points.push(p);
		
		return Region{
			key: Region::key_from_point(p.x, p.y, p.z),
			data: points,
			data_bounds: Cuboid::from_point(p.x, p.y, p.z),
			};
	}
	
	
	fn add_point(&mut self, p:XYZV){
		self.data.push(p);
		self.data_bounds.include_point(p.x, p.y, p.z);
	}
	
	
//	pub fn x_y_between(&self, min_z: usize, max_z: usize) -> impl Iterator<Item = (usize , usize)>{
//		return self.data.iter().filter(move |p| p.z >= min_z && p.z <= max_z).map(|p| (p.x , p.y));
//	}
	
	
	pub fn values_between(&self, min_z:usize ,max_z:usize) -> impl Iterator<Item = Amplitude> {
		return self.data.iter().filter(move |p| p.z >= min_z && p.z <= max_z).map(|p| p.v);
	}
	
	
	fn merge(&mut self, other: &Region){
		self.data.extend(other.data.clone());
		self.data_bounds = self.data_bounds.merged(&other.data_bounds);
	}

}

#[derive(Clone,Debug)]
pub struct Cluster{
	pub regions: Vec<Region>,
	bounds: Cuboid,
}



impl Cluster{
	
	
	fn from_point(point:XYZV) -> Cluster {
		Cluster{regions: vec![Region::from_point(point)],
		bounds: Cuboid::from_point(point.x, point.y, point.z),
		}
	}
	
	
	fn is_adjacent(&self , point:&XYZV , x_gap:usize , y_gap:usize , z_gap:usize) -> bool{
		//check bounds of whole cluster.
		if !self.bounds.buffered(x_gap,y_gap,z_gap).contains(point.x,point.y,point.z){
			return false;
		}
		for region in self.regions.iter(){
			if region.is_adjacent(point,x_gap,y_gap,z_gap){
				return true;
			}
		}
		return false;
	}
	
	
	
	fn merge(clusters:Vec<Cluster>) -> Cluster{
		let mut r = Cluster{regions: Vec::new() , bounds:clusters[0].bounds};
		for c in clusters{
			for region in c.regions{
				r.upsert_region(region)
			}
		}
		r.update_bounds();
		return r;
	}


	fn update_bounds(&mut self){
		let mut b:Cuboid = self.regions[0].data_bounds;
		for r in self.regions.iter(){
			b = b.merged(&r.data_bounds);
			}
		self.bounds = b.clone();
	}


	pub fn volume(&self) -> usize{
		let mut vol:usize = 0;
		for r in self.regions.iter(){
			vol += r.data.len();
		}
		return vol;
	}
	
	
	//mutable reference to region
	pub fn get_region_mut(&mut self, key:(usize,usize,usize)) -> Option<&mut Region>{
		self.regions.iter_mut().find(|r| r.key==key)
	}
	
	
		//mutable reference to region
	pub fn get_region(&self, key:(usize,usize,usize)) -> Option<&Region>{
		self.regions.iter().find(|r| r.key == key)
	}
	
	
	/*only used to merge.
	add new region or add points to existing.
	remember to update cluster bounds after this
	*/
	fn upsert_region(&mut self, region:Region){
		if let Some(existing) = self.get_region_mut(region.key){
			existing.merge(&region);
		}
		else{
			self.regions.push(region);
		}
	}
	
	
	
	fn add_point(&mut self , point: XYZV){
		self.bounds.include_point(point.x, point.y, point.z);
		
		if let Some(existing) = self.get_region_mut(Region::key_from_point(point.x,point.y,point.z)){
			existing.add_point(point);
		}
		else{
			self.regions.push(Region::from_point(point));
		}
	}
	
//	pub fn x_y_between(&self, min_z: usize, max_z: usize) -> Vec<(usize , usize)>{
	//	self.regions.iter().flat_map(|r| r.x_y_between(min_z,max_z)).collect()

	//	return self.points.iter().filter(|p| p.z >= min_z && p.z <= max_z).map(|p| (p.x , p.y)).collect();
//	}
	
	
	pub fn values_between(&self , min_z:usize , max_z:usize) -> Vec<Amplitude>{
		self.regions.iter().flat_map(|r| r.values_between(min_z,max_z)).collect()
	}
	
	

	
}



#[derive(Debug,Clone)]
pub struct Clusterer{
	pub active:Vec<Cluster>,
	pub inactive:Vec<Cluster>,
	pub x_gap:usize,
	pub y_gap:usize,
	pub z_gap:usize,
	//cluster_counter:usize,
}



impl Clusterer{
	
	
	
	pub fn new(x_gap:usize,y_gap:usize,z_gap:usize) -> Clusterer{
		Clusterer{active:Vec::with_capacity(100), inactive:Vec::new(), x_gap:x_gap, y_gap:y_gap, z_gap:z_gap}
	}
	
	
	
	//find adjacent clusters. add new cluster if 0,add point if 1, merge if >1
	pub fn add_point(&mut self , x:usize , y:usize , z:usize , value:Amplitude){
		
		let p = XYZV{x:x,y:y,z:z,v:value};
		let adj:Vec<usize> = self.active.iter().enumerate().filter(|(_i,c)| c.is_adjacent(&p, self.x_gap, self.y_gap , self.z_gap)).map(|(i,_c)| i).collect();
	//	println!("adj {:?}", adj);

		
		match adj.len(){
			0 => self.active.push(Cluster::from_point(p)),
			
			1 => self.active[adj[0]].add_point(p),
			
			_ => {
				let mut to_merge:Vec<Cluster> = Vec::with_capacity(adj.len());
				for i in adj.into_iter().rev(){
					to_merge.push(self.active.swap_remove(i));
				}
				
				let mut merged = Cluster::merge(to_merge);
				merged.add_point(p);
				self.active.push(merged);
				
				
			}
		}
	}
	



	pub fn clusters(&self) -> Vec<Cluster>{
		return self.active.clone();
	}
	
	
}






//cargo test cluster -- --nocapture

#[cfg(test)]
mod cluster_tests{
	use super::*;

	#[test]
	fn test_cluster(){
		
		let mut points: Vec<XYZV> = Vec::new();
		
		for i in 0..30{
			points.push(XYZV{x:i , y:2 , z:10,v:5});
		}
		
		
		for i in 100..120{
			points.push(XYZV{x:i , y:10 , z:10,v:5});
		}
		
		
		let mut c = Clusterer::new(1,1,1);
		for p in points{
			c.add_point(p.x,p.y,p.z,p.v);
			println!("clusters:{:?}",c.clusters());
		};
		
		let clusters = c.clusters();
		assert_eq!(clusters.len(),2);
	}
	
	
}



