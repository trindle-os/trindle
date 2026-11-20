# machine translated (Japanese), not yet reviewed by a person.

# Settings, Bluetooth page: the adapter, pairing, connecting and forgetting devices.

## Bluetooth page (src/settings/panels/bluetooth.rs)

# Said in red under the Bluetooth switch when switching it on failed for no known reason.
settings-bluetooth-on-failed = Bluetooth をオンにできませんでした。
# Said in red under the Bluetooth switch when switching it off failed for no known reason.
settings-bluetooth-off-failed = Bluetooth をオフにできませんでした。
# Said in red when Bluetooth refuses something because it is not switched on; the first sentence says what could not be done.
settings-bluetooth-cause-not-ready = { $could_not } Bluetooth がオンになっていません。
# Said in red when the PIN or number typed was wrong; the first sentence says what could not be done.
settings-bluetooth-cause-auth-failed = { $could_not } PIN または数字が受け付けられませんでした。もう一度試してください。
# Said in red when the pairing was turned down; the first sentence says what could not be done.
settings-bluetooth-cause-turned-down = { $could_not } ペアリングが断られました。こちらか相手のデバイスで断られた可能性があります。
# Said in red when the other device did not answer in time; the first sentence says what could not be done.
settings-bluetooth-cause-timeout = { $could_not } 相手のデバイスが時間内に応答しませんでした。ペアリングモードにしてから、もう一度試してください。
# Said in red when the same thing is already being done; the first sentence says what could not be done.
settings-bluetooth-cause-in-progress = { $could_not } Trindle はすでにこの処理をしています。少し待ってください。
# Said in red when the device did not answer; the first sentence says what could not be done.
settings-bluetooth-cause-no-answer = { $could_not } デバイスが応答しませんでした。電源が入っていて近くにあるか確かめてから、もう一度試してください。
# Said in red when the device is not ready to be used; the first sentence says what could not be done.
settings-bluetooth-cause-not-available = { $could_not } デバイスを使える状態ではありません。ペアリングモードにしてから、もう一度試してください。
# The word for the system bus in "The system bus is not available".
settings-bluetooth-the-bus = システムバス
# Headline of the page while Bluetooth is looked for.
settings-bluetooth-top-looking = Bluetooth を確認しています…
# Headline of the page when the Bluetooth service is not running.
settings-bluetooth-top-gone = このコンピューターでは Bluetooth が動いていません。
# Headline of the page when Bluetooth is off.
settings-bluetooth-top-off = Bluetooth はオフです。
# Headline of the page when Bluetooth is off at a switch or key on the computer itself.
settings-bluetooth-top-hard = コンピューターのスイッチまたはキーで Bluetooth がオフになっています。
# Headline of the page when the computer has no Bluetooth.
settings-bluetooth-top-none = このコンピューターには Bluetooth がありません。
# Headline of the page when the computer has an adapter that Bluetooth has not finished setting up.
settings-bluetooth-top-starting = Bluetooth を起動しています…
# Headline of the page when the adapter was there a moment ago and has gone.
settings-bluetooth-top-lost = Bluetooth アダプターがなくなりました。
# Headline of the page when Bluetooth is on and searching.
settings-bluetooth-top-searching = オンです。デバイスを探しています…
# Headline of the page when Bluetooth is on and nothing is connected.
settings-bluetooth-top-idle = オンですが、何も接続されていません。
# Headline of the page when Bluetooth is on and devices are connected; the names are already joined into a list.
settings-bluetooth-top-connected =
    { $n ->
       *[other] オンです。{ $names } が接続されています。
    }
