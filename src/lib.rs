use std::sync::RwLock;

use axum::{http::Uri, response::{Html, IntoResponse, Redirect}, Router};
use bt_core_config::{app_config::AppConfig, server_config::ServerConfig};
use bt_logger::{log_fatal, log_info, log_trace};
use server::get_server_listener;
use tokio::signal;
use lazy_static::lazy_static;

mod server;

pub async fn server_start(app_configuration:  &AppConfig, server_config: &ServerConfig, routes: Router, func_shutdown: Option<fn()>, ){
    log_info!("server_start","Starting {} {}",app_configuration.get_app_name(),app_configuration.get_version());
    
    //let svr_params = get_server_listener(&app_configuration).await;
    let svr_params = get_server_listener(server_config).await;
    let app_path = app_configuration.get_app_path();

    let current_app_url = if svr_params.svr_secure {
        format!("https://localhost:{}{}",&svr_params.svr_port,&app_path)
    }else{
        format!("http://localhost:{}{}",&svr_params.svr_port, &app_path)
    };
    set_static_app_url(current_app_url.clone());
    log_info!("main","Welcome to {} {}. To start open {}",app_configuration.get_app_name(),app_configuration.get_version(), &current_app_url);

    let server = axum::serve(svr_params.svr_listener, routes).with_graceful_shutdown(graceful_shutdown(func_shutdown));
   
    if let Err(err) = server.await{
        log_fatal!("server_start","Web Server Error: {}", err);
    }else{
        log_info!("server_start","Good bye!");
    }
}

lazy_static! {
    static ref STATIC_APP_URL: RwLock<String> = RwLock::new(String::new());
}

// This function will populate the static URL at runtime
fn set_static_app_url(value: String) {
    let mut static_value = STATIC_APP_URL.write().unwrap();
    *static_value = value;
}

/// Graceful shutdown handler
async fn graceful_shutdown(func_shutdown: Option<fn()>) {
    // Wait for a termination signal (Ctrl+C, SIGTERM, etc.)
    //signal::ctrl_c().await.unwrap();
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(unix)]
    let quit = async {
        signal::unix::signal(signal::unix::SignalKind::quit())
            .expect("failed to install quit signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Now, trigger the standard graceful shutdown
    tokio::select! {
        _ = ctrl_c => {log_info!("graceful_shutdown","CTRL-C Received");
                        if let Some(func) = func_shutdown{
                            func();
                        }
                      },
        _ = terminate => {log_info!("graceful_shutdown","Shutdown/Stop signal Received");
                            if let Some(func) = func_shutdown{
                               func();
                            }
                        },
        _ = quit => {log_info!("graceful_shutdown","Quite signal Received");
                        if let Some(func) = func_shutdown{
                            func();
                        }        
                    },
    }

    log_info!("graceful_shutdown","Shutting down server...");
}


#[deprecated(since = "0.2.0", note = "Use `generate_default_html` instead")]
pub fn generate_html() -> String {
    let static_app_url = STATIC_APP_URL.read().unwrap();
    format!("<!DOCTYPE html><html><head><title>BachueTech</title></head><body><h1>Bachuetech AI</h1><br/><h2>Open <a href=\"{}\">{}</a></h2></body></html>",&static_app_url,&static_app_url )
}

pub(crate) fn generate_default_html() -> String {
    let static_app_url = STATIC_APP_URL.read().unwrap();
    format!("<!DOCTYPE html><html><head><title>BachueTech</title></head><body><h1>Bachuetech AI</h1><br/><h2>Open <a href=\"{}\">{}</a></h2></body></html>",&static_app_url,&static_app_url )
}

///Default handler open Root with default message with link to APP.
/// Arguements:
/// f_def_html: Function to return a default html page. Must return a String with the HTML.
pub async fn default_handler() -> impl IntoResponse { //Redirect {
    log_trace!("handler","Default root.");
    let html_txt = generate_default_html();
    Html(html_txt)
}

pub async fn fallback_root(uri: Uri) -> impl IntoResponse {
    log_trace!("fallback", "Redirecting to default (root) page. Page not found: {}", uri);
    Redirect::temporary("/")
}




//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod tests_http {
    use axum::{routing::get, Router};
    use bt_core_config::{app_config::AppConfig, app_info::AppInfo, server_config::ServerConfig};
    use bt_logger::{build_logger, LogLevel, LogTarget};

    use crate::{default_handler, fallback_root, server_start};

    fn func_shutdown(){
        println!("EXECUTING Shutdown functions!!");
    }

    #[tokio::test]
    async fn test_websvr_defaults() {
        build_logger("BACHUETECH", "BT.HTTP_SERVER", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");
        //const YML_CONTENT: &str = include_str!("../config/core/app-config.yml");          
        let ac = AppConfig::new(Some("secure".to_owned()), &app_info, None).unwrap();
        let sc = ServerConfig::new(ac.get_environment(), None).unwrap();          
        let r = Router::new().route("/", get(default_handler)).fallback(fallback_root);
        server_start(&ac, &sc, r, None).await;
    }

    #[tokio::test]
    async fn test_websvr_dev() {
        build_logger("BACHUETECH", "BT.HTTP_SERVER", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");
        //const YML_CONTENT: &str = include_str!("../config/core/app-config.yml");           
        let ac = AppConfig::new(Some("dev".to_owned()), &app_info, None).unwrap();
        let sc = ServerConfig::new(ac.get_environment(), None).unwrap();         
        let r = Router::new().route("/", get(default_handler)).fallback(fallback_root);        
        server_start(&ac,&sc, r, Some(func_shutdown)).await;
    }
}
