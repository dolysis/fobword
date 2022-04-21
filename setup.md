1. flash the sdcard.img file
2. move the init_usb_gadget to the device
3. move the dys-template to the device
4. move the qwerty-layout to the device
5. give chmod +x to init_usb_gadget and dys-template
6. add "none /sys/kernel/config configfs rw,relatime 0 0 to /etc/fstab
7. reboot
8. run mkdir -p /sys/kernel/config/usb_gadget
9. ln /usr/bin/dys-template /etc/init.d/S60Fobword
10. ln /usr/bin/init_usb_gadget /etc/init.d/S30initgadget