#!/bin/bash

# Function to ask the user
ask() {
    local name="$1"
    local cmd="$2"

    while true; do
        read -p "Do you want to run '$name'? (y/n/all): " answer
        case "$answer" in
            y|Y|yes|YES)
                eval "$cmd"
                return 0
                ;;
            n|N|no|NO)
                echo "Skipped: $name"
                return 0
                ;;
            all|ALL|a|A)
                echo "ALL mode enabled - everything will be installed"
                RUN_ALL=true
                eval "$cmd"
                return 0
                ;;
            *)
                echo "Please enter y, n, or all"
                ;;
        esac
    done
}

RUN_ALL=false

run_step() {
    local name="$1"
    local cmd="$2"

    if [ "$RUN_ALL" = true ]; then
        echo ">>> Running: $name"
        eval "$cmd"
    else
        ask "$name" "$cmd"
    fi
}

# ============ Install Tools ============
run_step "Install base tools (netcat, helix, git, ...)" '
sudo pacman -S --noconfirm netcat helix git base-devel rust python3 mpv xclip wl-clipboard ranger ttf-jetbrains-mono xf86-input-libinput ffmpeg scrot xorg-xwininfo xdotool rust-analyzer ty clang lldb
zip unzip'

# ============ USB ============
run_step "Install USB tools (thunar, gvfs, mtpfs)" '
sudo pacman -S --noconfirm thunar gvfs mtpfs gvfs-mtp
'

# ============ QEMU ============
run_step "Install QEMU and Virt-Manager" '
sudo pacman -S --noconfirm qemu-full virt-manager dnsmasq
sudo systemctl enable libvirtd
sudo systemctl start libvirtd
sudo usermod -aG libvirt $(whoami)
sudo usermod -aG kvm $(whoami)
'

# ============ Copy i3 config ============
run_step "Copy i3 config and rice-kit" '
mkdir -p ~/.config/i3
cp config ~/.config/i3/config
cp -r rice-kit ~/.config/i3
cp .tmux.conf ~/.tmux.conf
'

# ============ Change shell to fish ============
run_step "Change default shell to fish" '
chsh -s /bin/fish
'

# ============ st ============
run_step "Install st (terminal)" '
git clone https://git.suckless.org/st
cp st-config.h st/config.h
cd st
sudo make clean install
cd ..
'

# ============ keyd ============
run_step "Configure keyd" '
sudo cp default.conf /etc/keyd/default.conf
sudo systemctl enable keyd
sudo systemctl start keyd
'

# ============ Dark theme ============
run_step "Install dark theme" '
unzip /theme/Kali-Dark.zip
sudo cp -r /theme/Kali-Dark /usr/share/themes/

mkdir -p ~/.config/gtk-3.0
echo "[Settings]
gtk-theme-name=Kali-Dark" > ~/.config/gtk-3.0/settings.ini

sudo pacman -S --noconfirm qt5ct qt6ct
echo "export QT_QPA_PLATFORMTHEME=qt5ct" >> ~/.profile
'

# ============ Touchpad ============
run_step "Configure touchpad" '
sudo mkdir -p /etc/X11/xorg.conf.d
sudo cp 30-touchpad.conf /etc/X11/xorg.conf.d/30-touchpad.conf
'

# ============ mpv ============
run_step "Configure mpv" '
mkdir -p ~/.config/mpv
echo "no-audio-display" > ~/.config/mpv/mpv.conf
echo "r cycle-values loop-file \"inf\" \"no\"" > ~/.config/mpv/input.conf
'

# ============ udev-rules ============
run_step "Copy udev rules" '
sudo mkdir -p /etc/udev/rules.d
sudo cp rule.rules /etc/udev/rules.d/rule.rules
'

# ============ helix ============
run_step "Configure helix" '
mkdir -p ~/.config/helix/runtime/themes
cp helix/config.toml ~/.config/helix/config.toml
'

# ============ Compile bar ============
run_step "Compile bar with cargo" '
cd ~/.config/i3/rice-kit/bar
cargo build --release
cd ~/rice-kit
'

# ============ Reboot ============
run_step "Reboot system" '
reboot
'

