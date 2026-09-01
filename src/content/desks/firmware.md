---
title: Firmware
stamp: desk 07
summary: Rust no_std laws now. Board support later.
frozen: "Host-tested laws · STM32 shell documented, not boarded"
order: 7
---

V1 does not pretend we have a flashing BSP. `firmware/` is excluded from the default workspace on purpose. The code that will sit on the F405 is the same crate the laptop already tests.

Bring-up order after a BSP exists: clocks, IMU sample, DShot at 0, arming castle with props off, then a bench spin at idle, then a leashed hover. Skip a step and you skip the bird.

Until then, `cargo test -p flight-core` is the honest green check.
