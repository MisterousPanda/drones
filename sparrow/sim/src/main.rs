use flight_core::{mix, to_dshot, Arming, ArmingEvent, Lvc, LvcAction};

fn main() {
    let mut arming = Arming::default();
    let mut lvc = Lvc::default();
    let pack_v = 12.2;
    let action = lvc.observe(pack_v);
    if action != LvcAction::Cut {
        arming.apply(ArmingEvent::RequestArm);
    }
    let motors = mix(0.42, 0.0, 0.0, 0.0);
    println!("pack {pack_v:.2} V  lvc={action:?}  armed={}", arming.armed());
    for (i, cmd) in motors.iter().enumerate() {
        println!("M{i}  {cmd:.3}  dshot={}", to_dshot(*cmd, arming.armed()));
    }
}
