use filter::filter_violence;

fn main() {

    let text = "A blind person was killed in yesterday's riots".to_string();

    let clean_text = filter_violence(text);

    println!("{}", clean_text);
}