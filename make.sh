#!/bin/bash

[ "$EUID" -ne 0 ] && { echo "try root." >&2; exit 1; }

read -p "emerge(e),pacman(p),apt(a): " a

if [ "$a" = "e" ]; then
    sudo emerge --sync
    echo "x11-drivers/xf86-input-libinput" >> /etc/portage/package.use
    sudo emerge --ask x11-wm/i3 x11-misc/i3lock x11-misc/i3status app-misc/tmux app-shells/zsh media-fonts/fira-code app-misc/ranger x11-misc/xclip sys-auth/elogind xf86-input-libinput netifrc iwd dhcpcd
    sudo mkdir -p /etc/iwd ; sudo cp iwd-main.conf /etc/iwd/main.conf
    sudo mkdir -p /etc/conf.d ; cp net /etc/conf.d/net
    sudo rc-update add iwd default ; sudo rc-update add net.wlan0 default ; sudo rc-update add dhcpcd default ; rc-update del NetworkManager default
    sudo cp rule.rules /etc/udev/rules.d/rule.rules
    sudo echo "tor ALL=(ALL) NOPASSWD: /sbin/shutdown, /sbin/loginctl" | sudo tee -a /etc/sudoers
    sudo rc-update add elogind
    sudo echo "media-video/ffmpeg x264 x265 fdk-aac mp3 opus" | sudo tee -a /etc/portage/package.use/ffmpeg media-video/mpv
    sudo emerge --ask media-video/ffmpeg media-gfx/scrot sys-fs/jmtpfs x11-apps/xwininfo x11-misc/xdotool media-sound/alsa-utile

elif [ "$a" = "p" ]; then
    sudo pacman -Syu
    sudo pacman -S i3-wm i3lock i3status tmux zsh ttf-fira-code ranger xclip xf86-input-libinput
    sudo pacman -S ffmpeg scrot jmtpfs xorg-xwininfo xdotool alsa-utils mpv

elif [ "$a" = "a" ]; then
    sudo apt update
    sudo apt install i3 i3lock i3status tmux zsh fonts-firacode ranger xclip xserver-xorg-input-libinput
    sudo apt install ffmpeg scrot jmtpfs x11-utils xdotool alsa-utils mpv

else
    echo "invalid!"
    exit
fi

su $USER
read -p "which? openrc(o) systemd(s): " b

if [ "$b" = "o" ]; then
    cp config-openrc ~/.config/i3
elif [ "$b" = "s" ]; then
    cp config-systemd ~/.config/i3
else
    echo "invalid!"
    exit
fi

cp -r rice-kit ~/.config/i3
cp .tmux.conf ~/.tmux.conf
cp .zshrc ~/.zshrc

git clone https://github.com/zsh-users/zsh-syntax-highlighting.git \
~/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting
git clone https://github.com/zsh-users/zsh-autosuggestions.git \
~/.oh-my-zsh/custom/plugins/zsh-autosuggestions

#st
git clone https://git.suckless.org/st
cp st-config.h st/config.h
cd st
sudo make clean install
cd ..

#dzen2
git clone https://github.com/robm/dzen.git
cd dzen
sudo make install
sudo cp dzen2 /usr/local/bin
cd ..

#touchpad
sudo mkdir -p /etc/X11/xorg.conf.d
sudo cp 30-touchpad.conf /etc/X11/xorg.conf.d/30-touchpad.conf

#mpv
mkdir ~/.config/mpv
echo "no-audio-display" > ~/.config/mpv/mpv.conf
echo "r cycle-values loop-file "inf" "no"" > ~/.config/mpv/input.conf

exit
