pub mod motor;
pub mod sensor;

pub struct Bot {
    pub drive_motor: motor::Motor,
    pub angle_motor: motor::Motor,
}
impl Bot {
    pub fn drive_forward(&self, voltage_percent: i8){
        assert!(voltage_percent >= -100 && voltage_percent <= 100, "Provided percent must be within -100 and 100 inclusively!");

        //drive motor
        println!("Good");
    }
    pub fn drive_angle(&self, angle_degrees: u16){
        assert!(angle_degrees >= 0 && angle_degrees <= 360, "Provided angle must be within 0 and 360 inclusively!");

        //drive angle
        println!("Good");
    }
}