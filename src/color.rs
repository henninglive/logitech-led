use std::os::raw::c_int;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    /// Red component [0,1]
    pub r: f32,
    /// Green component [0,1]
    pub g: f32,
    /// Blue component [0,1]
    pub b: f32,
}

pub type BGRA = [u8; 4];

fn clampf(n: f32) -> f32 {
    if !n.is_normal() { 0.0 }
    else if n < 0.0 { 0.0 }
    else if n > 1.0 { 1.0 }
    else { n }
}

fn f2u8(n: f32) -> u8 {
    let i = (clampf(n) * 255.0) as i32;
    if i < 0 { 0 }
    else if i > 255 { 255 }
    else { i as u8 }
}

fn hsv2rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s <= 0.0 {
        return (v,v,v);
    }
    let mut hh = h;
    if hh >= 360.0 {
        hh = 0.0;
    }
    hh = hh / 60.0;
    let i = hh.floor() as u32;
    let ff = hh - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - (s * ff));
    let t = v * (1.0 - (s * (1.0 - ff)));
    match i {
        0 => (v,t,p),
        1 => (q,v,p),
        2 => (p,v,t),
        3 => (p,q,v),
        4 => (t,p,v),
        5 => (v,p,q),
        _ => panic!("Unexpected value in hsv2rgb: i: {} h: {}", i, h),
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color{ r: r, g: g, b: b }.clamp()
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Color {
        hsv2rgb(h, s, v).into()
    }

    pub fn clamp(&self) -> Color {
        let mut c = *self;
        c.clamp_mut();
        c
    }

    pub fn clamp_mut(&mut self) {
        self.r = clampf(self.r);
        self.g = clampf(self.g);
        self.b = clampf(self.b);
    }
}

impl From<BGRA> for Color {
    fn from(bgra: [u8; 4]) -> Color {
        let a = bgra[3] as f32 / 255.0;
        Color{
            r:  (bgra[2] as f32 / 255.0) * a,
            g:  (bgra[1] as f32 / 255.0) * a,
            b:  (bgra[0] as f32 / 255.0) * a,
        }.clamp()
    }
}

impl From<Color> for BGRA {
    fn from(c: Color) -> [u8; 4] {
        [f2u8(c.b), f2u8(c.g), f2u8(c.r), 255]
    }
}


impl From<(f32, f32, f32)> for Color {
    fn from(t: (f32, f32, f32)) -> Color {
        Color{ r: t.0, g: t.1, b: t.2, }.clamp()
    }
}

impl From<Color> for (f32, f32, f32) {
    fn from(mut c: Color) -> (f32, f32, f32) {
        c.clamp_mut();
        (c.r, c.g, c.b)
    }
}

impl From<[f32; 3]> for Color {
    fn from(t: [f32; 3]) -> Color {
        Color{ r: t[0], g: t[1], b: t[2] }.clamp()
    }
}

impl From<Color> for [f32; 3] {
    fn from(mut c: Color) -> [f32; 3] {
        c.clamp_mut();
        [c.r, c.g, c.b]
    }
}

pub fn to_precent(mut c: Color) -> (c_int, c_int, c_int) {
    c.clamp_mut();
    (
        (c.r * 100.0) as c_int,
        (c.g * 100.0) as c_int,
        (c.b * 100.0) as c_int,
    )
}

pub fn from_precent(p: (c_int, c_int, c_int)) -> Color {
    Color::new(
        p.0 as f32 / 100.0,
        p.1 as f32 / 100.0,
        p.2 as f32 / 100.0,
    )
}
