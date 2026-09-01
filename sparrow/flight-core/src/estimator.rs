//! Mahony complementary filter for roll/pitch. Yaw is integrated from gyro only.

use crate::types::{Attitude, ImuSample};

#[derive(Clone, Copy, Debug)]
pub struct Mahony {
    pub kp: f32,
    pub ki: f32,
    q: [f32; 4],
    bias: [f32; 3],
}

impl Default for Mahony {
    fn default() -> Self {
        Self {
            kp: 0.5,
            ki: 0.0,
            q: [1.0, 0.0, 0.0, 0.0],
            bias: [0.0, 0.0, 0.0],
        }
    }
}

impl Mahony {
    pub fn attitude(&self) -> Attitude {
        let [w, x, y, z] = self.q;
        Attitude {
            roll: libm::atan2f(2.0 * (w * x + y * z), 1.0 - 2.0 * (x * x + y * y)),
            pitch: libm::asinf((2.0 * (w * y - z * x)).clamp(-1.0, 1.0)),
            yaw: libm::atan2f(2.0 * (w * z + x * y), 1.0 - 2.0 * (y * y + z * z)),
        }
    }

    pub fn step(&mut self, imu: ImuSample, dt: f32) {
        let mut gx = imu.gyro[0] - self.bias[0];
        let mut gy = imu.gyro[1] - self.bias[1];
        let mut gz = imu.gyro[2] - self.bias[2];

        let an = libm::sqrtf(imu.acc[0] * imu.acc[0] + imu.acc[1] * imu.acc[1] + imu.acc[2] * imu.acc[2]);
        if an > 1e-6 {
            let ax = imu.acc[0] / an;
            let ay = imu.acc[1] / an;
            let az = imu.acc[2] / an;
            let [qw, qx, qy, qz] = self.q;
            let vx = 2.0 * (qx * qz - qw * qy);
            let vy = 2.0 * (qw * qx + qy * qz);
            let vz = qw * qw - qx * qx - qy * qy + qz * qz;
            let ex = ay * vz - az * vy;
            let ey = az * vx - ax * vz;
            let ez = ax * vy - ay * vx;
            self.bias[0] += -self.ki * ex * dt;
            self.bias[1] += -self.ki * ey * dt;
            self.bias[2] += -self.ki * ez * dt;
            gx += self.kp * ex;
            gy += self.kp * ey;
            gz += self.kp * ez;
        }

        let half = 0.5 * dt;
        let [qw, qx, qy, qz] = self.q;
        self.q = [
            qw + (-qx * gx - qy * gy - qz * gz) * half,
            qx + (qw * gx + qy * gz - qz * gy) * half,
            qy + (qw * gy - qx * gz + qz * gx) * half,
            qz + (qw * gz + qx * gy - qy * gx) * half,
        ];
        let n = libm::sqrtf(self.q.iter().map(|v| v * v).sum::<f32>()).max(1e-9);
        for v in &mut self.q {
            *v /= n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_accel_stays_near_zero() {
        let mut f = Mahony::default();
        let imu = ImuSample {
            acc: [0.0, 0.0, 9.81],
            gyro: [0.0, 0.0, 0.0],
        };
        for _ in 0..200 {
            f.step(imu, 0.002);
        }
        let a = f.attitude();
        assert!(a.roll.abs() < 0.05);
        assert!(a.pitch.abs() < 0.05);
    }
}
