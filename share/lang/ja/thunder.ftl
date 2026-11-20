# machine translated (Japanese), not yet reviewed by a person.

## Thunderbolt (src/thunder): the question asked before a Thunderbolt or USB4 device is let in.

# Why it asks, in one plain sentence, under the device's name.
thunder-why = Thunderbolt と USB4 のデバイスは、このコンピューターのメモリに直接アクセスできます。よく知っていて信頼できるデバイスだけを許可してください。
# Title of the question. $name is the device's name.
thunder-let-connect = { $name } の接続を許可しますか?
# The device's name when it has none.
thunder-no-name = Thunderbolt デバイス
# Under the device's name when its maker did not say who it is.
thunder-no-maker = 製造元は不明です。
# Under the device's name: who made it. $maker is the maker's name.
thunder-made-by = 製造元: { $maker }
# Warning under the question when the device gives the name of one allowed before, but cannot be proved to be it.
thunder-doubt = 以前に許可したデバイスと同じ名前ですが、Trindle はそのデバイスかどうかを確認できませんでした。
# Button that refuses the device.
thunder-never = 許可しない
# Button that lets the device in this time only.
thunder-once = 今回だけ許可
# Button that lets the device in every time.
thunder-always = 常に許可
# Red words under the question when the device could not be connected.
thunder-not-connected = 接続できませんでした。
# Red words under the question when it could not be connected after it was unplugged and plugged in again.
thunder-unplug = Trindle は接続できませんでした。取り外してからもう一度差し込み、再度お試しください。
# Red words under the question when the part of Trindle that answers for the guard is not running.
thunder-guard-silent = Trindle の Thunderbolt 保護が応答していません。