# Help text (i button) of the Bluetooth card.
settings-bluetooth-card-about = スイッチの下の名前は、Bluetooth アダプターの名前です。「他のデバイスから見える」がオンの間は、近くのデバイスがこのコンピューターを見つけられます。
# Heading of the Bluetooth card, of its switch's line, and the name of its switch.
settings-bluetooth-title = Bluetooth
# Heading of the line with the switch that makes this computer visible, and the name of that switch.
settings-bluetooth-visible = 他のデバイスから見える
# Heading of the card when Bluetooth is off at a switch on the computer itself.
settings-bluetooth-hard-line = コンピューター本体でオフになっています
# Small line under it.
settings-bluetooth-hard-line-sub = オンにするには、コンピューターのスイッチまたはキーを使ってください。
# Heading of the card when the computer has no Bluetooth adapter.
settings-bluetooth-no-adapter = Bluetooth アダプターがありません
# Help text (i button) of that card.
settings-bluetooth-no-adapter-about = Trindle はこのコンピューターに Bluetooth アダプターを見つけられませんでした。USB ポートに差し込むと、すぐここに表示されます。
# Line in that card.
settings-bluetooth-none-found = 見つかりません
# Line in the no-adapter card while Bluetooth finishes setting up the adapter.
settings-bluetooth-starting = 起動中
# Line in the no-adapter card when the adapter was there a moment ago.
settings-bluetooth-lost = アダプターがなくなりました
# Small line under it.
settings-bluetooth-lost-sub = Trindle は探し続け、戻ってきたらここに表示します。戻らないときは、コンピューターを再起動してください。
# Shown beside the Bluetooth switch while the password is asked for.
settings-bluetooth-waiting = パスワードを待っています…
# Said in red when this computer could not be made visible to other devices.
settings-bluetooth-visible-failed = Trindle はこのコンピューターを他のデバイスから見えるようにできませんでした。
# Said in red when this computer could not be hidden from other devices.
settings-bluetooth-hidden-failed = Trindle はこのコンピューターを他のデバイスから隠せませんでした。
# Shown on the two device lists while Bluetooth is off.
settings-bluetooth-switch-on-first = これらを使うには Bluetooth をオンにしてください。
# Line in the My devices card when nothing is paired.
settings-bluetooth-none-paired = まだペアリングしたものはありません
# Heading of the card that lists paired devices.
settings-bluetooth-mine = マイデバイス
# Help text (i button) of the My devices card.
settings-bluetooth-mine-about = このコンピューターとペアリングしたデバイスです。読み取れるときは、バッテリー残量も表示します。ページを開いたときに接続していたデバイスが先に並び、開いている間は順番が変わりません。デバイスを削除すると、もう一度ペアリングが必要です。
# Line in the Nearby card while the search runs.
settings-bluetooth-looking = デバイスを探しています…
# Tip of the button that stops the search.
settings-bluetooth-stop-looking = 探すのをやめる
# Heading of the line that starts the search, and tip of its button.
settings-bluetooth-look = デバイスを探す
# Heading of the card that lists devices nearby.
settings-bluetooth-nearby = 近くのデバイス
# Help text (i button) of the Nearby card.
settings-bluetooth-nearby-about = 「デバイスを探す」を押し、相手のデバイスをペアリングモードにして、横の「ペアリング」を押してください。ページを開いたときは、電波が最も強いデバイスが先に並び、新しく見つかったものは最後に加わります。デバイスによっては、ここと同じ数字が表示されるので、見比べてください。PIN を求めるものもあります。PIN は説明書に書いてあることが多く、0000 の場合もあります。
# Said in red when the search could not be stopped.
settings-bluetooth-stop-failed = Trindle はデバイスを探すのをやめられませんでした。
# Said in red when the search could not be started.
settings-bluetooth-start-failed = デバイスを探せませんでした。
# Small line of a device that is connected.
settings-bluetooth-connected = 接続済み
# Small line of a paired device that is not connected.
settings-bluetooth-not-connected = 未接続
# Part of a device's small line: its battery level in per cent.
settings-bluetooth-battery = バッテリー { $n }%
# Tip of the button that pairs a device, and the button of the same name in the pairing question.
settings-bluetooth-pair = ペアリング
# Tip of the button that disconnects a device.
settings-bluetooth-disconnect = 切断
# Tip of the button that connects a device.
settings-bluetooth-connect = 接続
# Tip of the bin button that forgets a device.
settings-bluetooth-forget = このデバイスを削除
# Said in red when a device could not be disconnected.
settings-bluetooth-disconnect-failed = 切断できませんでした。
# Said in red when a device could not be forgotten.
settings-bluetooth-forget-failed = 削除できませんでした。
# Shown under a device while it is being paired.
settings-bluetooth-pairing = ペアリング中…
# Said in red when a device could not be paired.
settings-bluetooth-pair-failed = ペアリングできませんでした。
# Shown under a device while it is being connected.
settings-bluetooth-connecting = 接続中…
# Said in red when a device could not be connected.
settings-bluetooth-connect-failed = 接続できませんでした。
# Said under a device that paired but could not be set to connect by itself.
settings-bluetooth-not-trusted = 接続しましたが、Trindle は次回から自動で接続する設定にできませんでした。使うときは、ここから接続してください。
# Said under a device that shows a code to be typed on it; the code is already written.
settings-bluetooth-type-code = デバイスで { $code } を入力し、Enter を押してください。
# Question under a device that shows a number to compare with the one on this computer.
settings-bluetooth-compare = 相手のデバイスに { $code } と表示されていますか。
# Tip of the button that answers no to a pairing question.
settings-bluetooth-no = いいえ
# Tip of the button that says the numbers match.
settings-bluetooth-yes-matches = はい、同じです
# Question under a device that asks to be paired.
settings-bluetooth-pair-question = このデバイスとペアリングしますか。
# Placeholder of the box where a device's PIN is typed.
settings-bluetooth-pin = PIN
# Tip of the button that refuses to type a PIN.
settings-bluetooth-cancel = キャンセル
# Help line above the box where a device's PIN is typed.
settings-bluetooth-pin-help = このデバイスの PIN を入力してください。説明書に書いてあることが多く、なければ 0000 を試してください。
# Words the settings search looks for on this page (not shown).
settings-bluetooth-keywords = ヘッドホン マウス キーボード ペアリング デバイス ワイヤレス bluetooth
# Help text at the top of the page.
settings-bluetooth-help = 「デバイスを探す」を押し、相手のデバイスをペアリングモードにして、横の「ペアリング」を押してください。

