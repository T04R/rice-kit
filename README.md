# rice-kit



### Custom Linux Compatible with OpenRC and Systemd

#### i3 Features:
- Terminal (`mod + enter`)
- Firefox (`mod + f`)
- Thunar (`mod + p`)
- Quick note (`mod + n`)
- Quick Ranger (`mod + m`)
- Kill tmux (`alt + shift + r`)
- Resize windows with `mod + shift + arrow keys`
- Move between windows with `mod + arrow keys`
- Move window to session with `ctrl + shift + 0-9`
- Vertical split (`mod + v`)
- Horizontal split (`mod + h`)
- Kill window (`ctrl + shift + q`)
- Restart i3 (`mod + shift + r`)
- Exit (`mod + shift + e`)
- Workspaces (`mod + 1-9`)

#### Media Mode (`mod + s`):
- Volume control: up/down arrows
- Brightness control: left/right arrows
- Mute (`m`)
- Screenshot (`s`)
- Video with microphone (`h`)
- Video without microphone (`v`)
- Webcam (`w`)
- Voice recording (`y`)

#### Which Mode (`mod + w`):
- Shutdown (`p`)
- Reboot (`r`)
- Suspend (`s`)
- Lock (`l`)
- Kill i3 (`k`)

#### Bar Features:
- CPU usage & temperature
- RAM usage
- DNS, IPv6, and IPv4 display
- Upload/download speed
- Brightness & volume levels
- Date & time
- Battery & uptime
- Video/webcam status
- USB connection status

#### Tmux Features:
- Switch windows with `alt + q` to `p`
- Switch sessions with `alt + 0-9`
- Move pane with `ctrl + alt + arrow keys`
- Navigate panes with `alt + arrow keys` or `alt + h/j/k/l`
- Vertical split (`alt + v`)
- Horizontal split (`alt + b`)
- Resize panes with `alt + shift + arrow keys` or `alt + shift + h/j/k/l`
- Choose tree (`alt + s`)
- Copy mode (`ctrl + v`):
  - Copy (`y`)
  - Exit (`ctrl + c`)
  - Copy & exit (`Y`)

#### Zsh Features:
- Kali Linux-like zsh with dzen2 alerts
  Example usage:
  `alert End process`
  `nmap -p- 127.0.0.1 ; alert end port scan`

#### ST (Terminal) Features:
- Kali Linux-like color scheme with minor tweaks
- Fira Code font
- Helix theme

#### Additional Features:
- Preconfigured nftables
- Automatic driver detection
- Touchpad gestures support

---

### Installation Guide:
```bash
git clone https://github.com/T04R/rice-kit.git
cd rice-kit
./make.sh
```

#### Quick Tools:
- **Quick Note** & **Quick Ranger**
- MPV in no-video mode (works as music player with Ranger)
  Press `mod + m` to toggle Quick Ranger (background music continues).
  Replace Elisa/Amberol with any music player in the config for a seamless quick-music setup.

#### Terminal Setup:
- ST + Tmux + Zsh for balance of efficiency and usability.

#### Dynamic Bar:
- Rust-written, responsive, and modular (toggle modules via `true`/`false` in config).

---

**Note:** Future updates may introduce new features or configuration changes.

---

