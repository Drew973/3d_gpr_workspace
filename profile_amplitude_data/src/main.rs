use shared::amplitude_data::AmplitudeData;




fn main() {
	
	
	let file = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11805\11805-92 L1 OS.txt";
	
	let r = AmplitudeData::from_text_file(file);
	r.unwrap();
    println!("Ok");
}
