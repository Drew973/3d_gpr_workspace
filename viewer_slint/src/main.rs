// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use rfd::FileDialog;//rust file dialog
use slint::SharedString;

use shared::amplitude_data::AmplitudeData;

slint::include_modules!();


//slint::slint! {
 //   export { AppWindow } from "app-window.slint";
//}



fn select_file() -> String{
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


struct AppData{
	amplitudes: AmplitudeData,
	longitudinal_position: usize
}


impl AppData{
	
	
	fn load_file(&mut self , file:&str){
		//self.amplitudes = AmplitudeData::from_text_file(file).unwrap_or(AmplitudeData::from_size(0,0,0))
		
		let data = AmplitudeData::from_text_file(file);
		if data.is_ok(){
			self.amplitudes = data.unwrap();
		}
		else{
			self.amplitudes = AmplitudeData::from_size(0,0,0);
			println!("error{:#?}",data);
		}
	}
	
	
}





fn main() -> Result<(), Box<dyn Error>> {
	
	let mut app_data = AppData{
		amplitudes: AmplitudeData::from_size(0,0,0),
		longitudinal_position:0,
		};
	
    let ui = AppWindow::new()?;

/*
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });
	*/
	
	
	ui.on_longitudinal_position_changed(move |pos| {
			println!("pos{:#?}",pos);
			app_data.longitudinal_position = pos as usize;
    });
		
	
	
	
	ui.on_select_file(
	{
		let ui_handle = ui.as_weak();
		let ui_h = ui_handle.unwrap();

		move || {
		let file:String = select_file();
		if file.len() > 0{
			let r = AmplitudeData::from_text_file(&file);
			if r.is_ok(){
				app_data.amplitudes = r.unwrap();
				ui_h.set_file(file.into());
			}
			else{
				app_data.amplitudes = AmplitudeData::from_size(0,0,0);
				ui_h.set_file(r.unwrap_err().to_string().into());
			}
			
			ui_h.set_data_size_l(app_data.amplitudes.longitudinal_size as i32);
			ui_h.set_data_size_t(app_data.amplitudes.transverse_size as i32);
			ui_h.set_data_size_d(app_data.amplitudes.depth_size as i32);

		}
		}
	}
	);


    ui.run()?;

    Ok(())
}
