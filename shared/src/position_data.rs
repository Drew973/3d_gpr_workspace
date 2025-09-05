
use std::error::Error;
use geo::{MultiPoint,Point,ConvexHull,Polygon,CoordsIter,LineString,Coord,MultiPolygon};
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::path::Path;
use wkt::ToWkt;
use crate::core::Amplitude;
use anyhow::{Result,bail};
use crate::core::{usize_last_multiple,usize_next_multiple};

use crate::clustering::{Cluster,GRID_SIZE,Region};



#[derive(Serialize,Deserialize,Debug)]
pub struct Layer{
	pub min_depth:usize,
	pub max_depth:usize,
	pub label:String,
}




#[derive(Serialize,Debug)]
pub struct ClusterFeature{
	depth_band:String,
	mean_amplitude: Amplitude,
	wkt: String,
}

impl ClusterFeature{
	pub fn write_csv(features:Vec<ClusterFeature>, output:&str) -> Result<(), Box<dyn Error>>{
		let mut wtr = Writer::from_path(output)?;
		for f in features{
			let _ = wtr.serialize(f);
		}
		Ok(())
	}
}




//for writing to csv
#[derive(Serialize)]
struct Position{
	longitudinal:usize,
	transverse:usize,
	x:f64,
	y:f64
}

//ok for slice up to at least 2^48 i16.
fn mean(numbers: &[Amplitude]) -> Amplitude {
	let mut sum:i64 = 0;
	for n in numbers.iter(){
		sum += *n as i64;
	}
    let count = numbers.len() as i64;
    return (sum / count) as Amplitude;
}






pub struct PositionData{
	data: Vec<Vec<Option<Point>>>//L,T
}

impl PositionData{

	pub fn get_point(&self, longitudinal: usize, transverse: usize) -> Option<Point>{
		*self.data.get(longitudinal)?.get(transverse)?
		}



	pub fn add_point(&mut self , longitudinal:usize, transverse:usize , pt:Point){
		self.data[longitudinal][transverse] = Some(pt);
	}



	pub fn new(longitudinal_size: usize , transverse_size: usize ) -> PositionData{
		PositionData{data: vec![vec![None;transverse_size];longitudinal_size]}
	}



	pub fn write_csv(&self, output:&Path) -> Result<(), Box<dyn Error>>{
		let mut wtr = Writer::from_path(output)?;
	//	wtr.write_record(&["longitudinal", "transverse", "x","y"])?;
		for (lon,col) in self.data.iter().enumerate(){
			for (tr,v) in col.iter().enumerate(){
				if let Some(p) = v{
					wtr.serialize(Position{longitudinal:lon,transverse:tr,x:p.x(),y:p.y()})?;
				}
			}
		}
		wtr.flush()?;
		Ok(())
	}
	
	
	
	pub fn geometry_from_cluster(&self , cluster:&Cluster, min_depth:usize , max_depth:usize) -> Result<MultiPolygon<f64>> {
		let mut polygons:Vec<Polygon> = Vec::with_capacity(cluster.regions.len());
		for r in cluster.regions.iter(){
			
			//let mut points:Vec<Point> = r.x_y_between(min_depth,max_depth).into_iter().filter_map(|p| self.get_point(p.0 , p.1 )).collect();//found points.
			let mut points:Vec<Point> = r.data.iter()
			.filter_map(|p| if p.z >= min_depth && p.z <= max_depth{self.get_point(p.x , p.y )} else {None})
			.collect();//found points.


			//add left edge of region to right
			if let Some(right) = cluster.get_region(Region::right(r.key)){
			//	println!("right:{:?}",right);

				
				let mut right_points: Vec<Point> = right.data.iter().filter_map(|p| if (p.z >= min_depth && p.z <= max_depth && p.x == Region::min_x(r.key) + GRID_SIZE) {self.get_point(p.x , p.y )} else {None})
				//let mut right_points: Vec<Point> = right.data.iter().filter_map(|p| if true {self.get_point(p.x , p.y )} else {None})
				.collect();
			//	println!("right_points:{:?}",right_points);
			//	panic!("breakpoint");
				points.append(&mut right_points);
			}
			
			
			//add bottom edge of region above
			if let Some(top) = cluster.get_region(Region::top(r.key)){
				let mut top_points: Vec<Point> = top.data.iter().filter_map(|p| if p.z >= min_depth && p.z <= max_depth &&p.y == Region::min_y(top.key) {self.get_point(p.x , p.y )} else {None})
				.collect();
				points.append(&mut top_points);
			}
			
			//add corner of region to top right
			if let Some(top_right) = cluster.get_region(Region::top_right(r.key)){
				if let Some(top_right_point) = top_right.data.iter().find_map(|p| if p.z >= min_depth && p.z <= max_depth && p.x == Region::min_x(top_right.key) && p.y == Region::min_y(top_right.key) {self.get_point(p.x , p.y )} else {None}){
					points.push(top_right_point);
				}
			}
			
			
			if points.len() > 0{
				polygons.push(MultiPoint::new(points).convex_hull());
			}
		}
		if polygons.len() == 0{
			bail!("no polygons");
		}
		
		return Ok(MultiPolygon::new(polygons));
	}
	
	
	pub fn features_from_clusters(&self , clusters:Vec<Cluster> , size_threshold:usize , layers:Vec<Layer>) -> Vec<ClusterFeature> {
		let mut features: Vec<ClusterFeature> = Vec::new();
		let count = clusters.len();
		for (i,cluster) in clusters.iter().enumerate(){
			if i % 100 == 0{
				println!("extracting features from cluster {:?} of {:?}" , i , count);			
			}

			
			if cluster.volume() >= size_threshold{
				for layer in layers.iter(){
					if let Ok(geom) = self.geometry_from_cluster(cluster,layer.min_depth,layer.max_depth){
						features.push(
							ClusterFeature{
								wkt: geom.wkt_string(),
								depth_band: layer.label.clone(),
								mean_amplitude: mean(&cluster.values_between(layer.min_depth,layer.max_depth)),
							}
						)
					}
				}
			}
			
			if i>100{
				return features;	
			}
			
		}
	
	return features
		
	}
	
	
}



struct GridArea{
		min_x:usize,
		max_x:usize,
		min_y:usize,
		max_y:usize,
		points:Vec<(usize,usize)>
}


/*
fn gridify(points: Vec<(usize,usize)>, grid_size:usize) -> Vec<GridArea>{
	min_x = usize_last_multiple(points.iter.min_by_key(|p| p.0),grid_size);
	max_x = points.iter.max_by_key(|p| p.0);
	min_y = usize_last_multiple(points.iter.min_by_key(|p| p.1),grid_size);
	max_y = points.iter.max_by_key(|p| p.1);

	areas: Vec<GridArea> = Vec::new();
	
	for x in (min_x..=max_x).step_by(grid_size){
		for y in (min_y..=max_y).step_by(grid_size){
			areas.push(GridArea{
				min_x:x,
				max_x:x+grid_size,
				min_y:y,
				max_y:y+grid_size,
				points:Vec::new(),
				}
			)
		}

	}
	
	//for p in points
	return areas;
}
*/



#[cfg(test)]
mod position_data_tests{
	use super::*;
	

	#[test]
	fn add_get_point(){
		let mut data = PositionData::new(5,10);
		let p = Point::new(999.9,111.1);
		data.add_point(2,2,p);
		data.get_point(2,2).unwrap();
	}
	
	
}
