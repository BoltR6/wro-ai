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
    fn drive_forward(voltage_percent: i8){
        assert!(voltage_percent < 100 && voltage_percent > 100, "Given percent oob!");

        //drive motor
    }
    fn drive_angle(angle_degrees: u16){
        assert!(angle_degrees > 0 && angle_degrees < 360, "Provided angle must be within 0 and 360");

        //drive angle
    }
}
fn main() {
    
}
