use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Config {
    #[serde(default = "default_work_path")]
    work_path: String,
    #[serde(default)]
    number: u32,
}

fn default_work_path() -> String {
    "./".to_string()
}


fn main() {
    let config_file = std::fs::File::open("sample.json").unwrap();
    let config: Config = serde_json::from_reader(config_file).unwrap();
    println!("{:?}", config);
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let json = r#"
        {
        }
    "#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config, Config{work_path :"./".to_string(), number: 0});

        let json = r#"
            {
            "workPath": "./config",
            "number": 5
            }
        "#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config, Config{work_path :"./config".to_string(), number: 5});


    }
}