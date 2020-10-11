use clap::Clap;
#[cfg(feature = "serve")]
use rocket_contrib::serve::StaticFiles;
use wiki_generator::config::config;
use wiki_generator::wiki::wiki::Wiki;

#[derive(Clap)]
#[clap(version = "0.1", author = "Jaeyong Sung <jaeyong0201@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    sub: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Build(Build),
    #[cfg(feature = "serve")]
    Serve(Serve),
}

#[derive(Clap)]
struct Build {}

#[cfg(feature = "serve")]
#[derive(Clap)]
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
