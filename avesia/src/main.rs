use avesia_desktop::AvesiaDesktopOptions;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn help() {
    println!("Usage: avesia desktop|server");
    println!("About: avesia {:?}", PKG_VERSION);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            if cfg!(feature = "desktop") {
                avesia_desktop::AvesiaDesktop::new().start().unwrap();
            } else if cfg!(feature = "server") {
                todo!()
            } else {
                help();
            }
        },
        n => {
            let cmd = &args[1];
            match &cmd[..] {
                "desktop" => {
                    match n {
                        3 => {
                            avesia_desktop::AvesiaDesktop::new()
                                .with_options(AvesiaDesktopOptions {
                                    url: args[2].clone()
                                })
                                .start()
                                .unwrap()
                        },
                        _ => {
                            avesia_desktop::AvesiaDesktop::new().start().unwrap()
                        }
                    }
                },
                "server" => {
                    todo!()
                }
                _ => {
                    help();
                }
            }
        }
    }
}
