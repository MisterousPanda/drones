---
title: Controls
stamp: desk 06
summary: Mixer, PID, Mahony, castle — already host-tested.
frozen: "X-quad mix · parallel PID · Mahony · arming castle"
order: 6
---

The laws live in `sparrow/flight-core` and compile `no_std`.

The mixer takes collective throttle plus body torques and writes four commands in `[0, 1]`. If any motor would go negative, the whole set is lifted so the lowest is zero, then clamped. That keeps a hard roll from commanding a reverse blade on a brushed-less ESC that cannot reverse in V1.

PID is parallel with an output clamp. Mahony fuses gyro and accel for roll/pitch; yaw is gyro-integrated. Do not fly heading-critical missions on V1 yaw.

Arming is a castle: boot disarmed, arm on request, disarm on request, failsafe, or LVC. There is no “maybe spinning.”
