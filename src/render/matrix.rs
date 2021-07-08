#[derive(Clone, Copy)]
pub struct Matrix {
    matrix: [[f32; 4]; 4]
}

impl Matrix {
    pub fn new() -> Self {

        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Self {
            matrix
        }
    }

    pub fn new_with_data(matrix: [[f32; 4]; 4]) -> Self {
        Self {
            matrix
        }
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        self.matrix
    }

    pub fn set_matrix(&mut self, matrix: [[f32; 4]; 4]) {
        self.matrix = matrix;
    }

    pub fn multiply_scale(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[0][0] *= x;
        self.matrix[1][1] *= y;
        self.matrix[2][2] *= z;
    }

    pub fn add_scale(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[0][0] += x;
        self.matrix[1][1] += y;
        self.matrix[2][2] += z;
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[0][0] = x;
        self.matrix[1][1] = y;
        self.matrix[2][2] = z;
    }

    pub fn get_scale(&self) -> (f32, f32, f32) {
        (self.matrix[0][0], self.matrix[1][1], self.matrix[2][2],)
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[3][0] += x;
        self.matrix[3][1] += y;
        self.matrix[3][2] += z;
    }

    pub fn get_translation(&self) -> (f32, f32, f32) {
        (self.matrix[3][0], self.matrix[3][1], self.matrix[3][2])
    }

    pub fn set_translation(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[3][0] = x;
        self.matrix[3][1] = y;
        self.matrix[3][2] = z;
    }
}

#[derive(Copy, Clone)]
pub struct WorldTransform {
    pub world: Matrix,
    pub projection: Matrix,
}

impl WorldTransform {
    pub fn to_uniform(&self) -> glium::uniforms::UniformsStorage<[[f32; 4]; 4], glium::uniforms::UniformsStorage<[[f32; 4]; 4], glium::uniforms::EmptyUniforms>>
    {
        uniform!{projection: self.projection.get_matrix(), view: self.world.get_matrix()}
    }
}