# darch 
Darch is a small tool allowing you to declarativelly install packages on Arch-based distributions.

- 🚀 automatically upgrades the system - both pacman and flatpak (AUR in the nearest future)
- 🔙 saves the old configuration on success in case something goes wrong

## Configuration
Configuration is a JSON file located at `/etc/sysconfig` (back-up configuration at `/etc/sysconfig.old`). If it does not exist, it's generated automatically using output from `pacman -Qe` and your configuration files.
