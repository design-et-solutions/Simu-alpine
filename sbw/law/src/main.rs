use ac::*;
use anyhow::Result;
use device::*;
use rand::Rng;
use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use vjoy_sys::*;
use window::*;
use windows::Win32::Devices::HumanInterfaceDevice::{HID_USAGE_GENERIC_X, IDirectInputEffect};

mod ac;
mod device;
mod model;
mod window;

#[derive(Debug)]
struct FFBData {
    speed_kmh: f32,
}

#[derive(Debug)]
pub struct SteeringTable {
    pub wheel_angles: [[f32; 14]; 13],
    pub key_steer_angle: [i32; 13],
    pub key_speed: [i32; 14],
    pub max_wheel_angle: f32,
    pub scalling_factor: f32,
}

impl SteeringTable {
    pub fn new() -> Self {
        let max_physical_steer = 45.0; // your wheel ±45°
        let old_max_steer = 210.0; // original table max steer
        let key_steer_angle = [0, 10, 15, 20, 25, 30, 40, 50, 70, 90, 120, 160, 210];
        let key_speed = [0, 5, 30, 50, 60, 70, 90, 110, 130, 160, 200, 250, 300, 350];

        let values = [
            [0.0; 14],
            [
                1.53846154, 1.53846154, 1.53846154, 1.33333333, 1.25, 1.17647059, 1.05263158,
                0.95238095, 0.86956522, 0.76923077, 0.74074074, 0.74074074, 0.74074074, 0.74074074,
            ],
            [
                2.30769231, 2.30769231, 2.30769231, 2.0, 1.875, 1.76470588, 1.57894737, 1.42857143,
                1.30434783, 1.15384615, 1.11111111, 1.11111111, 1.11111111, 1.11111111,
            ],
            [
                3.07692308, 3.07692308, 3.07692308, 2.66666667, 2.5, 2.35294118, 2.10526316,
                1.9047619, 1.73913043, 1.53846154, 1.48148148, 1.48148148, 1.48148148, 1.48148148,
            ],
            [
                3.84615385, 3.84615385, 3.84615385, 3.33333333, 3.125, 2.94117647, 2.63157895,
                2.38095238, 2.17391304, 1.92307692, 1.86769814, 1.86769814, 1.86769814, 1.86769814,
            ],
            [
                4.61538462, 4.61538462, 4.61538462, 4.0, 3.75, 3.52941176, 3.15789474, 2.85714286,
                2.60869565, 2.39020536, 2.39020536, 2.39020536, 2.39020536, 2.39020536,
            ],
            [
                6.15384615, 6.15384615, 6.15384615, 5.33333333, 5.0, 4.70588235, 4.21052632,
                3.80952381, 3.68771311, 3.68771311, 3.68771311, 3.68771311, 3.68771311, 3.68771311,
            ],
            [
                7.69230769, 7.69230769, 7.69230769, 6.66666667, 6.25, 5.88235294, 5.26315789,
                4.91282766, 4.91282766, 4.91282766, 4.91282766, 4.91282766, 4.91282766, 4.91282766,
            ],
            [
                10.7692308, 10.7692308, 10.7692308, 9.33333333, 8.75, 8.23529412, 7.6343775,
                7.6343775, 7.6343775, 7.6343775, 7.6343775, 7.6343775, 7.6343775, 7.6343775,
            ],
            [
                13.8461538, 13.8461538, 13.8461538, 12.0, 11.25, 10.7737694, 10.7737694,
                10.7737694, 10.7737694, 10.7737694, 10.7737694, 10.7737694, 10.7737694, 10.7737694,
            ],
            [
                18.4615385, 18.4615385, 18.4615385, 16.0, 15.307668, 15.307668, 15.307668,
                15.307668, 15.307668, 15.307668, 15.307668, 15.307668, 15.307668, 15.307668,
            ],
            [
                24.6153846, 24.6153846, 24.6153846, 22.2863036, 22.2863036, 22.2863036, 22.2863036,
                22.2863036, 22.2863036, 22.2863036, 22.2863036, 22.2863036, 22.2863036, 22.2863036,
            ],
            [
                32.3076923, 32.3076923, 32.3076923, 32.3076923, 32.3076923, 32.3076923, 32.3076923,
                32.3076923, 32.3076923, 32.3076923, 32.3076923, 32.3076923, 32.3076923, 32.3076923,
            ],
        ];

        Self {
            wheel_angles: values,
            key_steer_angle,
            key_speed,
            max_wheel_angle: 32.3076923,
            scalling_factor: old_max_steer / max_physical_steer,
        }
    }

