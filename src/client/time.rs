use crate::client::renderer::{state::State, gpu::uniform::Uniform};

pub struct Time {
    pub dt64: f64,
    pub dt32: f32,

    pub fps: Fps,
    pub uf: Uniform<i32>,
    start: std::time::Instant,
    last: std::time::Instant,
}

impl Time {
    pub fn new(state: &State) -> Self {
        let uf = Uniform::new(
            &state.device,
            0,
            "Time uniform",
            wgpu::ShaderStages::VERTEX,
        );

        let start = std::time::Instant::now();

        Self { dt64: 0., dt32: 0., fps: Fps::new(), uf, start, last: start }
    }

    /// Executes every *frame* frames
    pub fn every<T: FnMut()>(&self, frame: usize, mut action: T) {
        if (self.fps.frame % frame) == 0 {
            action()
        }
    }

    /// Executes every *s* seconds (Can be imprecise - can skip or add extra executions)
    // pub fn every_s_approximate<T: FnMut()>(&self, s: f64, mut action: T) {
    //     let div = (s / self.fps.avg_dt) as usize;
    //     if div == 0 || (self.fps.frame % (s / self.fps.avg_dt) as usize) == 0 {
    //         action()
    //     }
    // }

    pub fn update(&mut self, state: &State) {
        self.dt64 = self.last.elapsed().as_secs_f64();
        self.last = std::time::Instant::now();

        self.dt32 = self.dt64 as f32;

        self.fps.update(self.dt64);

        let time = self.start.elapsed().as_millis() as u16;
        self.uf.data = time as i32 - 32768;
        self.uf.update(&state.queue);
    }
}

const RUNNING_FRAME_UPDATE: f64 = 1500.;
pub struct Fps {
    pub frame: usize,
    pub avg_fps: f64,
    pub avg_dt: f64,
    running_time: f64,
    running_frames: f64,
}

impl Fps {
    fn new()-> Self { Self {frame: 0, avg_fps: 0., avg_dt: 0., running_frames: 0., running_time: 0.} }

    fn update(&mut self, dt: f64) {
        self.frame += 1;

        if self.running_frames > RUNNING_FRAME_UPDATE {self.running_frames = 0.; self.running_time = 0.}
        self.running_frames += 1.;
        self.running_time += dt;

        self.avg_dt = self.running_time / self.running_frames;
        self.avg_fps = 1. / self.avg_dt;
    }
}