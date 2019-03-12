use std::fs::File;
use std::process::Command;

use i3ipc::I3Connection;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(long = "blur-sigma", default_value = "20")]
    blur_sigma: usize,

    swaylock_args: Vec<String>,
}

fn main() {
    let args = Args::from_args();

    let outputs: Vec<i3ipc::reply::Output> = I3Connection::connect()
        .expect("failed to connect to i3/Sway ipc")
        .get_outputs()
        .expect("failed to get outputs")
        .outputs
        .into_iter()
        .filter(|output| output.active)
        .collect();

    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");

    let mut swaylock_args = args.swaylock_args.clone();

    outputs.iter().enumerate().for_each(|(i, output)| {
        let screenshot_path_string = temp_dir
            .path()
            .join(format!("{}.png", i))
            .to_string_lossy()
            .to_string();
        let blur_path_string = temp_dir
            .path()
            .join(format!("{}blur.png", i))
            .to_string_lossy()
            .to_string();
        File::create(&screenshot_path_string).expect("failed to create tempfile");
        Command::new("grim")
            .args(&["-o", &output.name, &screenshot_path_string])
            .spawn()
            .expect("failed to execute grim")
            .wait()
            .expect("failed to wait on grim");
        Command::new("ffmpeg")
            .args(&[
                "-i",
                &screenshot_path_string,
                "-vf",
                &format!("gblur=sigma={}", args.blur_sigma),
                &blur_path_string,
            ])
            .spawn()
            .expect("failed to execute ffmpeg")
            .wait()
            .expect("failed to wait on ffmpeg");
        swaylock_args.append(&mut vec![
            "-i".to_string(),
            format!("{}:{}", &output.name, &blur_path_string),
        ]);
    });

    Command::new("swaylock")
        .args(&mut swaylock_args)
        .spawn()
        .expect("failed to execute swaylock")
        .wait()
        .expect("failed to wait on swaylock");
}
