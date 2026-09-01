//! 3S low-voltage policy. Warn, then cut. Never climb out of a cut on bounce.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvcAction {
    Ok,
    Warn,
    Cut,
}

#[derive(Clone, Copy, Debug)]
pub struct Lvc {
    pub warn_v: f32,
    pub cut_v: f32,
    latched_cut: bool,
}

impl Default for Lvc {
    fn default() -> Self {
        Self {
            warn_v: 10.5, // 3.50 V/cell
            cut_v: 9.9,   // 3.30 V/cell
            latched_cut: false,
        }
    }
}

impl Lvc {
    pub fn observe(&mut self, pack_v: f32) -> LvcAction {
        if self.latched_cut || pack_v <= self.cut_v {
            self.latched_cut = true;
            LvcAction::Cut
        } else if pack_v <= self.warn_v {
            LvcAction::Warn
        } else {
            LvcAction::Ok
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_pack_is_ok() {
        let mut lvc = Lvc::default();
        assert_eq!(lvc.observe(12.4), LvcAction::Ok);
    }

    #[test]
    fn cut_latches() {
        let mut lvc = Lvc::default();
        assert_eq!(lvc.observe(9.6), LvcAction::Cut);
        assert_eq!(lvc.observe(11.8), LvcAction::Cut);
    }
}
