pub struct Screen {
    //near clip
    pub n: f32,
    //far clip
    pub f: f32,
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
    //near clipの大きさ
    //width
    pub w: f32,
    //height
    pub h: f32,
}

impl Screen {
    pub fn new(n: f32, f: f32, right: f32, left: f32, top: f32, bottom: f32) -> Self {
        let w = right - left;
        let h = top - bottom;
        Self { n, f, right, left, top, bottom, w, h }
    }
}