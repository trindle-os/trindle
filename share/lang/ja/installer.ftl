# machine translated (Japanese), not yet reviewed by a person.

## The installer (src/installer): puts Trindle on a disk, and the welcome shown when it runs from a stick

## Window and steps along the top (src/installer/mod.rs)

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = { $name }をインストール
# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = ようこそ
# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = ディスク
# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = プライバシー
# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = パスワード
# Name of the fifth stop on that line, the page about the person.
installer-stop-you = あなた
# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = 準備完了
# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = インストール中
# Name of the last stop on that line, shown when the install is done.
installer-stop-done = 完了
# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = 準備しています
# Red line at the foot when the installer's helper could not be started.
installer-no-start = インストーラーを開始できませんでした: { $why }
# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = プランを読み込めませんでした。
# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = プランを読み込めませんでした: { $why }
# Red line at the foot when the helper stopped without saying why.
installer-stopped = インストールが止まりました。これ以上は何も書き込まれていません。

## Foot of every page

# Name of the round button at the foot that goes back a page.
installer-back = 戻る
# Name of the main button at the foot that goes to the next page.
installer-next = 次へ
# Name of the main button at the foot of the Ready page that starts the install.
installer-install = インストール
# Name of that button after one press, when a second press goes ahead.
installer-sure = 本当に?
# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = 今すぐ再起動
# Beside the keyboard test box on the first page.
installer-try-keyboard = キーボードを試す
# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = ここに入力してキーボードを試します
# Note beside Install after one press, when the second press writes beside another system.
installer-arm = もう一度クリックするとインストールします
# Note beside Install after one press, when the second press erases a whole disk.
installer-arm-erase = もう一度クリックすると消去してインストールします
# Note beside Next on the first page while no language is chosen.
installer-missing-language = 言語を選んでください
# Note beside Next on the first page while Trindle's language is not chosen.
installer-missing-trindle-language = Trindleの言語を選んでください
# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = キーボードを選んでください
# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = タイムゾーンを設定してください
# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Trindleを入れる場所を選んでください
# Note beside Next on the disk page while the disks are read again, after one turned out not to be the disk shown.
installer-disks-rereading = ディスクを読み直しています
# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = ディスクのパスワードを2回入力してください。同じものにしてください
# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = ログイン名を入力してください
# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = このコンピューターに名前を付けてください
# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = パスワードを2回入力してください。同じものにしてください

## First page: language, keyboard and time

# Title of the first page.
installer-welcome-title = { $name }へようこそ
# Sentence under the title of the first page.
installer-welcome-help = 言語を選ぶと、Trindleがそれに合わせて残りを設定します。
# Head of the card for choosing the computer's language (dates, spelling, other programs), and the name of its search box.
installer-language = 言語
# Head of the card for choosing the language of Trindle's own words (only those Trindle has), and the name of its search box.
installer-trindle-language = Trindleの言語
# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = キーボード
# Head of the card for choosing the time zone, and the name of its search box.
installer-time = 時刻
# Under a card's head while nothing has been chosen in its list.
installer-select = 選択

## Disk page

# Title of the disk page.
installer-disks-title = { $name }を入れる場所
# Sentence under the title of the disk page.
installer-disks-help = 選んだディスクの中身はすべて置き換わります。すでにあるものと並べてインストールする場合を除きます。
# Shown on the disk page when no disk is big enough.
installer-disks-none = 十分な大きさのディスクが見つかりませんでした。20 GBが必要です。
# Name and words of the choice to erase a whole disk.
installer-disks-whole = ディスク全体を使い、中身をすべて置き換える
# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = { $what }を残してディスクを分け合う
# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = 今あるものに{ $kept }を残し、Trindleに{ $mine }を使います
# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }、{ $size }
# Name of a partition with nothing on it.
installer-empty-space = 空き領域
# Names run together in a list, the last one after the word and.
installer-list-and = { $rest }と{ $last }
# Under a disk's name: what is on it.
installer-disk-has = { $list }が入っています
# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = { $list }が入っています(暗号化済み)
# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = 暗号化されたシステムが入っています
# Under a disk's name: there is nothing on it.
installer-disk-empty = 空
# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = ファイルが入っていますが、Trindleが認識できるシステムはありません
# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = { $gb } GBのスワップファイル
# Name of the switch that makes a swap file.
installer-swap-switch = スワップファイルを作る
# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Secure Bootが有効なため、このコンピューターは休止状態にできませんが、メモリが足りないときにスワップファイルは役に立ちます。ディスクに十分な空きがある場合だけ、Trindleが作ります。
# Under the swap file's head when the computer can hibernate.
installer-swap-note = スワップファイルがあると、コンピューターを休止状態にできます。休止状態では、電源を切っても開いているものがそのまま残ります。メモリが足りないときの予備にもなります。なければ、休止状態にはできません。ディスクに十分な空きがある場合だけ、Trindleが作ります。
# Under the swap file switch when the chosen disk or space is too small for one; the switch is then off and cannot be turned on.
installer-swap-too-small = この領域はスワップファイルには小さすぎます。

