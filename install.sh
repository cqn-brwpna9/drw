#!/bin/sh
#requires sudo permissions

#add syntax highlighter
sudo chown root drw.nanorc
sudo chmod 664 drw.nanorc
sudo ln ./drw.nanorc /usr/share/nano #allows for updating the formatter

#do a build
#make an env variable point here at some point
cargo build

