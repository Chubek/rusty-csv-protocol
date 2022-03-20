use data_encoding::BASE64;
use regex::Regex;

pub fn decode_str(tbd: &str) -> String {
    let decoded = BASE64.decode(tbd.as_bytes());
    match decoded {
        Ok(res) => String::from_utf8(res).unwrap(),
        Err(err) => {
            println!("{:?}", err);
            String::from("123")
        }
    }
     

}



pub fn get_size(body: String) -> usize{
    let re_match = Regex::new("SET \\d+").unwrap();
    let re_digit = Regex::new("\\d+").unwrap();

    if re_match.is_match(body.as_str().clone()) {
        let all_found = re_digit.find(body.as_str().clone()).unwrap();

        return all_found.as_str().parse::<usize>().unwrap()
    }

    return 0usize
}