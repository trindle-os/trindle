# machine translated (Japanese), not yet reviewed by a person.

# Words of the shared parts that no one program owns: why an app cannot be uninstalled, what updating and renewing say, what the keyring says, the X11 rooms, and the "in use by" lines. Each program shows them where it needs them.

## Uninstalling (src/common/removal.rs)

# Said instead of an Uninstall button on an app that is part of Trindle itself.
common-removal-trindle = Trindle の一部なので、残ります。
# Said instead of an Uninstall button on an app that the computer cannot work without.
common-removal-needed = コンピューターに必要なので、残ります。
# Said instead of an Uninstall button on an app whose removal would take a part of Trindle with it.
common-removal-trindle-needs = Trindle の一部が必要としているので、残ります。
# Red words when an app's saved data was not deleted because its id is not a plain name.
common-removal-data-name = アプリの名前に特殊な文字が含まれているため、データは削除されませんでした。
# Red words when an app's saved data folder is a link or not a folder, so nothing was deleted.
common-removal-data-link = フォルダーが別の場所へのリンクなので、データは削除されませんでした。
# Red words when an app's saved data could not be deleted.
common-removal-data-failed = データを削除できませんでした。
# Said when only an app's saved data is to go and the app is running.
common-removal-data-running = アプリが実行中のため、データは削除されませんでした。先にアプリを閉じてください。
# Said when the app's data path is a file, not a folder.
common-removal-data-not-folder = フォルダーがあるはずの場所にファイルがあったため、データは削除されませんでした。

## One update at a time (src/common/updating.rs)

# Name of the updates button, and its tip, while the ring's own update button is putting updates in.
common-updating-short-ring = 更新中
# Name of an update button while Apps is putting updates in.
common-updating-short-apps = アプリで更新中
# Name of an update button while the daily security fixes for the apps' boxes are going in.
common-updating-short-boxes = セキュリティ修正を追加中
# Name of an update button while Debian's own installer is at work.
common-updating-short-system = Debian が更新中
# Name of an update button while something not named is installing updates.
common-updating-short-unknown = 別の場所で更新中
# Headline of an updates page while the ring is installing updates.
common-updating-sentence-ring = リングから更新をインストールしています。
# Headline of an updates page while Apps is installing updates.
common-updating-sentence-apps = アプリから更新をインストールしています。
# Headline of an updates page while the daily security fixes are going into the apps' boxes.
common-updating-sentence-boxes = アプリ用のセキュリティ修正をインストールしています。
# Headline of an updates page while Debian's installer is at work (the daily security updates, apt in a terminal, or another person).
common-updating-sentence-system = Debian のインストーラーが動いています。毎日のセキュリティ更新で動きます。
# Headline of an updates page while something not named is installing updates.
common-updating-sentence-unknown = 別の場所で更新をインストールしています。

## What an update does to a version (src/common/updating.rs)

# Beside an app in a list of updates, when what waits is a newer build of the version already installed. { $version } is the version.
common-updating-newer-build-of = バージョン { $version } の新しいビルド
# Beside an app in a list of updates: the installed version, then the version that waits.
common-updating-version-to = バージョン { $from } から { $to } へ
# Beside an app in a list of updates, when only the installed version is known. { $version } is that version.
common-updating-now-version = 現在のバージョンは { $version }
# Beside an app in a list of updates, when only the new version is known. { $version } is that version.
common-updating-to-version = バージョン { $version } へ
# Beside a thing whose own version is shown, when what waits is a newer build of the same version.
common-updating-newer-waiting = 新しいビルドがあります
# Beside a thing whose own version is shown: the version that waits.
common-updating-version-waiting = バージョン { $version } があります
# Beside a thing whose own version is shown, when the new version is not known.
common-updating-update-waiting = 更新があります

## Why the administrator's password is asked for (src/common/updating.rs)

# Said in the password box when Debian's and Trindle's updates are put in together.
common-updating-reason-all = Debian と Trindle の更新は、このコンピューターを使うすべての人に影響するため、管理者のパスワードが必要です。自分でインストールしたアプリは、パスワードなしで更新できます。
# Said in the password box when only Debian's updates are put in.
common-updating-reason-debian = Debian の更新は、このコンピューターを使うすべての人に影響するため、管理者のパスワードが必要です。
# Said in the password box when only Trindle's own update is put in.
common-updating-reason-trindle = Trindle の更新は、このコンピューターを使うすべての人に影響するため、管理者のパスワードが必要です。
# Said in the password box when new firmware is put in.
common-updating-reason-firmware = 新しいファームウェアはコンピューター本体を変更するため、管理者のパスワードが必要です。
# Said in the password box when the lists of the whole computer's updates are fetched away from the computer's own screen.
common-updating-reason-refresh = コンピューター全体の更新を確認するには、管理者のパスワードが必要です。コンピューター本体の画面から操作する場合は不要です。

