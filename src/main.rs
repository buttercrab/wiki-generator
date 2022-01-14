use clap::Parser;
#[cfg(feature = "serve")]
use rocket_contrib::serve::StaticFiles;
use wiki_generator::config;
use wiki_generator::wiki::Wiki;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Jaeyong Sung <jaeyong0201@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    sub: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Build(Build),
    #[cfg(feature = "serve")]
    Serve(Serve),
}

#[derive(Parser, Debug)]
struct Build {}

#[cfg(feature = "serve")]
#[derive(Parser, Debug)]
struct Serve {}

async fn build() {
    let mut wiki = Wiki::new(config::get_config()).await;
    wiki.render();
}

#[cfg(feature = "serve")]
async fn serve() {
    build().await;
    rocket::ignite()
        .mount("/", StaticFiles::from("public"))
        .launch();
}

#[tokio::main]
async fn main() {
    let opt: Opts = Opts::parse();

    match opt.sub {
        SubCommand::Build(_) => build().await,
        #[cfg(feature = "serve")]
        SubCommand::Serve(_) => serve().await,
    }
}
