use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, ToSocketAddrs};

pub fn is_private_url(url: &str) -> bool {
    !is_safe_destination(url)
}

pub fn is_safe_destination(url: &str) -> bool {
    let parsed = match url::Url::parse(url) {
        Ok(u) => u,
        Err(_) => return false,
    };

    if parsed.scheme() == "file" {
        return false;
    }

    let host = match parsed.host_str() {
        Some(h) => h,
        None => return false,
    };

    if is_private_host(host) {
        return false;
    }

    if let Ok(addrs) = resolve_host(host, parsed.port().unwrap_or(443)) {
        for addr in addrs {
            if is_private_ip(addr.ip()) {
                return false;
            }
        }
    }

    true
}

fn resolve_host(
    host: &str,
    port: u16,
) -> std::io::Result<impl Iterator<Item = std::net::SocketAddr>> {
    format!("{}:{}", host, port).to_socket_addrs()
}

fn is_private_host(host: &str) -> bool {
    let h = host.to_lowercase();
    h == "localhost"
        || h.ends_with(".local")
        || h.ends_with(".internal")
        || h.ends_with(".localhost")
        || h == "metadata.google.internal"
        || h == "169.254.169.254"
}

fn is_private_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_private_ipv4(v4),
        IpAddr::V6(v6) => is_private_ipv6(v6),
    }
}

fn is_private_ipv4(ip: Ipv4Addr) -> bool {
    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_unspecified()
        || is_cgnat(ip)
        || is_reserved_v4(ip)
}

fn is_cgnat(ip: Ipv4Addr) -> bool {
    let o = ip.octets();
    o[0] == 100 && (64..=127).contains(&o[1])
}

fn is_reserved_v4(ip: Ipv4Addr) -> bool {
    let o = ip.octets();
    o[0] == 0
        || o[0] >= 224
        || (o[0] == 169 && o[1] == 254)
        || (o[0] == 192 && o[1] == 0 && o[2] == 0)
}

fn is_private_ipv6(ip: Ipv6Addr) -> bool {
    ip.is_loopback()
        || ip.is_unspecified()
        || is_ipv6_ula(&ip)
        || is_ipv6_link_local(&ip)
        || is_ipv6_mapped_v4(&ip)
}

fn is_ipv6_ula(ip: &Ipv6Addr) -> bool {
    (ip.segments()[0] & 0xfe00) == 0xfc00
}

fn is_ipv6_link_local(ip: &Ipv6Addr) -> bool {
    (ip.segments()[0] & 0xffc0) == 0xfe80
}

fn is_ipv6_mapped_v4(ip: &Ipv6Addr) -> bool {
    let s = ip.segments();
    s[..5].iter().all(|&x| x == 0) && s[5] == 0xffff
}
