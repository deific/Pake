use crate::app::config::{PakeConfig, WindowConfig};
use crate::util::get_data_dir;
use std::{path::PathBuf, str::FromStr};
use tauri::{App, Config, Url, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[cfg(target_os = "macos")]
use tauri::{Theme, TitleBarStyle};

pub fn set_window(app: &mut App, config: &PakeConfig, tauri_config: &Config) -> WebviewWindow {
    let package_name = tauri_config.clone().product_name.unwrap();
    let _data_dir = get_data_dir(app.handle(), package_name);

    let window_config = config
        .windows
        .first()
        .expect("At least one window configuration is required");

    let user_agent = config.user_agent.get();

    let url = match window_config.url_type.as_str() {
        "web" => WebviewUrl::App(window_config.url.parse().unwrap()),
        "local" => WebviewUrl::App(PathBuf::from(&window_config.url)),
        _ => panic!("url type can only be web or local"),
    };

    let config_script = format!(
        "window.pakeConfig = {}",
        serde_json::to_string(&window_config).unwrap()
    );

    let mut window_builder = WebviewWindowBuilder::new(app, "pake", url)
        .title("")
        .visible(false)
        .user_agent(user_agent)
        .resizable(window_config.resizable)
        .fullscreen(window_config.fullscreen)
        .inner_size(window_config.width, window_config.height)
        .always_on_top(window_config.always_on_top)
        .disable_drag_drop_handler()
        .initialization_script(&config_script)
        .initialization_script(include_str!("../inject/component.js"))
        .initialization_script(include_str!("../inject/event.js"))
        .initialization_script(include_str!("../inject/style.js"))
        .initialization_script(include_str!("../inject/custom.js"));

    if !config.proxy_url.is_empty() {
        window_builder =
            window_builder.proxy_url(Url::from_str(config.proxy_url.as_str()).unwrap());
    }

    #[cfg(target_os = "macos")]
    {
        let title_bar_style = if window_config.hide_title_bar {
            TitleBarStyle::Overlay
        } else {
            TitleBarStyle::Visible
        };
        window_builder = window_builder.title_bar_style(title_bar_style);

        if window_config.dark_mode {
            window_builder = window_builder.theme(Some(Theme::Dark));
        }

        window_builder = window_builder
            .data_directory(_data_dir)
            .title(app.package_info().name.clone());
    }

    window_builder.build().expect("Failed to build window")
}

pub fn new_window(app: tauri::AppHandle)  -> WebviewWindow {
    let js_script="document.cookie='tpass_se8dc9bba4s9466a93bb2qaab2b8c9ca=eyJhbGciOiJIUzUxMiJ9.eyJsb2dpbl91c2VyX2tleSI6IjEyMDExYmUyMWRlYzQ5NTU5NmM0MDM3ZDU4ZTYwYmE2In0.ijARVehgaHpxTTNT7kiuAizjU7CRvzWjW0jzqJl_4SIEm0-QK01d9CDdq_HX4ja3Bn7-7V8WxcVtDYH_iUarOA;path=/; domain=.chinatax.gov.cn;'";
    let webview = tauri::WebviewUrl::External("https://etax.beijing.chinatax.gov.cn:8443/loginb/".parse().unwrap());

    // let js_script="document.cookie='tpass_ue7c9954acea492784ac6g78939gc2e9=eyJhbGciOiJIUzUxMiJ9.eyJsb2dpbl91c2VyX2tleSI6IjFiOTY1MTlmOTY2YTQ3ZDU4MDRmMTBhY2U2NDdiZjE5In0.CFJqs-Hu5eU-Wl2VT_FSSNKOKRPlvc6tJGn1Oh0uX3I2T_wHDHDUlD7R5KIrCM546woLXYgUwsm76F4n4RiDKw; MM_mq4qQammP3BA3=N2FhNWVmOTU4ZDIyMzJmMPM6Qm6eao4OhbeTXMxzzFm+3JEWpTECFqrLYCxZKpv8JjdtnFhatxaaY+WdS/V9Vw; R8z1ETxCbJcfKium=621638dce93946409be65aee40b9d333; R8z1ETxCbJcfKiun=2e144be259c9ef43a7a5a3a77c078807;path=/; domain=.chinatax.gov.cn;'";
    // let webview = tauri::WebviewUrl::External(
    //     "https://etax.hunan.chinatax.gov.cn:8443/loginb/"
    //         .parse()
    //         .unwrap(),
    // );

    // let webview = tauri::WebviewUrl::App("tax.html".parse().unwrap());
    let window_builder = WebviewWindowBuilder::new(&app, "sub", webview)
        .content_protected(true)
        .maximized(true)
        .initialization_script(js_script)
        .initialization_script(include_str!("../inject/component.js"))
        .initialization_script(include_str!("../inject/event.js"))
        .initialization_script(include_str!("../inject/style.js"))
        .initialization_script(include_str!("../inject/custom.js"))
        .use_https_scheme(true)
        .focused(true);

    window_builder.build()
        .expect("Failed to build new window")
}