# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Audio": speakers, microphone, volume, mute and a test sound.

## Page (src/settings/panels/audio.rs)

# Name of the Settings page about sound, in the list and as its heading.
settings-audio-title = オーディオ
# Words a person might type into the Settings search to find the audio page; separate them with spaces.
settings-audio-keywords = sound volume speakers headphones microphone mic mute test
# Introduction under the heading of the audio page, and the words behind the page i.
settings-audio-help = 音を出すスピーカーまたはヘッドフォンと、使うマイクを選びます。

## Top sentence

# Red sentence at the top of the page when PipeWire, the sound system, is not installed.
settings-audio-top-missing = PipeWire がインストールされていないため、音が出ません。
# Red sentence at the top of the page when PipeWire is installed but not answering.
settings-audio-top-failed = 音のサービスである PipeWire が応答しないため、音が出ません。
# Red sentence at the top of the page when there are no speakers or headphones.
settings-audio-top-no-output = スピーカーもヘッドフォンもありません。
# Plain sentence at the top of the page while the page is still finding out about sound.
settings-audio-top-listening = 音を確認しています…
# Warning at the top of the page when sound is muted; { $device } is the speakers or headphones.
settings-audio-top-muted = { $device }はミュートされています。
# Warning at the top of the page when the volume is at nothing; { $device } is the speakers or headphones.
settings-audio-top-down = { $device }の音量が最小です。
# Sentence at the top of the page when all is well; { $device } is the speakers or headphones and { $percent } the volume.
settings-audio-top-playing = { $device }から音量 { $percent }% で音が出ています。

## Cards

# Title of the card for speakers and headphones.
settings-audio-out-title = スピーカーとヘッドフォン
# Title of the card for microphones.
settings-audio-in-title = マイク
# Heading of the line shown when no speakers or headphones are found.
settings-audio-out-none = スピーカーが見つかりません
# Heading of the line shown when no microphone is found.
settings-audio-in-none = マイクがありません
# Small words under that line for speakers.
settings-audio-out-plug = スピーカーかヘッドフォンをつないでください。
# Small words under that line for microphones.
settings-audio-in-plug = マイクかヘッドセットをつないでください。
# Heading of the line with the list of devices.
settings-audio-device = デバイス
# Name of the list of speakers and headphones.
settings-audio-out-device-name = 音を出す場所
# Name of the list of microphones.
settings-audio-in-device-name = マイク
# Heading of the volume line.
settings-audio-volume = 音量
# Name of the volume slider for the speakers.
settings-audio-out-volume-name = 音量
# Name of the volume slider for the microphone.
settings-audio-in-volume-name = マイクの音量
# Heading of the mute line.
settings-audio-mute = ミュート
# Name of the switch that mutes the sound.
settings-audio-out-mute-name = 音をミュートする
# Name of the switch that mutes the microphone.
settings-audio-in-mute-name = マイクをミュートする
# Heading of the line that plays a test sound.
settings-audio-test = テスト
# Name of the button that plays a test sound.
settings-audio-test-button = テスト音を鳴らす
# Words behind the i of the speakers card.
settings-audio-out-about = 音を出すスピーカーまたはヘッドフォンを選びます。ここでの音量とミュートは、キーボードのキーや、リングの音の円と同じように働きます。
# Heading of the line with the microphone level bar.
settings-audio-level = レベル
# Words behind the i of the microphone card.
settings-audio-in-about = 使うマイクを選びます。このページを開いている間、バーが拾った音を表します。Trindle は何も録音せず、保存もしません。
# Title of the card shown when PipeWire is not answering.
settings-audio-failed-title = 音
# Heading of the line in that card.
settings-audio-failed-heading = PipeWire が応答しません
# Small words under that line.
settings-audio-failed-about = ログイン時に音が起動しなかった可能性があります。

## Red words

# Red words under the volume line when the volume could not be changed.
settings-audio-no-volume = Trindle は音量を変えられませんでした。音が起動していない可能性があります。ログアウトして、もう一度ログインしてから、やり直してください。
# Red words under the mute line when the sound could not be muted.
settings-audio-no-mute = Trindle は音をミュートできませんでした。音が起動していない可能性があります。ログアウトして、もう一度ログインしてから、やり直してください。
# Red words under the mute line when the sound could not be turned back on.
settings-audio-no-unmute = Trindle は音を戻せませんでした。音が起動していない可能性があります。ログアウトして、もう一度ログインしてから、やり直してください。
# Red words under the device line when the device could not be chosen.
settings-audio-not-chosen = そのデバイスに切り替えられませんでした。
# Red words under the test line when the test sound would not play.
settings-audio-test-failed = このデバイスからテスト音が出ませんでした。

## Where sound goes

# A device's name with what kind it is; { $name } is the device and { $kind } one of the kind words below.
settings-audio-with-kind = { $name }({ $kind })
# Kind of output: a Bluetooth device.
settings-audio-kind-bluetooth = Bluetooth
# Kind of output: a screen or TV on a cable.
settings-audio-kind-hdmi = HDMI
# Kind of output: a device on a USB cable.
settings-audio-kind-usb = USB
# Kind of output: headphones or a headset on a cable.
settings-audio-kind-headphones = ヘッドフォン
# Kind of output: the computer's own speakers.
settings-audio-kind-speakers = スピーカー
# Heading of the line shown while Bluetooth plays the sound.
settings-audio-bt-now = 音は Bluetooth に出ています
# Tooltip of the button that moves the sound to the speakers; { $device } is the speakers, kind included.
settings-audio-use-speakers = { $device }で鳴らす
# Heading of the line when Bluetooth devices are connected but have no sound output; { $names } lists them.
settings-audio-bt-silent = { $names }は接続されていますが、音を出せません。
# Words under that heading.
settings-audio-bt-silent-about = 音の設定ができていない可能性があります。いったん切断して、もう一度接続してください。
# Red words when the sound devices could not be read.
settings-audio-not-read = Trindle は音のデバイスを見つけられませんでした。
