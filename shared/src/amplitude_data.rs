
//use std::io;

use serde::{Serialize};

//use std::cmp::min;

//use anyhow::{Result,anyhow};
use geo::{Polygon,MultiPoint,Point,ConvexHull,ConcaveHull};
//use shapefile::{dbase,Polygon as ShapefilePolygon};
use csv::Writer;
use wkt::ToWkt;
use std::error::Error;
use std::collections::{HashSet,HashMap};

use crate::trace::{Trace,XYZ,TraceParser};
use crate::clustering::{Clusterer , Cluster};
use std::fmt;
use std::cmp::min;
use crate::core::Amplitude;
use ndarray::{Array3, Array};



//const C:f64 = 299792458.0;//speed of light
//const TIME_INTERVAL:f64 = 9.765625E-11;
//const EPSILON:f64 = 5.0;//depends on material a little.


//distance between elements in meters.
const X_SPACING: f64 = 0.072;
const Y_SPACING: f64 = 0.0762;
pub const Z_SPACING:f64 = 0.01309289678;// TIME_INTERVAL*C/EPSILON.sqrt();







/*
pub struct element_iterator{
	pub source: AmplitudeData,
	pub filter: fn(&AmplitudeData,usize,usize,usize)-> bool,
	pub x:usize,
	pub y:usize,
	pub z:usize
	
}


impl element_iterator{
	
}
*/



#[allow(dead_code)]
pub struct AmplitudeData{
	pub amplitudes: Array3<Option<Amplitude>>,
	pub longitudinal_size: usize,
	pub transverse_size: usize,
	pub depth_size: usize,
	pub points:Vec<Vec<Option<Point>>>,

}



impl fmt::Debug for AmplitudeData {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AmplitudeData. longitudinal_size:{},transverse_size:{},depth_size:{}", self.longitudinal_size, self.transverse_size,self.depth_size)
    }
}

impl AmplitudeData{
	
	
	fn set_value(&mut self, longitudinal:usize, transverse:usize, depth:usize, value:Option<Amplitude>){
		self.amplitudes[(longitudinal,transverse,depth)] = value;
	}
	
	
	fn value(&self, longitudinal:usize, transverse:usize, depth:usize) -> Option<Amplitude>{
		self.amplitudes[(longitudinal,transverse,depth)]
	}
	
	
	pub fn from_size(longitudinal:usize , transverse:usize , depth:usize ) -> AmplitudeData{
		return AmplitudeData{amplitudes:Array::from_elem((longitudinal, transverse, depth), None), //L,T,D
		longitudinal_size:longitudinal,
		transverse_size:transverse,
		depth_size:depth
		,points:vec![vec![None;transverse];longitudinal]
		}
		//return AmplitudeData{transverse:vec![Transverse::new(transverse,depth);longitudinal],longitudinal_size:longitudinal,transverse_size:transverse,depth_size:depth};
	}
	
	


	//pub fn set_column(&mut self , longitudinal:usize, transverse:usize , amplitudes:Vec<Option<Amplitude>>){
	//	self.amplitudes[longitudinal][transverse] = amplitudes;
	//}

	
	pub fn from_text_file(filename:&str) -> Result<AmplitudeData, Box<dyn Error>>{
	
		let parser: TraceParser = TraceParser::new(filename)?;
		
		//println!("size:{:?}",parser.size);// x: 25, y: 1310, z: 255 }

		let mut d = AmplitudeData::from_size(parser.x_lines , parser.in_lines , parser.samples);

	
		for row in parser{
			if let Ok(trace) = row{
				for(i,v) in trace.amplitudes.into_iter().enumerate(){
					d.set_value(trace.longitudinal, trace.transverse, i, v);
				}
				//d.amplitudes.column_mut((trace.longitudinal,trace.transverse));
				
				d.points[trace.longitudinal][trace.transverse] = Some(Point::new(trace.proj_x, trace.proj_y));
				}
		}
		
		
		return Ok(d);
	}
	
	

		/*
	//returns None where out of bounds
	pub fn value(&self , longitudinal:usize , transverse: usize , depth: usize) -> Option<Amplitude>{
		return * self.amplitudes.get(longitudinal)?.get(transverse)?.get(depth)?;
		}
	
	*/

	

	
	
	
	
	
	
}







#[cfg(test)]
mod amplitude_data_tests{
	use super::*;
	
	//const F:&str = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\Test_Metric_Real.txt";
	const F:&str = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60\11820-60 L2 C.txt";

	//cargo flamegraph --unit-test -- test_from_text
//cargo flamegraph --unit-test -- amplitude_data_::amplitude_data_tests::test_from_text

//cargo flamegraph --unit-test -- tests::test_from_text
//cargo flamegraph --unit-test -- amplitude_data::test_from_text
//cargo flamegraph --release --test "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\with_slint\target\release\slint-rust-template.exe"


	
	#[test]
	fn test_from_text(){
		let data = AmplitudeData::from_text_file(F).unwrap();
		let v = data.value(0,0,0).expect("Should have value for 0,0,0");
		assert_eq!(v,-32754,"first amplitude in file is -32754")
	}
	

	
	/*
	#[test]
	fn test_point(){
		let f = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\Test_Metric_Real.txt";
		let data = AmplitudeData::from_text_file(F).unwrap();
		for t in 0..data.transverse_size{
			
			if t!=12{
				for lon in 0..data.longitudinal_size{
					let pt = data.get_point(lon,t).unwrap_or_else(|| panic!("Point not found for L:{lon} T{t}"));
					assert!(pt.x() > 10.0 , "x <10 for L:{lon} T{t}");
					assert!(pt.y() > 10.0 , "y <10 for L:{lon} T{t}");
				}
			}
		}
		
	}	
	*/
	
	

	
	
}





	
	

//fn clusters_by_density  -> Vec<Cluster>

//fn features_from_clusters(clusters,zero_depth,thicknesses) -> Vec<Feature>

//fn write_csv(features:Vec<Feature>)


//