## How an update ended (src/common/updating.rs)

# The end of a sentence saying why nothing was installed. { $why } is the reason, such as "no administrator's password was given".
common-updating-because = { $why }ためです。
# Red words beside an update that did not finish and gave no reason.
common-updating-no-reason = 完了しませんでした。理由はわかりません。
# Red words beside an update that did not finish. { $why } is the reason the system gave.
common-updating-failed = 完了しませんでした。{ $why }
# Red words beside an update that was never started. { $why } is the end of a sentence such as "because no administrator's password was given."
common-updating-not-installed = インストールされませんでした。{ $why }

## Why a step behind the administrator's password was refused (src/common/refused.rs)

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = 管理者のパスワードが入力されなかった
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = 管理者のパスワードが受け付けられなかった
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = 正しいパスワードでも、あなたのアカウントにはその操作が許可されていない
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = パスワードの入力欄が表示されなかった
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = 管理者のパスワードが受け付けられなかったか、あなたのアカウントにその操作が許可されていない
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = 管理者のパスワードが入力されませんでした
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = 管理者のパスワードが受け付けられませんでした
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = 正しいパスワードでも、あなたのアカウントにはその操作が許可されていません
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = パスワードの入力欄が表示されませんでした
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = 管理者のパスワードが受け付けられなかったか、あなたのアカウントにその操作が許可されていません
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = 操作は行われませんでした。管理者のパスワードが入力されなかったためです。
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = 操作は行われませんでした。管理者のパスワードが受け付けられなかったためです。
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = 操作は行われませんでした。正しいパスワードでも、あなたのアカウントにはその操作が許可されていません。一度ログアウトして、ログインし直してから、もう一度お試しください。
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = 操作は行われませんでした。パスワードの入力欄が表示されなかったためです。一度ログアウトして、ログインし直してから、もう一度お試しください。
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = 操作は行われませんでした。管理者のパスワードが受け付けられなかったか、あなたのアカウントにその操作が許可されていません。パスワードの入力欄が表示されなかった場合は、一度ログアウトして、ログインし直してから、もう一度お試しください。
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = 操作は行われませんでした。失敗しましたが、理由はわかりません。

## The keyring (src/common/keyring_said.rs)

# Red words when the keyring will not open without the person's password.
common-keyring-locked = キーリングがロックされています。ログアウトしてログインし直すか、ロックを解除してから、もう一度お試しください。
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = 保存先のキーリングがこのコンピューターにありません。
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = キーリングから時間内に応答がありませんでした。
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = キーリングから応答がありませんでした。
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Trindle がキーリングにアクセスできなかったため ({ $why })、キーリングは使われませんでした。
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] キーリングからまだ応答がないため、{ $what }は削除されていません。キーリングが後から削除した場合は、このメッセージでお知らせします。
       *[other] キーリングからまだ応答がないため、{ $what }は保存されていません。キーリングが後から保存した場合は、このメッセージでお知らせします。
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] キーリングの応答が遅れましたが、{ $what }は削除されました。
       *[other] キーリングの応答が遅れましたが、{ $what }は保存されました。
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] キーリングから応答がなかったため、{ $what }は削除されていません。
       *[other] キーリングから応答がなかったため、{ $what }は保存されていません。
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] キーリングの応答が遅れ、{ $what }は削除されませんでした: { $why }
       *[other] キーリングの応答が遅れ、{ $what }は保存されませんでした: { $why }
    }

## After an update, which parts are still the old program (src/common/renewed.rs)

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the ring.
common-renewed-ring = リング
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the window that opens and saves files for other programs.
common-renewed-portal = 開く・保存するウィンドウ
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the wallpaper.
common-renewed-wallpaper = 壁紙
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the clipboard.
common-renewed-clipboard = クリップボード
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the keyring.
common-renewed-keyring = キーリング
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that dims the screen and locks it.
common-renewed-idle = 画面を暗くしてロックする機能
# Name of a part of Trindle, said in the sentence about what is not yet the new version: night light.
common-renewed-nightlight = 夜間モード
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that lets other computers find this one.
common-renewed-wsdd = ネットワークファインダー
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the plugins that run in the background.
common-renewed-plugin = バックグラウンドで動くプラグイン
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the desktop itself.
common-renewed-compose = デスクトップ本体
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the link an AI assistant is using.
common-renewed-mcp = AI アシスタントが使っている接続
# Joins two or more names of parts in a list, all but the last: "the ring, the keyring". { $first } is the list so far; { $next } is the next name.
common-renewed-join = { $first }、{ $next }
# Joins the last name of a list of parts to the rest: "the ring, the keyring and night light". { $most } is the list so far.
common-renewed-join-last = { $most }、{ $last }
# Sentence on Settings and Apps after an update, when some parts stay the old program until the next log out. { $list } is the names of the parts.
common-renewed-sentence = この更新はすでに使われています。ただし、{ $list }は、次にログアウトした後に新しいバージョンに切り替わります。
# Sentence on Settings and Apps after an update, when some parts may still be the old program and will change at the next safe moment or the next log out. { $list } is the names of the parts.
common-renewed-sentence-latest = この更新はすでに使われています。ただし、{ $list }は、遅くとも次にログアウトしたときに新しいバージョンに切り替わります。

