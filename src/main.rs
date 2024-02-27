mod request;
use std::fs::File;
use std::io::{self, BufRead, Write};
use clap::{App, Arg};
use env_logger;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let matches = App::new("Hostifier")
        .version("1.0")
        .author("Capitan")
        .about("Vhost discovery tool")
        .arg(Arg::with_name("wordlist")
            .short("w")
            .long("wordlist")
            .value_name("FILE")
            .help("Sets subdomain wordlist file")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("domain")
            .short("d")
            .long("domain")
            .value_name("DOMAIN")
            .help("Sets primary domain")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("target")
            .short("t")
            .long("target")
            .value_name("TARGET")
            .help("Sets the base url address to use for requests")
            .required(true)
            .takes_value(true))
        .get_matches();

    let wordlist_path = matches.value_of("wordlist").unwrap();
    let domain = matches.value_of("domain").unwrap();
    let ip_address = matches.value_of("target").unwrap();
    let wordlist = read_wordlist(wordlist_path)?;

    for subdomain in wordlist {
        let fqdn = format!("{}.{}", subdomain.trim(), domain);
        match request::fetch_url(&fqdn, ip_address).await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Vhost: {}\t {:?}", fqdn, response.status());
                }
            }
            Err(err) => {
                if err.to_string().contains("too many redirects") {
                    continue;
                }
                print_colored(format!("Error for {}: {}", fqdn, err), Color::Red);
            }
        }
    }

    Ok(())
}

fn read_wordlist(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut wordlist = Vec::new();

    for line in reader.lines() {
        wordlist.push(line?);
    }

    Ok(wordlist)
}

fn print_colored(message: String, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    writeln!(&mut stdout, "{}", message).unwrap();
    stdout.reset().unwrap();
}
