use std::{
    env, io,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::Parser;

#[derive(Parser)]
#[clap(about = "rsync for Windows.\nMost of the options are the same as the original rsync.")]
struct Args {
    /// SSH identity file [default: "C:/Users/<YOUR USER NAME>/.ssh/id_rsa"]
    #[clap(short, long)]
    identity: Option<String>,

    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    quiet: bool,

    #[clap(short, long)]
    checksum: bool,

    #[clap(short, long)]
    archive: bool,

    #[clap(short, long)]
    recursive: bool,

    #[clap(long)]
    delete: bool,

    #[clap(long)]
    exclude: Option<String>,

    #[clap(long)]
    progress: bool,

    #[clap(long)]
    bwlimit: Option<u64>,

    #[clap(short = '4', long)]
    ipv4: bool,

    #[clap(short = '6', long)]
    ipv6: bool,

    #[clap(short, long)]
    src: String,

    #[clap(short, long)]
    dest: String,
}

fn main() {
    let args = Args::parse();
    let use_ssh = args.src.contains('@') || args.dest.contains('@');
    if use_ssh {
        rsync_with_ssh(&args);
    } else {
        rsync(&args);
    }
}

/// Executes rsync with ssh.
fn rsync_with_ssh(args: &Args) {
    let identity = path_win_to_unix(
        match &args.identity {
            Some(id) => id.clone(),
            None => default_identity(),
        }
        .as_str(),
    );

    // prepare arguments for rsync
    let mut rsync_args = Vec::new();
    rsync_args.push("-e".to_owned());
    rsync_args.push(prepare_ssh_arg(&identity));
    if args.verbose {
        rsync_args.push("-v".to_owned());
    }
    if args.quiet {
        rsync_args.push("-q".to_owned());
    }
    if args.checksum {
        rsync_args.push("-c".to_owned());
    }
    if args.archive {
        rsync_args.push("-a".to_owned());
    }
    if args.recursive {
        rsync_args.push("-r".to_owned());
    }
    if args.delete {
        rsync_args.push("--delete".to_owned());
    }
    if let Some(exclude) = &args.exclude {
        rsync_args.push(format!("--exclude='{}'", exclude));
    }
    if args.progress {
        rsync_args.push("--progress".to_owned());
    }
    if let Some(bwlimit) = &args.bwlimit {
        rsync_args.push(format!("--bwlimit={}", bwlimit));
    }
    if args.ipv4 {
        rsync_args.push("-4".to_owned());
    }
    if args.ipv6 {
        rsync_args.push("-6".to_owned());
    }
    rsync_args.push(args.src.clone());
    rsync_args.push(args.dest.clone());

    Command::new(path_rsync().expect("get rsync path failed"))
        .args(rsync_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("execute rsync failed");
}

/// Executes rsync.
fn rsync(args: &Args) {
    // prepare arguments for rsync
    let mut rsync_args = Vec::new();
    if args.verbose {
        rsync_args.push("-v".to_owned());
    }
    if args.quiet {
        rsync_args.push("-q".to_owned());
    }
    if args.checksum {
        rsync_args.push("-c".to_owned());
    }
    if args.archive {
        rsync_args.push("-a".to_owned());
    }
    if args.recursive {
        rsync_args.push("-r".to_owned());
    }
    if args.delete {
        rsync_args.push("--delete".to_owned());
    }
    if let Some(exclude) = &args.exclude {
        rsync_args.push(format!("--exclude='{}'", exclude));
    }
    if args.progress {
        rsync_args.push("--progress".to_owned());
    }
    if let Some(bwlimit) = &args.bwlimit {
        rsync_args.push(format!("--bwlimit={}", bwlimit));
    }
    if args.ipv4 {
        rsync_args.push("-4".to_owned());
    }
    if args.ipv6 {
        rsync_args.push("-6".to_owned());
    }
    rsync_args.push(args.src.clone());
    rsync_args.push(args.dest.clone());

    Command::new(path_rsync().expect("get rsync path failed"))
        .args(rsync_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("execute rsync failed");
}

fn prepare_ssh_arg(identity: &str) -> String {
    format!(
        "\"{}\" -o \"StrictHostKeyChecking=no\" -o \"IdentitiesOnly=yes\" -i \"{}\"",
        path_ssh().expect("get ssh path failed").display(),
        identity
    )
}

/// Returns the default identity file path.
fn default_identity() -> String {
    let mut path = home::home_dir().expect("get home dir failed");
    path.push(".ssh");
    path.push("id_rsa");
    path.to_str().unwrap().to_string()
}

/// Converts a Windows path to a Unix path.
fn path_win_to_unix(path: &str) -> String {
    let out = Command::new(path_cygpath().expect("convert path failed"))
        .args(["-u", path])
        .output()
        .expect("failed to execute process");
    out.stdout
        .iter()
        .map(|&x| x as char)
        .collect::<String>()
        .trim()
        .to_string()
}

/// Returns the path to the cygpath executable.
fn path_cygpath() -> io::Result<PathBuf> {
    let mut dir = path_cygwin_dir()?;
    dir.push("cygpath.exe");
    Ok(dir)
}

/// Returns the path to the rsync executable.
fn path_rsync() -> io::Result<PathBuf> {
    let mut dir = path_cygwin_dir()?;
    dir.push("rsync.exe");
    Ok(dir)
}

/// Returns the path to the ssh executable.
fn path_ssh() -> io::Result<PathBuf> {
    let mut dir = path_cygwin_dir()?;
    dir.push("ssh.exe");
    Ok(dir)
}

/// Returns the path to the cygwin directory.
fn path_cygwin_dir() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("cygwin64");
    Ok(dir)
}
