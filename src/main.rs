use std::{env, io, thread, time};
use std::io::Write;

fn format_duration(d : time::Duration) -> String {
    let d_secs = d.as_secs();
    let hours = d_secs / 3600;
    let minutes = d_secs % 3600 / 60;
    let seconds = d_secs % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

fn print_help() {
    println!("Usage: sleepcount NUMBER[SUFFIX]...
    Pause for NUMBER seconds, where NUMBER is an integer or floating-point.
    SUFFIX may be 's','m','h', or 'd', for seconds, minutes, hours, days.
    With multiple arguments, pause for the sum of their values.
    ");
}

fn get_time_to_sleep() -> Result<time::Duration, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        print_help();
        return Err("missing operand".to_string());
    }
    let mut total_secs = 0u64;
    for arg in &args[1..] {
        let mut multiplier = 1;
        let mut arg_without_suffix = arg.clone();
        if arg_without_suffix.ends_with("d") {
            arg_without_suffix.pop();
            multiplier = 24 * 3600;
        } else if arg_without_suffix.ends_with("h") {
            arg_without_suffix.pop();
            multiplier = 3600;
        } else if arg_without_suffix.ends_with("m") {
            arg_without_suffix.pop();
            multiplier = 60;
        } else if arg_without_suffix.ends_with("s") {
            arg_without_suffix.pop();
        }
        let num = match arg_without_suffix.parse::<u64>() {
            Ok(num) => Ok(num),
            Err(_) => {
                print_help();
                Err(format!("invalid time specification: {}", arg))
            },
        }?;
        total_secs += num * multiplier;
    }
    return Ok(time::Duration::from_secs(total_secs));
}

fn main() -> Result<(), String> {
    let duration = get_time_to_sleep()?;
    let target_time = time::Instant::now() + duration;
    let mut current_time = time::Instant::now();
    let mut sleep_target_time = current_time;

    if current_time >= target_time {
        return Ok(());
    }

    let mut remaining_time_str = format_duration(target_time - current_time);
    print!("{}", remaining_time_str);
    let _ = io::stdout().flush();

    while current_time < target_time {
        sleep_target_time += time::Duration::from_secs(1);
        current_time = time::Instant::now();
        if sleep_target_time < current_time {
            sleep_target_time += time::Duration::from_secs((current_time - sleep_target_time).as_secs());
            continue;
        }
        thread::sleep(sleep_target_time - current_time);
        current_time = time::Instant::now();

        let last_remaining_time_str_len = remaining_time_str.len();
        remaining_time_str = format_duration(target_time - current_time);
        let cur_remaining_time_str_len = remaining_time_str.len();
        for _ in 0..last_remaining_time_str_len {
            print!("\x08");
        }
        print!("{}", remaining_time_str);
        let chars_to_erase = last_remaining_time_str_len - cur_remaining_time_str_len;
        for _ in 0..chars_to_erase {
            print!(" ");
        }
        for _ in 0..chars_to_erase {
            print!("\x08");
        }
        let _ = io::stdout().flush();
    }

    println!();
    return Ok(());
}
