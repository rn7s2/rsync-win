use std::{
    env, io,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Rsync for Windows

Allowed formats for <SRC>/<DEST>:
  local: C:/path/to/file
    ssh: [USER@]HOST:/path/to/file (use --ssh-port to specify the port)
  rsync: rsync://[USER@]HOST[:PORT]/path/to/file
", long_about = None)]
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
    partial: bool,

    #[clap(long)]
    progress: bool,

    #[clap(long)]
    bwlimit: Option<u64>,

    #[clap(short = '4', long)]
    ipv4: bool,

    #[clap(short = '6', long)]
    ipv6: bool,

    #[clap(long)]
    ssh_port: Option<u16>,

    #[clap(short, long)]
    src: String,

    #[clap(short, long)]
    dest: String,
}

fn main() {
    let args = Args::parse();

    let mut rsync_args = if is_ssh_path(&args.src) || is_ssh_path(&args.dest) {
        prepare_rsync_options_with_ssh(&args)
    } else {
        prepare_rsync_options(&args)
    };
    rsync_args.push(path_win_to_unix(&args.src));
    rsync_args.push(path_win_to_unix(&args.dest));

    Command::new(path_rsync())
        .args(&rsync_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("execute rsync failed");
}

fn is_ssh_path(path: &str) -> bool {
    if path.contains("rsync://") {
        false
    } else {
        path.contains('@')
    }
}

fn prepare_rsync_options_with_ssh(args: &Args) -> Vec<String> {
    fn default_identity_file() -> String {
        let mut path = home::home_dir().expect("get home dir failed");
        path.push(".ssh/id_rsa");
        path.to_str().unwrap().to_string()
    }

    fn prepare_ssh_args(identity: &str, port: &Option<u16>) -> String {
        fn path_ssh() -> io::Result<PathBuf> {
            let mut dir = path_cygwin_dir();
            dir.push("ssh.exe");
            Ok(dir)
        }

        let port = if let Some(p) = port { *p } else { 22 };
        format!(
            r#""{}" -o "StrictHostKeyChecking=no" -o "IdentitiesOnly=yes" -i "{}" -p {}"#,
            path_ssh().expect("get ssh path failed").display(),
            identity,
            port,
        )
    }

    let identity = path_win_to_unix(
        match &args.identity {
            Some(path) => path.clone(),
            None => default_identity_file(),
        }
        .as_str(),
    );

    let mut options = prepare_rsync_options(args);
    options.push("-e".to_owned());
    options.push(prepare_ssh_args(&identity, &args.ssh_port));
    options
}

fn prepare_rsync_options(args: &Args) -> Vec<String> {
    let mut options = Vec::new();
    if args.verbose {
        options.push("-v".to_owned());
    }
    if args.quiet {
        options.push("-q".to_owned());
    }
    if args.checksum {
        options.push("-c".to_owned());
    }
    if args.archive {
        options.push("-a".to_owned());
    }
    if args.recursive {
        options.push("-r".to_owned());
    }
    if args.delete {
        options.push("--delete".to_owned());
    }
    if let Some(exclude) = &args.exclude {
        options.push(format!("--exclude='{}'", exclude));
    }
    if args.partial {
        options.push("--partial".to_owned());
    }
    if args.progress {
        options.push("--progress".to_owned());
    }
    if let Some(bwlimit) = &args.bwlimit {
        options.push(format!("--bwlimit={}", bwlimit));
    }
    if args.ipv4 {
        options.push("-4".to_owned());
    }
    if args.ipv6 {
        options.push("-6".to_owned());
    }
    options
}

fn path_win_to_unix(path: &str) -> String {
    fn path_cygpath() -> io::Result<PathBuf> {
        let mut dir = path_cygwin_dir();
        dir.push("cygpath.exe");
        Ok(dir)
    }

    let out = Command::new(path_cygpath().expect("convert path failed"))
        .args(["-u", path])
        .output()
        .expect("failed to execute process");
    String::from_utf8(out.stdout)
        .expect("convert path failed: not a valid utf8 sequence")
        .trim()
        .to_string()
}

fn path_rsync() -> PathBuf {
    let mut dir = path_cygwin_dir();
    dir.push("rsync.exe");
    dir
}

fn path_cygwin_dir() -> PathBuf {
    let mut dir = env::current_exe()
        .expect("get full filesystem path of the current running executable failed");
    dir.pop();
    dir.push("cygwin64");
    dir
}
