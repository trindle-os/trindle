# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Ring": how the ring opens, notifications, the clipboard and voice control.

## Page (src/settings/panels/ring.rs)

# Name of the Settings page about the ring, in the list and as its heading.
settings-ring-title = リング
# Words a person might type into the Settings search to find the ring page; separate them with spaces.
settings-ring-keywords = お気に入り ランチャー 端 クリック ホットコーナー 音声 ウィンドウ クリップボード コピー 貼り付け 履歴 忘れる 通知 favourites favorites launcher edge click hot corner voice speech windows open windows clipboard copy paste history forget notifications
# Introduction under the heading of the ring page, and the words behind the page i.
settings-ring-help = お気に入りは、リングの中央のまわりに並びます。お気に入りはリングの中で選びます。リングの中のアプリを右クリックすると追加や削除ができ、お気に入りをドラッグすると順番を変えられます。
# Sentence at the top of the ring page saying how the ring opens.
settings-ring-headline = リングは画面の端で開き、開いているウィンドウがその中に表示されます。
## Opening the ring

# Line saying how many favourites the ring has; { $n } is the number.
settings-ring-favourites =
    { $n ->
       *[other] お気に入り { $n } 個
    }
# Title of the card about opening the ring.
settings-ring-opening = リングを開く
# Words behind the i of the card about opening the ring.
settings-ring-opening-about = ポインターを画面の端に動かしてダブルクリックするか、Superキーを押すと、リングが開きます。お気に入りはリングの中央のまわりに並び、開いているウィンドウはその外側のリングに並びます。ウィンドウをクリックすると手前に出て、右クリックするとボタンが表示されます。リングの中のアプリを右クリックすると、追加や削除ができます。
## Notifications

# Heading of the switch line, and the spoken name of the switch, for showing notifications as they arrive.
settings-ring-notify-show = 届いたらすぐ表示
# Heading of the list line, and the name of the list, for how long a notification stays.
settings-ring-notify-long = 表示する長さ
# Added to the Notifications card's i.
settings-ring-notify-long-about = 緊急の通知は、閉じるまで残ります。
# Heading of the list line, and the name of the list, for which corner notifications appear in.
settings-ring-notify-where = 表示する場所
# Added to the Notifications card's i.
settings-ring-notify-where-about = この角から順に重なっていきます。
# Title of the card about notifications.
settings-ring-notify-title = 通知
# Words behind the i of the card about notifications.
settings-ring-notify-about = プログラムからお知らせがあると、通知が数秒表示されてから消えます。これをオフにすると何も表示されませんが、通知はすべてリングの通知の円に集められます。緊急の通知は、ここでの選択に関係なく閉じるまで残るので、警告を見逃しません。
## Clipboard

# Heading of the list line, and the name of the list, for when copied lines are forgotten.
settings-ring-clip-forget = コピーしたものを忘れる時期
# Heading of the line saying how many lines the clipboard holds now.
settings-ring-clip-now = 今コピーしているもの
# Small words under that line; { $n } is how many lines are kept and { $pinned } how many of them are pinned.
settings-ring-clip-count =
    { $n ->
       *[other] { $n } 行、固定 { $pinned } 行。
    }
# Title of the card about the clipboard.
settings-ring-clip-title = クリップボード
# Words behind the i of the card about the clipboard.
settings-ring-clip-about = リングのクリップボードの円は、コピーしたものを残すので、前にコピーしたものを貼り付けられます。固定した行は、ここでの選択に関係なく、ずっと残ります。
## Voice

# Small words under the voice line while voice is being added.
settings-ring-voice-adding = 音声を追加しています。作業を続けられます。
# Small words under the voice line while its build tools are being removed.
settings-ring-voice-removing-tools = ビルドツールを削除しています。
# Small words under the voice line while voice is being removed.
settings-ring-voice-removing = 音声を削除しています。
# Heading of the voice line, and the spoken name of its switch.
settings-ring-voice-control = 音声操作
# Said, in green, under the voice switch after voice is turned on.
settings-ring-voice-onoff-on = オン
# Said, in green, under the voice switch after voice is turned off.
settings-ring-voice-onoff-off = オフ
# Heading of the line, and the name of its button, that removes voice.
settings-ring-voice-remove = 音声を削除
# Added to the Voice card's i once voice is added.
settings-ring-voice-remove-about = 約150 MBが空きます。
# Words on the line offering to remove the build tools that stayed after voice was added.
settings-ring-voice-tools-left = 準備ができました。ビルドツールは残してあります。
# Name of the button that removes the build tools voice left behind.
settings-ring-voice-tools-remove = ビルドツールを削除
# Name of the button that adds voice control to the computer.
settings-ring-voice-add = 音声操作を追加
# Added to the Voice card's i before voice is added.
settings-ring-voice-optional = 任意です。約150 MB必要です。
# Title of the card about voice.
settings-ring-voice-title = 音声
# Words behind the i of the voice card; the quoted words are examples of what to say.
settings-ring-voice-card-about = クリックの代わりに、アプリの名前、「音量を上げて」、「Wi-Fiの設定」などと話しかけます。リングの中央をタップしてから話してください。話し終えると、Trindleは聞くのをやめます。音声はこのコンピューターの中にとどまり、どこにも送られません。音声の追加には約150 MBと数分かかりますが、その間も作業を続けられます。削除すると、音声エンジンとそのモデルが消えます。
# Words beside the round progress dial while voice is being added.
settings-ring-voice-working-add = 音声を追加中
# Words beside the round progress dial while voice build tools are being removed.
settings-ring-voice-working-tools = ビルドツールを削除中
# Words beside the round progress dial while voice is being removed.
settings-ring-voice-working-remove = 音声を削除中

## Clipboard history choices (src/common/clipboard.rs)

# Choice in the list of when copied lines are forgotten: every time the computer restarts.
settings-ring-forget-reboot = コンピューターを再起動するたび
# Choice in the list of when copied lines are forgotten: after a day.
settings-ring-forget-day = 1日後
# Choice in the list of when copied lines are forgotten: after a week.
settings-ring-forget-week = 1週間後
# Choice in the list of when copied lines are forgotten: after two weeks.
settings-ring-forget-fortnight = 2週間後
# Choice in the list of when copied lines are forgotten: never.
settings-ring-forget-never = 忘れない

# Heading of the switch line, and the spoken name of the switch, for showing a window's title under its picture when pointing at it in the ring.
settings-ring-names = ポイントしたときにウィンドウのタイトルを表示
# Added to the Opening card's i.
settings-ring-names-about = オフでも、スクリーンリーダーはタイトルを読み上げます。
# Red words when voice could not be added.
settings-ring-voice-not-added = Trindleは音声を追加できませんでした。
# Red words when voice could not be removed.
settings-ring-voice-not-removed = Trindleは音声を削除できませんでした。
# Red words when the build tools left after adding voice could not be removed.
settings-ring-voice-tools-not-removed = Trindleはビルドツールを削除できませんでした。
