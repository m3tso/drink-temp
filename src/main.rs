use std::env;
use std::f64::consts::E;
use std::fs::read_to_string;
use std::io::stdin;
use std::time::SystemTime;
use serde::Deserialize;
use serde_json::from_str;

fn main() {
    // Starting timer as soon as program is ran.
    let start_time = SystemTime::now();

    const H: f64 = 10.0;
    const DENSITY: f64 = 1000.0;
    let config: Config = load_config();
    let volume_ml: f64 = match config.check_volume {
        true => read_volume(),
        false => 330.0
    };

    // Convert ml to m^3.
    let volume: f64 = volume_ml / 1000000.0;

    loop {
        let elapsed = start_time.elapsed().unwrap();

        let minutes: f64 = match elapsed.as_secs_f64() {
            t if t > 0.0 => t / 60.0,
            _ => 0.0,
        };

        let temperature = soda_temp(minutes,
                                    config.environment_temperature,
                                    config.initial_temperature,
                                    H,
                                    config.can_area,
                                    DENSITY, volume, config.specific_heat_capacity);

        println!("{:.1}°C", temperature);
        std::thread::sleep(std::time::Duration::from_millis(167));
        clear_terminal_screen();
    }
}

fn clear_terminal_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn soda_temp(
    t: f64,           // Time in minutes.
    t_env: f64,       // Ambient temperature in degrees Celsius.
    t_0: f64,         // Initial temperature of the soda in degrees Celsius.
    h: f64,           // Convective heat transfer coefficient in W/m²·K.
    a: f64,           // Surface area of the can in m².
    rho: f64,         // Density of the soda in kg/m³.
    v: f64,           // Volume of the soda in m³.
    c_p: f64,         // Specific heat capacity of the soda in J/kg·K.
) -> f64 {
    // Calculate the cooling constant.
    let k = (h * a) / (rho * v * c_p);

    let temp_at_time = t_env + (t_0 - t_env) * E.powf(-k * t * 60.0);

    temp_at_time
}

fn read_volume() -> f64 {
    print!("Enter volume of drink in ml: ");
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();
    let res = match buffer.trim_end() {
        "" => 330.0,
        string_value => from_str::<f64>(string_value).unwrap_or_else(|_| 330.0),
    };
    res
}

#[derive(Deserialize)]
struct Config {
    environment_temperature: f64,
    initial_temperature: f64,
    can_area: f64,
    specific_heat_capacity: f64,
    check_volume: bool,
}

fn load_config() -> Config {
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().expect("There is always a parent dir.").to_str().unwrap();
    serde_json::from_str(
        read_to_string(dir.to_owned() + "./config.json")
            .expect("Config file is found and valid.")
            .as_str())
        .expect("Config file is found.")
}
