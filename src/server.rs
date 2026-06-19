use bt_any_error::any_err::AnyErr;
use tokio::net::TcpListener;

use bt_core_config::server_config::ServerConfig;
use bt_logger::log_info;

pub(crate) struct ServerParams{
    pub svr_listener: TcpListener,
    pub svr_secure: bool,
    pub svr_port: u16,
}

pub(crate) async fn get_server_listener(server_config: &ServerConfig) -> Result<ServerParams, AnyErr> {
    let listener = TcpListener::bind(server_config.get_tcp_listener()).await?;
    log_info!("", "listening on {:?}", listener.local_addr()); 
    Ok(ServerParams{
        svr_listener: listener,
        svr_secure: server_config.is_secure(),
        svr_port: server_config.get_port(), 
    })
}

//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod tests_server {
    use bt_core_config::{app_config::AppConfig, app_info::AppInfo, server_config::ServerConfig};
    use bt_logger::{build_logger, LogLevel, LogTarget};

    use super::get_server_listener;


    #[tokio::test]
    async fn test_success_listener_dev() {
        build_logger("BACHUETECH", "BT.HTTP_SERVER", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");
        //const YML_CONTENT: &str = include_str!("../config/core/app-config.yml");         
        let ac = AppConfig::new("dev", &app_info, None).unwrap();
        let sc = ServerConfig::new(Some(ac.get_environment()), None).unwrap();
        //let l = get_server_listener(&ac, None).await;
        let l = get_server_listener(&sc).await.unwrap();
        assert_eq!(l.svr_port,3002);
        assert_eq!(l.svr_secure,false);

    }

    #[tokio::test]
    async fn test_success_listener_dsecure() {
        build_logger("BACHUETECH", "BT.HTTP_SERVER", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");
        //const YML_CONTENT: &str = include_str!("../config/core/app-config.yml");         
        let ac = AppConfig::new("secure", &app_info, None).unwrap();
        let sc = ServerConfig::new(Some(ac.get_environment()), None).unwrap();        
        let l = get_server_listener(&sc).await.unwrap();
        assert_eq!(l.svr_port,3003);
        assert_eq!(l.svr_secure,true);
    }

    #[tokio::test]
    async fn test_success_listener_defaults() {
        build_logger("BACHUETECH", "BT.HTTP_SERVER", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let app_info = AppInfo::get_app_info("AppName", "default_version", "Bachuetech", "Core Test");
        //const YML_CONTENT: &str = include_str!("../config/core/app-config.yml");         
        let ac = AppConfig::new("unknown", &app_info, None).unwrap();
        let sc = ServerConfig::new(Some(ac.get_environment()), None).unwrap();        
        let l = get_server_listener(&sc).await.unwrap();
        assert_eq!(l.svr_port,3002);
        assert_eq!(l.svr_secure,false);
    }
  
}