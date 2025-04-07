from collections import Counter

def analyze(log_lines):
    domains = []
    ips = []
    for line in log_lines:
        parts = line.strip().split(",")
        if len(parts) >= 3:
            _, domain, ip = parts
            domains.append(domain)
            ips.append(ip)

    top_domains = Counter(domains).most_common(5)
    tops_ips = Counter(ips).most_common(5)

    return {
        "top domains": top_domains,
        "top ips": tops_ips
    }