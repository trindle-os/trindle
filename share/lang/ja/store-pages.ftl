# machine translated (Japanese), not yet reviewed by a person.

# Apps: the pages of apps, the installed list, sources and moving apps (src/store/pages/).

## Pages of apps (src/store/pages/)

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
       *[other] { $count } 個のアプリ
    }
# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
       *[other] { $count } 件の更新があります
    }
# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = Flathub から選べるアプリは { $apps }。{ $more }。
# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = Flathub と AppImageHub から選べるアプリは { $apps }。{ $more }。
# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = AppImageHub から選べるアプリは { $apps }。
# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = 更新の確認はオフです
# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = お使いのものはすべて最新です
# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }。
# Line under the title of a search that found nothing.
store-grid-none-found = 「{ $text }」に合うアプリはありません。
# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = OpenAI がダウンロードに署名したら、ChatGPT が戻ってきます。
# Line under the title of an empty page of apps.
store-grid-none-here = ここにはまだアプリがありません。
# Advice in the empty page of a search.
store-grid-try-shorter = もっと短い言葉で探すか、左でカテゴリを選んでください。
# Advice in the empty page of a kind of apps.
store-grid-search-or-look = 上で名前を検索するか、左で別のカテゴリを選んでください。
# Under a page of apps: how many are shown of how many there are.
store-grid-showing = { $apps } 件中 { $shown } 件を表示しています。
# Link under a page of apps that shows more of them.
store-grid-show-more = さらに { $next } 件表示
# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Flathub からアプリの一覧を取得しています。初回は1分ほどかかります。
# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = アプリは Flathub に接続できませんでした。
# Under the title of the waiting page when the computer is not connected.
store-wait-offline = このコンピューターはインターネットに接続されていません。接続されたら、アプリがもう一度試します。
# In the waiting page when the computer is not connected.
store-wait-comes-from = 選べるアプリは、インターネット経由で Flathub から届きます。
# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = 左の { $pages } とアプリの設定は、一覧がなくても使えます。
# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = 左の { $pages } とアプリの設定は、Flathub がなくても使えます。
# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = 左の { $pages } とアプリの設定は、インターネットがなくても使えます。
# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = もう一度試す
# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = 検索結果
# Red words under the search box when no web browser opens a link.
store-no-browser = { $url } を開くウェブブラウザがありません。先にアプリでブラウザをインストールしてください。

## The Installed page (src/store/pages/)

# Said in the card or under the title while the lists are being read.
store-looking = 確認しています…
# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = インストール済みの一覧とリングに入りました
# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = 削除しました
# The word on an app's dial when installing or removing did not work.
store-done-not = 失敗
# The word on an app's dial when it was installed.
store-done-installed = インストール済み
# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak が失敗しました。
# Tab of the Installed page that shows everything.
store-tab-all = すべて
# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Flatpak ファイル
# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage
# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = deb ファイル
# Tab of the Installed page that shows plugins.
store-tab-plugins = プラグイン
# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = まだ何もインストールされていません。試したいアプリは「見つける」で探せます。
# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
       *[other] { $n } 個のプラグインがインストールされています
    }
# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
       *[other] { $n } 個のプログラムがインストールされています
    }
# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n } 個がインストールされています
# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
       *[other] { $start }、すべて Flathub から
    }
# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
       *[other] { $start }、すべて Flatpak ファイルから
    }
# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
       *[other] { $start }、すべて AppImage
    }
# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
       *[other] { $start }、すべて .deb ファイルから
    }
# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }、{ $parts }
# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }
# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = Flathub から { $n } 個
# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
       *[other] Flatpak ファイルから { $n } 個
    }
# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
       *[other] AppImage { $n } 個
    }
# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
       *[other] アプリ { $n } 個
    }
# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
       *[other] .deb ファイルから { $n } 個
    }
# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
       *[other] プラグイン { $n } 個
    }
# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }。{ $state }
# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = 最新です。
# End of the Installed line when everything is up to date.
store-installed-state-all = すべて最新です。
# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
       *[other] { $n } 個に更新があります。
    }
# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $total } 個のプログラムのうち { $shown } 個が「{ $find }」に合います。
# The same when plugins are among the things.
store-installed-match-things = { $total } 個のうち { $shown } 個が「{ $find }」に合います。
# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }
# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = AppImage を追加
# Placeholder of the typing box on the Installed page.
store-installed-search = インストール済みを検索
# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = AppImage を追加できませんでした。
# In the card of the Installed page when nothing is installed.
store-installed-empty = インストールしたものはすべてここに表示されます。「見つける」や GitHub のアプリ、AppImage、.deb ファイルのプログラム、プラグインです。
# In the card of the Installed page when the search matches nothing.
store-installed-no-match = 「{ $find }」に合うインストール済みのものはありません。
# On an installed thing's line: its version.
store-installed-version = バージョン { $version }
# On an installed thing's line: its updates are held back.
store-installed-held = 更新を保留中
# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = 更新を保留中。{ $waits }
# Tip and spoken name of the button while an app is updating.
store-installed-updating = { $name } を更新中
# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = { $name } を更新
# Tip of the round button that shows an installed app's page.
store-installed-about = 詳細
# Under an installed thing's row while it is being taken off.
store-installed-removing = 削除しています…

## Adding an AppImage or a Flatpak file (src/store/pages/)

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = Trindle は今、ファイル選択を開けません。
# Title of the window that asks which AppImage to add.
store-add-choose-title = AppImage を選ぶ
# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage
# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = Trindle はファイル選択を開けません。
# Words in the Installed card while an AppImage is being added.
store-add-adding = 追加しています…
# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } はすでにインストールされています。「インストール済み」にあります。
# Words once an AppImage is in the ring.
store-add-in-ring = 追加しました。一覧に入っています。
# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = このコンピューターで使える Flatpak リファレンスではありません。
# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } をインストールしました。
# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak はインストールできませんでした。
# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = Trindle は { $name } をインストールできませんでした。{ $why }
# Words in the Installed card while an AppImage replaces another.
store-add-replacing = 置き換えています…
# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } はすでにあります
# Small heading over the AppImage being brought.
store-replace-new = 新しいもの
# Small heading over the AppImage already there.
store-replace-old = すでにあるもの
# Tip of the round button that keeps the AppImage already there.
store-replace-skip = スキップして古いほうを残す
# Button that replaces the AppImage already there with the new one.
store-replace-do = 置き換える
# Heading of the card that asks what to do with a Flatpak being removed.
store-flatpak-ask-head = { $name } を削除しますか?
# Under that heading: how much saved data the app has.
store-flatpak-ask-size = 保存データは { $size } を使っています。
# Under that heading when the app has saved no data.
store-flatpak-ask-none = 保存データはありません。
# Under that heading when the folder of the saved data cannot be read, so its size is not known.
store-flatpak-ask-uncounted = 保存データの大きさを測れませんでした。
# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = 保存データの大きさを測っています…
# Under the choices: the two that delete data cannot be undone.
store-flatpak-ask-undone = 削除したデータは元に戻せません。
# Button: remove the app and leave the data it saved.
store-flatpak-keep = データを残して削除
# Button (red): remove the app and delete its saved data.
store-flatpak-wipe = データも一緒に削除
# Button (red): delete only the saved data and keep the app.
store-flatpak-data-only = データだけを削除
# Why Delete its data only is greyed.
store-flatpak-data-only-none = 削除する保存データがありません。
# Tip of the round button that shuts the card with nothing done.
store-flatpak-cancel = キャンセル
# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = 保存データを削除しました。
# Words while only the data is being deleted.
store-flatpak-data-going = 保存データを削除しています…
# Red words under a button when a job ended and gave no reason.
store-job-no-reason = 完了しませんでした。理由は示されませんでした。
# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = 問題の場所を表示

