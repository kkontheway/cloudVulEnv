use std::env;

struct Config {
    Vuln: String,
    DockerVersion: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
}

fn parse_config(args: &[String]) -> Config {
    let Vuln = args[1].clone();
    let DockerVersion = args[2].clone();

    Config {
        Vuln,
        DockerVersion,
    }
}
