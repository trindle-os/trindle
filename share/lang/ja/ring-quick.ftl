# machine translated (Japanese), not yet reviewed by a person.

# The ring's quick pages: sound, Bluetooth, Wi-Fi, brightness, clipboard, notifications and battery, opened from the round buttons in the ring. Part of the ring's own words (see ring.ftl).

## Quick pages (src/ring/quick/)

# Tip of the pin button on a clipboard or notification item that is not pinned: keeps it for good.
ring-quick-pin = 固定して残す
# Tip of the pin button on a clipboard or notification item that is pinned.
ring-quick-unpin = 固定を解除
# Tip of the bin button that forgets one clipboard item or notification; it asks twice.
ring-quick-forget-one = これを消す
# Tip of the bin button that forgets everything on the page except what is pinned; it asks twice.
ring-quick-forget-all = 固定したもの以外をすべて消す
# Name of the volume slider for the speakers on the sound page.
ring-quick-speakers = スピーカー
# Name of the volume slider for the microphone on the sound page.
ring-quick-microphone = マイク
# Tip of the battery page button for the power profile that saves the battery.
ring-quick-profile-saver = 省電力
# Tip of the battery page button for the balanced power profile.
ring-quick-profile-balanced = バランス
# Tip of the battery page button for the power profile that favours speed.
ring-quick-profile-performance = パフォーマンス
# Heading of the sound page.
ring-quick-title-sound = サウンド
# Heading of the Bluetooth page.
ring-quick-title-bluetooth = Bluetooth
# Heading of the Wi-Fi page.
ring-quick-title-wifi = Wi-Fi
# Heading of the brightness page.
ring-quick-title-brightness = 明るさ
# Heading of the clipboard page.
ring-quick-title-clipboard = クリップボード
# Heading of the notifications page.
ring-quick-title-notifications = 通知
# Title of the panel in the ring's middle that lists every tray icon.
ring-quick-title-tray = トレイのアイコン
# Heading of the shortcut list page, opened with Super and K.
ring-quick-title-shortcuts = ショートカット
# Heading of the battery page.
ring-quick-title-battery = バッテリー
# Tip of the round button at the top of a quick page that opens the matching page in Settings.
ring-quick-more-settings = 設定でさらに表示
# Screen reader name of the brightness slider.
ring-quick-brightness-slider = 画面の明るさ
# Tip of the button that turns a muted sound back on.
ring-quick-sound-on = 音を戻す
# Button on the Sound page that plays a short tune through the speakers, and its label.
ring-quick-test-speakers = スピーカーをテスト
# What the microphone's level bar shows, for the screen reader.
ring-quick-mic-level = マイクの音量レベル
# Tip of the button that mutes the sound.
ring-quick-mute = ミュート
# Name of the Bluetooth switch, and the label beside it, at the top of the Bluetooth page.
ring-quick-bt-switch = Bluetooth
# Line on the Bluetooth page when Bluetooth is switched off.
ring-quick-bt-off = Bluetooth はオフです。
# Line on the Bluetooth page while it looks for devices and none is found yet.
ring-quick-bt-searching = 探しています… 機器をペアリングモードにしてください。
# Tip of the button beside a connected Bluetooth device that disconnects it.
ring-quick-bt-disconnect = 切断
# Tip of the button beside a paired Bluetooth device that connects it.
ring-quick-bt-connect = 接続
# Tip of the button beside a new Bluetooth device that pairs and connects it.
ring-quick-bt-pair = ペアリングして接続
# Name used for a Bluetooth device that has no name of its own.
ring-quick-bt-device = Bluetooth 機器
# Line under the Bluetooth page while pairing with a device.
ring-quick-bt-pairing = { $name } とペアリング中…
# Line under the Bluetooth page while connecting a device.
ring-quick-bt-connecting = { $name } に接続中…
# Line under the Bluetooth page while disconnecting a device.
ring-quick-bt-disconnecting = { $name } を切断中…
# Line under the Bluetooth page when connecting or disconnecting did not work.
ring-quick-bt-failed = うまくいきませんでした。電源が入っていて、近くにありますか。
# Line under the Bluetooth page when pairing failed because the codes were not accepted.
ring-quick-bt-codes = ペアリングできませんでした。コードが合いませんでした。
# Line under the Bluetooth page when pairing failed.
ring-quick-bt-not-paired = ペアリングできませんでした。ペアリングモードで、近くにありますか。
# Line under the Bluetooth page when pairing worked but connecting did not.
ring-quick-bt-paired-not-connected = ペアリングはできましたが、接続できませんでした。少し待ってからもう一度お試しください。
# Question in the ring when a Bluetooth device wants a PIN typed.
ring-quick-bt-ask-pin = この機器の PIN を入力してください。機器の説明書に書かれていることが多いです。0000 も試せます。
# Typing box for the PIN of a Bluetooth device, shown quietly while empty.
ring-quick-bt-pin-box = PIN
# Button in the ring that sends the PIN and pairs, or agrees to pair.
ring-quick-bt-ask-pair = ペアリング
# Button in the ring that gives up a question about pairing or joining.
ring-quick-cancel = キャンセル
# Instruction in the ring when a Bluetooth device shows a code to be typed on the device itself.
ring-quick-bt-type-on-device = 機器でこれを入力し、機器の Enter を押してください。
# Question in the ring when a Bluetooth device shows a number to compare.
ring-quick-bt-same-number = 機器にも同じ番号が表示されていますか。
# Button in the ring that agrees the two numbers match.
ring-quick-bt-matches = はい、同じです
# Button in the ring that says no to a question about pairing.
ring-quick-no = いいえ
# Question in the ring when a Bluetooth device asks to pair.
ring-quick-bt-pair-this = この機器とペアリングしますか。
# Name of the Wi-Fi switch, and the label beside it, at the top of the Wi-Fi page.
ring-quick-wifi-switch = Wi-Fi
# Line on the Wi-Fi page when Wi-Fi is switched off.
ring-quick-wifi-off = Wi-Fi はオフです。
# Line on the Wi-Fi page while it looks for networks.
ring-quick-wifi-looking = ネットワークを探しています…
# Tip of the button beside the Wi-Fi network in use that leaves it.
ring-quick-wifi-disconnect = 切断
# Tip of the button beside a Wi-Fi network that joins it.
ring-quick-wifi-connect = 接続
# Typing box for the password of a Wi-Fi network, shown quietly while empty.
ring-quick-wifi-password-box = パスワード
# Button in the ring that joins a Wi-Fi network with the typed password.
ring-quick-wifi-join = 参加
# Line under the Wi-Fi page while joining a network.
ring-quick-wifi-joining = { $ssid } に参加中…
# Question text in the ring when the Wi-Fi password was not accepted.
ring-quick-wifi-wrong-password = パスワードが違います。
# Line under the Wi-Fi page when joining failed and the system gave no reason.
ring-quick-wifi-failed = 参加できませんでした。
# Line under the Wi-Fi page when leaving the network did not work.
ring-quick-wifi-leave-failed = 切断できませんでした。
# Line on the battery page when the computer has no battery.
ring-quick-battery-none = このコンピューターにバッテリーはありません。
# Line under a quick page when the screen has no brightness control.
ring-quick-brightness-fixed = この画面の明るさはここからは変えられません
# Line on the clipboard page when nothing has been copied yet.
ring-quick-clip-empty = まだ何もコピーしていません。
# Line on the clipboard page for a copied picture.
ring-quick-clip-picture = 画像
# Tip of the button that copies a clipboard item again.
ring-quick-clip-again = もう一度コピー
# Typing box at the top of the clipboard page that filters the items, shown quietly while empty.
ring-quick-clip-filter = 絞り込み
# Line on the notifications page when nothing has been said yet.
ring-quick-notes-empty = 通知はまだありません。
# Tip of the button on one notification's page that goes back to the list.
ring-quick-notes-back = すべての通知に戻る
# Tip of the copy button for a moment after it was pressed.
ring-quick-copied = コピーしました
# Tip of the button on a crash notification that copies its details for an assistant.
ring-quick-copy-assistant = アシスタント用にコピー
# Line on the battery page while charging, with the time until full.
ring-quick-charging-full = 充電中: 満充電まであと約 { $time }
# Line on the battery page while charging, when the time is not known.
ring-quick-charging = 充電中
# Line on the battery page on battery, with the time left and the power in watts.
ring-quick-on-battery-left = バッテリー駆動中です。残り約 { $time }、消費電力は { $rate } W です。
# Line on the battery page on battery, when the time is not known.
ring-quick-on-battery = バッテリー駆動中
# Line on the battery page when the battery is full.
ring-quick-full = 満充電
# Line on the battery page when plugged in and not charging.
ring-quick-plugged = 電源に接続中、充電はしていません
# Line on the battery page about how much charge the battery still holds. The health is good, worn, or something else for worn out.
ring-quick-health =
    { $how ->
        [good] バッテリーは良好です。新品のときの { $capacity }% まで充電できます。
        [worn] バッテリーは劣化しています。新品のときの { $capacity }% まで充電できます。
       *[old] バッテリーが寿命です。交換の時期です。新品のときの { $capacity }% まで充電できます。
    }
# Line on the battery page about how many times the battery has been charged.
ring-quick-cycles =
    { $cycles ->
       *[other] { $cycles } 回充電されました。
    }
# Tip of the button that turns notification circles off.
ring-quick-notes-on = 通知はオンです。オフにする
# Tip of the button that turns notification circles back on.
ring-quick-notes-off = 通知はオフです。オンにする
# A time of under an hour on the battery page.
ring-quick-minutes = { $m } 分
# A time of whole hours on the battery page.
ring-quick-hours = { $h } 時間
# A time of hours and minutes on the battery page.
ring-quick-hours-minutes = { $h } 時間 { $m } 分
