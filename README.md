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
- Touchpad gestures support

---

### Installation Guide:
Gentoo
```bash
echo "x11-drivers/xf86-input-libinput" >> /etc/portage/package.use
mkdir -p /etc/portage.accept.keywords
echo "sys-fs/jmtpfs ~amd64" | sudo tee -a /etc/portage.accept.keywords/jmptfs
echo "media-video/ffmpeg x264 x265 fdk-aac mp3 opus" | sudo tee -a /etc/portage/package.use/ffmpeg
echo "$USER ALL=(ALL) NOPASSWD: /sbin/shutdown, /sbin/loginctl" | sudo tee -a /etc/sudoers

sudo emerge --sync
sudo emerge --ask x11-wm/i3 x11-misc/i3lock x11-misc/i3status app-misc/tmux app-shells/zsh media-fonts/fira-code app-misc/ranger x11-misc/xclip sys-auth/elogind xf86-input-libinput media-video/ffmpeg media-gfx/scrot x11-apps/xwininfo x11-misc/xdotool alsa-utils alsa-libs media-video/mpv jmptfs
sudo rc-update add elogind
```
Arch
```bash
sudo pacman -Syu; sudo pacman -S i3-wm i3lock i3status tmux zsh ttf-fira-code ranger xclip xf86-input-libinput ffmpeg scrot jmtpfs xorg-xwininfo xdotool alsa-utils mpv dzen2
```
Debian
```bash
sudo apt update; sudo apt install -y i3 i3lock i3status tmux zsh fonts-firacode ranger xclip xserver-xorg-input-libinput ffmpeg scrot jmtpfs x11-utils xdotool alsa-utils mpv dzen2
```
And:
```bash
git clone https://github.com/T04R/rice-kit.git
cd rice-kit
./make.sh
```
build bar 
```bash
cd ~/.config/i3/rice-kit/bar
cargo build --release
```
#### Quick Tools:
- **Quick Note** & **Quick Ranger**
- Quick Note `mod + n` 
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

