extern crate csv;

use std::error::Error;
use std::io::Read;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CSVDataFrame {
    pub headers:Vec<String>,
    pub col_size:usize,
    pub dat: Vec<HashMap<String, String>>,
}

impl CSVDataFrame {
    pub fn new<R: Read>(read: R) -> Result<Self, Box<dyn Error>> {
        CSVDataFrame::csv_parse(read)
    }

    pub fn csv_parse<R: Read>(read: R) -> Result<CSVDataFrame, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .quote(b'\'')
            .has_headers(true)
            .from_reader(read);

        let line = rdr.headers()?;
        let headers: Vec<String> = line.iter().map(|x|String::from(x)).collect();
        let col_size = headers.len();
        let mut dat_vec: Vec<HashMap<String, String>> = Vec::<HashMap<String, String>>::new();

        for result in rdr.records() {
            let record = result?;
            let record_vec: Vec<String> = record.iter().map(|x|String::from(x)).collect();
            let record_hash: HashMap<String, String> = headers.clone().into_iter().zip(record_vec.into_iter()).collect();
            dat_vec.push(record_hash);
        }
        
        Ok(
            CSVDataFrame {
                headers,
                col_size,
                dat: dat_vec,
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;
    #[test]
    fn it_works() {
        macro_rules! map_init {
            ( $($key:expr => $val:expr),* ) => {{
                // HashMapを生成
                let mut tmp = std::collections::HashMap::new();
                $ (
                    // 繰り返し値を挿入
                    tmp.insert($key , $val);
                ) *
                tmp // オブジェクトを返す
            }}
        }

        let test_input = r#"
Header,Name,Comment,Other
,,,
INCLUDE, お試し0, コメント, その他
INCLUDE, お試し1, コメント, その他
INCLUDE, お試し2, コメント, その他
, お試し3, "コメント", その他
INCLUDE, お試し4, "コメント\n改行", その他
, お試し5, コメント, その他
INCLUDE, お試し6, コメント, その他
"#;

        let data_frame = match CSVDataFrame::new(test_input.as_bytes()) {
            Ok(data_frame) => {
                println!("Successfully parsed CSV");
                data_frame
            },
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        };
    
        assert_eq!(data_frame.headers,
            ["Header", "Name", "Comment", "Other"]);

        assert_eq!(data_frame.col_size, 4);

        let arr1: Vec<HashMap<String, String>> = vec![
             map_init![String::from("Header") => "".to_string(), String::from("Name") => "".to_string(), String::from("Comment") => "".to_string(), String::from("Other") => "".to_string()],
             map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し0".to_string(), String::from("Comment") => " コメント".to_string(), String::from("Other") => " その他".to_string()],
             map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し1".to_string(), String::from("Comment") => " コメント".to_string(), String::from("Other") => " その他".to_string()],
             map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し2".to_string(), String::from("Comment") => " コメント".to_string(), String::from("Other") => " その他".to_string()],
             map_init![String::from("Header") => "".to_string(), String::from("Name") => " お試し3".to_string(), String::from("Comment") => " \"コメント\"".to_string(), String::from("Other") => " その他".to_string()],
             map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し4".to_string(), String::from("Comment") => " \"コメント\\n改行\"".to_string(), String::from("Other") => " その他".to_string()],
             map_init![String::from("Header") => "".to_string(), String::from("Name") => " お試し5".to_string(), String::from("Comment") => " コメント".to_string(), String::from("Other") => " その他".to_string()],
             map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し6".to_string(), String::from("Comment") => " コメント".to_string(), String::from("Other") => " その他".to_string()]
        ];

        for (i, data) in data_frame.dat.iter().enumerate() {
            assert_eq!(*data, arr1[i]);
        }

        let arr1: Vec<HashMap<String, String>> = vec![
            map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し0".to_string(), String::from("Comment") => " コメント".to_string(), String::from("Other") => " その他".to_string()],
            map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し1".to_string(), String::from("Comment") => " コメント".to_string(), String::from("Other") => " その他".to_string()],
            map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し2".to_string(), String::from("Comment") => " コメント".to_string(), String::from("Other") => " その他".to_string()],
            map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し4".to_string(), String::from("Comment") => " \"コメント\\n改行\"".to_string(), String::from("Other") => " その他".to_string()],
            map_init![String::from("Header") => "INCLUDE".to_string(), String::from("Name") => " お試し6".to_string(), String::from("Comment") => " コメント".to_string(), String::from("Other") => " その他".to_string()]
        ];

        for (i, data) in data_frame.dat.iter().filter(|d| d.get("Header").unwrap_or(&"".to_string()) == "INCLUDE").enumerate() {
            assert_eq!(*data, arr1[i]);
        }
    }
}