# Said in red under the Bluetooth switch when restarting the sound service after Bluetooth came back on failed; $why is the system's own reason.
settings-bluetooth-sound-failed = Trindle は Bluetooth の音声を再起動できませんでした。再びログインするまで、ヘッドホンの音が普段より悪くなることがあります。{ $why }
# Tooltip of the help button beside the Bluetooth switch.
settings-bluetooth-sound-help = Bluetooth をオフにしてオンに戻すと、ヘッドホンの音が悪くなる理由
# Said in red when a device is no longer in BlueZ's list; the first sentence says what could not be done.
settings-bluetooth-cause-gone = { $could_not } このデバイスは、もうこのコンピューターに保存されていません。もう一度探してください。
# Said in red when a paired device's saved pairing is refused; the first sentence says what could not be done.
settings-bluetooth-cause-old-pairing = { $could_not } デバイスが、保存されたペアリングを受け付けなくなりました。デバイスを削除して、もう一度ペアリングしてください。
# Said in red under a device that connected and then dropped at once.
settings-bluetooth-dropped = 接続しましたが、すぐに切れました。保存されたペアリングが古い可能性があります。
# Tooltip of the button that forgets a paired device and looks for it afresh.
settings-bluetooth-pair-again = 削除して、もう一度ペアリング
# Said under Look for devices after a device was forgotten to be paired afresh.
settings-bluetooth-forgotten-pair = デバイスを削除しました。ペアリングモードにして、下に表示されたら「ペアリング」を押してください。
# Tooltip of the button that opens Help from a failed connection.
settings-bluetooth-more = これについて詳しく
