use crate::common::math::{angle::Angle, vec::Vec3, mat::Mat4};


pub struct PhysicalCamera {
    pub position: Vec3<f32>,
    pub yaw: Angle<f32>,
    pub pitch: Angle<f32>,
}

impl PhysicalCamera {
    pub fn new<
        V: Into<Vec3<f32>>,
    >(
        position: V,
        yaw: f32,
        pitch: f32,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: Angle::from_rad(yaw),
            pitch: Angle::from_rad(pitch),
        }
    }

    pub fn calc_matrix(&self) -> Mat4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        Mat4::look_to_rh(
            self.position,
            Vec3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw
            ).normalize(),
            Vec3::unit_y(),
        )
    }
}
