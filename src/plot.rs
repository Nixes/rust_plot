//use graphics::{ self, Graphics, ImageSize };
use graphics::*;

pub struct Plot {
    line_values: Vec<u32>,
    line_max_samples: usize, // number of samples to show before dropping values from the beginning
    line_height_scale_factor:f64, // how much to multiply the sample value by to produce the number of pixels high the line is
    line_gap:u16,
    line_thickness:f64
}

impl Plot {
    // takes a usize that represents the number of samples that should be maintained, and a u32 indicating the width of the graph in pixels
    pub fn new(max_samples:usize,window_width:u32) -> Plot {
        // calculate the line thickness based on the size of window to fill and the max sample number
        let line_gap :u16 = 0;
        let line_thickness:f64 = (window_width as f64 / max_samples as f64) - line_gap as f64;
        println!("Line thickness calced: {}",line_thickness);
        Plot{line_values: Vec::with_capacity(max_samples),line_max_samples:max_samples,line_height_scale_factor:1.0,line_gap:line_gap,line_thickness:line_thickness }
    }
    // very similar to a HSV to RGB conversion
    fn get_color(value: u32,max_value: u32) -> [f32; 4] { // used for heatmap colors
        let mut color = [0.0, 0.5, 0.0, 1.0];
        if value >= 1000 { // invalid result (connection lost?)
            color = [0.0, 0.0, 0.0, 0.5];
        } else if value >= max_value {
            color = [1.0, 0.0, 0.0, 1.0];
        }
        // normalise the input value
        let norm_value :f32 = value as f32 / max_value as f32;

        if norm_value <= 0.25 {
            color = [0.0, norm_value*4.0, 1.0, 1.0]; // Cyan.
        } else if norm_value > 0.25 && norm_value <= 0.5 {
            color = [0.0, 1.0, 1.0-((norm_value-0.25)*4.0), 1.0]; // Green.
        } else if norm_value > 0.5 && norm_value <= 0.75 {
            color = [(norm_value-0.5)*4.0, 1.0, 0.0, 1.0]; // Yellow.
        } else if norm_value > 0.75 && norm_value <= 1.0 {
            color = [1.0, 1.0-((norm_value-0.75)*4.0), 0.0, 1.0]; // Red.
        }
        color
    }
    pub fn add_sample (&mut self,sample:u32) {
        if self.line_values.len() >= self.line_max_samples {
            self.line_values.remove(0);// remove oldest value / value in first index
            self.line_values.push(sample); // then add latest value to end
        } else {
            self.line_values.push(sample); // not yet filled so just add value
        }
    }
    pub fn gen_test(&mut self,min:u32,max:u32) {
        let step_amount: u32 = (max - min) / self.line_max_samples as u32;

        for i in 0..self.line_max_samples as u32 {
            self.add_sample(i * step_amount);
            println!("Gen sample: {}",i * step_amount);
        }
    }
    pub fn draw <G: Graphics>(&mut self, context: Context, graphics: &mut G) {
        for (item_no,&item) in self.line_values.iter().enumerate() {
            //println!("One bar drawn value: {} number: {}",item,itemNo);
            let color = Plot::get_color(item,200);
            rectangle(color,[item_no as f64 * (self.line_thickness + self.line_gap as f64),
                                            300.0,
                                            self.line_thickness,
                                            item as f64 * self.line_height_scale_factor * -1.0]
                                            /* x,y,width,height */
                                            ,context.transform, graphics);
        }
    }
}
