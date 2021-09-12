//
// Rustcat (rc)
// Licence: MIT
//

use structopt::StructOpt;

mod input;
mod listener;
mod utils;

#[cfg(unix)]
mod unixshell;

fn main() {
    let opts = input::Opts::from_args();

    let (opt_host, opt_port) = if opts.port != None {
        ("0.0.0.0".to_string(), opts.port.unwrap())
    } else if opts.host.len() == 2 {
        (opts.host[0].to_string(), opts.host[1].to_string())
    } else {
        utils::print_error("Missing port number".to_string());
        return;
    };

    // Reverse Shell
    if opts.rshell != None {
        // Block usage on windows
        if cfg!(windows) {
            utils::print_error("Reverse shells is currently not supported for windows".to_string());
            return;
        }

        #[cfg(unix)]
        if let Err(err) = unixshell::shell(opt_host, opt_port, opts.rshell.unwrap()) {
            utils::print_error(err.to_string());
        }
        return;
    }
    // Listen mode
    else if opts.listen_mode {
        let opts = utils::Opts {
            host: opt_host,
            port: opt_port,
            transport: if opts.udp_mode {
                utils::Protocol::Udp
            } else {
                utils::Protocol::Tcp
            },
            mode: if opts.history {
                utils::Mode::History
            } else {
                utils::Mode::Normal
            },
        };

        if let Err(err) = listener::listen(&opts) {
            utils::print_error(err.to_string());
            return;
        };
    }
}