## Older programs in boxes (src/common/x11_rooms.rs)

# Title of the notice, and red words in Apps, when the older-programs display for a box would not start.
common-x11-would-not-start = Xwayland が起動しませんでした
# Second line of that notice, when the desktop will try the older-programs display again soon.
common-x11-try-again = 1 分後にもう一度お試しください

## What is using the microphone, camera, screen, input or copied files (src/common/in_use.rs)

# Head of the card in Settings, Permissions, and of the page in Welcome, that shows each mark with what it means.
common-in-use-key = マークの意味
# Under the marks in that key: where a mark shows up. On the screen it is a half circle hung from the top edge; on a window tab it is a small dot.
common-in-use-where = 半円は画面の上の中央に下がって表示されます。ウィンドウのタブには小さな点が表示されます。

# Screen reader name of the microphone dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-microphone = マイクを使用中
# Line on pointing at the dots, and its screen reader name, when programs use the microphone. { $who } is the programs' names.
common-in-use-microphone-by = { $who } がマイクを使用中
# Screen reader name of the camera dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-camera = カメラを使用中
# Line on pointing at the dots, and its screen reader name, when programs use the camera. { $who } is the programs' names.
common-in-use-camera-by = { $who } がカメラを使用中
# Screen reader name of the screen dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-screen = 画面を録画中
# Line on pointing at the dots, and its screen reader name, when programs record the screen. { $who } is the programs' names.
common-in-use-screen-by = { $who } が画面を録画中
# Screen reader name of the input dot and its row, and the line shown on pointing at the dots, when no program is named: the mouse and keyboard are in another program's hands.
common-in-use-input = マウスとキーボードを使用中
# Line on pointing at the dots, and its screen reader name, when programs have taken the mouse and keyboard. { $who } is the programs' names.
common-in-use-input-by = { $who } がキーボードとマウスを使っています
# Screen reader name of the copied files dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-files = コピーしたファイルを読み取り中
# Line on pointing at the dots, and its screen reader name, when programs read the files the person just copied. { $who } is the programs' names.
common-in-use-files-by = { $who } がコピーしたファイルを読み取っています

# The ring's who-is-using-what row while a plugin reaches devices on the local network.
common-in-use-home-network = ローカルネットワークを使用中

# The same with the plugin's name.
common-in-use-home-network-by = { $who } がローカルネットワークを使用中

# The ring's who-is-using-what row while a plugin uses scanners and USB devices.
common-in-use-usb = スキャナーと USB デバイスを使用中

# The same with the plugin's name.
common-in-use-usb-by = { $who } がスキャナーと USB デバイスを使用中
# Line on pointing at the dots when the microphone is switched off and the program listening hears nothing.
common-in-use-microphone-off = マイクはオフです
# Line on pointing at the dots when the microphone is switched off but programs want it. { $who } is the programs' names.
common-in-use-microphone-off-by = マイクはオフです。{ $who } が使おうとしました。
# Line under a microphone's mute switch in Settings and the ring's sound page while the microphone is off for every program.
common-in-use-microphone-off-note = リングのマイクボタンで、すべてのプログラムに対してオフにしました。

## A settings file that is damaged or cannot be read (src/common/damaged.rs). Help section: settings-unreadable

# Said where the person is working when a file of their own was damaged, was put aside beside itself and a new one begun. { $what } names what the file keeps, such as "Launch counts".
common-damaged-reset = ファイルが壊れていたため、{ $what }をリセットしました。壊れたファイルはそのそばに残してあります。
# Said where the person is working when a file of their own is there but cannot be read, so nothing is saved into it and it is left as it is. { $what } names what the file keeps, such as "Launch counts".
common-unreadable-kept = Trindle がファイルを読めなかったため、{ $what }は保存されませんでした。何も変更していません。
# Said where the person is working when one of Steam's own settings files is there but cannot be read, so it is left as it is.
common-steam-unreadable = Steam の設定を読めなかったため、何も変更していません。
# Said when a permission is asked to be changed for something that is not an app's name.
common-flatpak-not-an-app = アプリの名前ではありません。
# Said when a permission is asked for that Trindle doesn't know.
common-flatpak-not-a-permission = Trindle が変更できる権限ではありません。
# Said where fonts are being added when the file chosen holds far more than a set of fonts is, so none of it was unpacked.
common-fonts-too-big = フォント一式としては大きすぎるファイルなので、何も展開しませんでした。
# Said where fonts are being added when the file chosen could not be unpacked.
common-fonts-unpack-failed = ファイルを展開できませんでした。壊れているか、種類が違う可能性があります。
# Said where fonts are being added when the program that unpacks files could not run. { $error } is the system's reason.
common-fonts-unpack-error = ファイルを展開できませんでした: { $error }
# Said where fonts are being added when the file chosen holds no fonts. Help has a section on adding fonts.
common-fonts-none = このファイルにフォントはありません。ヘルプの「フォントの追加」をご覧ください。
## Displays (src/common/outputs.rs)

