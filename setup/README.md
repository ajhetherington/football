# With WSL
Setup is a bit of a pain but it is possible to run all the rust, vscode & stuff through WSL, but then also display things through to windows through a window.

It's not that easy however....


## Step 1: Install VcXsrv
You can grab it [here](https://sourceforge.net/projects/vcxsrv/), don't be worried about sourceforge, it should be fine (think it's open source so should be alright). This is basically the windows software that allows for other programs to output gui to it, through a server (as far as i can understant). This is on the windows side, & also requires you to allow the program to pass through the firewall. (again, don't worry, but it seems a bit bad...)

This actually appears (at least for me) as `Xserver` in the start menu. When starting check all the checkboxes in the second page, it shouldn't output anything 

Here im going to assume you've got a vscode dev environment that remotes into an ubuntu wsl machine. Here you need to run the following:

```bash
sudo sh -c "dbus-uuidgen > /etc/machine-id"
## Append the following to your ~/.bashrc or ~/.zshrc, remember to resource
# X Server
export $(dbus-launch) # not needed if you have systemd enabled
export LIBGL_ALWAYS_INDIRECT=0 # conflicting on whether this is needed... setting to 0 fixed raylib

export DISPLAY=$(route.exe print | grep 0.0.0.0 | head -1 | awk '{print $4}'):0.0 
```
This was piced together through [this](https://gist.github.com/djfdyuruiry/3150b9e5f3dadba89ea323df49ea7ab1) and then the last line is from [here](https://github.com/microsoft/WSL/issues/4106#issuecomment-803607362)

All that's left is to install x11-apps

```bash
sudo apt-get install -y x11-apps
xeyes
```

& That should be it, you should see a pair of eyes pacing around the screen

## Deving
For development, you need
1. Start Xserver
2. Startup vscode & go to the `football` project
3. run `xeyes` to make sure it's working
4. `cargo run`, should output a window


### Update
