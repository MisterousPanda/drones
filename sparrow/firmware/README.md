# firmware

STM32F405 + ICM-42688-P target shell. Excluded from the default Cargo workspace until a board support package is wired.

V1 ships the host-tested laws in `flight-core` (mixer, PID, Mahony, arming, LVC). This folder is the documented place those laws will sit on the bird: `no_std`, DShot300, props off for bench bring-up.