# Said where displays are set when the desktop refused the change and gave no reason.
common-displays-refused = デスクトップがその設定にできませんでした。
# Said where displays are set when the desktop refused the change. { $why } is the desktop's own reason.
common-displays-refused-why = デスクトップがその設定にできませんでした: { $why }。
# Said where displays are set when the displays changed again and again while the change was being made.
common-displays-kept-changing = ディスプレイが何度も変わったため、何も変更しませんでした。
# Said where displays are set when Settings is not running in a desktop.
common-displays-no-desktop = Trindle がデスクトップを見つけられませんでした。
# Said where displays are set when the desktop has no way to list its displays.
common-displays-cannot-say = このデスクトップでは、ディスプレイの一覧を取得できません。
# Said where displays are set when the desktop gave no list of its displays.
common-displays-did-not-say = デスクトップからディスプレイの一覧が返ってきませんでした。
# Said where displays are set when the change would switch every display off.
common-displays-none-on = それではすべてのディスプレイがオフになってしまいます。
# Said where displays are set when the desktop gave no answer within a few seconds, or went away.
common-displays-stopped-answering = デスクトップが応答しなくなりました。
# Said where the lock screen PIN is set when what was typed is not the right length. { $n } is how many digits a PIN has.
common-pin-digits = PIN は { $n } 桁です。
# Said where the lock screen PIN is set when it could not be turned into the secret form it is kept in.
common-pin-not-hashed = PIN を保存できませんでした。
# Said where a theme is chosen when no theme has the name asked for.
common-themes-no-such = その名前のテーマはありません。
# Said where a theme is saved when it could not be kept. { $error } is the system's reason.
common-themes-not-kept = テーマを保存できませんでした: { $error }。
# Said where a theme file is added, when its name doesn't end in .css. { $name } is the file's name.
common-theme-add-not-css = { $name } はテーマではありません。テーマは、名前が .css で終わる 1 つのファイルです。
# Said where a theme file is added, when it can't be read. { $name } is the file's name.
common-theme-add-unreadable = Trindle は { $name } を読めませんでした。
# Said where a theme file is added, when it is over 64 KB. { $name } is the file's name.
common-theme-add-too-big = { $name } はテーマとしては大きすぎます。テーマは最大 64 KB です。
# Said where a theme file is added, when it holds more than plain text. { $name } is the file's name.
common-theme-add-not-a-sheet = { $name } はスタイルシートではありません。テキスト以外のものが含まれています。
# Said where a theme file is added, when reading it took too long. { $name } is the file's name.
common-theme-add-too-slow = { $name } の読み取りに時間がかかりすぎました。

# Under Trindle's own look on Settings, Appearance: a few words about it.
common-theme-trindle-about = Trindle 独自のデザイン。深い青の夜空に、やわらかな角。
# Under the Bold theme on Settings, Appearance.
common-theme-bold-about = 白と黒のくっきりした縁取りで、文字もボタンも大きめです。弱視の方向けです。
# Under the Midnight theme on Settings, Appearance.
common-theme-midnight-about = 暗くて平らな、グレーの段階だけのデザイン。黒とグレーで、縁取りはなく、角は小さめです。
# Under the Paper theme on Settings, Appearance.
common-theme-paper-about = クリーム色の机に置いた、あたたかい白の紙。明るい部屋向けの明るいデザインです。
# Under the Snug theme on Settings, Appearance.
common-theme-snug-about = 小さめで間隔はせまく、角ばった冷たいグレー。小さな画面にたくさん表示できます。
## Drives (src/common/volumes.rs)

# Said where a drive is unlocked when the passphrase did not open it.
common-volumes-wrong-passphrase = そのパスフレーズでは開けませんでした。
# Said where a drive is opened when it is open already.
common-volumes-already-open = すでに開いています。
# Said where a drive is opened when it is locked and no passphrase has been given.
common-volumes-needs-passphrase = 開くにはパスフレーズが必要です。
# Said where a drive is opened when the small window that asks for its passphrase could not be shown.
common-volumes-window-failed = パスフレーズのウィンドウを開けませんでした。
# What the file kept by the list of programs a box may use is called, in the words above.
common-unreadable-name-box-permissions = アプリの権限
# What the file that notes which questions an app has already put to the person is called, in the words above.
common-unreadable-name-box-asked = 質問済みの内容
# What the file that notes which apps came with the image is called, in the words above.
common-unreadable-name-box-offered = 初回ログイン時に提案したアプリ
# What the lists of a fenced program (its folders, hosts, settings and programs) are called, in the words above.
common-unreadable-name-fence = フェンスのリスト
# What the list of icons the desktop found missing is called, in the words above.
common-unreadable-name-missing-icons = 見つからないアイコンのリスト
# What the screen arrangement an older version kept in display.sh is called, in the words above.
common-unreadable-name-old-display = 画面の配置

