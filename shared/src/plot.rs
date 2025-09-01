


use plotters::prelude::*;
use colorgrad::{Gradient,LinearGradient};
use std::error::Error;
use crate::core::Amplitude;
use ndarray::{Array2, Array};





fn make_rectangle(gradient: &LinearGradient, x: usize , x_size:f64 , y: usize ,y_size:f64, value:i16) -> Rectangle<(f64, f64)> {
	let c = gradient.at(value as f32).to_rgba8();
	let color = RGBColor(c[0] , c[1] , c[2]);
	println!("color:{:?}",color);

	return Rectangle::new([(x as f64 * x_size, y as f64 * y_size), ((x + 1) as f64 * x_size, (y + 1) as f64 *y_size)] , color.filled());
}



/*
data for plotting heatmap.


x and y refer to plot coordinates - not to coordinates within amplitude data

*/

pub struct PlotData{
	pub min_x_index: usize,
	pub min_y_index: usize,
	pub marker_x: usize,
	pub marker_y: usize,
	pub x_scale:f64, //meters per index
	pub y_scale:f64, //meters per index
	pub x_label: String,
	pub y_label: String,
//	pub amplitudes: Vec<Vec<Option<i16>>>,  // row of col
	pub amplitudes: Array2<Option<Amplitude>>
	
}



const PIXELS: u32 = 1024;
const LINES: u32 = 768;






impl PlotData{

		
		
	fn plot(&self , file: &str) -> Result<(), Box<dyn Error>>{
		let min_x_val = self.min_x_index as f64 * self.x_scale;
		let min_y_val = self.min_y_index as f64 * self.y_scale;
		let max_x_val = (self.min_x_index+self.amplitudes.shape()[0]) as f64 * self.x_scale;
		let max_y_val = (self.min_y_index+self.amplitudes.shape()[1]) as f64 * self.y_scale;

		let gradient = colorgrad::GradientBuilder::new()
		.html_colors(&["red", "blue"])
		.domain(&[i16::MIN as f32, i16::MAX as f32])
		.build::<colorgrad::LinearGradient>()?;
		
		let root_drawing_area = SVGBackend::new(file, (PIXELS, LINES))
		.into_drawing_area();
		
		root_drawing_area.fill(&WHITE)?;

		let mut chart = ChartBuilder::on(&root_drawing_area)
			.margin(5)
			.x_label_area_size(40)
			.y_label_area_size(60)
			.build_cartesian_2d(min_x_val .. max_x_val, max_y_val .. min_y_val as f64)?;

		chart
		.configure_mesh()
		.y_desc(self.y_label.clone())
		.x_desc(self.x_label.clone())
		.disable_x_mesh()
		.disable_y_mesh()
		.label_style(("sans-serif", 20))
		.draw()?;
		
		let mut rects: Vec<Rectangle<(f64, f64)>> = Vec::new();
		
		for (ind,a) in self.amplitudes.indexed_iter(){
				if let Some(v) = a{
					let c = gradient.at(*v as f32).to_rgba8();
					let color = RGBColor(c[0] , c[1] , c[2]);
					let x = ind.0;
					let y = ind.1;
					rects.push(
						Rectangle::new([(min_x_val + self.x_scale * x as f64, min_y_val + self.y_scale * y as f64), (min_x_val + self.x_scale * (x+1) as f64, min_y_val + self.y_scale * (y+1) as f64)] ,
						color.filled())
					);
				}
		}
		
		/*
		
		for (y,row) in self.amplitudes.iter().enumerate(){
			for (x,a) in row.iter().enumerate(){
				if let Some(v) = a{
					let c = gradient.at(*v as f32).to_rgba8();
					let color = RGBColor(c[0] , c[1] , c[2]);
					
					rects.push(
						Rectangle::new([(min_x_val + self.x_scale * x as f64, min_y_val + self.y_scale * y as f64), (min_x_val + self.x_scale * (x+1) as f64, min_y_val + self.y_scale * (y+1) as f64)] ,
						color.filled())
					);
				}
			}
		}
		*/
		chart.draw_series(rects)?;
		root_drawing_area.present()?;
		return Ok(());
			
	}
	
}





#[cfg(test)]
mod plot_tests{
	use super::*;
	const X_SPACING: f64 = 0.072;
	const Y_SPACING: f64 = 0.0762;
	const Z_SPACING:f64 = 0.01309289678;// TIME_INTERVAL*C/EPSILON.sqrt();



	#[test]
	fn test_plot(){
		const image: &str = r"C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\outputs\test_plot.svg";
	
		let pd = PlotData{min_x_index: 6,
		min_y_index: 2,
		marker_x:8,
		marker_y:3,
		x_scale:Y_SPACING, //meters per index
		y_scale:Z_SPACING, //meters per index
		x_label: "Transverse(m)".to_string(),
		y_label: "Depth(m)".to_string(),
		amplitudes: vec![vec![Some(0),Some(1000),Some(2000),Some(3000)],vec![Some(3000),Some(4000),Some(5000),None]],
		};
		
		let _ = pd.plot(image);

		
	}
}