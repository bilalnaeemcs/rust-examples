//use std::error;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
pub struct Config {
    query: String,
    filepath: String,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        let config = Config {
            query: args[1].to_lowercase().clone(),
            filepath: args[2].clone(),
        };

        //dbg!(config.clone());

        return Ok(config);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let ret = fs::read_to_string(&config.filepath).unwrap();

    for line in ret.split("\n") {
        if line.to_lowercase().contains(&config.query) {
            println!("{line}")
        }
    }
    //dbg!(config);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failing() {
        let config = Config::build(vec![
            "fir".to_string(),
            "poem".to_string(),
            "poem.txt".to_string(),
        ])
        .unwrap();

        run(config).unwrap();
        assert!(true)
    }
}
