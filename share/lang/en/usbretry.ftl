## USB retry (src/usbretry): a USB port that did not start is switched off and on again, and the result is kept for Settings.

# Settings' words for a port: it did not start, Trindle switched it off and on once, and a device came up. $port is the port's name.
usbretry-worked = USB port { $port } did not start, so Trindle switched it off and on once. A device came up.
# Settings' words for a port: it did not start, Trindle restarted the USB controller, and a device came up. $port is the port's name.
usbretry-worked-controller = USB port { $port } did not start, so Trindle restarted its USB controller. A device came up.
# Settings' words for a port: it did not start, the USB controller was restarted, and nothing came up. $port is the port's name.
usbretry-nothing-controller = USB port { $port } did not start. Trindle restarted its USB controller and nothing came up. Switch the computer off and on to bring it back.
# Settings' words for a port: nothing came back after it was switched off and on, and the controller was not restarted. $port is the port's name.
usbretry-nothing = USB port { $port } did not come back after a restart. Switch the computer off and on to bring it back.
