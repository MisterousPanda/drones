//! Arming castle: sticks and LVC must agree before motors leave zero DShot.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlightMode {
    Disarmed,
    Armed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmingEvent {
    RequestArm,
    RequestDisarm,
    Failsafe,
    LowVoltageCut,
}

#[derive(Clone, Copy, Debug)]
pub struct Arming {
    mode: FlightMode,
}

impl Default for Arming {
    fn default() -> Self {
        Self {
            mode: FlightMode::Disarmed,
        }
    }
}

impl Arming {
    pub fn mode(&self) -> FlightMode {
        self.mode
    }

    pub fn armed(&self) -> bool {
        self.mode == FlightMode::Armed
    }

    pub fn apply(&mut self, event: ArmingEvent) {
        self.mode = match (self.mode, event) {
            (_, ArmingEvent::Failsafe | ArmingEvent::LowVoltageCut | ArmingEvent::RequestDisarm) => {
                FlightMode::Disarmed
            }
            (FlightMode::Disarmed, ArmingEvent::RequestArm) => FlightMode::Armed,
            (mode, _) => mode,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boots_disarmed() {
        assert!(!Arming::default().armed());
    }

    #[test]
    fn lvc_kills_motors() {
        let mut a = Arming::default();
        a.apply(ArmingEvent::RequestArm);
        a.apply(ArmingEvent::LowVoltageCut);
        assert!(!a.armed());
    }
}
