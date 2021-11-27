pub struct Screen {
    //near clip
    pub near_clip_distance: f32,
    //far clip
    pub far_clip_distance: f32,
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
    //near clipの大きさ
    //width
    pub width: f32,
    //height
    pub height: f32,
}

impl Screen {
    pub fn new(near_clip_distance: f32, far_clip_distance: f32, right: f32, left: f32, top: f32, bottom: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        Self { near_clip_distance, far_clip_distance, right, left, top, bottom, width, height }
    }
}