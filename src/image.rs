use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::color::Color;
use std::process::Command;

pub struct Image{
    pub screen: Vec<Vec<Color>>,
    pub height: usize,
    pub width: usize,
}

impl Image{
    pub fn new(image_width: usize, image_height: usize) -> Image{
        Image{screen: vec![vec![Color::new(); image_width]; image_height], width: image_width, height: image_height}
    }

    pub fn plot(&mut self, x: i32, y: i32, color: Color){
        if x >= 0 && y >= 0 && x < self.width as i32&& y < self.height as i32{
            self.screen[(self.height - 1) - y as usize][x as usize].plot_color(color);
        }
    }

    fn create_data(&self) -> String{
        let mut result: String = format!("P3\n{} {}\n255\n", self.screen[0].len(), self.screen.len());
     
        for i in 0..self.screen.len(){
            for v in 0..self.screen[i].len(){
                result.push_str(&self.screen[i][v].to_string().to_owned());
                result.push_str("  ");
            }
            result.push_str("\n");
        }
    
        return result;
    }

    pub fn create_file(&self, file_name: &str){
        let path = Path::new(&file_name);

        let mut file = match File::create(&path){
            Err(error) => panic!("failed to create image file because {}", error),
            Ok(file) => file,
        };

        let result = self.create_data();

        match file.write_all(result.as_bytes()){
            Err(error) => panic!("failed to write image file because {}", error),
            Ok(_) => {},
        };
    }

    pub fn clear(&mut self){
        for i in 0..self.screen.len(){
            for v in 0..self.screen[0].len(){
                self.screen[i][v].reset_color();
            }
        }
    }

    pub fn display(&mut self){
        self.create_file("/tmp/imageDisplay.ppm");
        Command::new("open")
        .arg("/tmp/imageDisplay.ppm")
        .spawn()
        .expect("failed to open image");
    }
}