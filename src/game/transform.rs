use glam::{Mat4, Quat, Vec3};

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub local_front: Vec3,
    pub local_right: Vec3,
    pub local_up: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            local_front: Vec3::Z,
            local_right: Vec3::X,
            local_up: Vec3::Y,
        }
    }
}

impl Transform {
    pub fn get_position_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_translation(-self.position)
    }

    pub fn get_rotation_matrix(&self) -> Mat4 {
        Mat4::from_quat(self.rotation)
    }

    pub fn get_scale_matrix(&self) -> Mat4 {
        Mat4::from_scale(self.scale)
    }

    pub fn get_world_matrix(&self) -> Mat4 {
        self.get_position_matrix() * self.get_rotation_matrix()
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.get_rotation_matrix() * self.get_position_matrix()
    }

    pub fn move_forward(&mut self, speed: f32, delta: f32) {
        self.position += self.local_front * speed * delta;
    }

    pub fn move_backward(&mut self, speed: f32, delta: f32) {
        self.position -= self.local_front * speed * delta;
    }

    pub fn move_left(&mut self, speed: f32, delta: f32) {
        self.position -= self.local_right * speed * delta;
    }

    pub fn move_right(&mut self, speed: f32, delta: f32) {
        self.position += self.local_right * speed * delta;
    }

    pub fn move_up(&mut self, speed: f32, delta: f32) {
        self.position += self.local_up * speed * delta;
    }

    pub fn move_down(&mut self, speed: f32, delta: f32) {
        self.position -= self.local_up * speed * delta;
    }

    pub fn move_global_up(&mut self, speed: f32, delta: f32) {
        self.position += glam::Vec3::Y * speed * delta;
    }

    pub fn move_global_down(&mut self, speed: f32, delta: f32) {
        self.position -= glam::Vec3::Y * speed * delta;
    }
}
