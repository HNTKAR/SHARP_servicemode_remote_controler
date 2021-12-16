use rppal::pwm;
use std::thread;
use std::time::Duration;
fn main() {
    const T: u64 = 425; //us
    let code = [0xaa, 0x5a, 0x8f, 0x30, 0xf5, 0x01];//カスタマーコード
    // let code = [0xaa, 0x5a, 0x8f, 0x12, 0x16, 0xd1];//電源
    let pwm = pwm::Pwm::new(pwm::Channel::Pwm0).expect("PWM error");
    pwm.set_frequency(38_000.0, 1.0/3.0).unwrap();
    pwm.disable().unwrap();
    thread::sleep(Duration::from_secs(1));
    // loop{
    //     pwm.enable().unwrap();
    //     thread::sleep(Duration::from_secs(3));
    //     pwm.disable().unwrap();
    //     thread::sleep(Duration::from_secs(1));
    // }

    for _loop_count in 0..3 {
    //リーダー部
    pwm.enable().unwrap();
    thread::sleep(Duration::from_micros(T * 8));
    pwm.disable().unwrap();
    thread::sleep(Duration::from_micros(T * 4));

    //データ部
    // println!("\n------");
    for i in code {
        // println!("\n{:2x}", i);
        for j in 0..8 {
            pwm.enable().unwrap();
            thread::sleep(Duration::from_micros(T));
            pwm.disable().unwrap();
            if 0b1 == (i >> j & 0b01) {
                thread::sleep(Duration::from_micros(T * 3));
                // print!("1");
            } else {
                thread::sleep(Duration::from_micros(T));
                    // print!("0");
            }
        }
    }
    pwm.enable().unwrap();
    thread::sleep(Duration::from_micros(T));
    pwm.disable().unwrap();
    thread::sleep(Duration::from_millis(10));
    thread::sleep(Duration::from_millis(150));
    }
}
