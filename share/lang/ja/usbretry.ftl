# machine translated (Japanese), not yet reviewed by a person.

## USB retry (src/usbretry): a USB port that did not start is switched off and on again, and the result is kept for Settings.

# Settings' words for a port: it did not start, Trindle switched it off and on once, and a device came up. $port is the port's name.
usbretry-worked = USB ポート { $port } が起動しなかったため、Trindle が一度オフにしてからオンにしました。デバイスが検出されました。
# Settings' words for a port: it did not start, Trindle restarted the USB controller, and a device came up. $port is the port's name.
usbretry-worked-controller = USB ポート { $port } が起動しなかったため、Trindle が USB コントローラーを再起動しました。デバイスが検出されました。
# Settings' words for a port: it did not start, the USB controller was restarted, and nothing came up. $port is the port's name.
usbretry-nothing-controller = USB ポート { $port } が起動しませんでした。Trindle は USB コントローラーを再起動しましたが、デバイスは検出されませんでした。ポートを元に戻すには、コンピューターの電源を切ってから入れ直してください。
# Settings' words for a port: nothing came back after it was switched off and on, and the controller was not restarted. $port is the port's name.
usbretry-nothing = USB ポート { $port } は、再起動してもまた起動しませんでした。ポートを元に戻すには、コンピューターの電源を切ってから入れ直してください。
