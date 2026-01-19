use glam::{Mat4, Quat, Vec3};
use render_derive::with_transform;

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

    fn zoom(&mut self, yoffset: f32) {
        self.fov += yoffset;
        self.fov = self.fov.clamp(1.0, 65.0);
    }
}

#[with_transform]
#[derive(Default)]
pub struct Camera {
    pub frustum: Frustum,
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
}

impl Camera {
    pub fn new(near: f32, far: f32) -> Self {
        let frustum = Frustum::new(45.0, near, far);

        Self {
            frustum,
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.08,
            ..Default::default()
        }
    }

    fn angle_front(&self) -> Quat {
        Quat::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), self.pitch.to_radians())
    }

    fn angle_up(&self) -> Quat {
        Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), self.yaw.to_radians())
    }

    pub fn getCameraRotation(&self) -> Mat4 {
        Mat4::from_quat(self.angle_front() * self.angle_up())
    }

    pub fn getViewMatrix(&self) -> Mat4 {
        self.getCameraRotation() * self.get_position_matrix() // TODO: Change to the actual camera position (needs traits)
    }
}
