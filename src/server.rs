use std::process;

use tokio::net::TcpListener;

use bt_core_config::{app_config::AppConfig, server_config::get_srv_config};
use bt_logger::{log_fatal, log_verbose};
use bt_app_codes::process_exit_codes::LISTENER_TCP_BINDING_ERROR;

pub(crate) struct ServerParams{
    pub svr_listener: TcpListener,
    pub svr_secure: bool,
    pub svr_port: u16,
}

pub async fn get_server_listener(app_configuration: &AppConfig) -> ServerParams{
    //let app_configuration = AppConfig::new(running_environment);
    let srv_config = get_srv_config(app_configuration.get_environment()); 

    let tcp_binding_result = TcpListener::bind(srv_config.get_tcp_listener()).await;

    let listener = match tcp_binding_result {
        Ok(result) => result,
        Err(e) => {
            log_fatal!("get_server_listener", "Fatal Error (PEC: {}) binding TCP {}. Error: {}", LISTENER_TCP_BINDING_ERROR, srv_config.get_tcp_listener(), e);
            process::exit(LISTENER_TCP_BINDING_ERROR); // Exit the program with code -100
        }
    };

    log_verbose!("get_server_listener", "listening on {}", listener.local_addr().unwrap()); 
    ServerParams{
        svr_listener: listener,
        svr_secure: srv_config.is_secure(),
        svr_port: srv_config.get_port(), 
    }
}

#[cfg(test)]
mod tests_server {
    use bt_core_config::app_config::AppConfig;
    use bt_logger::{build_logger, LogLevel, LogTarget};

    use super::get_server_listener;


    #[tokio::test]
    async fn test_success_listener() {
        build_logger("BACHUETECH", "BT.HTTP_SERVER", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let ac = AppConfig::new(Some("dev".to_owned()));
        let l = get_server_listener(&ac).await;
        assert_eq!(l.svr_port,3001);
        assert_eq!(l.svr_secure,false);

    }

    #[tokio::test]
    async fn test_success_listener_defaults() {
        build_logger("BACHUETECH", "BT.HTTP_SERVER", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let ac = AppConfig::new(Some("secure".to_owned()));
        let l = get_server_listener(&ac).await;
        assert_eq!(l.svr_port,3001);
        assert_eq!(l.svr_secure,true);
    }

    #[tokio::test]
    async fn test_success_listener_dev() {
        build_logger("BACHUETECH", "BT.HTTP_SERVER", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let ac = AppConfig::new(Some("dev".to_owned()));
        let l = get_server_listener(&ac).await;
        assert_eq!(l.svr_port,3001);
        assert_eq!(l.svr_secure,false);
    }    
}