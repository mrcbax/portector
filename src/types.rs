use serde_derive::{Serialize, Deserialize};

pub const LOG_FILE: &str = "/var/log/portector.log";
pub const AIPDB_LOG_FILE: &str = "/var/log/portector.aipdb.log";
#[derive(Serialize, Deserialize, Clone)]
pub struct BanParams {
    pub sequential: bool,
    pub connection_limit: usize,
    pub distance: usize
}

impl Default for BanParams {
    fn default() -> BanParams {
        return BanParams {
            sequential: true,
            connection_limit: 5,
            distance: 30
        };
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub ports: Vec<u16>,
    pub logging: bool,
    pub ban_parameters: BanParams,
    pub log_as_aipdb: bool
}

impl Default for Config {
    fn default() -> Config {
        let ports: Vec<u16> = vec![1,7,9,11,15,17,69,70,79,88,109,110,111,113,119,135,138,139,143,179,201,389,464,512,513,514,515,540,554,635,1080,1311,1337,1524,2001,2222,2049,2967,4001,5742,6001,6667,8080,8081,8088,8443,8888,12345,12346,20034,30303,32771,32772,32773,32774,31337,40421,40425,49724,54320];
        return Config {
            ports: ports,
            ban_parameters: BanParams::default(),
            logging: true,
            log_as_aipdb: true
        };
    }
}
