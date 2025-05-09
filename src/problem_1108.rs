#[allow(dead_code)]
pub fn defang_ip_addr(address: &str) -> String {
    address.replace('.', "[.]")
}