## Privacy page

# Title of the page about encrypting the disk.
installer-privacy-title = ディスクの暗号化
# Sentence under the title of that page.
installer-privacy-help = 暗号化したディスクは、このコンピューターから取り外されても、誰も読めません。
# Head of the card with the three choices.
installer-privacy-locked = ディスクのロック方法
# Name and words of the choice not to encrypt the disk.
installer-privacy-none = ディスクを暗号化しない
# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = 暗号化し、起動時にパスワードを入力する
# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = 暗号化し、このコンピューターのTPMチップでロックを解除する
# Head of the card about the TPM chip.
installer-privacy-about = チップについて
# In the card about the chip, on a computer that has one.
installer-privacy-chip = このコンピューターにはTPMチップがあります。インストール後の最初の起動では、チップを設定するために、パスワードを1回入力します。その後はチップがディスクのロックを解除するので、何も入力しません。チップを交換したときや、ディスクを別のコンピューターに移したときに必要なので、パスワードは保管してください。
# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = このコンピューターにはTrindleが使えるTPMチップがないので、起動のたびにパスワードを聞きます。チップのあるコンピューターでは、チップがディスクのロックを自動で解除できます。

## Disk password page

# Title of the disk password page.
installer-disk-title = ディスクのパスワード
# Sentence under the title of the disk password page.
installer-disk-help = コンピューターの起動時、最初に入力します。
# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = ディスクのパスワード
# Name of the round button that makes up a passphrase.
installer-disk-suggest = パスフレーズを提案
# Label and name of the second box of a password, where it is typed again.
installer-same-again = もう一度入力
# Red line under the password boxes when the two do not match.
installer-mismatch = 2つのパスワードが一致しません。
# Head of the card that tells the person to write the password down.
installer-disk-write-down = 書き留めてください
# Words in that card.
installer-disk-write-down-text = このパスワードを紙に書いて、コンピューターから離れた安全な場所に保管してください。なくすと、誰もディスクを開けられません。あなたも同じです。元に戻す方法はありません。ふつうの単語4つのパスフレーズは、入力しやすく、推測されにくいです。

## Account page

# Title of the account page.
installer-you-title = あなたについて
# Sentence under the title of the account page.
installer-you-help = ログインに使うアカウントです。
# Head of the card with the name boxes.
installer-account = あなたのアカウント
# Label and name of the box for the person's name.
installer-your-name = あなたの名前
# Label and name of the box for the sign-in name.
installer-user-name = ログイン名
# Label and name of the box for the computer's name.
installer-host-name = このコンピューターの名前
# Head of the card with the password boxes.
installer-your-password = あなたのパスワード
# Label and name of the first password box.
installer-password = パスワード
# Label of the strength meter and the name a screen reader gives it.
installer-strength = 推測されにくさ
# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = ログイン名を入力してください。
# Red line under the account boxes: a name is too long.
installer-name-too-long = その名前は長すぎます。
# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = a-zの文字で始めてください。
# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = a-z、0-9、-、_だけを使ってください。
# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = その名前はすでにシステムが使っています。
# Red line under the account boxes: the computer's name is empty.
installer-host-needed = このコンピューターに名前を付けてください。
# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = 使えるのは文字、数字、-だけです。
# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = コンピューターならすぐに推測できます。
# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
       *[other] コンピューターなら約{ $n }分で推測できます。
    }
# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
       *[other] コンピューターなら約{ $n }時間で推測できます。
    }
# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
       *[other] コンピューターなら約{ $n }日で推測できます。
    }
# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
       *[other] コンピューターが推測するには約{ $n }年かかります。
    }
# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = コンピューターが推測するには、有史以来の時間より長くかかります。

## Ready page

# Title of the Ready page.
installer-ready-title = インストールの準備ができました
# Head of the card that lists what will happen.
installer-ready-nothing-changed = まだ何も変更されていません
# Head of the card about security updates.
installer-ready-security = セキュリティ更新
# In the card about security updates.
installer-ready-security-text = { $name }は、Debianのセキュリティ修正を毎日自動でインストールします。設定の「このコンピューターについて」でオフにできます。
# Sentence under the title when a whole disk will be replaced.
installer-ready-replaced = { $disk }の中身はすべて置き換わります。
# Sentence under the title when the install goes beside what is there.
installer-ready-kept = { $part }の中身は残りますが、先にバックアップを取ってください。縮小中に停電すると、ファイルを失うことがあります。
# Sentence under the title when no disk has been chosen.
installer-ready-none = まだディスクが選ばれていません。
# Name of a row in the list: the whole disk will be used.
installer-ready-whole = { $disk }の全体
# Words of that row.
installer-ready-whole-text = 中身はすべて{ $name }に置き換わります。
# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = { $part }の隣
# Words of that row.
installer-ready-beside-text = { $part }を{ $size }に縮小し、空いた領域に{ $name }をインストールします。
# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = ディスクなし
# Words of that row.
installer-ready-no-disk-text = ディスクを選んでください。
# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = 暗号化なし
# Words of that row.
installer-ready-plain-text = ディスクを取り外した人は、誰でも読めます。
# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = 暗号化あり
# Words of that row when a password is typed at each start.
installer-ready-locked-password = コンピューターの起動のたびにパスワードを入力します。
# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = パスワードを1回入力すると、以後はこのコンピューターのTPMチップがディスクのロックを解除します。
# Added to the end of the disk's row on the last page when a swap file is to be made, with its size in GB.
installer-ready-swap = さらに{ $gb } GBのスワップファイルを作ります。
# Under the choice to let the TPM chip unlock the disk, on a computer with no usable chip.
installer-privacy-chip-off = Trindleが使えるTPMチップが必要です。
# In the card about the chip when the disk is not being encrypted, on a computer that has a chip.
installer-privacy-chip-none = このコンピューターにはTPMチップがあり、暗号化したディスクのロックを解除できます。
# In the card about the chip when the disk is not being encrypted, on a computer with no chip Trindle can use.
installer-privacy-no-chip-none = このコンピューターにはTrindleが使えるTPMチップがありません。
# Words of the language row: Trindle's language (its own name), the keyboard and the time zone chosen.
installer-ready-language = Trindleの言語は{ $trindle }、キーボードは{ $keyboard }、時刻は{ $zone }です。

## Install page

# Title of the page shown while the install runs.
installer-installing-title = { $name }をインストールしています
# Head of the card that holds the short tour.
installer-tour = かんたんな紹介

## Done page

# Title of the last page.
installer-done-title = { $name }をインストールしました
# Sentence under the title of the last page.
installer-done-help = USBメモリーを取り外してから、再起動してください。
# Head of the card that repeats the warning about the disk password.
installer-done-keep = ディスクのパスワードを大切に保管してください
# In that card when the TPM chip unlocks the disk.
installer-keep-chip = ディスクは暗号化されています。コンピューターの最初の起動では、チップを設定するために、ディスクのパスワードを1回聞かれます。その後は自動でディスクが開きます。そのパスワードを紙に書いて、このコンピューターから離れた安全な場所に保管してください。なくすと、誰もディスクを開けられません。あなたも同じです。
# In that card when the password is typed at each start.
installer-keep-password = ディスクは暗号化されています。まだなら今すぐ、ディスクのパスワードを紙に書いて、このコンピューターから離れた安全な場所に保管してください。なくすと、誰もディスクを開けられません。あなたも同じです。

## Slides shown while the disk is written (src/installer/brand.rs)

