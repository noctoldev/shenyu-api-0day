# Apache ShenYu SSRF Vulnerability

## Vulnerability Details

- **Product**: Apache ShenYu
- **Component**: `shenyu-bootstrap`
- **Version**: Latest (tested with `apache/shenyu-bootstrap` Docker image) v2.7.0
- **Vulnerability Type**: Server-Side Request Forgery (SSRF)
- **Status**: undisclosed, icba
- **Researcher**: Noctoldev / flamingo
- **Discovered**: April 2025

### Description

Apache ShenYu is an API gateway that supports plugin-based routing and load balancing. A Server-Side Request Forgery (SSRF) vulnerability exists in the `shenyu-bootstrap` component’s `/shenyu/plugin/selectorAndRules` endpoint. By sending a crafted JSON payload with a malicious `upstreamUrl`, an attacker can force the server to make unauthorized HTTP requests to internal or external services, potentially leading to data leakage, internal network enumeration, or service disruption.

The vulnerability is exploitable when the `shenyu.local.enabled` configuration is set to `true` and the `localKey` (default: `123456`) is known. The endpoint allows unauthenticated or weakly authenticated requests to configure the `divide` plugin, which triggers HTTP requests to the specified `upstreamUrl`.

### Impact

- **Internal Service Access**: Attackers can target internal services (e.g., `http://localhost:8080`) to access sensitive data or perform unauthorized actions.
- **Network Enumeration**: SSRF can be used to map internal networks or cloud metadata services.
- **Service Disruption**: Malicious requests may overload internal services or trigger unintended behaviors.
- **Severity**: High (pending CVSS score, estimated 7.5-8.5 due to potential for internal access).

## Proof of Concept (PoC)

### Prerequisites

- **System**: Arch Linux (or any system with Docker and Rust)
- **Tools**:
  - Docker (to run `shenyu-admin` and `shenyu-bootstrap`)
  - Rust (to compile and run the PoC)
  - Python or `netcat` (to observe SSRF requests)
- **ShenYu Setup**:
  - `shenyu-admin` Docker container (port 9095)
  - `shenyu-bootstrap` Docker container (port 9195) with `shenyu.local.enabled=true`
- **Target Service**: Local HTTP server on `http://localhost:8080` (e.g., `python3 -m http.server 8080`)

### Setup Instructions

1. **Start ShenYu Containers**:
-   docker network create shenyu
-   docker pull apache/shenyu-admin
-   docker run -d --name shenyu-admin -p 9095:9095 --net shenyu apache/shenyu-admin
-   docker pull apache/shenyu-bootstrap
-   docker run -d --name shenyu-bootstrap -p 9195:9195 --net shenyu -e "shenyu.local.enabled=true" apache/shenyu-bootstrap

------
Thank you for reading! 
im a 15 year old malware developer under the alias "flamingo" just having fun! 
feel free to disclose on behalf of myself as i am too lazy
