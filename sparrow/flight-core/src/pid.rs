//! Parallel PID with output clamp. Gains are per-sample (dt baked in by the caller).

#[derive(Clone, Copy, Debug)]
pub struct Pid {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub out_min: f32,
    pub out_max: f32,
    integral: f32,
    prev_err: f32,
    primed: bool,
}

impl Pid {
    pub const fn new(kp: f32, ki: f32, kd: f32, out_min: f32, out_max: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            out_min,
            out_max,
            integral: 0.0,
            prev_err: 0.0,
            primed: false,
        }
    }

    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_err = 0.0;
        self.primed = false;
    }

    pub fn step(&mut self, error: f32) -> f32 {
        let d = if self.primed { error - self.prev_err } else { 0.0 };
        self.primed = true;
        self.prev_err = error;

        let unsat = self.kp * error + self.ki * self.integral + self.kd * d;
        let sat = unsat.clamp(self.out_min, self.out_max);
        // Integrator only moves when it would help leave saturation.
        let unsaturated = libm::fabsf(sat - unsat) < f32::EPSILON;
        let same_sign = (sat >= 0.0) == (error >= 0.0);
        if unsaturated || same_sign {
            self.integral = (self.integral + error).clamp(self.out_min, self.out_max);
        }
        sat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_term_scales() {
        let mut pid = Pid::new(2.0, 0.0, 0.0, -10.0, 10.0);
        assert!((pid.step(0.5) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn clamp_holds() {
        let mut pid = Pid::new(10.0, 0.0, 0.0, -1.0, 1.0);
        assert_eq!(pid.step(5.0), 1.0);
    }
}