## Hardware with no driver (src/common/no_driver.rs)

# Said where the person works when a known chip has no driver yet. The kernel is a version number.
common-no-driver-known-driver = { $chip } のドライバーは、まだここにありません。Linux { $kernel } が、ドライバーを含む最初のバージョンです。
# Said when a known chip is missing a firmware file. The package and version name where it comes from.
common-no-driver-known-firmware = { $chip } のファームウェアがありません。{ $package } { $version } に含まれています。
# Said of a chip nothing is known about that has no driver. The class is Wi-Fi, Bluetooth, sound or graphics; the id is two hex numbers.
common-no-driver-generic = この { $class } チップ ({ $id }) のドライバーがありません。
# Said of a chip nothing is known about whose firmware file could not be loaded.
common-no-driver-generic-firmware = { $class } のファームウェアがありません: { $file }。
# Added after a Wi-Fi line.
common-no-driver-stick = 今のところ、USB の Wi-Fi アダプターなら使えます。
# Tip of the small round i button that opens Help.
common-no-driver-more = 詳しく見る
common-no-driver-class-wifi = Wi-Fi
common-no-driver-class-bluetooth = Bluetooth
common-no-driver-class-sound = サウンド
common-no-driver-class-graphics = グラフィックス
# Said of a device with no driver that nothing is known about. The name is its name, or its two hex numbers.
common-no-driver-device-none = { $name } のドライバーがありません。

# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = 詳細
common-more-spoken = 詳しく見る。ヘルプが開きます

## Pocketkey's "ask for the password again after" (src/common/pocketkey.rs)

# First choice of "Ask for the password again after": the phone stays good for as long as it likes.
common-pocketkey-never = しない
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } minutes.
common-pocketkey-minutes =
    { $n ->
       *[other] { $n }分
    }
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } hours.
common-pocketkey-hours =
    { $n ->
       *[other] { $n }時間
    }
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } days.
common-pocketkey-days =
    { $n ->
       *[other] { $n }日
    }
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } weeks.
common-pocketkey-weeks =
    { $n ->
       *[other] { $n }週間
    }
# On the lock and login screens, when the time chosen has passed and the phone is not accepted: { $time } is such as "1 hour".
common-pocketkey-due = { $time }以上たったため、もう一度パスワードを入力してください。
# Said under the greyed Earlier versions card.
common-snap-no-snapper = 以前のバージョンには snapper が必要ですが、このコンピューターにはインストールされていません。
# Said under the greyed Earlier versions card.
common-snap-from-snapshot = スナップショットからコンピューターを起動している間は使えません。
# Said under the greyed Earlier versions card.
common-snap-part-of-system = このコンピューターでは、ホームフォルダーは独立したディスク領域ではなくシステムの一部です。そのため、システムとは別に以前のバージョンを保存できません。
# Said under the greyed Earlier versions card.
common-snap-network = ホームフォルダーが別のコンピューターにあるため、ここには以前のバージョンを保存できません。
# Said under the greyed Earlier versions card: { $fs } is the kind of file system, such as ext4.
common-snap-not-btrfs = 以前のバージョンには Btrfs 上のホームフォルダーが必要ですが、ここでは { $fs } です。
# Said under the greyed Earlier versions card.
common-snap-not-whole = ホームフォルダーが独立したディスク領域ではなく別の領域の中にあるため、Trindle は以前のバージョンを保存できません。
# Said under the greyed Earlier versions card.
common-snap-read-only = 今はホームフォルダーに書き込めないため、何も保存できません。
# Said under the greyed Earlier versions card.
common-snap-locked-files = ここのホームフォルダーはロックされた1つのファイルとして保存されており、毎日コピーするとディスクがいっぱいになります。
# Said under the greyed Earlier versions card.
common-snap-elsewhere = ホームフォルダーが、コピーを取る /home の中にありません。
# Said under the greyed Earlier versions card.
common-snap-apart = ホームフォルダーがロックされているか他と分けて保存されているため、コピーに含められません。
# Said under the greyed Earlier versions card.
common-snap-own-part = ホームフォルダーは独立したディスク領域にあり、/home のコピーには含まれません。
# Why a plugin's port line in plugin.ini is refused. { $text } is what was written.
common-firewall-not-protocol-port = "{ $text }" は「プロトコル:ポート」の形式(例: tcp:24810)ではありません。
# Why a plugin's port line in plugin.ini is refused. { $text } is what was written before the colon.
common-firewall-bad-protocol = "{ $text }" は tcp でも udp でもありません。小文字で書いてください。
# Why a plugin's port line in plugin.ini is refused. { $text } is what was written, { $lowest } the lowest port allowed.
common-firewall-bad-port = "{ $text }" は { $lowest } から 65535 までのポートではありません。
# Said when a port you open by hand isn't written as protocol:port.
common-firewall-own-form = 「プロトコル:ポート」の形式(例: tcp:37666)で書いてください。
# Said when a port you open by hand starts with something other than tcp or udp.
common-firewall-own-protocol = tcp か udp を使ってください。
# Said when a port you open by hand isn't a number from 1 to 65535.
common-firewall-own-port = 1 から 65535 までのポートではありません。
# Joins the last port to the others in a list of ports. { $first } is the others, comma separated.
common-firewall-and = { $first }、{ $last }
# Heads a list of ports. { $count } is how many, { $list } is the ports, such as TCP 24810 and UDP 24811.
common-firewall-ports =
    { $count ->
       *[other] ポート { $list }
    }
