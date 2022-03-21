use std::fmt;

#[derive(Copy, Clone)]
pub struct Color{
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl Color{
    pub fn new() -> Color{
        Color{r: 0, g:0, b:0}
    }

    pub fn new_color(r: i32, g: i32, b: i32) -> Color{
        Color{r: r, g: g, b: b}
    }
    
    pub fn plot_color(&mut self, new_color: Color){
        self.r = new_color.r;
        self.g = new_color.g;
        self.b = new_color.b;
    }
    
    pub fn reset_color(&mut self){
        self.r = 0;
        self.b = 0;
        self.g = 0;
    }
}

impl fmt::Display for Color{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}