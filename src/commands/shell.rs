use clap::Args;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::utils::paths::Paths;

use super::Runnable;

#[derive(Args, Debug)]
pub struct ShellCmd {
    #[arg(trailing_var_arg = true)]
    pub message: Vec<String>,

    #[arg(short, long)]
    pub daemon: bool,

    #[arg(short, long)]
    pub show: bool,

    #[arg(short, long)]
    pub log: bool,

    #[arg(short, long)]
    pub kill: bool,

    #[arg(long = "log-rules", value_name = "RULES")]
    pub log_rules: Option<String>,
}

impl ShellCmd {
    fn call_qs(&self, args: &[&str]) -> Result<String, Box<dyn Error>> {
        let mut cmd_args = vec!["-c", "ferret"];
        cmd_args.extend(args);

        let output = Command::new("qs").args(&cmd_args).output()?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            dbg!(&output);
            return Err(format!("Command failed: {}", err).into());
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    fn should_print_log(&self, line: &str, paths: &Paths) -> bool {
        let filter_str = format!(
            "Cannot open: file://{}/imagecache/",
            paths.f_cache_dir.to_string_lossy()
        );
        !line.contains(&filter_str)
    }

    fn start_shell(&self, paths: &Paths) -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::new("qs");
        cmd.args(["-c", "ferret", "-n"]);

        if let Some(rules) = &self.log_rules {
            cmd.args(["--log-rules", rules]);
        }

        if self.daemon {
            cmd.arg("-d");
            cmd.spawn()?.wait()?;
        } else {
            cmd.stdout(Stdio::piped());

            let mut child = cmd.spawn()?;

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);

                for line in reader.lines() {
                    let line = line?;
                    if self.should_print_log(&line, paths) {
                        println!("{}", line);
                    }
                }
            }
            child.wait()?;
        }
        Ok(())
    }
}

impl Runnable<&Paths> for ShellCmd {
    fn run(&self, paths: &Paths) -> Result<(), Box<dyn Error>> {
        match self {
            s if s.show => {
                let output = s.call_qs(&["ipc", "show"])?;
                print!("{}", output);
            }

            s if s.log => {
                let mut args = vec!["log"];
                if let Some(rules) = &s.log_rules {
                    args.push("-r");
                    args.push(rules);
                }
                let output = s.call_qs(&args)?;
                for line in output.lines() {
                    if s.should_print_log(line, paths) {
                        println!("{}", line);
                    }
                }
            }

            s if s.kill => {
                s.call_qs(&["kill"])?;
            }

            s if !s.message.is_empty() => {
                let mut args = vec!["ipc", "call"];
                let msg_refs: Vec<&str> = s.message.iter().map(|x| x.as_str()).collect();
                args.extend(msg_refs);
                let output = s.call_qs(&args)?;
                print!("{}", output);
            }

            _ => self.start_shell(paths)?,
        }

        Ok(())
    }
}
