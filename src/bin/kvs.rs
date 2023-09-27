use kvs::KvStore;

fn main() {
    let matches = clap::Command::new(env!("CARGO_PKG_NAME"))
        .bin_name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            clap::command!("get")
                .about("get value from key")
                .arg(clap::arg!(<KEY> "KEY")),
        )
        .subcommand(
            clap::command!("set")
                .about("set value by key")
                .arg(clap::arg!(<KEY> "KEY"))
                .arg(clap::arg!(<VALUE> "VALUE")),
        )
        .subcommand(
            clap::command!("rm")
                .about("remove value of key")
                .arg(clap::arg!(<KEY> "KEY")),
        )
        .get_matches();

    let mut store = KvStore::new();
    match matches.subcommand() {
        Some(("get", matches)) => {
            let key = matches.get_one::<String>("KEY").unwrap();
            store.get(key.to_owned());
        }
        Some(("set", matches)) => {
            let key = matches.get_one::<String>("KEY").unwrap();
            let value = matches.get_one::<String>("VALUE").unwrap();
            store.set(key.to_owned(), value.to_owned());
        }
        Some(("rm", matches)) => {
            let key = matches.get_one::<String>("KEY").unwrap();
            store.remove(key.to_owned());
        }
        _ => unreachable!("!"),
    };
}