## The GitHub page (src/store/pages/)

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = アプリを検索 (Enter で GitHub も検索)
# Tick box under the Search page's box: look on Flathub, for Flatpaks.
store-gh-place-flathub = Flathub (Flatpak)
# Tick box: look on AppImageHub, for AppImages.
store-gh-place-hub = AppImageHub (AppImage)
# Tick box: look on GitHub, which needs Enter.
store-gh-place-github = GitHub
# Heading of the card of matches from AppImageHub.
store-gh-hub = AppImageHub ({ $n })
# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = このコンピューターはインターネットに接続されていないため、Trindle は GitHub を検索できませんでした。
# Note on the GitHub page while GitHub is being asked.
store-gh-asking = GitHub を確認しています…
# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = GitHub にはこれ以上ありません。
# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
       *[other] { $n } 個のプロジェクトから AppImage と Flatpak のファイルを探しています…
    }
# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = GitHub の検索結果に、このコンピューター向けの AppImage や Flatpak のファイルはありません。
# Heading of the card of matches from GitHub.
store-gh-on = GitHub
# Heading of the card that holds the search box.
store-gh-search-card = GitHub を検索
# Line under the title of the GitHub page.
store-gh-line = GitHub のアプリです。AppImage または Flatpak のファイルで入手します。誰も確認していないので、信頼できる人のものだけをインストールしてください。
# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = このコンピューターがインターネットに接続されていない間は、GitHub を検索できません。

## A project on GitHub and the GitHub key (src/store/pages/)

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak と AppImage
# Tip of the round button that shows or hides a project's files.
store-gh-see-files = ファイルを見る
# Button that installs a GitHub project or file.
store-gh-install = インストール
# Link to a project's page on GitHub.
store-gh-open = GitHub で開く
# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }、{ $size }、リリース { $tag }
# Name of the GitHub key's button and of its box.
store-gh-key = GitHub キー
# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = GitHub キーを保存済み
# In the GitHub key's box: what the key is for.
store-gh-key-words = ほとんどの場合、キーは要りません。キーがないと、このコンピューターからは1時間に60回まで GitHub に問い合わせできます。キーがあれば5,000回までです。キーで増えるのはこの上限だけです。公開されているファイルを見るためにだけ使います。Trindle があなたのアカウントに入る手段にはならず、作るときに権限へチェックを入れることもありません。キーはこのコンピューターのキーリングに保存され、GitHub にだけ送られます。
# In the GitHub key's box: how to make a key.
store-gh-key-how = GitHub でキーを作り、好きな名前を付けて、権限はそのままにします。「Generate token」を押し、表示された文字をここに貼り付けてください。
# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = 削除できませんでした: { $why }
# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = 保存できませんでした: { $why }
# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = 空のままにすると今のキーを残します
# Placeholder of the GitHub key's box.
store-gh-key-paste = キーを貼り付け
# Tip of the round button that removes the stored GitHub key.
store-gh-key-remove = キーを削除
# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = GitHub でキーを作る
# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = キーを保存
# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = 最新リリースには、このコンピューターにインストールできるものがありません。

## The Move your apps page (src/store/pages/)

# Word on the file's dial and under a GitHub file when it was added.
store-added = 追加しました
# Line under the title of the Move your apps page.
store-bk-line = アプリと設定を1つのファイルにまとめて、どこへでも持ち運べます。
# Heading of the card that makes a backup.
store-bk-card = バックアップ
# Tip of the round button while the backup is being made.
store-bk-making-tip = バックアップを作成中
# Main button that makes the backup.
store-bk-make = バックアップを作る
# Tip of the round button that shows the backup in Folders.
store-bk-show = Folders で表示
# Name of the switch for the list of apps in a backup.
store-bk-apps = アプリ
# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
       *[other] { $n } 個の Flatpak アプリの一覧です。
    }
# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list }
# Name of the switch for the apps' own data in a backup.
store-bk-data = アプリ自身のデータ
# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage
# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Trindle の設定
# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } 現在 { $size }。
# Name of the row that says where the backup goes.
store-bk-where = 保存先
# Under that row: the folder and the file's name.
store-bk-where-sub = ホームフォルダの { $name }
# Heading of the card that puts a backup back.
store-bk-back-card = バックアップを復元
# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = バックアップファイルを選ぶ
# Note in the card that puts a backup back.
store-bk-back-note = 復元すると、アプリがインストールされ、データと設定が元に戻ります。書類と写真はバックアップに入っていないので、自分でコピーしてください。
# Said while a backup is being made.
store-bk-making = バックアップ中です。数分かかることがあります。
# Said when a backup is made: the file and its size.
store-bk-made = バックアップは { $path } ({ $size }) です。USB メモリやクラウドストレージにコピーして持ち出せます。
# Red words when no connection to the file chooser exists.
store-bk-no-bus = Trindle がファイル選択に接続できないため、バックアップが選ばれませんでした。
# Title of the window that asks which backup to use.
store-bk-choose-title = バックアップを選ぶ
# Name of the filter in the window that asks which backup to use.
store-bk-filter = Trindle と Flat のバックアップ
# Red words when the file chooser gave no answer.
store-bk-no-answer = ファイル選択から応答がなかったため、バックアップが選ばれませんでした。
# Said while a backup file is read.
store-bk-reading = バックアップを読み込んでいます…
# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } すべての設定を反映するには、ログアウトしてもう一度ログインしてください。
# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
       *[other] { $apps } 個のアプリが見つかり、{ $with } 個に設定があります。開始します…
    }

## The Software sources page (src/store/pages/)

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] ソフトウェアソースはまだ追加されていません。
       *[other] { $n } 個のソフトウェアソースが追加され、Debian の更新に合わせて最新に保たれます。
    }
# Under a maker's source that is added and installed.
store-src-added = { $what } は追加済みで、更新も自動で行われます。
# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }。ソースは追加済みですが、プログラムはインストールされていません。
# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }。別の方法でインストールされています。
# Tip of the round button that removes a maker's source and its program.
store-src-remove-both = これとソースを削除
# Link to a maker's page, naming the program.
store-src-open = { $name } を開く
# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Tailscale にログイン
# Tip of the round button that adds a maker's source, naming the program.
store-src-add = { $name } を追加
# Heading of the card of makers' sources that can be added.
store-src-ready = 追加できます
# Name of the field for the address of a source.
store-src-address = アドレス
# Name of the field for the suite of a source.
store-src-suite = スイート
# Name of the field for the components of a source.
store-src-components = コンポーネント
# Name of the field for a source's signing key.
store-src-key = 署名キー
# Name of the field for the program to install from a source.
store-src-program = インストールするプログラム
# Placeholder of the program field.
store-src-program-hint = 省略できます
# Tip of the round button that adds the source typed.
store-src-add-this = このソースを追加
# Heading of the card that adds another source.
store-src-another = 別のソースを追加
# Note in that card.
store-src-another-note = ソースはコンピューター全体の更新をインストールできます。信頼できる提供元だけを追加してください。追加のたびに管理者パスワードが求められます。
# Tip of the round button that removes a source that is already on the computer.
store-src-remove-this = このソースを削除
# Heading of the card of sources already on the computer.
store-src-already = このコンピューターにすでにあるもの
# Red words when the addresses of a new source are not secure.
store-src-bad-address = 2つのアドレスは、どちらも https:// で始めてください。
# Red words when the suite or components are not plain words.
store-src-bad-words = スイートとコンポーネントは、ふつうの単語にしてください。
# Red words when the program is not a package name.
store-src-bad-package = パッケージ名ではありません。
# Word on a source's dial while its job starts.
store-src-starting = 開始中
# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Tailscale のログインページを開いています…
# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = 開いたページでログインしてください。ログインすると、Trindle が自動で続けます。
# Said under Tailscale's row when it is connected.
store-src-connected = 接続済み

