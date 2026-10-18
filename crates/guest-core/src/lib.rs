//! The guest game's rules, rebuilt in plain Rust.
//!
//! Nothing here knows about the host game or Windows. The host bridge feeds a [`HostInput`] every
//! tick and applies the returned [`CoreOutput`]. Units are metres, seconds and radians (see docs/CONTRACT.md).
//!
//! The jump model below is a starting example of one mechanic. Replace or extend it with the
//! guest game's real systems.

/// Fixed tick rate for the core, independent of the host frame rate. Set it to the guest game's rate.
pub const TICK_HZ: f32 = 60.0;
pub const TICK_DT: f32 = 1.0 / TICK_HZ;

/// Vertical movement tuning.
///
/// UNVERIFIED: the defaults are placeholders, not measured guest-game values. Replace them with numbers
/// measured from guest-game footage or found in its movement code, and note the source here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JumpParams {
    /// Upward speed at the moment of the jump, m/s.
    pub jump_velocity: f32,
    /// Downward acceleration, m/s².
    pub gravity: f32,
    /// Fastest fall speed, m/s (positive number).
    pub max_fall_speed: f32,
}

impl JumpParams {
    /// Derive jump speed and gravity from what can be measured in footage: how high the jump goes
    /// and how long it takes to get there.
    pub fn from_apex(height: f32, time_to_apex: f32, max_fall_speed: f32) -> Self {
        Self {
            jump_velocity: 2.0 * height / time_to_apex,
            gravity: 2.0 * height / (time_to_apex * time_to_apex),
            max_fall_speed,
        }
    }
}

impl Default for JumpParams {
    fn default() -> Self {
        // UNVERIFIED placeholder: 1.6 m apex reached in 0.4 s.
        Self::from_apex(1.6, 0.4, 20.0)
    }
}

/// What the host tells the core each tick.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct HostInput {
    /// Jump button went down this tick.
    pub jump_pressed: bool,
    /// The host's collision says the player is standing on something.
    pub grounded: bool,
}

/// What the core asks the host to do each tick.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CoreOutput {
    /// Vertical velocity to apply, m/s, positive is up.
    pub vertical_velocity: f32,
    pub airborne: bool,
}

/// Guest-style vertical movement state (example mechanic).
#[derive(Debug, Clone, Default)]
pub struct Movement {
    pub params: JumpParams,
    vertical_velocity: f32,
    airborne: bool,
}

impl Movement {
    pub fn new(params: JumpParams) -> Self {
        Self { params, ..Self::default() }
    }

    /// Advance one fixed tick.
    pub fn step(&mut self, input: HostInput) -> CoreOutput {
        if input.grounded && self.vertical_velocity <= 0.0 {
            self.vertical_velocity = 0.0;
            self.airborne = false;
        }

        if input.jump_pressed && !self.airborne {
            self.vertical_velocity = self.params.jump_velocity;
            self.airborne = true;
        } else if self.airborne || !input.grounded {
            self.airborne = true;
            self.vertical_velocity = (self.vertical_velocity - self.params.gravity * TICK_DT)
                .max(-self.params.max_fall_speed);
        }

        CoreOutput { vertical_velocity: self.vertical_velocity, airborne: self.airborne }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simulate_jump(params: JumpParams) -> (f32, f32) {
        let mut m = Movement::new(params);
        m.step(HostInput { jump_pressed: true, grounded: true });
        let (mut height, mut apex, mut ticks) = (0.0f32, 0.0f32, 0u32);
        loop {
            let out = m.step(HostInput { jump_pressed: false, grounded: false });
            height += out.vertical_velocity * TICK_DT;
            ticks += 1;
            if out.vertical_velocity <= 0.0 {
                apex = apex.max(height);
                return (apex, ticks as f32 * TICK_DT);
            }
            apex = apex.max(height);
        }
    }

    #[test]
    fn from_apex_round_trips() {
        let p = JumpParams::from_apex(1.6, 0.4, 20.0);
        let (apex, time) = simulate_jump(p);
        assert!((apex - 1.6).abs() < 0.1, "apex {apex}");
        assert!((time - 0.4).abs() < 2.0 * TICK_DT, "time {time}");
    }

    #[test]
    fn fall_speed_is_capped() {
        let mut m = Movement::new(JumpParams::default());
        let mut out = CoreOutput::default();
        for _ in 0..600 {
            out = m.step(HostInput { jump_pressed: false, grounded: false });
        }
        assert_eq!(out.vertical_velocity, -m.params.max_fall_speed);
    }

    #[test]
    fn landing_stops_falling_and_allows_another_jump() {
        let mut m = Movement::new(JumpParams::default());
        m.step(HostInput { jump_pressed: true, grounded: true });
        for _ in 0..60 {
            m.step(HostInput { jump_pressed: false, grounded: false });
        }
        let landed = m.step(HostInput { jump_pressed: false, grounded: true });
        assert!(!landed.airborne);
        assert_eq!(landed.vertical_velocity, 0.0);
        let jumped = m.step(HostInput { jump_pressed: true, grounded: true });
        assert!(jumped.airborne && jumped.vertical_velocity > 0.0);
    }

    #[test]
    fn no_jump_in_mid_air() {
        let mut m = Movement::new(JumpParams::default());
        m.step(HostInput { jump_pressed: true, grounded: true });
        let before = m.step(HostInput { jump_pressed: false, grounded: false });
        let after = m.step(HostInput { jump_pressed: true, grounded: false });
        assert!(after.vertical_velocity < before.vertical_velocity);
    }
}
