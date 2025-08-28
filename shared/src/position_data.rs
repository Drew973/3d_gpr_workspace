
use std::error::Error;
use geo::{MultiPoint,Point,ConcaveHull};//Polygon,ConvexHull
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::path::Path;
use wkt::ToWkt;
use crate::core::Amplitude;


use crate::clustering::{Cluster};



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
	
	
	pub fn features_from_clusters(&self , clusters:Vec<Cluster> , size_threshold:usize , layers:Vec<Layer>) -> Vec<ClusterFeature> {
		
		let mut features: Vec<ClusterFeature> = Vec::new();
		
		let count = clusters.len();
		for (i,cluster) in clusters.iter().enumerate(){
			
			if i % 100 == 0{
				println!("extracting features from cluster {:?} of {:?}" , i , count);			
			}
			
			if cluster.volume() >= size_threshold{
				
				for layer in layers.iter(){
					let points:Vec<Point> = cluster.x_y_between(layer.min_depth,layer.max_depth).iter().filter_map(|p| self.get_point(p.0,p.1)).collect();
					if points.len() > 0{
						
						features.push(ClusterFeature{
							wkt: MultiPoint::new(points).concave_hull(1.0).wkt_string(),
							depth_band: layer.label.clone(),
							mean_amplitude: mean(&cluster.values_between(layer.min_depth,layer.max_depth)),
							})
					}

				}
			}
	
		}
	return features
		
	}
	
	
}







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
