use std::env;
use std::fs::File;
use std::process::Command;

use i3ipc::I3Connection;

fn main() {
    let outputs: Vec<i3ipc::reply::Output> = I3Connection::connect()
        .expect("failed to connect to i3/Sway ipc")
        .get_outputs()
        .expect("failed to get outputs")
        .outputs
        .into_iter()
        .filter(|output| output.active)
        .collect();

    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");

    let mut lock_command = vec!["swaylock".to_string()];

    outputs.iter().enumerate().for_each(|(i, output)| {
        let path_string = temp_dir
            .path()
            .join(i.to_string())
            .to_string_lossy()
            .to_string();
        File::create(&path_string).expect("failed to create tempfile");
        Command::new("grim")
            .args(&["-o", &output.name, &path_string])
            .spawn()
            .expect("failed to execute grim")
            .wait()
            .expect("failed to wait on grim");
        Command::new("convert")
            .args(&[&path_string, "-blur", "0x8", &path_string])
            .spawn()
            .expect("failed to execute imagemagick")
            .wait()
            .expect("failed to wait on imagemagick");
        lock_command.append(&mut vec![
            "-i".to_string(),
            format!("{}:{}", &output.name, &path_string),
        ]);
    });

    lock_command.append(&mut env::args().collect::<Vec<String>>()[1..].to_owned());

    Command::new(&lock_command[0])
        .args(&lock_command[1..])
        .spawn()
        .expect("failed to execute swaylock")
        .wait()
        .expect("failed to wait on swaylock");
}
