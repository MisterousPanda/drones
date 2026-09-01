# Sparrow Notes

A pencil-drawn field guide for **Sparrow**, a 245 g educational quadrotor programmed in **Rust**. Nine specialist desks — battery, rotors, motors, weight, aerodynamics, controls, firmware, microprocessors, operating systems — researched the bird. A chief engineer froze one spec. This site is that notebook.

Built with **Astro**. V1 is the static field guide plus host-tested `flight-core` laws.

## Frozen spec

| Desk | What we fly |
| --- | --- |
| Battery | 3S 650 mAh LiPo, 50–52 g, XT30 |
| Rotors | 3.0×2.0 in 2-blade plastic, full rings |
| Motors | 1404 3700 Kv, 15 A 4-in-1, DShot300 |
| Airframe | 245 g AUW |
| Brain | STM32F405 + ICM-42688-P, Rust `no_std` |

Never cut motors in the air. Charge LiPos in a fire bag. Props off for bench tests.

## Site

```bash
npm install
npm run dev
```

Production build:

```bash
npm run build
```

## Flight math (laptop)

```bash
cd sparrow
cargo test -p flight-core
cargo run -p sim
```

`firmware/` is a documented shell for the STM32 target. It is excluded from the default workspace until a board support package is wired. The mixer, PID, Mahony filter, arming castle, and LVC policy are host-tested in `flight-core`.

## Safety

Props are sharp. Packs can fire. Fly indoors first. Fingers stay outside the disc.
