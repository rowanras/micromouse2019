use pid_control::Controller;
use pid_control::PIDController;

use crate::motors::Encoder;
use crate::motors::Motor;

pub struct MotorControl<P, M, E>
where
    P: Fn(u32) -> i32,
    M: Motor,
    E: Encoder,
{
    pid: PIDController,
    target_position: i32,
    current_position: i32,
    calculate_position: P,
    last_time: Option<u32>,
    motor: M,
    encoder: E,
}

impl<P, M, E> MotorControl<P, M, E>
where
    P: Fn(u32) -> i32,
    M: Motor,
    E: Encoder,
{
    pub fn new(
        p: f64,
        i: f64,
        d: f64,
        calculate_position: P,
        motor: M,
        encoder: E,
    ) -> MotorControl<P, M, E> {
        let mut pid = PIDController::new(p, i, d);
        pid.set_limits(-10000.0, 10000.0);

        MotorControl {
            pid,
            target_position: 0,
            current_position: 0,
            calculate_position,
            last_time: None,
            motor,
            encoder,
        }
    }

    pub fn update(&mut self, now: u32) {
        let delta_t = match self.last_time {
            Some(last_time) => now - last_time,
            None => 0,
        };

        let new_target_position = (self.calculate_position)(now);
        let new_current_position = self.encoder.count();

        self.pid.set_target(new_target_position as f64);
        let motor_velocity = self
            .pid
            .update(new_current_position as f64, delta_t as f64);

        self.motor.change_velocity(motor_velocity as i32);

        self.last_time = Some(now);
        self.target_position = new_target_position;
        self.current_position = new_current_position;
    }

    pub fn position(&self) -> i32 {
        self.current_position
    }

    pub fn error(&self) -> i32 {
        self.target_position - self.current_position
    }

    pub fn target(&self) -> i32 {
        self.target_position
    }
}
