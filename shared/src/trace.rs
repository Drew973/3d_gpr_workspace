use std::str::FromStr;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use std::fs::File;
use anyhow::{Result,anyhow,bail};
use serde::Serialize;
//use proj::{Proj, Coord};
use crate::core::Amplitude;


pub const MAX_LINES:usize = usize::MAX;//max number of lines to read from file.
//pub const MAX_LINES:usize = 10000;//max number of lines to read from file.



 
#[derive(Serialize,Debug,PartialEq,Hash, Eq, Copy,Clone)]
pub struct XYZ {
	pub x:usize,
	pub y: usize,
	pub z:usize,
}
 
 
 
	
	#[derive(Debug,Clone)]
	pub struct Trace {
		pub transverse: usize,
		pub longitudinal: usize,
		pub proj_x: f64,
		pub proj_y: f64 ,
		pub amplitudes: Vec<Option<Amplitude>>,
	}





	impl Trace{
		
		
		pub fn new(depth:usize) -> Trace{
			Trace{proj_x : 0.0 , proj_y : 0.0 , transverse : 0 , longitudinal : 0 , amplitudes : vec![None;depth]}
		}
		
		
		
		
		pub fn from_line(line: String , transverse:usize , longitudinal:usize , samples:usize) -> Result<Trace> {
			
			let mut r = Trace{proj_x : 0.0 , proj_y : 0.0 , transverse : transverse , longitudinal : longitudinal , amplitudes : vec![None; samples]};
			
			for (i, p) in line.split("\t").enumerate(){
				match i {
					
					0 => {
							r.proj_x = p.parse::<f64>()?;
						}
						
					1 => {
							r.proj_y = p.parse::<f64>()?;
						}	
						
					_other => {
						if i <= samples+2{
							r.amplitudes[i-2] = Some(p.parse::<Amplitude>()?);
						}
						else{
							bail!("Too many columns");
						}
					}
				}
			}
			// middle sensor seems faulty. return trace with valid geom but null amplitudes.
			if transverse == 12{
				r.amplitudes = vec![None; samples];
			}
			Ok(r)
		}		
				
		
	}



	//parsing

	//parse line like:
	//#Volume: X-lines=11019, In-lines=25, Samples=255
	//(x_lines,in_lines,samples)
	fn size_from_line(line:String) -> Result<(usize,usize,usize)>{
		//x
		let x_expr = Regex::new(r"X-lines=(?<x>\d+)($|\D)").unwrap();
		let Some(caps) = x_expr.captures(&line) else {
			 anyhow::bail!("no x size(X-lines)");
			};
		let x_lines = usize::from_str(&caps["x"])?;
		
		//y
		let y_expr = Regex::new(r"In-lines=(?<y>\d+)($|\D)").unwrap();
		let Some(y_caps) = y_expr.captures(&line) else {
			 anyhow::bail!("no y size(In-lines)");

			};
		let in_lines = usize::from_str(&y_caps["y"])?;
		
		//z
		let z_expr = Regex::new(r"Samples=(?<z>\d+)($|\D)").unwrap();
		let Some(z_caps) = z_expr.captures(&line) else {
			 anyhow::bail!("no z size (Samples)");

			};
		let samples = usize::from_str(&z_caps["z"])?;

		
		return Ok((x_lines,in_lines,samples));

	}
		





	#[derive(Debug)]
	pub struct TraceParser
		{
			lines: std::io::Lines<BufReader<File>>,
			current: usize,
			start_line: usize,//1st line that sucessfully parses into trace
			pub x_lines: usize,
			pub in_lines: usize,
			pub samples: usize,
		}



	impl TraceParser {
		
		
		//data starts on start line
		pub fn new(filename : &str) -> Result<TraceParser>{
			let f = File::open(filename)?;
			
			let mut lines = BufReader::new(f).lines();		
			
			//abort quickly if don't have sizes in 1st 100 lines instead of slowly searching every line.
			for i in 0..= 100{
				if let Some(res) = lines.next(){
					if let Ok(line) = res{
						if let Ok(s) = size_from_line(line){
							return Ok(TraceParser {lines: lines, current:i , start_line:0 , x_lines:s.0 , in_lines:s.1 , samples:s.2});
						}
					}
				}
			}
			
			Err(anyhow!("Check file. Could not find sizes(line like '#Volume: X-lines=11019, In-lines=25, Samples=255' within 1st 100 lines."))
						
		}
		

	}

	impl Iterator for TraceParser {
		type Item = Result<Trace>;
		
		fn next(&mut self) -> Option<Result<Trace>> {
			
			if self.current>MAX_LINES{
				return None
			} 
			
			
			if self.start_line ==0{
				for _ in self.current..= MAX_LINES{
					if let Ok(line) = self.lines.next()?{
						self.current += 1;
						if let Ok(t) = Trace::from_line(line , 0 , 0 , self.samples){
							self.start_line = self.current;
							return Some(Ok(t));
						}
					}
				}
			return None;
			}
			

			if let Ok(line) = self.lines.next()?{	//		self.lines.next() gives	Option<Result<String, std::io::Error> >
				self.current += 1;
				
				let tr: usize = (self.current - self.start_line) % self.x_lines;
				let lon: usize = (self.current - self.start_line) / self.x_lines;
				return Some(Trace::from_line(line , lon , tr , self.samples));
			}
			
			return None;
		}
		
		
		
		

		
		
		
		
		
	}
	
	
	
	
	
	
	
	
	

