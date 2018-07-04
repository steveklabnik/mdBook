extern crate iron;
extern crate staticfile;
extern crate ws;

use std;
use self::iron::{status, AfterMiddleware, Chain, Iron, IronError, IronResult, Request, Response,
                 Set};
use mdbook::MDBook;
use mdbook::utils;
use mdbook::errors::*;
use {get_book_dir, open};
#[cfg(feature = "watch")]
use watch;

struct ErrorRecover;

#[derive(StructOpt)]
pub struct ServeArgs {
    #[structopt(help = "A directory for your book{n}(Defaults to Current Directory when omitted)")]
        dir: Option<String>,
        #[structopt(short = "p", long = "port", help = "Use another port{n}", default_value = "3000")]
        port: String,
        #[structopt(short = "w", long = "websocket-port", help = "Use another port for the websocket connection (livereload){n}", default_value = "3001")]
        ws_port: String,
        #[structopt(short = "i", long = "interface", help = "Interface to listen on{n}", default_value = "localhost")]
        interface: String,
        #[structopt(short = "a", long = "address", help = "Address that the browser can reach the websocket server from{n}(Defaults to the interface address)")]
        address: Option<String>,
        #[structopt(short = "o", long = "open",  help = "Open the compiled book in a web browser")]
        open : bool,
}

///  Serve command implementation
pub fn execute(args: ServeArgs) -> Result<()> {
    let book_dir = get_book_dir(args.dir);
    let address = format!("{}:{}", args.interface, args.port);
    let ws_address = format!("{}:{}", args.interface, args.ws_port);
    let livereload_url = format!(
        "ws://{}:{}",
        args.address.unwrap_or(args.interface),
        args.ws_port
    );

    let mut book = MDBook::load(&book_dir)?;
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

    if args.open {
        open(serving_url);
    }
    #[cfg(feature = "watch")]
    watch::trigger_on_change(&mut book, move |path, book_dir| {
        info!("File changed: {:?}", path);
        info!("Building book...");

        // FIXME: This area is really ugly because we need to re-set livereload :(

        let livereload_url = livereload_url.clone();

        let result = MDBook::load(&book_dir)
            .and_then(move |mut b| {
                b.config.set("output.html.livereload-url", &livereload_url)?;
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
