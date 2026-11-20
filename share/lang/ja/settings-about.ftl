# machine translated (Japanese), not yet reviewed by a person.

# The About this computer page of Settings: the computer's name, what it is made of, what runs on it and how long it takes to start.

## About (src/settings/panels/about.rs)

# Name of the About page in the Settings side bar and its title.
settings-about-title = このコンピューターについて
# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = ホスト名 コンピューター名 ネットワーク名 このコンピューターについて システム バージョン debian trindle メモリー ram プロセッサー cpu ディスク グラフィックス 画面 モデル カーネル アクティビティ 実行中 プログラム 忙しい タスクマネージャー システムモニター ドライブ usb 空き容量 更新 アップグレード 確認 自動 セキュリティ 修正 hostname computer name network name about system version memory processor disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives space updates upgrade check automatic automatically security fixes
# Text behind the info button of the About page.
settings-about-help = このページでは、ネットワーク上のほかのコンピューターから見えるこのコンピューターの名前、構成、起動にかかる時間を確認できます。「アクティビティ」では、コンピューターが今していることと、メモリーとプロセッサーを使っているプログラムが分かります。「ドライブ」では、ドライブとその空き容量が分かります。どちらも別のウィンドウで開きます。「詳細」の矢印から開いてください。
# What is shown when a fact about the computer cannot be found.
settings-about-unknown = 不明
# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
       *[other] { $model }、{ $cores } スレッド
    }
# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Intel グラフィックス
# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = AMD グラフィックス
# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = NVIDIA グラフィックス
# The graphics line inside a virtual machine.
settings-about-gpu-virtual = 仮想マシンのグラフィックス
# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = グラフィックス
# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height }、{ $hz } Hz
# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } 使用中
# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } 空き
# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = 全体 { $amount }
# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }、電源を入れてから { $on } です。
# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }、電源を入れてから { $on } です。ディスクがほぼいっぱいです。
# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } 秒
# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } 分
# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
       *[other] { $n } 分
    }
# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
       *[other] { $n } 時間
    }
# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
       *[other] { $n } 日
    }
# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = 使えるのは英字、数字、ハイフンだけです。ハイフンで始めたり終えたりしないでください。
# Words under the computer name box while the password window is open.
settings-about-name-waiting = パスワードを待っています…
# Red words under the computer name box when renaming failed.
settings-about-name-failed = コンピューターの名前を変更できませんでした。
# Words under the computer name box after renaming worked.
settings-about-name-done = すでに開いているプログラムは、次にログインしたときから新しい名前を使います。
# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = コンピューター名
# Tip of the save button beside the computer name box.
settings-about-rename = このコンピューターの名前を変更
# Bold line of the computer name setting.
settings-about-name = コンピューター名
# Heading of the card with the computer's name.
settings-about-name-group = 名前
# Text behind the info button of the Name card.
settings-about-name-about = ネットワーク上と、ターミナルのプロンプトに表示されるコンピューターの名前です。英字、数字、ハイフンを使えますが、ハイフンで始めたり終えたりはできません。変更には、パスワードの入力が必要です。
# Heading of the card about the computer's hardware.
settings-about-hardware = コンピューター
# Text behind the info button of the hardware card.
settings-about-hardware-about = コンピューターのメーカーとモデル、プロセッサー、グラフィックスを表示します。プロセッサーが同時に動かせるスレッド数と、使っている各画面のサイズとリフレッシュレートも表示します。
# Heading of the card with the switch for looking for updates.
settings-about-updates = 更新
# Text behind the info button of the updates card.
settings-about-updates-about = Trindle は、ログインの少しあとと、その後は数時間おきに、アプリ、プラグイン、ファームウェア、Trindle 自身の新しいバージョンを確認します。これをオフにすると、「アプリ」の「更新を確認」か、リングの更新ボタンを押したときだけ確認します。Debian のセキュリティ修正は毎日インストールされ、別のスイッチがあります。そちらをオフにすると、自動ではインストールされなくなり、更新するまで既知のセキュリティの穴が残ります。どちらのスイッチを変えるにも管理者のパスワードが必要で、コンピューター全体にその設定が使われます。
# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = 更新を確認
# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = パスワードを待っています…
# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = セキュリティ更新
# Warning line under the Security updates switch when it is off.
settings-about-security-off = セキュリティ修正は自動ではインストールされません。
# Heading of the card about the software.
settings-about-software = ソフトウェア
# Text behind the info button of the software card.
settings-about-software-about = Trindle は Debian の上で動くデスクトップです。Debian が下で動くプログラムの面倒を見ます。カーネルは、ハードウェアとやり取りする部分です。デスクトップは、Wayland を使う Trindle 独自のコンポジターが描きます。
# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle 独自のコンポジター
# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
       *[other] アプリ { $n } 個
    }
# Heading of the card with the ways to Activity and Drives.
settings-about-more = 詳細
# Text behind the info button of the More detail card.
settings-about-more-about = 「アクティビティ」では、コンピューターが今していることと、メモリーとプロセッサーを使っているプログラムが分かります。「ドライブ」では、すべてのドライブの使用量と状態、USB メモリーが分かります。
# Bold line of the setting that opens Activity.
settings-about-activity = 動いているものを見る
# Tip of the arrow that opens the Activity program.
settings-about-activity-name = アクティビティ
# Bold line of the setting that opens Drives.
settings-about-drives = ドライブと空き容量
# Tip of the arrow that opens the Drives program.
settings-about-drives-name = ドライブ
# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle は起動時間をダウンロードに保存できませんでした。
# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = 日時,秒
# File name of the saved start-up times, in Downloads.
settings-about-times-file = Trindle 起動時間.csv
# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = ダウンロードに { $file } として保存しました
# Label of the word button that saves every start-up time.
settings-about-times-save = 保存
# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = すべての起動時間をダウンロードに保存
# Bold line of the setting that saves the start-up times.
settings-about-times-keep = 起動時間をそれぞれ保存
# Added after the words behind the i of its card.
settings-about-times-keep-sub = スプレッドシートとしてダウンロードに保存します。
# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = 1本が1回の起動です。いちばん新しいものが右です。
# Heading of the card with the last ten start-up times.
settings-about-times = 直近10回の起動
# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle は、電源ボタンからログイン画面までの起動時間を計ります。ディスクのパスワードを入力していた時間は含みません。いちばん新しいものが上です。
# Heading of the memory tile.
settings-about-memory = メモリー
# Heading of the disk tile.
settings-about-disk = ディスク

## Problem reports card (src/settings/panels/report_card.rs)

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = 問題の報告
# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = 報告は、Trindle づくりを手伝うコミュニティ Buildmorphic に届きます。メールアドレスだけで参加できます。最初に報告した問題には、直せなくても、クレジットとレベルがもらえます。報告に含まれるのは決まった項目だけで、送信を押すまで何も送られません。ユーザー名や GitHub 名は任意で、ユーザー名がメールアドレスになることはありません。
# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = 問題が起きたときに報告の送信を案内する
# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Buildmorphic のユーザー名
# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = GitHub 名
# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = コミュニティについて
# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
       *[other] オフにしました。送信待ちの報告 { $n } 件を削除しました。
    }
# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
       *[other] オフにしました。送信待ちの報告 { $n } 件を削除できませんでした。
    }
# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = オフにしましたが、報告アドオンの送信待ちの報告は削除されませんでした。
