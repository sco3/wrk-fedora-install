# Installing wrk on Fedora

`wrk` is a HTTP benchmarking tool capable of generating significant load when run on a single multi-core CPU. 


## Prerequisites

Before building `wrk`, you need to install the necessary development tools and Perl dependencies required for the build scripts to run correctly on Fedora.

### 1. Install Development Tools
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel git
```