# Why a plugin's ports are refused. { $max } is the most it may name.
common-firewall-too-many = プラグインが指定できるポートは最大 { $max } 個です。
# Why a plugin's ports are refused. { $port } is a port such as TCP 24810.
common-firewall-twice = { $port } が2回指定されています。
# Why a plugin's ports are refused. { $port } is a port such as TCP 80, { $lowest } the lowest allowed.
common-firewall-too-low = { $port } は { $lowest } より小さい番号です。これらのポートはシステム用です。
# Why a plugin's port is refused. { $port } is the number.
common-firewall-reserved = ポート { $port } は Trindle または一般的なサービスが使っているため、プラグインは指定できません。
# Why a plugin's port can't be opened. { $port } is such as TCP 24810, { $plugin } is the other plugin's name.
common-firewall-taken = { $port } は { $plugin } がすでに使っているため、このプラグイン用には開けません。
# Why a plugin cannot open another port: { $max } is how many may be open.
common-firewall-full = プラグイン用に開けるポートの上限 { $max } 個に達しています。
# Told by a notice when the ring has been started again.
common-ring-held-back = リングが続けて何度も止まったため、Trindle はしばらく停止しました。今は再び動いています。
# Told by a notice when the ring has been started again.
common-ring-not-started = リングをしばらく起動できませんでした。今は再び動いています。
# Told by a notice when the ring has been started again.
common-ring-not-ended = リングが応答しなくなり、しばらく終了できませんでした。今は再び動いています。
# Told by a notice.
common-ring-not-done = コンピューターにスリープ、再起動、シャットダウンを求めましたが、実行されませんでした。
# Told by a notice; the ring is what the round menu is called.
common-ring-by-name = デスクトップがこのリングを自分で起動できなかったため、全画面のウィンドウの上には開けず、通知も最前面に保てません。ログアウトして、もう一度ログインすると直る場合があります。
# A group of apps in the ring.
common-category-internet = インターネット
# A group of apps in the ring (as above).
common-category-office = オフィス
# A group of apps in the ring (as above).
common-category-graphics = グラフィックス
# A group of apps in the ring (as above).
common-category-media = サウンドとビデオ
# A group of apps in the ring (as above).
common-category-games = ゲーム
# A group of apps in the ring (as above).
common-category-development = 開発
# A group of apps in the ring (as above).
common-category-learning = 学習
# A group of apps in the ring (as above).
common-category-utilities = ユーティリティ
# A group of apps in the ring (as above).
common-category-system = システム
# Why an app cannot be started: its launcher names nothing to run.
common-app-no-command = 実行するコマンドがありません。
# Why a launcher was left alone: { $why } is the reason.
common-launcher-unchanged = { $why }ため、何も変更しませんでした。
# The ring's notice when something is plugged in.
common-drive-arrived-usb = USB ドライブが接続されました
# The ring's notice when something is plugged in (as above).
common-drive-arrived-card = メモリーカードが接続されました
# The ring's notice when something is plugged in (as above).
common-drive-arrived-disc = ディスクが挿入されました
# The ring's notice when something is plugged in (as above).
common-drive-arrived-other = ドライブが接続されました
# Said in the password box after what the password is for.
common-drive-every-time = ドライブを変更するたびにパスワードを求めます。このボックスを閉じると、何も変わりません。
# Said in the password box: { $rest } is what is being done, such as "format this drive".
common-drive-password-needed-to = { $rest }には管理者のパスワードが必要です
# Said in the password box when the action is not named.
common-drive-password-needed = このドライブを変更するには管理者のパスワードが必要です
# Why a folder cannot be shared.
common-share-not-yours = 共有できるのは自分のフォルダーだけです。
# Why a share cannot have the name asked for.
common-share-name-is-login = その名前はこのコンピューターのユーザーのログイン名なので、共有名にできません。別の名前を選んでください。
# Why a share cannot have the name asked for.
common-share-bad-name = 共有名には % < > * ? | / \ + = ; : " とカンマは使えません。
# Why a folder cannot be shared.
common-share-no-guests = このコンピューターの Samba 設定では、パスワードなしの共有は許可されていません。
# Why a folder cannot be shared.
common-share-most = このコンピューターは、許可されている数のフォルダーをすでに共有しています。
# Why a folder cannot be shared.
common-share-gone = そのフォルダーはもうありません。
# Why a folder cannot be shared, when the program said nothing.
common-share-no-reason = Samba は理由を返しませんでした。
# Why a folder cannot be shared: { $said } is what the sharing program said.
common-share-said = Samba からのメッセージ: { $said }
# Why a screenshot could not be taken.
common-shot-no-desktop = 撮影できるデスクトップがありません。
# Why a screenshot could not be taken.
common-shot-no-display = 撮影できるディスプレイがありません。
# Why a screenshot could not be taken.
common-shot-no-display-there = デスクトップのその場所には画面がありません。
# Why a screenshot could not be taken.
common-shot-cannot = このデスクトップは撮影できません。
# Why a screenshot could not be taken.
common-shot-too-big = スクリーンショットが大きすぎます。
# Why a screenshot could not be taken.
common-shot-no-size = 画面が空の画像を返したため、Trindle はスクリーンショットを撮れませんでした。
# Why a screenshot could not be taken.
common-shot-unreadable = デスクトップから、読み取れる画像が提供されませんでした。
# Why a screenshot could not be taken.
common-shot-no-memory = 画像用のメモリーがありませんでした。
# Why a screenshot could not be taken.
common-shot-stopped = デスクトップが画面の撮影を止めました。
# Why a screenshot could not be taken.
common-shot-failed = デスクトップが画面を撮影できませんでした。
# Said where the person is working when no screen reader is installed.
common-reader-none-yet = スクリーンリーダーがインストールされていません。
# Said wherever hiding a drive is offered or shown.
common-drive-not-a-lock = 隠してもロックや保護にはなりません。まったく同じ情報を持つ双子のドライブも一緒に隠れます。
# Said when typed words are not a change to a key.
common-remap-not-a-key = Trindle が知らないキーです。Escape、Ctrl+C、Left Click などを試してください。
# Said when typed words are not a change to a key.
common-remap-too-long = 1つの変更に入れられるキーの数を超えています。
# Said when typed words are not a change to a key or a button.
common-no-equals = 変更には、押すもの、等号、変わった後のものが必要です。
# Said when typed words are not a change to a key.
common-remap-tapped-held = タップしたときと長押ししたときを書いてください。例: Escape tapped, Ctrl held
# Said when typed words are not a change to a controller button.
common-pads-not-a-button = Trindle が知らないボタンです。South、Left Trigger、Escape などを試してください。
# Said when typed words are not a change to a controller.
common-pads-not-a-zone = デッドゾーンは 0% から 50% です。
# Said of a line in the controllers file that is not understood.
common-pads-not-a-heading = Trindle はこのコントローラーの見出しを認識できません。
# Said of a line in the controllers file that is not understood.
common-pads-no-heading = どのコントローラーの見出しの下にもありません。
# What a theme's tile says when its file holds more than a theme may.
common-theme-too-big = このファイルはテーマとしては大きすぎるため、使われません。テーマは最大 64 KB です。
# What a theme's tile says when it is not an ordinary file.
common-theme-not-a-sheet = これは通常のテキストファイルではないため、テーマにできず、使われません。
# What is said of a theme that was chosen and whose file has since gone.
common-theme-gone = そのファイルはもうありません。
# What is said of a theme on a disk that does not answer.
common-theme-too-slow = ファイルの読み込みに時間がかかりすぎたため、Trindle はこれを使っていません。
# What is said when the folders of themes are on a disk that does not answer.
common-theme-list-slow = テーマのフォルダーの読み込みに時間がかかりすぎたため、Trindle 標準の見た目だけを表示しています。
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-big = あなたのスタイルシート style.css は 256 KB を超えているため、使われません。
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-not-a-sheet = あなたのスタイルシート style.css は通常のテキストファイルではないため、使われません。
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-slow = あなたのスタイルシート style.css の読み込みに時間がかかりすぎたため、使われません。
# What is said when colours in the person's own style sheet fall short of what can be read.
common-theme-own-hard-to-read = 読みにくい色があります。
# What is said under the tiles when a folder holds more themes than are shown.
common-theme-not-all = フォルダーにはここに表示されている以外のテーマがあるかもしれません。表示するのは1つのフォルダーにつき最初の40個のテーマまでで、調べるのは最初の400個のファイルまでです。
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-no-target = あなたのスタイルシート style.css は、存在しないファイルへのリンクのため、使われません。
# The name of a file set aside when damaged, said in a sentence: Clipboard history.
common-damaged-clipboard = クリップボードの履歴
# The name of a file set aside when damaged, said in a sentence: Last-used apps.
common-damaged-last-used = 最近使ったアプリ
# What kind of virtual machine this is, said in a sentence.
common-vm-qemu = QEMU 仮想マシン(GNOME Boxes、virt-manager など)
# What kind of virtual machine this is, said in a sentence.
common-vm-apple = Mac の仮想マシン
# What kind of virtual machine this is, said in a sentence: { $name } is the name systemd gives it.
common-vm-other = 仮想マシン({ $name })
# What a program offered in Software sources is for, in a line.
common-source-claude = Anthropic の Claude アプリ: チャット、Cowork、Claude Code
# What a program offered in Software sources is for, in a line.
common-source-tailscale = 自分のデバイス同士をつなぐプライベートネットワーク
# What a program offered in Software sources is for, in a line.
common-source-syncthing = クラウドなしで、コンピューター間のフォルダーを同じ状態に保ちます。Debian 標準のバージョンは古めです。
# What a program offered in Software sources is for, in a line.
common-source-jellyfin = 家のどの画面でも見られる、自分用の映画、音楽、写真のサーバー
# What a program offered in Software sources is for, in a line.
common-source-nodejs = 開発者向けの Node.js の最新の長期サポート版。Debian 標準はバージョン 20 です。
# What a program offered in Software sources is for, in a line.
common-source-github-cli = ターミナルから使う GitHub。開発者向け。Debian 標準のバージョンは古めです。
# What a program offered in Software sources is for, in a line.
common-source-docker = コンテナでプログラムを実行します。開発者向け
# Said in the box that asks for a drive's passphrase: { $name } is the drive.
common-volumes-passphrase-for = { $name } のパスフレーズ
# The title of the box that asks for a drive's passphrase.
common-volumes-unlock-title = ドライブのロックを解除

