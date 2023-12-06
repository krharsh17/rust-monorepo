use filter::*;

fn main() {

    let text = "A blind person was killed in yesterday's riots".to_string();

    let mut clean_text = filter_ableism(text);
    clean_text = filter_violence(clean_text);

    println!("{}", clean_text);
}