    pub fn get_wheel_angle(&self, speed: f32, steer_angle: f32) -> f32 {
        let sign = steer_angle.signum(); // Save the original sign
        let steer_abs = steer_angle.abs() * self.scalling_factor; // Use absolute value for table lookup

        // Find closest speed indices in the key table
        let s_idx = match self
            .key_speed
            .binary_search_by(|v| v.partial_cmp(&(speed as i32)).unwrap())
        {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let s_next = (s_idx + 1).min(self.key_speed.len() - 1);
        let speed0 = self.key_speed[s_idx] as f32;
        let speed1 = self.key_speed[s_next] as f32;
        let t_speed = if speed1 - speed0 == 0.0 {
            0.0
        } else {
            (speed - speed0) / (speed1 - speed0)
        };

        // Find closest steer angle indices
        let a_idx = match self
            .key_steer_angle
            .binary_search_by(|v| v.partial_cmp(&(steer_abs as i32)).unwrap())
        {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let a_next = (a_idx + 1).min(self.key_steer_angle.len() - 1);
        let steer0 = self.key_steer_angle[a_idx] as f32;
        let steer1 = self.key_steer_angle[a_next] as f32;
        let t_steer = if steer1 - steer0 == 0.0 {
            0.0
        } else {
            (steer_abs - steer0) / (steer1 - steer0)
        };

        // Bilinear interpolation
        let v00 = self.wheel_angles[a_idx][s_idx];
        let v10 = self.wheel_angles[a_next][s_idx];
        let v01 = self.wheel_angles[a_idx][s_next];
        let v11 = self.wheel_angles[a_next][s_next];

        let interp_steer0 = v00 * (1.0 - t_steer) + v10 * t_steer;
        let interp_steer1 = v01 * (1.0 - t_steer) + v11 * t_steer;
        let wheel_angle = interp_steer0 * (1.0 - t_speed) + interp_steer1 * t_speed;

        wheel_angle * sign
    }
}

type DWORD = u32;
const HID_USAGE_X: u32 = 0x30;

fn main() -> Result<()> {
    let table = SteeringTable::new();
    unsafe {
        let vjoy = vJoyInterface::new(Path::new(r"C:\Program Files\vJoy\x64\vJoyInterface.dll"))?;
        let device_id = 1;
        if vjoy.AcquireVJD(device_id) == 0 {
            panic!("Failed to acquire vJoy device {}", device_id);
        }
        let hwnd = create_window("Design & Solutions: Law", "WindowClass")?;
        let di = initialize_dirent_input()?;
        let (_name, instance) = found_device(&di)?;
        let device = create_device(&di, instance, hwnd)?;

        // let start = Instant::now();
        loop {
            if let Some(data) = read_ac_data() {
                let speed = data.speed_kmh;
                let steer_angle = read_axis_x(&device)?;

                let wheel_angle = table.get_wheel_angle(speed, steer_angle);

                // Normalize the *target wheel angle* for vJoy
                let normalized = (wheel_angle / table.max_wheel_angle).clamp(-1.0, 1.0);

                let vjoy_value = float_to_vjoy_axis(normalized);
                let result = vjoy.SetAxis(vjoy_value as i32, device_id as u32, HID_USAGE_X);
                print!("\x1B[2J\x1B[1;1H"); // ANSI escape: clear screen + move cursor to top-left
                if result == 0 {
                    eprintln!("Failed to set vJoy axis");
                }
                println!(
                    "\rCar Speed: {:6.1} km/h | Steer Angle: {:6.1}° | Target Wheel Angle: {:6.1}° | vJoy value: {:5}",
                    speed, steer_angle, wheel_angle, normalized,
                );
                std::io::stdout().flush().unwrap();
            }
            std::thread::sleep(std::time::Duration::from_millis(3));
        }
    }
}

unsafe fn float_to_vjoy_axis(value: f32) -> u32 {
    let clamped = value.clamp(-1.0, 1.0);
    ((clamped + 1.0) / 2.0 * 32768 as f32) as u32
}
