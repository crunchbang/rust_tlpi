use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, read, write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 || args[1] == String::from("--help") {
        println!("usage: {} old-file new-file", args[0]);
        return;
    }
    let src_path = &args[1];
    let dest_path = &args[2];
    cp(&src_path, &dest_path);
}

// TODO: Figure out how to pass the error code back to the shell
fn cp(src_path: &str, dest_path: &str) {
    let src_file = match open(&src_path[..], OFlag::O_RDONLY, Mode::empty()) {
        Ok(n) => n,
        Err(e) => {
            println!("error {}", e.desc());
            return;
        }
    };

    let open_flags = OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_TRUNC;
    let file_perm = Mode::S_IRUSR
        | Mode::S_IWUSR
        | Mode::S_IRGRP
        | Mode::S_IWGRP
        | Mode::S_IROTH
        | Mode::S_IWOTH;

    let dest_file = match open(&dest_path[..], open_flags, file_perm) {
        Ok(n) => n,
        Err(e) => {
            println!("error {}", e.desc());
            return;
        }
    };

    const BUF_SIZE: usize = 1024;
    let mut buffer = [0; BUF_SIZE];

    loop {
        let nread = match read(src_file, &mut buffer) {
            Ok(0) => break, // This is AMAZING!
            Ok(n) => n,
            Err(e) => {
                println!("error {}", e.desc());
                return;
            }
        };

        println!("read {}usize", nread);

        let nwrite = match write(dest_file, &buffer[..nread]) {
            Ok(n) => {
                if n != nread {
                    println!("mismatch in the amount of data written");
                    break;
                } else {
                    n
                }
            }
            Err(e) => {
                println!("error {}", e.desc());
                return;
            }
        };

        println!("wrote {}usize", nwrite);
    }
    println!("copied data from {} to {}", src_path, dest_path);

    match close(src_file) {
        Err(e) => println!("error {}", e.desc()),
        _ => (),
    }

    match close(dest_file) {
        Err(e) => println!("error {}", e.desc()),
        _ => (),
    }
}
