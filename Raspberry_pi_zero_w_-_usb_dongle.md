# Raspberry Pi Zero W - Setup <!-- omit in toc -->

## Table of Contents <!-- omit in toc -->

- [**Before you begin**](#before-you-begin)
  - [**Minimum Requirements**](#minimum-requirements)
- [**Downloading/Installing the OS**](#downloadinginstalling-the-os)
- [**Manual Setup**](#manual-setup)
  - [**Navigate Raspberry OS**](#navigate-raspberry-os)
    - [**Wifi** {#manual-wifi}](#wifi-manual-wifi)
    - [**Change password** {#manual-pass-change}](#change-password-manual-pass-change)
    - [**Enable SSH** {#manual-ssh}](#enable-ssh-manual-ssh)
- [**Headless Setup**](#headless-setup)
  - [**Wifi** {#headless-wifi}](#wifi-headless-wifi)
  - [**Enable SSH** {#headless_ssh}](#enable-ssh-headless_ssh)
  - [**Change password** {#headless_pass_change}](#change-password-headless_pass_change)
- [**Change File Contents** {#file-change}](#change-file-contents-file-change)
- [**Connect using SSH**](#connect-using-ssh)
  - [**Linux and MacOS**](#linux-and-macos)
  - [**Windows**](#windows)
  - [**Android & iOS**](#android--ios)
- [**Finished**](#finished)
- [**Additional**](#additional)
  - [**Creating a new user**](#creating-a-new-user)
    - [**User Groups**](#user-groups)
  - [**Changing the hostname**](#changing-the-hostname)

## **Before you begin**

The scope of this document is setting up a Raspberry Pi Zero W, with Raspberry Pi OS Lite, and enabling SSH so that you can access it from your computer.

In this document, the actions will be presented first and explained second. This is so that you can skip past explanations, if not desired.

### **Minimum Requirements**

- Raspberry Pi Zero W
- MicroSD (8GB minimum recommended)
- MicroUSB power supply (this can be [official power supply](https://www.raspberrypi.org/documentation/hardware/raspberrypi/power/README.md) or MicroUSB to USB connected to a computer)

There will be two ways to set the Raspberry Pi up in the document.
The headless setup, which uses the minimal requirements, and manual setup, which requires extra cables/adapters. The manual setup requires a mini HDMI to HDMI adapter/cable to connect a Display, and microUSB to USB port to connect a keyboard.

## **Downloading/Installing the OS**

There are two ways to install a Raspberry Pi OS on a microSD:

- The first way is using the [Raspberry Pi Imager](https://www.raspberrypi.org/software/). This tool is an easy way to install any of the available Raspberry Pi OSs to the microSD. After you finish downloading and starting it up, you will be met with 3 buttons.
  When you select `operating system` it gives a list of available options. You want to select `Raspberry Pi OS (other)`, and from there get Raspberry Pi OS lite (32-bit). Then select the SD card you want to write the OS to and press write, and wait for the process to complete.

- The second way is writing it manually.
  You can download Raspberry Pi OS lite from the [official website](https://www.raspberrypi.org/software/operating-systems/).
  Then you need to burn it to the microSD using an external program ([Etcher](https://www.balena.io/etcher/), [UUbyte](https://www.uubyte.com/dmg-editor.html), [Win32 Disk Imager](https://win32diskimager.download/)).

After either process is done, you can proceed to [Manual Setup](#manual-setup) or [Headless Setup](#headless-setup).
For the manual setup, you can slot the microSD into the Raspberry Pi. For the headless setup, you can leave the microSD in the computer as you need to change some files.

## **Manual Setup**

With this setup, you need to connect 3 externals.

- A keyboard. The Raspberry Pi has a microUSB slot to which the keyboard will be connected. You can use a microUSB connector to USB port, or if the keyboard has a microUSB cable you can connect them directly.
- A display. The Raspberry Pi has a micro HDMI slot which you need to connect a display to. Either use a mini HDMI to HDMI cable, or an adapter, and plug it into the display and Raspberry.
- A microUSB to USB cable. Using it, plug the microUSB into the power(pwr) slot on the Raspberry Pi and connect it with the computer or a compatible power supply.

Once you have plugged the power in, the display should turn on, and display the boot sequence of the Raspberry Pi. After it finishes, it will ask you for a login and password. These are `pi` and `raspberry` respectively.

Note: if the login and/or password prompt appears and further boot sequence lines appear during typing, just hit `backspace` until you're sure no characters are remaining, and then continue typing as usual.

### **Navigate Raspberry OS**

Because the default user does not have elevated permissions, you will need to use [Sudo](https://en.wikipedia.org/wiki/Sudo) to run the command `sudo raspi-config`. This will open the configuration tool [Raspi-Config](https://www.raspberrypi.org/documentation/configuration/raspi-config.md). You can navigate this menu by using up and down arrow keys, the right and left arrow keys will switch between select and finish.

You need to do 3 things in this menu: connect to wifi, change default password and enable SSH.

#### **Wifi** {#manual-wifi}

To connect your Raspberry Pi to the wifi, press `enter` to go into the System Options menu. Then select `Wireless LAN`, and select your timezone from the list. You can press the first letter of your timezone (e.g. L to go to the L section), to skip to that section. Then enter the SSID (the wifi name), and passkey for the wifi.

If you'd like to check whether the connection was successful, press `ALT+F2` to view the command line again. Login, same as before, and type `ip a`. If you have connections shown, it should have worked. Then, press `ALT+F1` to go back to the config menu.

#### **Change password** {#manual-pass-change}

You want to change the password of the user, to make sure that it stays secure. The reason for this is that when you open the Raspberry Pi to SSH, anyone on the wifi can log in to the Raspberry if they know the credentials. This can be a risk if you're on open Wifi.

Navigate to the `System Options` menu, select `Password`, and change the password to something secure.

#### **Enable SSH** {#manual-ssh}

To enable SSH, navigate to the `Interface Options` menu and select `Enable SSH`. After enabling it, it will stay enabled until you disable it manually.

After completing these three steps, you can select `finish` in the base menu of `raspi-config`. This will confirm the settings and reboot the device. After it's done rebooting and you have logged in, you will need to enter `hostname -I` to reveal the device's IP address, which you will need later to connect with SSH. Then type `sudo poweroff` and after it is shutdown, insert the microSD back into the computer. Then continue from the section [Change File Contents](#file-change).

## **Headless Setup**

With the headless setup, you can setup the device for SSH without having to connect the device to a display and keyboard. You will need to create some files in `root` to do this.

### **Wifi** {#headless-wifi}

To [enable wifi](https://www.raspberrypi.org/documentation/configuration/wireless/headless.md), you need to create a file called `wpa_supplicant.conf` inside the root. Inside this file you need to put

```conf
ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
update_config=1
country=<Insert 2 letter ISO 3166-1 country code here>

network={
 ssid="<Name of your wireless LAN>"
 psk="<Password for your wireless LAN>"
}
```

and insert your own indicated information in the <> enclosures. Remove the <> symbols when doing so.

You can find a list of country ISO codes [here](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes).

### **Enable SSH** {#headless_ssh}

To enable [SSH](https://www.raspberrypi.org/documentation/remote-access/ssh/README.md), you need to create a file called `ssh`, without any extension, into the `root`. The content of this file does not matter. When the Raspberry Pi boots up, it will look for this file. If it detects the file, it will delete the file and enable SSH.

### **Change password** {#headless_pass_change}

You can change the password _after_ you have established the connection with SSH as you need the Raspberry Pi prompt for this action.
You can change it by using the command `passwd`, which will change it for the current user.
Alternatively you can use [sudo raspi-config](#manual-pass-change), which is explained in the manual setup. We can't use raspi-config for the other two setting changes, because those changes are necessary to connect with SSH and open the Raspberry prompt.

## **Change File Contents** {#file-change}

After the setup, there are two edits you will have to make in files.

First, in `config.txt` in `root`, you need to add `#dtoverlay=dwc2` at the end of the file.

- `dtoverlay` stands for _device tree overlay_. A [device tree](https://www.raspberrypi.org/documentation/configuration/device-tree.md#part1) describes the hardware of a device in a tree format. An [overlay](https://www.raspberrypi.org/documentation/configuration/device-tree.md#part2) is applied to the base device tree at a later point in order to apply changes (e.g. add a device, configure/enable a present one).
- `dwc` stands for _DesignWare Core_. `dwc_otg` is a driver for a USB controller built into the Raspberry PI itself, however this driver has been optimised to a point where it only does [host](https://en.wikipedia.org/wiki/USB#HOST) mode and not [OTG](https://en.wikipedia.org/wiki/USB_On-The-Go) mode. The `DWC2` is an upstream driver which can do the OTG host/peripheral flip. Note: if the device is purely used for host mode, dwc_otg has better performance.

Second, in cmdline.txt you will have to add `modules-load=dwc2,g_ether` after `rootwait`.

- The Linux kernel accepts a [commandline of parameters](https://www.raspberrypi.org/documentation/configuration/cmdline-txt.md) during boot, contained in the file `cmdline.txt`. The formatting of the `cmdline.txt` file is strict; commands are separated by spaces and newlines are not allowed.
- The `modules-load` parameter loads the given modules' dwc2, which is explained above, and `g_ether`, which allows ethernet emulation over USB. This then allows SSH, NFS, etc. over the USB while also charging/powering the device.

Then, you can remove the microSD and insert it into the Raspberry Pi. Connect the Raspberry to a power supply and continue to the next section.

## **Connect using SSH**

Connecting to the device through SSH can be different for every computer OS. You will need to know your [Raspberry Pi IP address](https://www.raspberrypi.org/documentation/remote-access/ip-address.md) for this section. There will be links to the Raspberry Pi website for a full explanation.

When connecting to the Raspberry Pi for the first time there will be a warning message, which describes the device, that will notify you that the device is not in the registry yet. This is normal and the process can be continued.

### **Linux and MacOS**

[These OSs](https://www.raspberrypi.org/documentation/remote-access/ssh/unix.md) have an SSH command build in. Using a terminal, use the command `ssh pi@<IP>`, replacing `<IP>` with your Raspberry Pi's IP. Then it will ask you for your password and you can log in.

### **Windows**

With Windows, it depends on the version.

[A Windows 10 version](https://www.raspberrypi.org/documentation/remote-access/ssh/windows10.md) that is using October 2018 update or later can enable a feature called `OpenSSH`. Go to Settings > Apps > Apps & Features > Manage Optional Features > Add A Feature, and choose to install OpenSSH Client. With this enabled, you can use the command `ssh pi@<IP>` in a terminal, replacing `<IP>` with your Raspberry Pi's IP. Then it will ask you for your password and you can log in.

With other versions of [Windows](https://www.raspberrypi.org/documentation/remote-access/ssh/windows.md), you will need to download an SSH client. You can use [Putty](https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html) for this purpose. Once you have downloaded and started Putty up, you can enter `Raspberrypi.local` or the IP address of the Raspberry Pi in the `Host Name` field. Then you can proceed to enter the username and password of the Raspberry Pi.

### **Android & iOS**

[Android](https://www.raspberrypi.org/documentation/remote-access/ssh/android.md) and [iOS](https://www.raspberrypi.org/documentation/remote-access/ssh/ios.md) do not have a command build in, and you will need to download a client for either of them.

For both Android and iOS, there is [termius](https://www.termius.com/). With this app, you want to add your Raspberry Pi as a new host. You will need to enter the Raspberry Pi's user ID. password and IP address. After confirming, make sure you are connected to the same network as the Raspberry Pi and tap the entry. This will open the Raspberry Pi prompt and you should have connected.

## **Finished**

After completing these steps you will have the Raspberry Pi prompt open to work with.

If you want to [shutdown](https://www.tecmint.com/shutdown-poweroff-halt-and-reboot-commands-in-linux/) the Raspberry Pi you can use `sudo poweroff`.

For more information on the Raspberry Pi, and how you can use it, you can use the [official documentation](https://www.raspberrypi.org/documentation/)

## **Additional**

### **Creating a new user**

[Making a new user](https://www.raspberrypi.org/documentation/linux/usage/users.md) on a Raspberry Pi is the same process as it would be on Linux.

Creating new users will be done with the `sudo adduser USER` command, where you replace `USER` with a username of your choice. The `sudo` is required because this is a `root` command (like `raspi-config` previously). After entering the command, you will be prompted for a password. You can leave this blank by just pressing enter if you do not want a password for this user. This will make a normal user without elevated privileges.

If you need to change the password of a user, there is the `passwd` command. After using this, the user will be prompted to enter a new password and confirm it. Alternatively you can run `passwd` from an elevated user to change the password of other users (e.g. `sudo passwd bob` will change the password for user `bob`).

#### **User Groups**

If a user needs to have elevated privileges, you need to add it to the `sudo` group. Appending users to a group can be done with `sudo adduser USER GROUP` where you replace `GROUP` with the group name, which in this case would be `sudo` (e.g. `sudo adduser bob sudo`).

You can also adjust an existing user's groups with the [`usermod`](https://linuxize.com/post/usermod-command-in-linux/) command. Because this is a `root` command, you need to execute it from a user with elevated privileges. To add an existing user to a group, or multiple groups, the syntax will be `sudo usermod -a -G GROUPS USER`.

- `-a` is to append a user to the supplemented groups without removing it from other groups.
- `-G` is for a list of supplemented groups. **Note:** `-G` needs to be capital G because `-g` is a different parameter, and it needs to be a comma separated list without spaces in between (e.g. `group1,group2,group3`). You can view more information about `usermod` and its parameters by typing `usermod --help` in the command line.

An example command will look like `sudo usermod -a -G sudo,builder bob`.


### **Changing the hostname**

Changing the hostname of the Raspberry Pi can be done in two ways.

**Note:** When changing the hostname only use letters a to z, digits 0 to 9 and the hyphen character. It may not begin or end with a hyphen, nor contain any other character than previously described.

The first is using the terminal to change the necessary file directly. To open the file, use `sudo nano /etc/hostname`.

- `nano` is a Linux command-line text editor.
- `/etc/` is a folder which contains all of your system configuration files. `etc` stands for et cetera, it is used for a collection of files that do not fit in other folders.
- `hostname` is the file we want to edit. It only contains one line, the name you want to edit.

Because `/etc/` contains configuration files you need to be careful with changing contents, and thus `sudo` is needed to confirm write privileges.
When you have opened the file, you should see raspberrypi. Changing this will change the hostname of the device _after reboot_. To save the changes use `CTRL-o` and press enter. To exit nano use `CTRL-x` and to see extra information/help use `CTRL-g`.

The second is using `sudo raspi-config`. In the menu, navigate to `System Options` and then `Hostname`, where - after a description of what _not_ to do - you can change and save the new hostname.
