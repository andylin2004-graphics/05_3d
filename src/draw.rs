use crate::Image;
use crate::Matrix;
use crate::Color;
use crate::CurveType;
use std::f32;

impl Image{
    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: Color){
        if x0 > x1{
            let mut tmp = x0;
            x0 = x1;
            x1 = tmp;
            tmp = y0;
            y0 = y1;
            y1 = tmp;
        }
        let slope: f32 = (y1-y0) as f32 / (x1-x0) as f32;
        if slope > 1.0{
            // octant 2
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0);
            let b = -2*(x1-x0);
            let mut d = 1/2*a + b; // emphasis on controlling y
            while y <= y1{
                self.plot(x, y, color);
                if d < 0{ // as b dominates a, and we need to hit 0
                    x += 1;
                    d += a;
                }
                y += 1;
                d += b;
            }
        }else if slope >= 0.0{
            // octant 1
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0);
            let b = -2*(x1-x0);
            let mut d = a + 1/2*b; // emphasis on controlling x
            while x <= x1{
                self.plot(x, y, color);
                if d > 0{ // as a dominates b, and we need to hit 0
                    y += 1;
                    d += b;
                }
                x += 1;
                d += a;
            }
        }else if slope < -1.0{
            // octant 7
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0); // since this is negative, you dont need to make the next part negative
            let b = 2*(x1-x0);
            let mut d = 1/2*a + b; // emphasis on controlling x
            while y >= y1{
                self.plot(x, y, color);
                if d < 0{ // as a dominates b, and we need to hit 0
                    x += 1;
                    d -= a; // basically adding
                }
                y -= 1;
                d -= b; // basically adding
            }
        }else{
            // octant 8
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0); // since this is negative, you dont need to make the next part negative
            let b = 2*(x1-x0);
            let mut d = a + 1/2*b; // emphasis on controlling y
            while x <= x1{
                self.plot(x, y, color);
                if d > 0{ // as b dominates a, and we need to hit 0
                    y -= 1;
                    d -= b; // basically adding
                }
                x += 1;
                d -= a; // basically adding
            }
        }
    }

    pub fn draw_lines(&mut self, matrix: &Matrix, color: &Color){
        for i in (0..matrix.matrix_array[0].len()).step_by(2){
            self.draw_line(matrix.matrix_array[0][i] as i32, matrix.matrix_array[1][i] as i32, matrix.matrix_array[0][i+1] as i32, matrix.matrix_array[1][i+1] as i32, *color);
        }
    }
}

impl Matrix{
    pub fn add_edge(&mut self, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32){
        if self.matrix_array.len() < 4{
            *self = Matrix::new(4,0);
        }
        self.add_point(x0,y0,z0);
        self.add_point(x1,y1,z1);
    }

    pub fn add_edge_int(&mut self, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32){
        if self.matrix_array.len() < 4{
            *self = Matrix::new(4,0);
        }
        self.add_point(x0 as f32,y0 as f32,z0 as f32);
        self.add_point(x1 as f32,y1 as f32,z1 as f32);
    }

    pub fn add_point(&mut self, x: f32, y: f32, z: f32){
        if self.matrix_array.len() < 4{
            *self = Matrix::new(4,0);
        }
        self.matrix_array[0].push(x);
        self.matrix_array[1].push(y);
        self.matrix_array[2].push(z);
        self.matrix_array[3].push(1.0);
    }

    pub fn add_circle( &mut self, cx: f32, cy: f32, cz: f32, r: f32, step: f32 ){
        let mut t = 0.0;
        let mut prev_x = 0.0;
        let mut prev_y = 0.0;
        while t < 1.0{
            let x = r*(2.0*f32::consts::PI*t).cos()+cx;
            let y = r*(2.0*f32::consts::PI*t).sin()+cy;
            if t > 0.0{
                self.add_edge(prev_x, prev_y, cz, x, y, cz);
            }
            prev_x = x;
            prev_y = y;
            t += step;
        }
    }

    /// x2, y2, x3, y3 are rx0, ry0, rx1, ry1 respectively if hermier
    pub fn add_curve( &mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, step: f32, curveType: &CurveType ){
        let matrix_x = Matrix::generate_curve_coefs(x0, x1, x2, x3, curveType);
        let matrix_y = Matrix::generate_curve_coefs(y0, y1, y2, y3, curveType);
        let mut t: f32 = 0.0;
        let mut prev_x = 0.0;
        let mut prev_y = 0.0;
        while t < 1.0{
            let x = (matrix_x.matrix_array[0][0]*t.powi(3))+(matrix_x.matrix_array[1][0]*t.powi(2))+(matrix_x.matrix_array[2][0]*t)+matrix_x.matrix_array[3][0];
            let y = (matrix_y.matrix_array[0][0]*t.powi(3))+(matrix_y.matrix_array[1][0]*t.powi(2))+(matrix_y.matrix_array[2][0]*t)+matrix_y.matrix_array[3][0];
            if t > 0.0{
                self.add_edge(prev_x, prev_y, 0.0, x, y, 0.0); 
            }
            prev_x = x;
            prev_y = y;
            t += step;
        }
    }
}