---
title: Microprocessors
stamp: desk 08
summary: STM32F405 and an ICM-42688-P, nothing fancier.
frozen: "STM32F405 · ICM-42688-P"
order: 8
---

The F405 is the freeze: enough flash and FPU for `no_std` Rust control without shopping a dual-core H7. The ICM-42688-P is the IMU. Sample it fast, filter it on purpose, and do not run the Mahony update on junk accel during a slam.

Keep debug probes and USB off the hover current path. A hanging ST-Link cable is a yaw spring.

No companion Linux computer on the airframe in V1. The laptop is for `sim` and tests, not a flight computer.
