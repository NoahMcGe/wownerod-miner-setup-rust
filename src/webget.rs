use scraper::{Html, Selector};
use reqwest::StatusCode;


#[tokio::main]
pub async fn wowgetvar() -> (String, String){

	let url :&str = "https://git.wownero.com/wownero/wownero/releases";
    let webpage = reqwest::get(url).await.unwrap();
    let page_html = match webpage.status() {
        StatusCode::OK => webpage.text().await.unwrap(),
        _ => panic!("Error"),
    };

    let document = Html::parse_document(&page_html);
    let article_selector = Selector::parse("h4.release-list-title").unwrap();
    let h2select = Selector::parse("a").unwrap();
    for element in document.select(&article_selector){
        let mut h2el = element.select(&h2select);
            match h2el.next() {
        Some(elref) => {
            let elref = elref.inner_html().to_string();
            let elref = varreverse(elref);
            let elref = &elref[0..8];
            let elref = varreverse(elref.to_string());
            //print!("{}", &elref);
            let elref2 = wowgetdown(document);
            //let elref2 = "test";
            //wowgetdown(document);
            return (elref.to_string(),elref2.to_string());
        }
        _ => {return ("Error".to_string(),"Error".to_string())}
    }
    //break;
    };


    return ("Error".to_string(),"Error".to_string())
}

pub fn wowgetdown(document: Html) -> String{
    
    let article_selector = Selector::parse("details.download").unwrap();
    //let h2select = Selector::parse("ul.list").unwrap();
    let h2select = Selector::parse("ul").unwrap();
    //
    for element in document.select(&article_selector){
        let mut h2el = element.select(&h2select);
            match h2el.next() {
        Some(elref) => {
            let elref = elref.inner_html().to_string();
            let elref = varreverse(elref);
            let elref = &elref[0..1000];
            let elref = varreverse(elref.to_string());
            //print!("{}", &elref);
            let e = &elref.find("https").unwrap();
            //println!(" Location: {}", e);
            let e_range = e+36..e+72;
            let elref = &elref[e_range];
            return elref.to_string();
        }
        _ => {return "Error Parsing Webpage".to_string()}
    }
    //break;
    };
    return "Error Finding Download Link.".to_string()
}

pub fn varreverse(s: String) -> String{
	s.chars().rev().collect()
}