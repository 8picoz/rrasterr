use cgmath::Vector2;

type Vec2f = Vector2<f32>;

pub struct BoundingBox {
    pub max: Vec2f,
    pub min: Vec2f,
}

impl BoundingBox {
    pub fn new(max: Vec2f, min: Vec2f) -> Self {
        Self { max, min }
    }

    pub fn calc_from_tree_vertex(vertexes: Vec<Vec2f>) -> Self {
        let x = vertexes.iter().map(|item| item.x);
        let y = vertexes.iter().map(|item| item.y);

        let x_max = x.clone().fold(f32::MIN, |m, v| v.max(m));
        let x_min = x.fold(f32::MAX, |m, v| v.min(m));

        let y_max = y.clone().fold(f32::MIN, |m, v| v.max(m));
        let y_min = y.fold(f32::MAX, |m, v| v.min(m));

        let max = Vec2f::new(x_max, y_max);
        let min = Vec2f::new(x_min, y_min);

        Self { max, min }
    }
}