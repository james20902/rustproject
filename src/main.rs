fn main() {
    println!("hello world");
    let drive_train_data = GearboxData {
        output_ratio: (3./1.), 
        encoder_cpr: 256,
        controller_type: ControllerType::TalonSRX(),
        motor_data: MotorData{ max_rpm: 2900, encoder_present:true }
    };
}

struct GearboxData {
    output_ratio: f64,
    encoder_cpr: i32,
    controller_type: ControllerType,
    motor_data: MotorData
}

struct MotorData{
        max_rpm: i32,
        encoder_present: bool,
}

enum ControllerType {
    PWM(),
    TalonSRX(),
    VictorSPX(),
    SparkMax(),
    Falcon500()
}