# What a snapshot is, in the Earlier versions list: made just before software was changed.
common-snap-before-change = ソフトウェア変更の前
# What a snapshot is, in the Earlier versions list: made just after software was changed.
common-snap-after-change = ソフトウェア変更の後
# What a snapshot is, in the Earlier versions list: made when the computer started.
common-snap-boot = コンピューターの起動時
# What a snapshot is, in the Earlier versions list: one taken on a schedule.
common-snap-regular = 定期コピー
# What a snapshot is, in the Earlier versions list, when it has no note.
common-snap-plain = スナップショット
# Said when a font file can't be added. { $name } is the file's name, { $error } the system's reason.
common-pack-not-copied = Trindle は { $name } をコピーできませんでした: { $error }
# Said when a file given as a font isn't one. { $name } is the file's name.
common-pack-not-fonts = { $name } はフォントでも、フォントのアーカイブでもありません。
# Said when a drive can't be hidden because it has no name of its own to remember it by.
common-hidden-no-name = このドライブには覚えるための固有の名前がないため、Trindle は覚えられません。
# Said when the list of hidden drives can't be read. { $error } is the system's reason.
common-hidden-left = 隠したドライブの一覧は変更されませんでした: { $error }
# Said when a drive can't be released because your login session can't be found.
common-leave-no-session = ログインセッションが見つからなかったため、Trindle は何も変更しませんでした。
# Said under the sound choices when PipeWire gives no answer and says nothing about why.
common-audio-no-answer = PipeWire から応答がありませんでした。
# Said when the screen reader can't be turned on or off. { $error } is the first problem, { $undo } the second.
common-screen-reader-undo-failed = { $error } Trindle は変更を元に戻すこともできませんでした: { $undo }
# How long copies are kept, in a sentence such as "kept for a week".
common-keep-week = 1週間
# How long copies are kept, in a sentence such as "kept for two weeks".
common-keep-two-weeks = 2週間
# How long copies are kept, in a sentence such as "kept for a month".
common-keep-month = 1か月
# How long copies are kept, in a sentence such as "kept for 3 days". { $n } is the number of days.
common-keep-days =
    { $n ->
       *[other] { $n }日
    }

# Said when an app can't be started. { $program } is the command, { $error } the system's reason.
common-app-not-started = Trindle は { $program } を起動できませんでした: { $error }

# Said when a settings file is too big to read.
common-keyfile-too-big = そのファイルは大きすぎて読み込めません。
# Said when a file Trindle was about to write at login isn't yours.
common-login-file-not-yours = そこにあるファイルはあなたのものではありません。
# What the security updates switch says on a hover while it is on.
common-auto-updates-on = 自動更新: オン
# What the security updates switch says on a hover while it is off.
common-auto-updates-off = 自動更新: オフ
# Said when polkit's list of the passwords it is keeping for this login couldn't be read, so Trindle can't make sure a password given for a drive is forgotten.
common-leave-list-unread = 保持されているパスワードを読み取れませんでした。
