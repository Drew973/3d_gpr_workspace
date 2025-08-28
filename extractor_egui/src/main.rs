
#![windows_subsystem = "windows"]//prevents console from opening



// Import eframe (egui's platform integration library)
use eframe::egui;
use rfd::FileDialog;//rust file dialog
use std::env;
use std::path::PathBuf;
use std::process::{Command};


const EXE:&str = "cli.exe";




fn select_input_file() -> String{
	let files = FileDialog::new()
		.add_filter("text", &["txt"])
		 //   .set_directory("/")
		.pick_file();
		if let Some(f) = files{
			let new_file = f.display().to_string();
			return new_file.into();
		}
	return "".into();
}


fn select_output_file() -> String{
	let file = FileDialog::new()
		.add_filter("CSV", &["csv"])
		.save_file();
		if let Some(f) = file{
			let new_file = f.display().to_string();
			return new_file.into();
		}
	return "".into();
}


// Entry point for the application
fn main() -> Result<(), eframe::Error> {
	let title = format!("GUI for {EXE}");
    // Create and run the egui app
    eframe::run_native(
        &title,                
        eframe::NativeOptions::default(), // Window configuration
        Box::new(|_| Ok(Box::new(MyApp::default()))), // Initialize the app
    )
}

// Define the application struct
struct MyApp {
    input_file: String,
	output_file: String,
    amplitude_threshold: i16,
	size_threshold:usize,
	max_gap: usize,
}

// Default state for the application
impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_file: String::from("input file.txt"),
			output_file: String::from("output file.csv"),
			amplitude_threshold: 10000,
			size_threshold:50,
			max_gap: 5,
        }
    }
}

// Define GUI logic for the app
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Create the GUI layout
        egui::CentralPanel::default().show(ctx, |ui| {

			ui.horizontal(|ui| {
				ui.label("Input file:");
				ui.text_edit_singleline(&mut self.input_file);
				if ui.button("Select...").clicked() {
					self.input_file = select_input_file();
				}
			});

			ui.horizontal(|ui| {
				ui.label("Output file:");
				ui.text_edit_singleline(&mut self.output_file);
				if ui.button("Select...").clicked() {
					self.output_file = select_output_file();
				}
			});

			ui.add(egui::Slider::new(&mut self.amplitude_threshold, i16::MIN..=i16::MAX).text("Output amplitudes > this."));
			ui.add(egui::Slider::new(&mut self.max_gap, 1..=50).text("Maximum gap for clustering. Larger means clusters more likely to merge."));
			ui.add(egui::Slider::new(&mut self.size_threshold, 0..=1000).text("Output clusters with > this many samples"));
		
			let mut path = env::current_dir().unwrap_or(PathBuf::from(EXE));
			path.push(EXE);
			
			ui.add(egui::Label::new("Command:"));

		
			let command = format!("\"{}\" --input \"{}\" --output \"{}\" --amplitude-threshold {} --size-threshold {} --max-gap {} --pause" ,
			path.display(),
			self.input_file ,
			self.output_file,
			self.amplitude_threshold,
			self.size_threshold,
			self.max_gap,
			);
			ui.add(egui::Label::new(&command));
			
			if ui.button("Run (open new terminal)").clicked() {
				//self.progress = Some(0.0);
				//start_task();
				//open_terminal(&command);
				
				//let p = Popen::create(&[path.display(),self.input_file.into(),self.output_file.into()],PopenConfig::default());
				let child = Command::new(&path).arg("--input").arg(&self.input_file).arg("--output").arg(&self.output_file).arg("--pause").spawn();
				println!("path:{:?}",&path);

				println!("child:{:?}",child);
				
			}
			
			
			
        });
    }
}




