pub struct Screen {
    //far clip
    pub f: f32,
    //near clip
    pub n: f32,
    //width
    pub w: f32,
    //height
    pub h: f32,
}

impl Screen {
    pub fn new(f: f32, n: f32, w: f32, h: f32) -> Self {
        Self { f, n, w, h }
    }
}