# Title of the first slide.
installer-slide-ring = すべてをひとつの輪に
# Words of the first slide.
installer-slide-ring-text = ポインターを画面の端に動かしてダブルクリックすると、輪が開きます。プログラムは外側に並び、時刻とスイッチは中央にあります。
# Title of the second slide.
installer-slide-snapshots = 失敗した更新を取り消す
# Words of the second slide.
installer-slide-snapshots-text = Trindleは、ソフトウェアを変更する前と後に、システムのスナップショットを取ります。設定から、システム全体をどのスナップショットにも戻せます。コンピューターが起動しないときは、ブートメニューから戻せます。
# Title of the third slide.
installer-slide-apps = じゃまにならないアプリ
# Words of the third slide.
installer-slide-apps-text = Flathubのアプリ、Debian標準のアプリ、AppImage、ソフトウェアの入手元がひとつの場所に集まり、バックアップは別のコンピューターへ持ち運べます。
# Title of the fourth slide.
installer-slide-yours = 自分好みに
# Words of the fourth slide.
installer-slide-yours-text = ライトとダークを選び、好きな音を使い、コピーしたものを覚えているクリップボードも使えます。
# Title of the fifth slide.
installer-slide-small = 小さくて静か
# Words of the fifth slide.
installer-slide-small-text = デスクトップは使うメモリが少なく、数秒で起動するので、古いノートパソコンも新品のように使えます。

## The tour with pictures (src/installer/tour.rs)

# Title of the tour slide about the ring.
installer-tour-ring = 輪がデスクトップのすべて
# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = ポインターをこの角に置くと輪が開きます
# Note on the ring picture, at the time.
installer-tour-ring-time = 時刻と日付は中央にあります
# Note on the ring picture, at the programs.
installer-tour-ring-programs = プログラムは外側に並びます
# Note on the ring picture, at the switches.
installer-tour-ring-switches = 音、画面、すべてのアプリ、スリープ防止、電源オフのスイッチ
# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = 開いているものの確認、クリップボードの利用、ボタンひとつで画面の片付け
# Title of the tour slide about Settings.
installer-tour-settings = 設定を、やさしい言葉で
# Note on the Settings picture, at the search box.
installer-tour-settings-search = ここに入力して設定を探します
# Note on the Settings picture, at Network.
installer-tour-settings-network = ネットワークでWi-Fiに接続します
# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = ライトでもダークでも、いつでも切り替えられます
# Note on the Settings picture, at the colours.
installer-tour-settings-colours = 20色から選ぶか、自分で入力できます
# Title of the tour slide about Apps.
installer-tour-apps = そのほかはアプリで
# Note on the Apps picture, at the search box.
installer-tour-apps-search = プログラムを名前や働きで探せます
# Note on the Apps picture, at the list.
installer-tour-apps-many = FlathubとDebianの無料プログラムが何千もあります
# Note on the Apps picture, at updates.
installer-tour-apps-updates = Trindle自体も含め、すべての更新ができます
# Title of the tour slide about voice control.
installer-tour-voice = コンピューターに話しかける
# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = 音声操作は、設定の「輪」にあります
# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = 「音声操作を追加」を1回押します。ダウンロードは約150 MBで、数分かかります
# Note on the voice picture, about what to say.
installer-tour-voice-say = 輪の中央をタップして、アプリの名前、「もっと大きく」、「Wi-Fiの設定」などと話します
# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Trindleはこのコンピューターの中で音声を理解し、他へは送りません
# Title of the tour slide about snapshots.
installer-tour-snapshots = 変更の前に戻る
# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = Trindleは更新のたびに、先にスナップショットを取ります
# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = 問題が起きたら、ブートメニューから元の状態に戻せます

## The welcome shown when running from a stick (src/installer/welcome.rs)

# Name of the picture of the ring, and the head of its card.
installer-live-caption = すべてが入った輪
# Sentence under the title of the welcome from a stick.
installer-live-help = USBメモリーから実行しているので、このコンピューターは何も変更されていません。
# Head of the card with the three first things to do.
installer-live-begin = はじめに
# Name of the first thing to do.
installer-live-open = 輪を開く
# Line under it.
installer-live-open-line = 画面の端でダブルクリックするか、Superキーを押します。
# Name of the second thing to do.
installer-live-speak = 声を使う
# Line under it when voice control is there.
installer-live-speak-line = 輪の中央をタップして、やりたいことを話します。
# Line under it when voice control has to be added.
installer-live-speak-add = Trindleをインストールしたら、設定の「輪」で音声操作を追加してください。
# Name of the third thing to do.
installer-live-tab = すべて輪の中に
# Line under it.
installer-live-tab-line = 開いているもの、通知、クリップボードがそこにあります。
# The choice of the dark look.
installer-live-dark = ダーク
# The choice of the light look.
installer-live-light = ライト
# Name the screen reader gives to the choice between dark and light.
installer-live-look = デスクトップ全体のライトとダーク
# Name of the round button that closes this window and opens the ring.
installer-live-around = 先に見て回る
# Name of the main button that opens the installer.
installer-live-install = { $name }をインストール

