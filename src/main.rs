use fastly::http::{header, Method, StatusCode};
use fastly::{kv_store::KVStore, mime, Error, Request, Response};

const RESOURCE_KV_STORE_NAME: &str = "hosfelt-resources";

fn fetch_resource_from_kv(key: &str) -> String {
    let store = KVStore::open(RESOURCE_KV_STORE_NAME).unwrap();
    let mut kv_result = store.unwrap().lookup(key).unwrap();
    kv_result.take_body().into_string()
}

fn fetch_image_resource_from_kv(key: &str) -> Vec<u8> {
    let store = KVStore::open(RESOURCE_KV_STORE_NAME).unwrap();
    let mut kv_result = store.unwrap().lookup(key).unwrap();
    kv_result.take_body().into_bytes()
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    match req.get_method() {
        // Block requests with unexpected methods
        &Method::POST | &Method::PUT | &Method::PATCH | &Method::DELETE => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, PURGE")
                .with_body_text_plain("This method is not allowed\n"))
        }

        // Let any other requests through
        _ => (),
    };

    match req.get_path() {
        "/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(fetch_resource_from_kv("index.html")))
        }
        "/index.xml" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_XML)
                .with_body(fetch_resource_from_kv("index.xml"))) 
        },
        "/sitemap.xml" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_XML)
                .with_body(fetch_resource_from_kv("sitemap.xml"))) 
        }
        "/posts/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(fetch_resource_from_kv("posts/index.html"))) 
        },
        "/posts/index.xml" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_XML)
                .with_body(fetch_resource_from_kv("posts/index.xml"))) 
        }
        "/about/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(fetch_resource_from_kv("about/index.html"))) 
        },
        "/resume/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(fetch_resource_from_kv("resume/index.html"))) 
        },
        "/css/style.min.dbbe08cb3b07bbce02de1a13a57d4221bb75487e75b0d1a5196a5353f7135921.css" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://www.hosfe.lt")
                .with_content_type(mime::TEXT_CSS_UTF_8)
                .with_body(fetch_resource_from_kv("style.min.dbbe08cb3b07bbce02de1a13a57d4221bb75487e75b0d1a5196a5353f7135921.css")))
        }
        "/css/style.min.2faed4cf7533c5236bf011e885da2e0c2670dea54d80f6b5ff1f370613f0983a.css" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://www.hosfe.lt")
                .with_content_type(mime::TEXT_CSS_UTF_8)
                .with_body(fetch_resource_from_kv("style.min.2faed4cf7533c5236bf011e885da2e0c2670dea54d80f6b5ff1f370613f0983a.css")))
        },
        "/js/bundle.min.038214de9d568246fadcfeb06c69349925de3209f332ec123861b6aa031d63c6.js" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://www.hosfe.lt")
                .with_content_type(mime::APPLICATION_JAVASCRIPT_UTF_8)
                .with_body(fetch_resource_from_kv("bundle.min.038214de9d568246fadcfeb06c69349925de3209f332ec123861b6aa031d63c6.js")))
        },
        "/js/link-share.min.24409a4f6e5537d70ffc55ec8f9192208d718678cb8638585342423020b37f39.js" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://www.hosfe.lt")
                .with_content_type(mime::APPLICATION_JAVASCRIPT_UTF_8)
                .with_body(fetch_resource_from_kv("link-share.min.24409a4f6e5537d70ffc55ec8f9192208d718678cb8638585342423020b37f39.js")))
        },
        // blog posts go after here
        "/posts/story/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(fetch_resource_from_kv("posts/story/index.html")))
        },
        "/posts/edge/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(fetch_resource_from_kv("posts/edge/index.html")))
        },
        "/posts/edge2/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(fetch_resource_from_kv("posts/edge2/index.html")))
        },
        "/images/edge-post-2_certificate.png" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::IMAGE_PNG)
                .with_body(fetch_image_resource_from_kv("images/edge-post-2_certificate.png")))
        },
        "/posts/adblocker/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(fetch_resource_from_kv("posts/adblocker/index.html")))
        },
        "/images/honeycomb.png" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::IMAGE_PNG)
                .with_body(fetch_image_resource_from_kv("public/images/honeycomb.png")))
        },
        // Catch all other requests and return a 404.
        _ =>  {
            Ok(Response::from_status(StatusCode::NOT_FOUND)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(fetch_resource_from_kv("404.html")))
        },
    }
}
