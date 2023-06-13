# ZYPP GUI 

It's a GUI application working on openSUSE distro. Application runs as a daemon to monitor the system update. Application can update the system, search, install and remove the package, change the settings of repos. It achieves some functions of command zypper.

zypp gui differs from GNOME software to:
- support general openSUSE desktop environments instead of GNOME only

zypp gui differs from YaST software management to:
- support desktop integration (e.g. update detection and notification)

## Targets:
- [x] Refresh the repos, download the packages, install the update.
- [x] Search, install and remove the package.
- [x] Offline update.
- [x] Add and remove repos.
- [x] Modify the repos.
- [x] Add mirror repos.
- [ ] Monitor the new release and update to the new release.

## Screenshot

![](./data/resources/screenshots/screenshot1.png)
![](./data/resources/screenshots/screenshot2.png)
![](./data/resources/screenshots/screenshot3.png)
![](./data/resources/screenshots/screenshot4.png)
![](./data/resources/screenshots/screenshot5.png)
![](./data/resources/screenshots/screenshot6.png)
![](./data/resources/screenshots/screenshot7.png)
![](./data/resources/screenshots/screenshot8.png)

## How to build:

#### meson
1. Install dependences:
```
zypper in meson gcc cargo glib2-devel gtk4-devel systemd-devel libpackagekit-glib2-devel polkit-devel libadwaita-devel
```
2. Compile:
```
meson --prefix /usr _builddir
meson compile -C _builddir
```
3. Install:
```
meson install -C _builddir
```

#### Open Build Service
[OBS Link](https://build.opensuse.org/package/show/home:xiaoguang_wang:branches:GNOME:Factory/openSUSE-software)
