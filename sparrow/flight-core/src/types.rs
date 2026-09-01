//! Shared units for the host-tested control laws.

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Attitude {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ImuSample {
    /// Body accelerometer, m/s².
    pub acc: [f32; 3],
    /// Body gyro, rad/s.
    pub gyro: [f32; 3],
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Setpoint {
    pub thrust: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Torque {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Thrust(pub f32);
