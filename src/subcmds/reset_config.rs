use crate::printer::print_header;
use crate::config::{SysConfig, generate_base_config};

pub fn reset_config() -> SysConfig {
    print_header("Regenerating config");
    generate_base_config("/etc/sysconfig")
}
