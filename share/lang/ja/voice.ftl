# machine translated (Japanese), not yet reviewed by a person.

# Trindle's voice control: the words the ring says back after a spoken
# request, the voice set-up's progress lines, and the reasons it did not start.
# The words the person says are matched in English and are not here.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## What the ring says back after a spoken request (src/voice/commands.rs)

# Said under the ring when a radio is switched on by speaking; the radio's name comes first.
voice-radio-on = { $name }をオンにしました
# Said under the ring when a radio is switched off by speaking; the radio's name comes first.
voice-radio-off = { $name }をオフにしました
# Said under the ring after "brighter" turns the screen up.
voice-brighter = 明るくしました
# Said under the ring after "dimmer" turns the screen down.
voice-dimmer = 暗くしました
# Said under the ring when the request is to let the screen rest again.
voice-keep-awake-off = 画面がスリープできるようになりました
# Said under the ring when the screen is kept awake.
voice-keep-awake-on = スリープしません
# Said under the ring when the request is to look for updates.
voice-updates = アップデートを確認しています
# Said under the ring when nothing was heard at all.
voice-nothing-heard = Trindle に声が聞こえませんでした。リングの真ん中をタップして話してください。
# Said under the ring when the volume goes up.
voice-louder = 音量を上げました
# Said under the ring when the volume goes down.
voice-quieter = 音量を下げました
# Said under the ring when the sound is switched back on.
voice-sound-on = 音をオンにしました
# Said under the ring when the sound is muted.
voice-muted = ミュートしました
# Said under the ring when the wallpaper changes to the next one.
voice-next-wallpaper = 次の壁紙
# Said under the ring after a screenshot is taken; Pictures is the folder's name.
voice-screenshot-saved = スクリーンショットをピクチャに保存しました
# Said under the ring when the request is to show every app.
voice-all-apps = すべてのアプリ
# Said under the ring before shutting down, which waits for a click to confirm.
voice-confirm-power-off = シャットダウンしますか? クリックで確定します
# Said under the ring before restarting, which waits for a click to confirm.
voice-confirm-reboot = 再起動しますか? クリックで確定します
# Said under the ring before logging out, which waits for a click to confirm.
voice-confirm-logout = ログアウトしますか? クリックで確定します
# Said under the ring when the request is to find an app in Apps; the words the person said for it come in.
voice-looking-for-app = アプリで{ $what }を探しています
# Said under the ring when the request opens Apps (the store).
voice-apps = アプリ
# Said under the ring when the request opens the disks page of Settings; Drives is the page's name.
voice-drives = ドライブ
# Said under the ring when the request opens Settings.
voice-settings = 設定
# Said under the ring when the request closes the window in front.
voice-closing-window = ウィンドウを閉じています
# Said under the ring when an open app is closed; the app's name comes in.
voice-closing-app = { $name }を閉じています
# Said under the ring when no app has the name the person said; the name they said comes in.
voice-no-app = { $name }というアプリはありません
# Said under the ring when a window is brought to the front; the app's name comes in.
voice-showing-app = { $name }を表示しています
# Said under the ring when an app is opened; the app's name comes in.
voice-opening-app = { $name }を開いています
# Said under the ring when the request was heard but not understood; what was heard comes in.
voice-heard-unknown = Trindle は「{ $heard }」というコマンドが分かりませんでした

## Why voice did not start (src/voice/mod.rs)

# Said in the ring when voice has not been added yet.
voice-optional = 音声は追加機能です。設定のリングのページで追加してください。
# Said in the ring when voice has been added but is switched off.
voice-switched-off = 音声はオフです。設定のリングのページでオンにしてください。
# Said in the ring when the microphone cannot be opened; the reason comes after it.
voice-mic-failed = Trindle はマイクを使えませんでした: { $why }
# The reason after "Trindle couldn't access the microphone:" when the session has no private folder to keep a recording in.
voice-no-private-folder = 録音を置く専用フォルダーがありません。
# Said in the ring when the microphone's recorder would not start.
voice-mic-not-started = 録音プログラムが起動しなかったため、Trindle はマイクを使えませんでした
# Said in the ring when listening stopped and nothing was recorded.
voice-nothing-recorded = 何も録音されませんでした。
# Said in the ring when the recording had no speech in it.
voice-nothing-heard-after = Trindle に声が聞こえませんでした。リングの真ん中をタップして話してください。
# Said in the ring when the speech engine failed and gave no reason.
voice-whisper-failed = Whisper が失敗しました

## Adding and removing voice (src/voice/setup.rs)

# Progress line while the build tools are added for the build; the names of the packages come in.
voice-installing-tools = ビルド用ツールを一時的にインストールしています: { $names }
# Progress line while whisper.cpp is built; its version comes in.
voice-building = 音声操作用に whisper.cpp { $version } をビルドしています。1、2分かかります。
# Progress line while the speech model is downloaded (148 MB).
voice-fetching-model = 音声モデルをダウンロードしています(148 MB)
# Progress line while the build tools are taken away again.
voice-removing-tools = ビルド用ツールを削除しています
# Said once voice is added and ready to use.
voice-ready = 音声操作の準備ができました。リングを開き、真ん中をタップして話してください。
# Said once voice is removed.
voice-removed = 音声操作を削除しました。
# Shown in the terminal window that stays open, asking for Enter to close it.
voice-press-enter = Enter キーを押すと閉じます。
# Shown when the command is run with neither install nor remove.
voice-usage = usage: trindle-voice-setup install | remove

# Said when a step of the voice set-up fails. { $program } is the program that was run, with its first word.
voice-step-failed = { $program }がうまくいきませんでした。
# Said when the whisper.cpp download isn't the one Trindle checks for, so nothing is built from it.
voice-whisper-fingerprint = ダウンロードした whisper.cpp が Trindle の想定したものと違うため、ビルドしませんでした。
# Said when the speech model download isn't the one Trindle checks for, so it isn't installed.
voice-model-fingerprint = ダウンロードした音声モデルが Trindle の想定したものと違うため、インストールしませんでした。
