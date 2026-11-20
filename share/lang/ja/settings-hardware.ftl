# machine translated (Japanese), not yet reviewed by a person.

# Settings, Hardware page: the computer's devices by kind and which ones have no working driver.

## The page (src/settings/panels/hardware.rs)

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = ハードウェア
# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = hardware devices drivers driver firmware pci usb graphics sound network bluetooth storage camera keyboard mouse missing problem chip ハードウェア デバイス ドライバー ファームウェア グラフィックス サウンド ネットワーク ストレージ カメラ キーボード マウス 不足 問題 チップ
# Text behind the info button of the Hardware page.
settings-hardware-help = Trindle が認識できるこのコンピューターの部品を、働きごとに分けて表示します。ドライバーがない部品や、ドライバーのファームウェアファイルが足りない部品は、先頭に表示されます。ドライバーはシステムに付属しているので、ここでダウンロードするものはありません。
# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
       *[other] { $n } 台のデバイスすべてで、ドライバーが動作しています。
    }
# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
       *[other] { $n } 台のデバイスに対応が必要です。
    }
# Card names, one for each kind of device.
settings-hardware-kind-graphics = グラフィックス
settings-hardware-kind-sound = サウンド
settings-hardware-kind-network = ネットワーク
settings-hardware-kind-bluetooth = Bluetooth
settings-hardware-kind-storage = ストレージ
settings-hardware-kind-input = キーボードとマウス
settings-hardware-kind-camera = カメラ
settings-hardware-kind-other = その他
# Under a device that works; name is the driver's name.
settings-hardware-driver = 動作中。ドライバー: { $name }
# Under a device that works when the driver's name is not known.
settings-hardware-working = 動作中
# Under a device that has no driver.
settings-hardware-no-driver = ドライバーなし
# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = ファームウェアが不足しています
# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = ドライバーは使われていません
# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = 不明なデバイス { $id }

## The security chip card, shown only after the TPM chip did not unlock the encrypted disk at start

# Card name.
settings-hardware-chip = セキュリティチップ
# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = 起動時にチップがディスクのロックを解除しなかったため、パスワードが求められました。BIOS の更新が原因のことがあります。
# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = もう一度チップを使う
# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = ディスクのパスワード
# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = キャンセル
# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = 先にディスクのパスワードを入力してください。
# Beside the line while it works.
settings-hardware-chip-waiting = チップを設定しています
# Once it worked.
settings-hardware-chip-done = 次の起動から、チップがディスクのロックを解除します。
