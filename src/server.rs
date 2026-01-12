use std::error::Error;

use tokio::net::TcpListener;

use bt_core_config::server_config::ServerConfig;
use bt_logger::{get_fatal, log_verbose};
//use bt_app_codes::process_exit_codes::LISTENER_TCP_BINDING_ERROR;

pub(crate) struct ServerParams{
    pub svr_listener: TcpListener,
    pub svr_secure: bool,
    pub svr_port: u16,
}

//pub(crate) async fn get_server_listener(app_configuration: &AppConfig, embed_cfg: Option<&str>) -> ServerParams{
pub(crate) async fn get_server_listener(server_config: &ServerConfig) -> Result<ServerParams, Box<dyn Error>> {
    //let app_configuration = AppConfig::new(running_environment);
    /*let srv_config = match get_srv_config(app_configuration.get_environment(), embed_cfg){
        Ok(sc) => sc,
        Err(e) => {
            log_fatal!("get_server_listener", "Fatal Error getting server configuration. Error {}",e);
            process::exit(LISTENER_TCP_BINDING_ERROR); // Exit the program with code -100
        },
    } ;*/

    let tcp_binding_result = TcpListener::bind(server_config.get_tcp_listener()).await;

    let listener = match tcp_binding_result {
        Ok(result) => result,
        Err(e) => {
            //log_fatal!("get_server_listener", "Fatal Error (PEC: {}) binding TCP {}. Error: {}", LISTENER_TCP_BINDING_ERROR, server_config.get_tcp_listener(), e);
            //process::exit(LISTENER_TCP_BINDING_ERROR); // Exit the program with code -100
            return Err(get_fatal!("get_server_listener", "Fatal Error binding TCP {}. Error: {}", server_config.get_tcp_listener(), e).into())
        }
    };

    log_verbose!("get_server_listener", "listening on {}", listener.local_addr().unwrap()); 
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
        let ac = AppConfig::new(Some("dev".to_owned()), &app_info, None).unwrap();
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
        let ac = AppConfig::new(Some("secure".to_owned()), &app_info, None).unwrap();
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
        let ac = AppConfig::new(Some("unknown".to_owned()), &app_info, None).unwrap();
        let sc = ServerConfig::new(Some(ac.get_environment()), None).unwrap();        
        let l = get_server_listener(&sc).await.unwrap();
        assert_eq!(l.svr_port,3002);
        assert_eq!(l.svr_secure,false);
    }
  
}