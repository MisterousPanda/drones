---
title: Operating systems
stamp: desk 09
summary: Bare-metal control loop. No flight Linux.
frozen: "Bare metal / future RTOS · not a Linux flyer"
order: 9
---

The inner loop is a hard real-time job: IMU, estimator, PID, mixer, DShot. That is not a fair fight for a general-purpose OS on this mass budget.

V1 is host tests plus a documented bare-metal destination. An RTOS (Embassy or similar) can own comms and logging later. It does not own the 1 kHz path until it proves jitter.

If someone wants ROS 2 on Sparrow, they can have a later notebook and a heavier bird. This one stays a microcontroller.
