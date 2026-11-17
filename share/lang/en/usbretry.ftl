## USB retry (src/usbretry): a USB port that did not start is switched off and on again, and the result is kept for Settings.

# Settings' words for a port: it did not start, Trindle switched it off and on once, and a device came up. $port is the port's name.
usbretry-worked = USB port { $port } didn't start, so Trindle switched it off and on once. A device was detected.
# Settings' words for a port: it did not start, Trindle restarted the USB controller, and a device came up. $port is the port's name.
usbretry-worked-controller = USB port { $port } didn't start, so Trindle restarted its USB controller. A device was detected.
# Settings' words for a port: it did not start, the USB controller was restarted, and nothing came up. $port is the port's name.
usbretry-nothing-controller = USB port { $port } didn't start. Trindle restarted its USB controller, but no device was detected. Switch the computer off and on again to bring the port back.
# Settings' words for a port: nothing came back after it was switched off and on, and the controller was not restarted. $port is the port's name.
usbretry-nothing = USB port { $port } didn't start again after a restart. Switch the computer off and on again to bring the port back.
