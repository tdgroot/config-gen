use actix_web::http::HeaderMap;
use http::header::HeaderValue;
use actix_web::http::Cookie;
use regex::Regex;

pub fn clone_headers(headers: &HeaderMap, target: String, replacer: String) -> HeaderMap {
    let regex = Regex::new(target.as_str()).unwrap();
    let mut hm = HeaderMap::new();
    for (key, value) in headers.iter().filter(|(key, _)| key.as_str() != "cookie") {
        let strs = value.to_str().unwrap();

        let next_string = match key.as_str() {
            "set-cookie" => {
                let mut c = Cookie::parse_encoded(strs).unwrap();
                c.set_domain("");
                c.to_string()
            },
            _ => strs.to_string()
        };

        let next = regex.replace(&next_string, replacer.as_str());
        let hv = HeaderValue::from_str(&next);
        hm.append(key.clone(), hv.unwrap());
    }
    hm
}

#[test]
pub fn test_clone_headers() {
    let mut hm = HeaderMap::new();
    hm.append("none-dup", "form_key=123456".parse().unwrap());
    hm.append("set-cookie", "form_key=123456; domain=www.neom.com".parse().unwrap());
    hm.append("set-cookie", "key=value; domain=www.neom.com".parse().unwrap());

    // cloned header map with domain re - written
    let cloned = clone_headers(&hm, "www.neom.com".to_string(), "127.0.0.1:8080".to_string());

//    println!("{:#?}", cloned);

    // expected header map
    let mut expected = HeaderMap::new();
    expected.append("none-dup", "form_key=123456".parse().unwrap());
    expected.append("set-cookie", "form_key=123456; Domain=".parse().unwrap());
    expected.append("set-cookie", "key=value; Domain=".parse().unwrap());

    assert_eq!(expected, cloned);
}

#[test]
pub fn test_ignores_cookie() {
    let mut hm = HeaderMap::new();
    hm.append("cookie", "form_key=123456".parse().unwrap());

    let cloned = clone_headers(&hm, "www.neom.com".to_string(), "127.0.0.1:8080".to_string());
    let expected = HeaderMap::new();

    assert_eq!(expected, cloned);
}