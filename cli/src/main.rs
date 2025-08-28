

use std::path::Path;
use std::fs::File;
use clap::Parser;
use shared::trace::{TraceParser,MAX_LINES};
use shared::position_data::{PositionData,ClusterFeature,Layer};
use shared::clustering::Clusterer;
use shared::core::Amplitude;
use serde::{Serialize};
use geo::Point;
//use serde_json;
use anyhow::{Context,bail};

use std::io::{stdin, stdout, Read, Write};



fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}






//const empty_path:Utf8PathBuf = Utf8PathBuf.from_str("").unwrap();

#[command(version, about, long_about = "Reads text file exported by examiner. Finds clusters of amplitudes > amplitude-threshold.
filters out clusters containing < size-threshold amplitudes. 
for depth ranges (0-50mm...450-500mm) gets geometry from parts of clusters within range.
Outputs csv with WKT geometry and label for depth range.
Also outputs points csv to [input]_points.csv

The tool makes a list of clusters (a cluster is a group of connected samples) from all samples >= amplitude threshold.
Samples count as connected when their x and y and z coordinates are within max_gap (5 sample units) of each other.

for each cluster containing >= size_threshold(50) samples
	it finds parts of the cluster within each depth range and outputs geometry and mean amplitude.




Parameters:
	--amplitude-threshold: Integer from -32768 to 32768
	--output-params: JSON file to write these arguments to for future reference.
	--max-gap: Integer >=1 . Higher max_gap makes clusters more likely to merge.
	Points considered connected if x,y and z within max_gap sample units of each other.
	--size-threshold: Integer >=0 .Ignore clusters containing < size-threshold samples.
	--pause: pause after running.
"
)]
#[derive(Debug , Parser , Serialize)]
struct Args {
    #[arg(short,long)]
    input: String,
	
	#[arg(short,long)]
    output: String,

	#[arg(long, default_value_t = String::from(""))]
    output_params: String,
	//output_params: Option<Utf8PathBuf>,
	
	#[arg(long, default_value_t = 10000)]
    amplitude_threshold: Amplitude,
	
	#[arg(long, default_value_t = 50)]
    size_threshold: usize,
	
	
	#[arg(long, default_value_t = 5)]
    max_gap: usize,
	
	#[arg(long, short, action)]
	pause: bool,
	
	
	#[arg(long, default_value_t = String::from("[{\"min_depth\":51,\"max_depth\":53,\"label\":\"0-50mm\"},
	{\"min_depth\":54,\"max_depth\":57,\"label\":\"50-100mm\"},
	{\"min_depth\":58,\"max_depth\":61,\"label\":\"100-150mm\"},
	{\"min_depth\":62,\"max_depth\":65,\"label\":\"150-200mm\"},
	{\"min_depth\":66,\"max_depth\":69,\"label\":\"200-250mm\"},
	{\"min_depth\":70,\"max_depth\":73,\"label\":\"250-300mm\"},
	{\"min_depth\":74,\"max_depth\":77,\"label\":\"300-350mm\"},
	{\"min_depth\":82,\"max_depth\":85,\"label\":\"350-400mm\"},
	{\"min_depth\":86,\"max_depth\":89,\"label\":\"400-450mm\"},
	{\"min_depth\":90,\"max_depth\":93,\"label\":\"450-500mm\"}]
	"))]
	layers: String,
	
	
}
//params can be JSON String or filePath




impl Args{



	fn extract_high_amplitudes(&self) -> anyhow::Result<()>{
		
		let layers: Vec<Layer> = serde_json::from_str(&self.layers).with_context(|| "Invalid layers")?;
		
		let mut clusterer = Clusterer::new(self.max_gap,self.max_gap,self.max_gap);
		
		//error if output exists. user try to overwrite input or .exe
		if Path::new(&self.output).exists(){
			 bail!("Output file '{0}' already exists", self.output);
		}

		let max_depth:usize = layers.iter().max_by_key(|layer| layer.max_depth).with_context(|| "No layers ")?.max_depth;
		let min_depth:usize = layers.iter().min_by_key(|layer| layer.min_depth).with_context(|| "No layers ")?.min_depth;


		let res = TraceParser::new(&self.input);
		if !res.is_ok(){
			println!("Error reading \"{}\"", self.input);
			return Err(res.err().unwrap().into());
		}
			
		let parser: TraceParser = res.unwrap();
		
		let mut positions = PositionData::new(parser.x_lines , parser.in_lines);
		
		let total:f32 = std::cmp::min(parser.x_lines,MAX_LINES) as f32 * parser.in_lines as f32;
		
		for (i,t) in parser.enumerate(){
			if i % 5000 ==0 {
				println!("Reading input and clustering:{:.2?}%",100.0 * i as f32 / total);			
			}
			if let Ok(tc) = t{
				positions.add_point(tc.longitudinal,tc.transverse , Point::new(tc.proj_x,tc.proj_y));
				
				for i in min_depth..=max_depth{
					if let Some(opt) = tc.amplitudes.get(i){
						if let Some(amplitude) = opt{
							if *amplitude > self.amplitude_threshold{
								clusterer.add_point(tc.longitudinal.into(), tc.transverse.into(), i.into(), *amplitude)
							} 
						}
					}
				}
			}
		}


		let _ = ClusterFeature::write_csv(
			positions.features_from_clusters(clusterer.clusters(),self.size_threshold.into(),layers)
			,&self.output
		);

		//write points to csv
		let input = Path::new(&self.input);
		let file_name = format!("{}_points",input.file_stem().unwrap().to_os_string().into_string().unwrap());
		let geom_csv = input.with_file_name(file_name).with_extension("csv");
		let points_write_result = positions.write_csv(Path::new(&geom_csv));
		if points_write_result.is_err(){
			println!("Error writing points csv:{:?}. Continuing",points_write_result);			
		}
		
		//write params to json
		if self.output_params != ""{
			let mut file = File::create(&self.output_params)?;
			let json = serde_json::to_string(self)?;
			file.write_all(json.as_bytes())?;
		}

		return Ok(());
		
	}
}



fn main() {
	println!("Starting...");
    let args = Args::parse();	
	let result = args.extract_high_amplitudes();
	println!("{:?}",result);
	if args.pause{
		pause();
	}
	
}




#[cfg(test)]
mod main_tests{
	
	
	const SMALL_INPUT: &str = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\Test_Metric_Real.txt";
	//let input = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60 L2 C.txt";

	
	//use super::*;
	/*
	#[test]
	fn test_export_poi_grid(){
		let data = AmplitudeData::from_text_file(SMALL_INPUT).unwrap();
		let size = XYZ{x:5,y:5,z:5};
		let step = XYZ{x:1,y:1,z:1};
		let output = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\Test_Metric_Real_og_grid.csv";
		let _ = data.export_poi_grid(output,size,step,0.2,25000);	
	}
	*/

	
	//#[test]
	//fn test_extract_high_amplitudes(){
	//	let output = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\Test_Metric_Real_clusters.csv";
	//	extract_high_amplitudes(SMALL_INPUT,output,25000,50).expect("");
	//}
	
	
	/*
		#[test]
	fn test_extract_high_amplitude_cuboid(){
		let output = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\Test_Metric_Real_cuboid.csv";
		extract_high_amplitude_cuboid(SMALL_INPUT,output,25000,0.2).expect("");
	}
	
	*/
}