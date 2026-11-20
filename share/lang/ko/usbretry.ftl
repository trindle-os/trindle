# machine translated (Korean), not yet reviewed by a person.

## USB retry (src/usbretry): a USB port that did not start is switched off and on again, and the result is kept for Settings.

# Settings' words for a port: it did not start, Trindle switched it off and on once, and a device came up. $port is the port's name.
usbretry-worked = USB 포트 { $port }이(가) 시작되지 않아서 Trindle이 한 번 껐다 켰어요. 기기가 감지되었어요.
# Settings' words for a port: it did not start, Trindle restarted the USB controller, and a device came up. $port is the port's name.
usbretry-worked-controller = USB 포트 { $port }이(가) 시작되지 않아서 Trindle이 USB 컨트롤러를 다시 시작했어요. 기기가 감지되었어요.
# Settings' words for a port: it did not start, the USB controller was restarted, and nothing came up. $port is the port's name.
usbretry-nothing-controller = USB 포트 { $port }이(가) 시작되지 않았어요. Trindle이 USB 컨트롤러를 다시 시작했지만 기기가 감지되지 않았어요. 컴퓨터를 껐다 켜서 포트를 되살려 보세요.
# Settings' words for a port: nothing came back after it was switched off and on, and the controller was not restarted. $port is the port's name.
usbretry-nothing = USB 포트 { $port }이(가) 다시 시작한 뒤에도 시작되지 않았어요. 컴퓨터를 껐다 켜서 포트를 되살려 보세요.