# Said under the backup row when its apps come from an http address on the local network: the link is not encrypted, and the restore goes on.
store-bk-http-line = アプリへのリンクは暗号化されていません。
# Title of the card shown before a restore from an http address over the internet.
store-bk-http-title = 暗号化なし
# Words of that card.
store-bk-http-card = この接続は暗号化されていません。途中で送られる内容を誰かに見られたり、バックアップを別のものにすり替えられたりするおそれがあります。
# Label of the tick box on that card; the restore button works only once it is on.
store-bk-http-tick = リスクを理解して受け入れます
# Button on that card that restores the backup.
store-bk-http-go = 復元
# Button on that card that gives the restore up.
store-bk-http-cancel = キャンセル
# Said before every Add of an AppImage from AppImageHub, in plain ink: what is and is not checked, and how Trindle boxes it.
store-hub-warning = AppImageHub はプロジェクトを一覧にしているだけです。誰も審査やスキャンをしておらず、署名も必須ではありません。Trindle は各 AppImage を、専用の非公開ホームフォルダを持つ箱の中で実行します。最初はインターネットと音は使えますが、あなたのファイルは一切見えません。
# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = AppImage が示す入手元は目安にすぎません。Trindle は確認していません。
# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = GitHub: { $repo }
# The same, with how many stars the project has.
store-from-github-stars = GitHub: { $repo }、スター { $stars } 個
# Where an AppImage comes from: a website, by its name.
store-from-site = GitHub にはありません。入手元は { $site } です
# Where an AppImage comes from, for one added by hand.
store-from-unknown = 入手元は不明
# Round button that opens the website an AppImage is fetched from.
store-from-site-open = ウェブサイトを開く
# When an app was last updated.
store-age-updated = 最終更新 { $date }
# The same, said in red, when that is more than six months ago.
store-age-updated-old = 最終更新 { $date }。6か月以上更新されていないため、修正されていないセキュリティ上の問題があるかもしれません。
# When the source does not say when an app was last updated.
store-age-updated-unknown = 最終更新: 不明
# When an app's project was made.
store-age-created = 作成 { $date }
# When the source does not say when an app's project was made.
store-age-created-unknown = 作成: 不明
# A date the source does not give, on a line of Facts.
store-age-not-known = 不明
# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = 追加する前に
# Title of the card that lists the AppImages.
store-browse-list = AppImage
# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
       *[other] AppImageHub に { $count } 個の AppImage が載っています。
    }
# The same while AppImageHub's list is being fetched.
store-browse-fetching = AppImageHub の一覧を取得しています…
# The same when the list could not be fetched.
store-browse-failed = Trindle は AppImageHub の一覧を取得できませんでした。
# The same with no internet and no copy kept.
store-browse-offline = このコンピューターがインターネットに接続されていない間は、AppImageHub の一覧を取得できません。
# In the list when the kind chosen has none.
store-browse-none = AppImageHub にはありません。
# Under the rows: how many are shown of how many.
store-browse-showing = { $n } 件中 { $shown } 件を表示しています。
# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Trindle のアプリに追加
# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = 詳細とファイルを見る
# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = 両方
# The same: only Flatpaks.
store-show-flatpaks = Flatpak
# The same: only AppImages.
store-show-appimages = AppImage
# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }
# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage
# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage は確認されていません。誰も審査やスキャンをしていません。
# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle は各 AppImage を、専用の非公開ホームフォルダを持つ箱の中で実行します。最初はインターネットと音は使えますが、あなたのファイルは一切見えません。
# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = 今後は表示しない
# Button on that card that goes on with the install.
store-appimage-warn-ok = OK
# Button on that card that gives the install up.
store-appimage-warn-cancel = キャンセル

# Printed by the command line after a backup. { $size } is how big the file is.
store-cli-backed-up = バックアップしました ({ $size })

# Printed by the command line after Trindle's own settings were put back.
store-cli-settings-back = Trindle の設定を復元しました
