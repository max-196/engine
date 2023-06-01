use crate::{common::math::{angle::Angle, vec::{Vec3, Vec2}}, client::InputManager};

const FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2 - f32::EPSILON;

pub struct CameraController {
    /// Z - +forward, -backward; Y - +up, -down; X - +right, -left
    translation: Vec3<f32>,
    rotate: Vec2<f32>,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            translation: Vec3::default(),
            rotate: Vec2::default(),
            speed,
            sensitivity,
        }
    }

    pub fn update(&mut self, inp: &InputManager) {
        self.rotate = Vec2::new(inp.mouse.mv.x as f32, inp.mouse.mv.y as f32);

        self.translation = Vec3::default();

        self.translation.z += inp.get_key("forward") as u8 as f32;
        self.translation.z -= inp.get_key("backward") as u8 as f32;

        self.translation.x += inp.get_key("right") as u8 as f32;
        self.translation.x -= inp.get_key("left") as u8 as f32;

        self.translation.y += inp.get_key("up") as u8 as f32;
        self.translation.y -= inp.get_key("down") as u8 as f32;
    }

    pub fn update_camera(&mut self, camera: &mut super::PhysicalCamera, time: &crate::client::Time) {
        let dt = time.dt32;

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.translation.z) * self.speed * dt;
        camera.position += right * (self.translation.x) * self.speed * dt;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (self.translation.y) * self.speed * dt;

        // Rotate
        camera.yaw += Angle::from_rad(self.rotate.x) * self.sensitivity * dt;
        camera.pitch += Angle::from_rad(-self.rotate.y) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate.x = 0.0;
        self.rotate.y = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Angle::from_rad(FRAC_PI_2) {
            camera.pitch = -Angle::from_rad(FRAC_PI_2);
        } else if camera.pitch > Angle::from_rad(FRAC_PI_2) {
            camera.pitch = Angle::from_rad(FRAC_PI_2);
        }
    }
}
