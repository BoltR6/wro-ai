mod bot;
mod pathfinding;
use pathfinding::visibolt::run;

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
    //run();
    let instances = wgpu::Instance::new(wgpu::Backends::all());
    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info())
    }
}
