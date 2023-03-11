pub struct Motor {
    max_voltage: u8,
    min_voltage: i8,
    port: i8,
}
pub struct Bot {
    drive_motor: Motor,
    angle_motor: Motor,
}
impl Bot {
    pub fn drive_forward(&self, voltage_percent: i8){
        assert!(voltage_percent >= -100 && voltage_percent <= 100, "Given percent oob!");

        //drive motor
        print!("Good");
    }
    pub fn drive_angle(&self, angle_degrees: u16){
        assert!(angle_degrees >= 0 && angle_degrees <= 360, "Provided angle must be within 0 and 360");

        //drive angle
        print!("Good");
    }
}
fn main() {
    let bot = Bot {
        drive_motor: Motor {
            max_voltage: 125,
            min_voltage: 125,
            port: 0
        },
        angle_motor: Motor {
            max_voltage: 125,
            min_voltage: 125,
            port: 0
        }
    };
    bot.drive_forward(100);
}
