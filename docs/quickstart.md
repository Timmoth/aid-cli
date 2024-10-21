## Installation

Manual installation is simple, just download the release and add it to your PATH environment variable, if you'd like an even easier way i've added scripts to do it for you.

### Linux / Mac
```
wget https://raw.githubusercontent.com/Timmoth/aid-cli/refs/heads/main/install.sh
chmod +x install.sh
sudo ./install.sh
```
### Windows (powershell)
```
Invoke-WebRequest -Uri https://raw.githubusercontent.com/Timmoth/aid-cli/refs/heads/main/install.ps1 -OutFile install.ps1
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
.\install.ps1
```

### Releases
[Download the latest release](https://github.com/Timmoth/aid-cli/releases)

### Build
If you'd like to build the latest version from source:
```
//Install rust https://www.rust-lang.org/tools/install
git clone https://github.com/Timmoth/aid-cli
cd aid-cli
cargo build --release
.\target\release\aid.exe
```
