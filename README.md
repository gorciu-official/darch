# darch 
Darch is a small tool allowing you to declarativelly install packages on Arch-based distributions. The goal is to make a full system configuration with includes, that will allow you to change everything through one organised file.

- 🚀 automatically upgrades the system: pacman, flatpak and AUR (if AUR helper exists)
- 🔙 saves the old configuration on success in case something goes wrong
- 🤖 does all the initial setup for you, merging the configuration you've already done into /etc/sysconfig

## Configuration
Configuration is a JSON file located at `/etc/sysconfig` (back-up configuration at `/etc/sysconfig.old`). If it does not exist, it's generated automatically using output from `pacman -Qe` and your configuration files.
