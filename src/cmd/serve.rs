#[cfg(feature = "watch")]
use super::watch;
use crate::{get_book_dir, open};
use clap::{App, Arg, ArgMatches, SubCommand};
use iron::{status, AfterMiddleware, Chain, Iron, IronError, IronResult, Request, Response, Set};
use mdbook::errors::*;
use mdbook::utils;
use mdbook::MDBook;

struct ErrorRecover;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("serve")
        .about("Serves a book at http://localhost:3000, and rebuilds it on changes")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'Output directory for the book{n}\
             Relative paths are interpreted relative to the book's root directory.{n}\
             If omitted, mdBook uses build.build-dir from book.toml or defaults to `./book`.'",
        )
        .arg_from_usage(
            "[dir] 'Root directory for the book{n}\
             (Defaults to the Current Directory when omitted)'",
        )
        .arg(
            Arg::with_name("hostname")
                .short("n")
                .long("hostname")
                .takes_value(true)
                .default_value("localhost")
                .empty_values(false)
                .help("Hostname to listen on for HTTP connections"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("3000")
                .empty_values(false)
                .help("Port to use for HTTP connections"),
        )
        .arg(
            Arg::with_name("websocket-hostname")
                .long("websocket-hostname")
                .takes_value(true)
                .empty_values(false)
                .help(
                    "Hostname to connect to for WebSockets connections (Defaults to the HTTP hostname)",
                ),
        )
        .arg(
            Arg::with_name("websocket-port")
                .short("w")
                .long("websocket-port")
                .takes_value(true)
                .default_value("3001")
                .empty_values(false)
                .help("Port to use for WebSockets livereload connections"),
        )
        .arg(
            Arg::with_name("websocket-url")
                .long("websocket-url")
                .takes_value(true)
                .empty_values(false)
                .help("URL to use for WebSockets livereload connections (Defaults to 'ws://{websocket-hostname}:{websocket-port})')"),
        )
        .arg_from_usage("-o, --open 'Opens the book server in a web browser'")
}

// Watch command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(&book_dir)?;

    let port = args.value_of("port").unwrap();
    let hostname = args.value_of("hostname").unwrap();
    let ws_port = args.value_of("websocket-port").unwrap();
    let ws_hostname = args.value_of("websocket-hostname").unwrap_or(hostname);
    let ws_url = match args.value_of("websocket-url") {
        Some(url) => String::from(url),
        None => format!("ws://{}:{}", ws_hostname, ws_port)
    };
    let ws_url = &ws_url[..];
    let open_browser = args.is_present("open");

    let address = format!("{}:{}", hostname, port);
    let ws_address = format!("{}:{}", hostname, ws_port);

    book.config
        .set("output.html.livereload-url", &ws_url)?;

    if let Some(dest_dir) = args.value_of("dest-dir") {
        book.config.build.build_dir = dest_dir.into();
    }

    book.build()?;

    let mut chain = Chain::new(staticfile::Static::new(book.build_dir_for("html")));
    chain.link_after(ErrorRecover);
    let _iron = Iron::new(chain)
        .http(&*address)
        .chain_err(|| "Unable to launch the server")?;

    let ws_server =
        ws::WebSocket::new(|_| |_| Ok(())).chain_err(|| "Unable to start the websocket")?;

    let broadcaster = ws_server.broadcaster();

    std::thread::spawn(move || {
        ws_server.listen(&*ws_address).unwrap();
    });

    let serving_url = format!("http://{}", address);
    info!("Serving on: {}", serving_url);

    if open_browser {
        open(serving_url);
    }

    #[cfg(feature = "watch")]
    watch::trigger_on_change(&book, move |paths, book_dir| {
        info!("Files changed: {:?}", paths);
        info!("Building book...");

        // FIXME: This area is really ugly because we need to re-set livereload :(

        let result = MDBook::load(&book_dir)
            .and_then(|mut b| {
                b.config
                    .set("output.html.livereload-url", &ws_url)?;
                Ok(b)
            })
            .and_then(|b| b.build());

        if let Err(e) = result {
            error!("Unable to load the book");
            utils::log_backtrace(&e);
        } else {
            let _ = broadcaster.send("reload");
        }
    });

    Ok(())
}

impl AfterMiddleware for ErrorRecover {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        match err.response.status {
            // each error will result in 404 response
            Some(_) => Ok(err.response.set(status::NotFound)),
            _ => Err(err),
        }
    }
}