## Words the root half sends to the window (src/installer/steps.rs)

# Foot of the install page while room is made on the disk.
installer-step-room = 領域を空けています
# Foot of the install page while the disk is prepared.
installer-step-prepare = ディスクを準備しています
# Foot of the install page while a place is made beside another system.
installer-step-place = Trindleの領域を空けています
# Foot of the install page while the disk is looked at.
installer-step-check = ディスクにすでにあるものを確認しています
# Foot of the install page while another system is made smaller.
installer-step-smaller = ほかのシステムを縮小しています
# Foot of the install page while the disk is encrypted.
installer-step-lock = ディスクを暗号化しています
# Foot of the install page while the TPM chip is set up.
installer-step-chip = TPMチップを設定しています
# Foot of the install page while the system is copied.
installer-step-copy = Trindleをディスクにコピーしています
# Foot of the install page while settings are written.
installer-step-settings = 設定を書き込んでいます
# Foot of the install page while the person's account is made.
installer-step-account = アカウントを作っています
# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = ライブ用の追加分を取り除いています
# Foot of the install page while the start-up files are made.
installer-step-startup = 起動用のファイルを作っています
# Foot of the install page while the boot menu is set up.
installer-step-boot = ブートメニューを設定しています
# Foot of the install page while snapshots are set up.
installer-step-snapshots = スナップショットを設定しています
# Foot of the install page at the very end.
installer-step-finishing = 仕上げています
# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program }を開始できませんでした: { $why }
# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program }が完了しませんでした。{ $why }
# Red line at the foot: a file could not be written.
installer-write-failed = { $path }を書き込めませんでした: { $why }
# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev }はディスクではありません。
# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev }はこのコンピューターのディスクではありません。
# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev }は、Trindleが実行されているUSBメモリーです。
# Red words under the disk, back on the disk page: something on the chosen disk is mounted, swapped to or open.
installer-disk-in-use = { $dev }は使用中です。使っているものを閉じてから、もう一度選んでください。
# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = ディスクが選ばれていません。
# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Trindleはそのログイン名を使えません。
# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Trindleはそのコンピューター名を使えません。
# Red line at the foot: the language cannot be used.
installer-bad-language = Trindleはその言語を使えません。
# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Trindleはそのキーボードを使えません。
# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Trindleはそのタイムゾーンを使えません。
# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = その名前にはTrindleが使えない文字が含まれています。
# Red line at the foot: a password has a line break in it.
installer-bad-password = パスワードに改行は使えません。
# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = インストーラーの後半は、rootとして実行する必要があります。
installer-already-running = このコンピューターでは、すでに別のインストーラーが実行されています。
# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = このディスクには共有できるブートパーティションがないので、すでにあるものと並べてTrindleをインストールできません。
# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = Trindleは新しいパーティションを見つけられませんでした。
# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = { $part }のサイズを読み取れませんでした。
# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindleは{ $fs }パーティションを縮小できません。
# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindleは種類が不明なパーティションを縮小できません。
# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = ディスクを暗号化するにはパスワードが必要です。
# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = システムをコピーできませんでした: { $why }
# Red line at the foot: the copy of the system failed.
installer-copy-failed = システムをコピーできませんでした。{ $why }
# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = スワップファイルの空きがありません。
# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = Trindleはスワップファイルがディスクのどこにあるか読み取れませんでした。
# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Trindleは新しいパーティションを識別できませんでした。
# Red line at the foot: the time zone could not be set.
installer-timezone-failed = タイムゾーンを設定できませんでした: { $why }
# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Trindleはこのコンピューターにブートメニューをインストールできませんでした。
# Said beside how far the install has got, once it has started.
installer-started = インストール中
# The button that ends an install under way.
installer-halt = 中止
# Beside Stop after its first press.
installer-halt-sure = インストールを中止しますか? ディスクの元の中身はすでに消えています。
# While an install is being stopped.
installer-halting = 中止しています
# When Stop could not let go of the disk.
installer-halt-stuck = Trindleはディスクを解放できませんでした。コンピューターを再起動して、もう一度試してください。
# The copy is done and the person is still answering.
installer-step-waiting = コピーが終わりました。入力を待っています。

# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindleには1 GB以上のメモリが必要ですが、このコンピューターはそれより少ないです。

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = ディスクを暗号化できるのはUEFIで起動するコンピューターだけで、このコンピューターは違います。

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = { $part }のWindowsは、休止状態か高速スタートアップで終了されています。今縮小すると、ファイルを失うことがあります。Windowsを起動し、「再起動」を選んでから、もう一度試してください。

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindleはインストールされましたが、スナップショットを設定できませんでした: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindleはインストールされましたが、スナップショットをブートメニューに追加できませんでした: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = 設定できなかったものがあります
# Red line under the disk password boxes: the computer gave no random numbers, so no passphrase could be made.
installer-suggest-failed = Trindleはパスフレーズを作れませんでした。ご自身で入力してください。
# Red words on the disk page: the disk was swapped for another after the list was read.
installer-disk-changed = { $dev }は、表示されていたディスクではありません。ディスクをもう一度選んでください。
# Red line at the foot: a disk with an old-style partition table cannot be shared with.
installer-no-gpt = { $dev }はGPTパーティションテーブルを使っていないので、すでにあるものと並べてTrindleをインストールできません。
# Red line at the foot: the boot partition on the disk is too small to share.
installer-boot-partition-small = このディスクのブートパーティションは共有するには小さすぎるので、すでにあるものと並べてTrindleをインストールできません。
# Note on the last page: the swap file was not made, with why.
installer-swap-failed = Trindleはインストールされましたが、スワップファイルを作れませんでした: { $why }
# Red line at the foot: a partition's identity could not be read after it was made.
installer-uuid-missing = Trindleは設定後の{ $dev }を識別できませんでした。
# Words on the last page: the disk that will be replaced, with what it holds.
installer-ready-replaced-known = { $disk }の中身はすべて置き換わります({ $holds })。
# A disk named with its model and size, then its place such as /dev/sda.
installer-disk-named = { $what }、{ $dev }
# Red line at the foot: a program the installer runs stopped before it had all it was given.
installer-run-stopped = { $program }は必要なものがそろう前に止まりました: { $why }
# Red words on the last page: the TPM chip could not be set up to open the disk, so the disk password is asked for at every start.
installer-chip-not-set-up = TPMチップを設定できなかったので、コンピューターの起動のたびにディスクのパスワードを聞きます。
# Red words on the last page: the keyboard for the disk password screen could not be set, so it may be the American one.
installer-start-keyboard-failed = Trindleはインストールされましたが、ディスクのパスワード画面は英語配列のキーボードになる場合があります。
# Red words on the last page: the login screen could not be switched on; { $why } is the reason.
installer-login-screen-failed = Trindleはインストールされましたが、ログイン画面をオンにできませんでした: { $why }
# Red words on the last page: Trindle could not be set to take its own updates; { $why } is the reason.
installer-updates-held = Trindleはインストールされましたが、まだ自分の更新を受け取れません: { $why }
# Red words on the last page: earlier versions of people's files (home snapshots) could not be switched on.
installer-home-snapshots-failed = Trindleはインストールされましたが、ファイルの以前のバージョンがオフです。設定の「スナップショット」でオンにできます。
# Red words on the last page: Trindle could not be added to the computer's own start-up list; { $why } is the reason.
installer-firmware-entry-failed = Trindleはインストールされましたが、コンピューター自体の起動リストに追加できませんでした: { $why }
# Red line: the account the stick ran as could not be removed from the new system, so the install stopped.
installer-live-account-left = USBメモリーのアカウント{ $user }を新しいシステムから削除できませんでした。

# Under the swap file's switch when a size smaller than memory is chosen.
installer-swap-no-hibernate = このコンピューターのメモリより小さいため、休止状態にできません。

# In the swap size list: the size of the computer's memory, which hibernating needs.
installer-swap-size-hibernate = { $gb } GB、休止状態にできる大きさ

# In the swap size list: a smaller size.
installer-swap-size = { $gb } GB

# Tip of the swap size list.
installer-swap-size-tip = スワップファイルのサイズ

# Why logind holds off sleep and shutting down while the installer changes the disk (shown by systemd-inhibit --list).
installer-power-held = Trindle をインストールしています
