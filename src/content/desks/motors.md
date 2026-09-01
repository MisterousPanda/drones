---
title: Motors
stamp: desk 03
summary: 1404 3700 Kv on a 15 A 4-in-1, DShot300 only.
frozen: "1404 3700 Kv · 15 A 4-in-1 · DShot300"
order: 3
---

1404 3700 Kv on 3S is the freeze. A 15 A 4-in-1 is enough if props stay 3 inch and AUW stays 245 g. DShot300 only — no analog oneshot, no “just PWM to see it spin.”

Motor map in firmware and in the mixer must be the same sentence: 0 FL, 1 FR, 2 RL, 3 RR. Spin: FL CW, FR CCW, RL CCW, RR CW.

`to_dshot` sends 0 when the castle is disarmed. Armed idle starts at 48. Full command is 2047. If you see a motor tick while disarmed, you have a bug, not a feature.
