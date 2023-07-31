// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{Error, Manager};
use futures::executor::block_on;
use std::time::Duration;
use settimeout::set_timeout;
use base64_light::*;
use tauri::{utils::config::AppUrl, window::WindowBuilder, WindowUrl};


const CLIENT_ID: &str = &*"U2tvQXBwLlByb2QuMGQzNDkyMTctOWE0ZS00MWVjLTlhZjktZGY5ZTY5ZTA5NDk0";
const CLIENT_SECRET: &str = &*"N2NiNGQ5YTgtMjU4MC00MDQxLTlhZTgtZDU4MDM4NjkxODNm";

#[derive(Deserialize, Clone, Debug, Serialize)]
struct SchoolAttributes {
    name: Option<String>,
    #[serde(rename = "addressLine1")]
    address_line1: Option<String>,
    #[serde(rename = "addressLine2")]
    address_line2: Option<String>,
    #[serde(rename = "addressLine3")]
    address_line3: Option<String>,
    #[serde(rename = "zipCode")]
    zip_code: Option<String>,
    city: Option<String>,
    country: Option<String>,
    #[serde(rename = "homePageUrl")]
    home_page_url: Option<String>,
    #[serde(rename = "emsCode")]
    ems_code: Option<String>,
    #[serde(rename = "emsOIDCWellKnownUrl")]
    ems_oidcwell_known_url: Option<String>
}

