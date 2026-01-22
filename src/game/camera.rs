use glam::{Mat4, Quat, Vec3};

use crate::game::Transform;

#[derive(Default)]
pub struct Frustum {
    pub near: f32,
    pub far: f32,
    pub fov: f32,
}

impl Frustum {
    fn new(fov: f32, near: f32, far: f32) -> Self {
        Self { near, far, fov }
    }

    pub fn zoom(&mut self, yoffset: f32) {
        self.fov += yoffset;
        self.fov = self.fov.clamp(1.0, 65.0);
    }
}

pub enum Projection {
    Perspective(f32),
    Orthographic(f32, f32, f32, f32),
}

#[derive(Default)]
pub struct Camera {
    pub frustum: Frustum,
    pub transform: Transform,
    pub pitch: f32,
    pub yaw: f32,
    sensitivity: f32,
    constrain_pitch: f32,
}

impl Camera {
    pub fn new(near: f32, far: f32) -> Self {
        let frustum = Frustum::new(45.0, near, far);

        Self {
            frustum,
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.08,
            constrain_pitch: 89.0,
            ..Default::default()
        }
    }

    pub fn turn(&mut self, xoffset: f32, yoffset: f32) {
        self.yaw += xoffset * self.sensitivity;
        self.pitch -= yoffset * self.sensitivity;

        // Constrain and normalize angles
        self.yaw = self.yaw % 360.0;
        self.pitch = self
            .pitch
            .clamp(-self.constrain_pitch, self.constrain_pitch);

        self.update_local_vectors();
    }

    pub fn update_local_vectors(&mut self) {
        let front = Vec3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };
        self.transform.local_front = front;
        self.transform.local_right = self.transform.local_front.cross(Vec3::Y).normalize();
        self.transform.local_up = self
            .transform
            .local_right
            .cross(self.transform.local_front)
            .normalize();
    }

    fn angle_front(&self) -> Quat {
        Quat::from_axis_angle(Vec3::X, self.pitch.to_radians())
    }

    fn angle_up(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, self.yaw.to_radians())
    }

    pub fn get_camera_rotation_matrix(&self) -> Mat4 {
        Mat4::from_quat(self.angle_front() * self.angle_up())
    }

    pub fn get_camera_view_matrix(&self) -> Mat4 {
        self.get_camera_rotation_matrix() * self.transform.get_position_matrix()
    }

    pub fn get_camera_world_matrix(&self) -> Mat4 {
        self.transform.get_position_matrix() * self.get_camera_rotation_matrix()
    }

    pub fn get_camera_projection_matrix(&self, projection: Projection) -> Mat4 {
        let fov = self.frustum.fov;
        match projection {
            Projection::Perspective(aspect) => Mat4::perspective_rh(
                fov.to_radians(),
                aspect,
                self.frustum.near,
                self.frustum.far,
            ),
            Projection::Orthographic(left, right, bottom, top) => Mat4::orthographic_rh(
                left * fov,
                right * fov,
                bottom * fov,
                top * fov,
                self.frustum.near,
                self.frustum.far,
            ),
        }
    }
}
