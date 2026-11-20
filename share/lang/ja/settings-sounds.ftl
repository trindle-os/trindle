# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Sounds" (event sounds), with the names of the sounds and moments from src/common/sounds.rs.

## Page (src/settings/panels/sounds.rs)

# Name of the Settings page about event sounds, in the list and as its heading.
settings-sounds-title = サウンド
# Words a person might type into the Settings search to find the sounds page; separate them with spaces.
settings-sounds-keywords = サウンド 音 チャイム 起動 ログイン ログアウト シャットダウン 充電器 接続 取り外し バッテリー 警告 ビープ オフ 消音 自分用 追加 ファイル wav ogg mp3
# Introduction under the heading of the sounds page, and the words behind the page i.
settings-sounds-help = ログインなど、場面ごとに音を選びます。自分の音を追加することもできます。
# First choice in each moment's list of sounds: that moment makes no sound.
settings-sounds-off = オフ

## Top sentence

# Sentence at the top of the page when no moment makes a sound.
settings-sounds-top-none = すべて消音です。
# Sentence at the top of the page when every moment makes a sound.
settings-sounds-top-all = すべての音がオンです。
# Sentence at the top of the page when some moments make a sound; { $n } is how many of the { $total } do.
settings-sounds-top-some =
    { $n ->
       *[other] { $total }個中{ $n }個の音がオンです。残りはオフです。
    }

## Moments and sounds

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-open = Trindleの起動
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-close = Trindleの終了
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-sunrise = 日の出
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-sunset = 日の入り
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-rising = 上昇
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-falling = 下降
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-alert = 警告
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-bell = ベル
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-glass = グラス
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-pop = ポン
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-login = ログイン
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-logout = ログアウト、再起動、シャットダウン
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-plug = 充電器の接続
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-unplug = 充電器の取り外し
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-battery = バッテリー残量が少ない
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-notification = 通知が表示されたとき
# Heading of the logging out line.
settings-sounds-moment-logout-short = ログアウト
# Heading of the notifications line.
settings-sounds-moment-notification-short = 通知
# Added to the Log in and out card's i.
settings-sounds-moment-logout-about = 再起動とシャットダウンも含みます。

## Lines and buttons

# Heading and slider name for the volume of the logging in and out sounds.
settings-sounds-volume-coming = ログイン・ログアウトの音量
# Heading and slider name for the volume of notification sounds.
settings-sounds-volume-notifications = 通知の音量
# Heading and slider name for the volume of the charger and battery sounds.
settings-sounds-volume-power = 電源の音の音量
# Name of the round button that plays the sound chosen for a moment, or an added sound.
settings-sounds-hear = 再生
# Name of the button that opens a window to add sound files.
settings-sounds-add = サウンドファイルを追加
# Name of the button that opens the folder where added sounds are kept.
settings-sounds-open-folder = 追加した音のフォルダーを開く
# Red words under the add sounds line when the folder could not be opened.
settings-sounds-folder-not-opened = フォルダーを開けませんでした。
# Red words under the add sounds line when the folder for added sounds could not be made.
settings-sounds-folder-not-made = Trindleは、あなたの音用のフォルダーを作れませんでした。
# Name of the button that plays the sound being added, when one of the same name is already there.
settings-sounds-hear-new = 新しい音を再生
# Name of the button that plays the sound already there, when one of the same name is being added.
settings-sounds-hear-old = 今ある音を再生
# Heading of the line for adding sounds.
settings-sounds-add-heading = サウンドを追加
# Name of the bin button that removes a sound added by the person.
settings-sounds-remove = この音を削除
# Heading of the line shown when no sounds have been added.
settings-sounds-none-yet = まだ追加していません

## Red and green words

# Red words under a moment's line when its chosen sound file is gone.
settings-sounds-gone = その音は、このコンピューターにありません。別の音を選んでください。
# Red words under a line when a sound would not play.
settings-sounds-would-not-play = 音が再生されませんでした。オーディオでスピーカーを確認してください。
# Title of the window for choosing sound files.
settings-sounds-chooser-title = サウンドファイルを選ぶ
# Name of the filter in the window for choosing sound files that shows only sounds.
settings-sounds-chooser-filter = サウンド
# Red words under the add sounds line when a chosen file is not a sound; { $name } is the file name.
settings-sounds-not-playable = ファイルの種類が分からないため、Trindleは{ $name }を再生できません。
# Red words under the add sounds line when a file could not be copied in; { $name } is the file name.
settings-sounds-not-copied = { $name }をコピーできませんでした。
# Green words under the add sounds line after sounds were added; { $n } is how many.
settings-sounds-added =
    { $n ->
       *[other] { $n }個の音を追加しました。上のどの場面にも選べます。
    }
# Red words under the question about a sound of the same name when it could not be replaced; { $name } is the file name.
settings-sounds-not-replaced = { $name }を置き換えられませんでした。
# Green words after one sound took the place of the one there; { $name } is the file name.
settings-sounds-replaced-one = { $name }を置き換えました。
# Green words after several sounds took the place of the ones there; { $n } of { $total } were done.
settings-sounds-replaced-some = { $total }個中{ $n }個を置き換えました。
# Red words under a sound's line when it could not be removed.
settings-sounds-not-removed = 削除できませんでした。
# Red words under a moment's line when its sound was removed and the moment could not be set back to its own sound.
settings-sounds-not-set-back = Trindleは音を削除しましたが、その場面を元の音に戻せませんでした。

## Cards

# Title of the card with the sounds for logging in and out and notifications.
settings-sounds-coming-title = ログイン・ログアウト
# Words behind the i of that card.
settings-sounds-coming-about = 音量は0から100まで、10ずつ変えられます。指を離すと、音が一度鳴ります。
# Title of the card with the charger and battery sounds.
settings-sounds-power-title = 電源
# Words behind the i of that card.
settings-sounds-power-about = 充電器の接続と取り外し、バッテリー残量が少ないときの音です。音量は0から100まで、10ずつ変えられます。指を離すと、音が一度鳴ります。
# Title of the card with the sounds the person added.
settings-sounds-own-title = 自分の音
# Words behind the i of that card.
settings-sounds-own-about = WAV、OGG、Opus、FLAC、MP3が使えます。Trindleがあなたのフォルダーにコピーするので、元のファイルは移動や削除をしても大丈夫です。ほかの音と同じように、どの場面にも選べます。追加した音を削除すると、それを使っていた場面は元の音に戻ります。
