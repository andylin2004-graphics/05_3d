use std::io::{BufReader, self, prelude::*};
use std::fs::File;
use crate::matrix::Matrix;
use crate::image::Image;
use crate::color::Color;
use std::process::Command;
use crate::matrix::CurveType;

/// Goes through the file named filename and performs all of the actions listed in that file.
/// 
/// The file follows the following format:
/// 
// Every command is a single character that takes up a line

/// Any command that requires arguments must have those arguments in the second line.

/// The commands are as follows:

/// line: add a line to the edge matrix -
/// takes 6 arguemnts (x0, y0, z0, x1, y1, z1)

/// ident: set the transform matrix to the identity matrix -

/// scale: create a scale matrix,
/// then multiply the transform matrix by the scale matrix -
/// takes 3 arguments (sx, sy, sz)

/// translate: create a translation matrix,
/// then multiply the transform matrix by the translation matrix -
/// takes 3 arguments (tx, ty, tz)

/// rotate: create a rotation matrix,
/// then multiply the transform matrix by the rotation matrix -
/// takes 2 arguments (axis, theta) axis should be x y or z

/// apply: apply the current transformation matrix to the edge matrix

/// display: clear the screen, then
/// draw the lines of the edge matrix to the screen
/// display the screen

/// save: clear the screen, then
/// draw the lines of the edge matrix to the screen
/// save the screen to a file -
/// takes 1 argument (file name)

/// quit: end parsing
/// 
/// circle: add a circle to the edge matrix - 
/// takes 4 arguments (cx, cy, cz, r)
/// 
/// hermite: add a hermite curve to the edge matrix -
///          takes 8 arguments (x0, y0, x1, y1, rx0, ry0, rx1, ry1)
/// 
/// bezier: add a bezier curve to the edge matrix -
///         takes 8 arguments (x0, y0, x1, y1, x2, y2, x3, y3)
///
/// See the file script for an example of the file format
pub fn parse_file( fname: &str, points: &mut Matrix, transform: &mut Matrix, screen: &mut Image, color: Color ) -> io::Result<()>{
    let file = File::open(&fname)?;
    let reader = BufReader::new(file);
    let mut doc_lines = vec![String::new(); 0];
    let mut i = 0;
    
    for line in reader.lines(){
        doc_lines.push(line?);
    }

    while i < doc_lines.len(){
        match &*doc_lines[i]{
            "line"=>{
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' '){
                    params.push(input.parse().unwrap());
                }
                points.add_edge(params[0], params[1], params[2], params[3], params[4], params[5]);
            }
            "ident"=>{
                transform.identity();
            }
            "scale"=>{
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' '){
                    params.push(input.parse().unwrap());
                }

                transform.multiply_matrixes(&Matrix::make_scale(params[0], params[1], params[2]));
            }
            "translate" | "move"=>{
                i += 1;
                let mut params = vec![0; 0];
                for input in doc_lines[i].split(' '){
                    params.push(input.parse().unwrap());
                }

                transform.multiply_matrixes(&Matrix::make_translate(params[0], params[1], params[2]));
            }
            "rotate"=>{
                i += 1;
                let mut params = vec![""; 0];
                for input in doc_lines[i].split(' '){
                    params.push(input);
                }

                match params[0]{
                    "x"=>{
                        transform.multiply_matrixes(&Matrix::make_rotX(params[1].parse().unwrap()));
                    }
                    "y"=>{
                        transform.multiply_matrixes(&Matrix::make_rotY(params[1].parse().unwrap()));
                    }
                    "z"=>{
                        transform.multiply_matrixes(&Matrix::make_rotZ(params[1].parse().unwrap()));
                    }
                    _=>{
                        panic!("Invalid input {} at 0 for rotation: please use x, y, or z.", params[0]);
                    }
                }
            }
            "apply"=>{
                points.multiply_matrixes(&transform);
            }
            "display"=>{
                screen.clear();
                screen.draw_lines(&points, color);
                screen.display();
            }
            "save"=>{
                screen.clear();
                screen.draw_lines(&points, color);
                i += 1;
                screen.create_file(&*doc_lines[i]);
                Command::new("magick")
                .arg("convert")
                .arg(&*doc_lines[i])
                .arg(&*doc_lines[i])
                .spawn()
                .expect("failed to convert image to desired format");
            }
            "quit"=>{
                break;
            }
            "circle"=>{
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' '){
                    params.push(input.parse().unwrap());
                }

                points.add_circle(params[0], params[1], params[2], params[3], 0.0001);
            }
            "hermite"=>{
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' '){
                    params.push(input.parse().unwrap());
                }

                points.add_curve(params[0], params[1], params[2], params[3], params[4], params[5], params[6], params[7], 0.0001, &CurveType::Hermite);
            }
            "bezier"=>{
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' '){
                    params.push(input.parse().unwrap());
                }

                points.add_curve(params[0], params[1], params[2], params[3], params[4], params[5], params[6], params[7], 0.0001, &CurveType::Bezier);
            }
            _ if doc_lines[i].starts_with('#') => {
            }
            _=>{
                panic!("Invalid command {} at line {}.", doc_lines[i], i+1);
            }
        }
        i += 1;
    }
    Ok(())
}