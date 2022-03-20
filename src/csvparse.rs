use csv::Reader;


pub fn parse_csv(csv_str: String) -> String {
    let mut rdr = Reader::from_reader(csv_str.as_bytes());
    let iter = rdr.records();

    let mut ret = "".to_string();

    for res in iter {
        let rec = res.unwrap();

        let mut tr = "<tr>".to_string();
        
        for cell in rec.into_iter() {
            tr.push_str(format!("<td>{}</td>", cell).as_str())
        }

        tr.push_str("</tr>");
        ret.push_str(tr.as_str());
    }

    return ret.to_string()
}