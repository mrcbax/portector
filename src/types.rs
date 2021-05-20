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

pub struct Config {
    pub ports: Vec<u16>,
    pub ban_parameters: BanParams,
    pub logging: bool
}

impl Default for Config {
    fn default() -> Config {
        let ports: Vec<u16> = vec![1,7,9,11,15,70,79,109,110,111,119,138,139,143,512,513,514,515,540,635,1080,1337,1524,2001,4001,5742,6001,6667,12345,12346,20034,30303,32771,32772,32773,32774,31337,40421,40425,49724,54320];
        return Config {
            ports: ports,
            ban_parameters: BanParams::default(),
            logging: true
        };
    }
}
