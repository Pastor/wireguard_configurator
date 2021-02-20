use crate::config::Config;

pub trait Transform<T> {
    fn transform(object: &T) -> String;
}

pub struct WireGuardTransform {}

impl Transform<Config> for WireGuardTransform {
    fn transform(object: &Config) -> String {
        let mut buf = String::new();
        buf += "[Interface]\n";
        buf.push_str(format!("Address = {}\n", object.address.clone()).as_str());
        buf.push_str(format!("ListenPort = {}\n", object.port).as_str());
        buf.push_str(format!("PrivateKey = {}\n", object.private_key).as_str());
        object.users.iter().for_each(|u| {
            let sg = object.security_groups.clone();
            u.1.security_group.clone().iter().for_each(|g| {
                let go = sg.get(g).expect(format!("SecurityGroup \"{}\" not defined", g).as_str());
                go.allowed_ip.iter().for_each(|ai| {
                    buf.push_str(format!("PostUp = iptables -I INPUT -s {} -d {} -j ACCEPT\n", u.1.ip, ai).as_str());
                    buf.push_str(format!("PostDown = iptables -D INPUT -s {} -d {} -j ACCEPT\n", u.1.ip, ai).as_str());
                });
            })
        });

        object.users.iter().for_each(|u| {
            buf.push_str(format!("\n").as_str());
            buf.push_str(format!("[Peer] #{}\n", u.0).as_str());
            buf.push_str(format!("PublicKey = {}\n", u.1.public_key).as_str());
            buf.push_str(format!("AllowedIPs = {}\n", u.1.ip).as_str());
        });
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_CONFIG: &str = "\
        [Gate]
        address = \"192.168.1.1/24\"
        port = 443
        pool = \"192.168.1.1/24\"
        private_key = \"Rz6EHuDj3NUih/QORBAJ+swtUaZnYsUAA0JOcuc/lT8=\"

        [[SecurityGroup]]
        name = \"group1\"
        allowed_ip = [\"192.168.1.2/24\"]

        [[User]]
        name = \"user1\"
        ip = \"192.168.0.1\"
        public_key = \"Rz6EHuDj3NUih/QORBAJ+swtUaZnYsUAA0JOcuc/lT8=\"
        security_group = [\"group1\"]
        ";

    #[test]
    fn wireguard_transform() {
        let config = Config::from_str(DEFAULT_CONFIG).unwrap();
        let text = WireGuardTransform::transform(&config);
        assert!(text.contains("192.168.1.1/24"));
    }
}

