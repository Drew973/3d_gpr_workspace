// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use rfd::FileDialog;//rust file dialog
use slint::SharedString;

use shared::amplitude_data::AmplitudeData;
use shared::core::{usize_subtract};

use viewer_slint::plot_slint::{plot_slint,plot_empty};
use std::cmp::min;
use std::cell::RefCell;
use std::rc::Rc;


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
	let app_data = Rc::new(RefCell::new(AppData{
		amplitudes: AmplitudeData::from_size(0,0,0),
		longitudinal_position:0,
		}));
	
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
	//(longitudinal:int , transverse: int , transverse_size:int , depth: int ,depth_size:int)
	let state_copy = app_data.clone();
	ui.on_plot_transverse(move |lon, trans, trans_size, depth, depth_size,width,height| {
		let data = state_copy.borrow();
		if let Ok(pd) = data.amplitudes.transverse_plot_data(lon as usize ,trans as usize, trans_size as usize, depth as usize, depth_size as usize){
			return plot_slint(&pd, width as u32, height as u32).unwrap();
		}
		else{
			return plot_empty().unwrap();
		}
    });
		
		
		
		//transverse_position_changed
	
	//longitudinal:usize , min_depth: usize , max_depth: usize , min_transverse:usize , max_transverse:usize
	
	
	let state_copy = app_data.clone();

	ui.on_select_file(
	{
		let ui_handle = ui.as_weak();
		let ui_h = ui_handle.unwrap();

		move || {
		let mut data = state_copy.borrow_mut();
		let file:String = select_file();
		if file.len() > 0{
			let r = AmplitudeData::from_text_file(&file);
			if r.is_ok(){
				data.amplitudes = r.unwrap();
				ui_h.set_file(file.into());
			}
			else{
				data.amplitudes = AmplitudeData::from_size(0,0,0);
				ui_h.set_file(r.unwrap_err().to_string().into());
			}
			
			ui_h.set_data_size_l(data.amplitudes.longitudinal_size as i32);
			ui_h.set_data_size_t(data.amplitudes.transverse_size as i32);
			ui_h.set_data_size_d(data.amplitudes.depth_size as i32);

		}
		}
	}
	);


    ui.run()?;

    Ok(())
}
