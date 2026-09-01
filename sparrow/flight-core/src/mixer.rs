//! X-quad mixer and DShot mapping.
//!
//! Motor order: 0 FL, 1 FR, 2 RL, 3 RR.
//! Rotation: FL CW, FR CCW, RL CCW, RR CW.

pub type MotorSet = [f32; 4];

/// Mix collective throttle and body torques into four [0, 1] commands.
pub fn mix(throttle: f32, roll: f32, pitch: f32, yaw: f32) -> MotorSet {
    let mut m = [
        throttle - roll + pitch - yaw, // FL
        throttle + roll + pitch + yaw, // FR
        throttle - roll - pitch + yaw, // RL
        throttle + roll - pitch - yaw, // RR
    ];
    let floor = m.iter().copied().fold(f32::INFINITY, f32::min);
    if floor < 0.0 {
        for v in &mut m {
            *v -= floor;
        }
    }
    for v in &mut m {
        *v = v.clamp(0.0, 1.0);
    }
    m
}

/// Map [0, 1] onto DShot. 0 is disarmed; 48..=2047 is throttle.
pub fn to_dshot(cmd: f32, armed: bool) -> u16 {
    if !armed {
        return 0;
    }
    48 + libm::roundf(cmd.clamp(0.0, 1.0) * 1999.0) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hover_is_even() {
        let m = mix(0.4, 0.0, 0.0, 0.0);
        for v in m {
            assert!((v - 0.4).abs() < 1e-6);
        }
    }

    #[test]
    fn positive_roll_lifts_left() {
        let m = mix(0.5, 0.1, 0.0, 0.0);
        assert!(m[0] < m[1]); // FL < FR
        assert!(m[2] < m[3]); // RL < RR
    }

    #[test]
    fn disarmed_is_zero() {
        assert_eq!(to_dshot(0.8, false), 0);
    }

    #[test]
    fn armed_idle_is_above_commands() {
        assert!(to_dshot(0.0, true) >= 48);
        assert_eq!(to_dshot(1.0, true), 2047);
    }
}
