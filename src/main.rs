mod bot;

fn main() {
    let bot = bot::Bot {
        drive_motor: bot::motor::Motor {
            max_voltage: 125,
            min_voltage: 125,
            port: 0
        },
        angle_motor: bot::motor::Motor {
            max_voltage: 125,
            min_voltage: 125,
            port: 0
        }
    };
    bot.drive_forward(100);
    println!("{}", bot.drive_motor.max_voltage);
}
