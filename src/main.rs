use parser::parse_file;
use image::Image;
use color::Color;
use matrix::Matrix;
use std::env;
use matrix::CurveType;
use rand::Rng;
use std::f32;
use std::process::Command;
mod parser;
mod matrix;
mod image;
mod color;
mod draw;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "art"{
        let mut screen = Image::new(500, 500);
        let mut color = Color::new_color(0, 255, 0);
        let mut edges = Matrix::new(0,0);
        let mut rng = rand::thread_rng();
        for i in 0..screen.screen.len(){
            for v in 0..screen.screen[i].len(){
                screen.screen[i][v].r = 255;
                screen.screen[i][v].g = 255;
                screen.screen[i][v].b = 255;
            }
        }
        for _ in 0..100{
            color.r = rng.gen_range(0..256);
            color.g = rng.gen_range(0..256);
            color.b = rng.gen_range(0..256);
            edges.add_sphere(rng.gen_range(0.0..500.0), rng.gen_range(0.0..500.0), 0.0, 20.0, 0.05);
            screen.draw_lines(&edges, color);
            edges = Matrix::new(0,0);
        }
        for _ in 0..100{
            color.r = rng.gen_range(0..256);
            color.g = rng.gen_range(0..256);
            color.b = rng.gen_range(0..256);
            edges.add_torus(rng.gen_range(0.0..500.0), rng.gen_range(0.0..500.0), 0.0, 5.0, 20.0, f32::consts::PI/10.0);
            edges.multiply_matrixes(&Matrix::make_rotX(90.0));
            edges.multiply_matrixes(&Matrix::make_translate(0, rng.gen_range(0..500), 0));
            screen.draw_lines(&edges, color);
            edges = Matrix::new(0,0);
        }
        screen.display();
        screen.create_file("cereal.ppm");
        Command::new("magick")
                .arg("convert")
                .arg("cereal.ppm")
                .arg("cereal.png")
                .spawn()
                .expect("failed to convert image to desired format");
        // parse_file("cereal", &mut edges, &mut transform, &mut screen, color);
    }else{
        let mut screen = Image::new(500, 500);
        let mut color = Color::new_color(0, 255, 0);
        let mut edges = Matrix::new(0,0);
        let mut transform = Matrix::new(4,4);
        parse_file("script", &mut edges, &mut transform, &mut screen, color);    
    }
}