#[cfg(test)]
mod trace_tests {

	use super::*;


	#[test]
	fn test_trace_from_line(){
		let row = "278816.194034936	678562.94960174209	-32694	-32678	-32623	-32540	-32534	-32629	-32563	-32395	-32401	-32659	-32473	-32166	-32518	-30691	-25564	-16950	-5370	7306	18134	23940	22479	13438	-1192	-17638	-31272	-25013	-22286	-23186	-24906	-28757	-26701	-11274	5997	20596	28844	29143	22293	10919	-1630	-12436	-19832	-23775	-25579	-26727	-28043	-29632	-30594	-29952	-29167	-29105	-29666	-30431	-30486	-30099	-29852	-29612	-28925	-27873	-27008	-26817	-27521	-29061	-31149	-32064	-30112	-28720	-28112	-28328	-29260	-30676	-32074	-31298	-29976	-28951	-28270	-27964	-28128	-28868	-30146	-31649	-32085	-31593	-32185	-31716	-29398	-26955	-25035	-24106	-24336	-25574	-27388	-29064	-29678	-29251	-28738	-28407	-28003	-27381	-26714	-26169	-25719	-25323	-25151	-25462	-26314	-27379	-28053	-28065	-27854	-27871	-28149	-28460	-28627	-28719	-28930	-29363	-29980	-30648	-31174	-31218	-30638	-29827	-29189	-28984	-29274	-29865	-30298	-30169	-29625	-28897	-27965	-26883	-25954	-25556	-25805	-26227	-25834	-24366	-22744	-21887	-22198	-23630	-25900	-28632	-30879	-29537	-27077	-25235	-24206	-23725	-23478	-23571	-24457	-26420	-29258	-32333	-30641	-29165	-28877	-29418	-29612	-29979	-30218	-30136	-29705	-29111	-28631	-28488	-28781	-29475	-30420	-31357	-31734	-31231	-30501	-29774	-29065	-28418	-27956	-27840	-28191	-28960	-29762	-29837	-29115	-28412	-28235	-28649	-29420	-30193	-30759	-31246	-31855	-32299	-31647	-30841	-30268	-30075	-30295	-30711	-30683	-29886	-28880	-28139	-27879	-28150	-28891	-29963	-31172	-32300	-32328	-31805	-31600	-31602	-31677	-31729	-31647	-31325	-30801	-30191	-29578	-28989	-28423	-27901	-27487	-27294	-27455	-28089	-29270	-30989	-32135	-29735	-27187	-24903	-23208	-22347	-22426	-23378	-24979	-26918	-27852	-29051	-30409	-30838	-29642	-28365	-27782	-28070	-28899	-29341	-28721	-27729	-28750	-29950	-28943".to_string();
		let r = Trace::from_line(row,0,0,255);
		//println!("r:{:?}",r);
	}



	#[test]
	fn test_trace_parser(){
		let f = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\M80\11870-05 L1 OS.txt";
		let p = TraceParser::new(f);
		//println!("r:{:?}",r);
	}



	#[test]
	fn test_size_from_line(){
		let r = size_from_line("#Volume: X-lines=11019, In-lines=25, Samples=255".to_string()).unwrap();
		assert_eq!(r, (11019,25,255));
	}
		
	
}



/*	fn upload(file:String) -> Result<(), Box<dyn std::error::Error>>{
		
		let mut client = Client::connect("postgresql://postgres:pts21@localhost/3d_gpr", NoTls)?;
		
		//let mut transaction = client.transaction()?;

		
		client.simple_query("delete from amplitude")?;
		let amplitude_insert = client.prepare("INSERT INTO amplitude (x , y, z , a) VALUES ($1, $2, unnest($3::int[]), unnest($4::int[]))")?;

		client.simple_query("delete from location")?;
		let location_insert = client.prepare("INSERT INTO location (x , y, pt) VALUES ($1, $2,St_make_point($3,$4))")?;

		
		
		let cp = TraceParser::new(file)?;
		for (i,row) in cp.enumerate(){
			println!("row {:#?}" , i);
			if row.is_some(){
				let r = row.unwrap();
				let mut amplitudes: Vec<Amplitude> = Vec::new();
				let mut z_values: Vec<Amplitude> = Vec::new();
				
				//println!("r {:#?}" , r)
				for (z,a) in r.amplitudes.iter().enumerate(){
					if a.is_some(){
						amplitudes.push(a.unwrap());
						z_values.push(z);// i32 <-> postgres int

					}
				}
				client.query(&amplitude_insert,&[&r.x, &(r.y as i32), &z_values, &amplitudes],)?;// i32 <-> postgres int
				client.query(&location_insert,&[&r.x, &(r.y as i32), &r.proj_x, &r.proj_y],)?;// i32 <-> postgres int

			}
		
		}
		 Ok(())
	}


	*/