#!/bin/bash

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
chsh -s /bin/zsh

mkdir -p ~/.oh-my-zsh/custom/plugins
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
