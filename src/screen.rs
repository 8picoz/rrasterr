pub struct Screen {
    //near clip
    pub n: f32,
    //far clip
    pub f: f32,
    //width
    pub w: f32,
    //height
    pub h: f32,
}

impl Screen {
    pub fn new(n: f32, f: f32, w: f32, h: f32) -> Self {
        Self { n, f, w, h }
    }
}