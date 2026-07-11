use crate::config_page::ConfigPage;

type CpuConfig = ConfigPage<CpuConfigDetails>;

struct CpuConfigDetails {
    pub num_cpus: u32,
}

impl CpuConfig {
    pub fn get_num_cpus(&self) -> u32 {
        self.num_cpus
    }
}