const BASE_URL: &str = "https://api.skolengo.com/api/v1/bff-sko-app";
const CAS_CALLBACK: &str = "skoapp-prod://sign-in-callback";
static mut SELECTED_SCHOOL: SchoolAttributes = SchoolAttributes {
    name: None,
    address_line1: None,
    address_line2: None,
    address_line3: None,
    zip_code: None,
    city: None,
    country: None,
    home_page_url: None,
    ems_code: None,
    ems_oidcwell_known_url: None
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
async fn search_school(q: String) -> Result<String, Error> {
    let res = reqwest::get(format!("{}/schools?page[offset]=0&page[limit]=100&filter[text]={}", BASE_URL, q)).await.expect("Error while fetching schools")
        .text();

    Ok(res.await.unwrap())
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Issuer {
    issuer: String,
    scopes_supported: Vec<String>,
    response_types_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    claim_types_supported: Vec<String>,
    claims_supported: Vec<String>,
    grant_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
    id_token_encryption_alg_values_supported: Vec<String>,
    id_token_encryption_enc_values_supported: Vec<String>,
    userinfo_signing_alg_values_supported: Vec<String>,
    userinfo_encryption_alg_values_supported: Vec<String>,
    userinfo_encryption_enc_values_supported: Vec<String>,
    request_object_signing_alg_values_supported: Vec<String>,
    request_object_encryption_alg_values_supported: Vec<String>,
    request_object_encryption_enc_values_supported: Vec<String>,
    introspection_endpoint_auth_methods_supported: Vec<String>,
    token_endpoint_auth_methods_supported: Vec<String>,
    code_challenge_methods_supported: Vec<String>,
    claims_parameter_supported: bool,
    request_uri_parameter_supported: bool,
    request_parameter_supported: bool,
    backchannel_logout_supported: bool,
    frontchannel_logout_supported: bool,
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: String,
    pushed_authorization_request_endpoint: String,
    registration_endpoint: String,
    end_session_endpoint: String,
    introspection_endpoint: String,
    revocation_endpoint: String,
    backchannel_logout_session_supported: bool,
    frontchannel_logout_session_supported: bool,
    jwks_uri: String,
}

async fn discover(mut issuer_url: String) -> Result<Issuer, reqwest::Error> {
    if issuer_url.ends_with("/.well-known") != true {
        issuer_url = format!("{}/.well-known/openid-configuration", issuer_url);
    }

    let res = reqwest::get(issuer_url)
        .await?
        .json::<Issuer>()
        .await?;

    Ok(res)
}

async fn get_authorization_url(sk_issuer: Issuer, client_id: String, response_type: String, redirect_uri: String) -> Result<String, Error> {
    Ok(format!("{}?client_id={}&response_type={}&redirect_uri={}&scope=openid", sk_issuer.authorization_endpoint, client_id, response_type, redirect_uri))
}

#[tauri::command()]
async fn auth_school(school: SchoolAttributes) -> Result<String, Error> {
    let eowku = school.ems_oidcwell_known_url.clone().expect("ems_oidcwell_known_url is not defined");
    unsafe {
        SELECTED_SCHOOL = school;
    }

    let decode_client_id = base64_decode_str(CLIENT_ID);

    let sk_issuer = discover(eowku).await.unwrap();
    let auth_url = get_authorization_url(sk_issuer, decode_client_id, "code".to_string(), CAS_CALLBACK.to_string()).await.unwrap();

    Ok(auth_url)
}

fn get_code(url: &str) -> Option<String> {
    let url = url::Url::parse(url).ok()?;
    let pairs = url.query_pairs();
    for (key, value) in pairs {
        if key == "code" {
            return Some(value.into_owned());
        }
    }
    None
}

#[derive(Deserialize, Debug, Serialize)]
struct TokenStruct {
    access_token: String,
    id_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

fn get_openid_token(
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    code: &str
) -> Result<TokenStruct, reqwest::Error> {
    let school = unsafe { &SELECTED_SCHOOL };
    let sk_issuer: Issuer = reqwest::blocking::get(format!("{}", school.ems_oidcwell_known_url.clone().unwrap())).unwrap().json::<Issuer>().unwrap();

    let token_url = reqwest::Url::parse(&sk_issuer.token_endpoint).unwrap();
    let client = reqwest::blocking::Client::new();

    let res = client
        .post(token_url)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .send();

    let token = res?.json::<TokenStruct>().unwrap();

    Ok(token)
}

#[derive(Deserialize, Debug, Serialize)]
struct AuthStruct {
    tokenSet: TokenStruct,
    school: SchoolAttributes
}

async fn wait(time: u64) {
    set_timeout(Duration::from_secs(time)).await;
}

#[tokio::main]
async fn main() {
    tauri_plugin_deep_link::prepare("fr.oriondev.scolengotoken");

    let port = portpicker::pick_unused_port().expect("failed to find unused port");

    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_school, auth_school])
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .setup(move |app| {
            WindowBuilder::new(
                app,
                "main".to_string(),
                if cfg!(dev) {
                    Default::default()
                } else {
                    window_url
                }
            )
                .inner_size(900.0, 800.0)
                .resizable(false)
                .title("Authentification auprès de Skolengo")
                .build()?;
            // If you need macOS support this must be called in .setup() !
            // Otherwise this could be called right after prepare() but then you don't have access to tauri APIs
            let handle = app.handle();

            tauri_plugin_deep_link::register(
                "skoapp-prod",
                move |request| {
                    let decode_client_id = base64_decode_str(CLIENT_ID);
                    let decode_client_secret = base64_decode_str(CLIENT_SECRET);

                    let code: String = get_code(&request).unwrap();
                    let tokenSet: TokenStruct = get_openid_token(&*decode_client_id, &*decode_client_secret, CAS_CALLBACK, &code).unwrap();

                    let school: SchoolAttributes;
                    unsafe {
                        school = SELECTED_SCHOOL.clone();
                    }

                    let auth = AuthStruct {
                        tokenSet,
                        school
                    };

                    let script = format!("{} {} {} {} {} {} {} {} {} {} {} {} {}", "const auth = document.getElementById('auth');", "const dllink = document.querySelector('.dllink');", "const formattedJSON = '", serde_json::to_string(&auth).unwrap(), "';", "auth.innerHTML = formattedJSON;", "dllink.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(formattedJSON));", "dllink.setAttribute('download', 'config.json');", "const btnCopy = document.getElementById('btn-copy');", "btnCopy.addEventListener('click', () => {", "btnCopy.hidden = true;", "navigator.clipboard.writeText(formattedJSON);", "})");

                    handle.windows().get("main").unwrap().set_title("Authentification réussie").unwrap();
                    let url = if cfg!(dev) {
                        WindowUrl::External("http://localhost:1430/".parse().unwrap())
                    } else {
                        WindowUrl::External(format!("http://localhost:{}/", port).parse().unwrap())
                    };
                    handle.windows().get("main").unwrap().eval(&*format!("window.location.href = '{}success'", url)).unwrap();
                    block_on(wait(1));
                    handle.windows().get("main").unwrap().eval(&*script).expect("TODO: panic message");
                },
            )
                .unwrap();


            #[cfg(not(target_os = "macos"))] // on macos the plugin handles this (macos doesn't use cli args for the url)
            if let Some(url) = std::env::args().nth(1) {
                let decode_client_id = base64_decode_str(CLIENT_ID);
                let decode_client_secret = base64_decode_str(CLIENT_SECRET);

                let code: String = get_code(&url).unwrap();
                let tokenSet: TokenStruct = get_openid_token(&*decode_client_id, &*decode_client_secret, CAS_CALLBACK, &code).unwrap();

                let school: SchoolAttributes;
                unsafe {
                    school = SELECTED_SCHOOL.clone();
                }

                let auth = AuthStruct {
                    tokenSet,
                    school
                };

                let script = format!("{} {} {} {} {} {} {} {} {} {} {} {} {}", "const auth = document.getElementById('auth');", "const dllink = document.querySelector('.dllink');", "const formattedJSON = '", serde_json::to_string(&auth).unwrap(), "';", "auth.innerHTML = formattedJSON;", "dllink.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(formattedJSON));", "dllink.setAttribute('download', 'config.json');", "const btnCopy = document.getElementById('btn-copy');", "btnCopy.addEventListener('click', () => {", "btnCopy.hidden = true;", "navigator.clipboard.writeText(formattedJSON);", "})");

                app.windows().get("main").unwrap().set_title("Authentification réussie").unwrap();
                let url = if cfg!(dev) {
                    WindowUrl::External("http://localhost:1430/".parse().unwrap())
                } else {
                    WindowUrl::External(format!("http://localhost:{}/", port).parse().unwrap())
                };
                app.windows().get("main").unwrap().eval(&*format!("window.location.href = '{}success'", url)).unwrap();
                block_on(wait(1));
                app.windows().get("main").unwrap().eval(&*script).expect("TODO: panic message");
            }

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
