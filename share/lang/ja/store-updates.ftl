# machine translated (Japanese), not yet reviewed by a person.

# Apps: the Updates page (src/store/update_page/).

## The Updates page (src/store/update_page/)

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Debian パッケージ
# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = バージョン { $have } で保留中。{ $waits }
# On a row held back: the one that waits.
store-upd-held-waits = 保留中。{ $waits }
# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = バージョン { $have } で保留中
# On a thing held back with no update waiting.
store-upd-held = 保留中
# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = バージョン { $have } で保留中です。新しいものはまだありません。
# On the Held back card: nothing newer waits.
store-upd-held-nothing = 保留中。新しいものはまだありません
# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
       *[other] 更新 { $n } 件
    }
# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
       *[other] 更新 { $n } 件の準備ができました
    }
# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }、合計 { $size }
# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
       *[other] { $n } 件は保留中
    }
# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }。{ $held }
# A line made into a sentence.
store-upd-sentence = { $text }。
# Two sentences one after the other.
store-upd-and-then = { $first } { $second }
# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = ファームウェアの更新を終えるには、再起動してください。
# Headline while the check runs.
store-upd-checking = 更新を確認しています。
# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = { $name } を更新しています。インストール中も作業を続けられます。
# Headline while all the updates go in.
store-upd-installing-all = 更新をインストールしています。インストール中も作業を続けられます。
# Headline when the updates are in and the computer must start again.
store-upd-restart = 更新のインストールを終えるには、コンピューターを再起動してください。
# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = インストールを終えるには、再起動してください。
# Added to the headline with no internet.
store-upd-no-more = このコンピューターはインターネットにつながっていないため、Trindle は新しい更新を探せません。
# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = 準備ができた更新はありません
# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Trindle が知っている待機中の更新はありません。このコンピューターがインターネットにつながっていない間は、新しい更新を探せません。
# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = アプリ
# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle と Debian
# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = ファームウェア
# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Trindle は { $names } の更新を確認できませんでした。
# Headline when everything is up to date.
store-upd-current = すべて最新です。
# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = 最後の確認: { $time }。
# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = 更新の自動確認はオフです。
# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = 未確認
# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = インターネットがないと確認できません
# Red words under a card when a check took too long.
store-upd-too-long = 確認に時間がかかりすぎました。
# Tip of the round button that checks again.
store-upd-check-again = もう一度確認
# Main button while the check runs.
store-upd-checking-main = 確認中
# Main button while the updates go in.
store-upd-installing-main = インストール中
# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
       *[other] 更新 { $n } 件をインストール
    }
# Main button that restarts the computer.
store-upd-restart-now = 今すぐ再起動
# Beside Restart now: the box that puts the restart off, then the ring's Updates button reminds. Its answers are the choices below.
store-upd-remind-later = あとで通知
# Red words under the headline when the Remind me later time could not be saved; the reason follows.
store-upd-remind-not-saved = リマインダーを保存できませんでした: { $why }。
# One answer in the Remind me later box.
store-upd-remind-hours =
    { $n ->
       *[other] { $n }時間後
    }
# Main button that checks for updates.
store-upd-check-for = 更新を確認
# Tip of the round button that releases a thing held back.
store-upd-release = { $name } の更新を許可
# Tip of the round button that holds a thing's updates back.
store-upd-hold = { $name } を保留
# Tip of the round button that ends an update that is running.
store-upd-stop = 停止
# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = 停止中
# Under a row whose update was stopped.
store-upd-stopped = 停止しました。続けるには「更新」を押してください。
# Under the Apps card when Install all was stopped.
store-upd-stopped-all = 停止しました。続けるには、もう一度インストールしてください。
# A card's state while it is being checked.
store-upd-status-checking = 確認中
# A card's state while its updates go in.
store-upd-status-installing = インストール中
# A card's state when its check failed; the reason follows.
store-upd-status-failed = 確認できませんでした。{ $why }
# A card's state: how many updates, and how big.
store-upd-status-size = { $count }、{ $size }
# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }。{ $held } 件は保留中
# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } 件は保留中
# A card's state when nothing waits.
store-upd-up-to-date = 最新です
# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = 入手元: { $from }、{ $change }
# On a row of the Updates page: where an app comes from.
store-upd-from = 入手元: { $from }
# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = 共有パーツ、入手元: { $from }
# Note under the Apps card.
store-upd-steam-restarts = Steam は、ゲームが動いていないときに再起動します。
store-upd-yours-alone = これらのアプリはあなた専用なので、パスワードなしで更新されます。アプリを更新すると、必要な共有パーツも一緒にインストールされます。
# Note under the Apps card: what holding back does.
store-upd-hold-why = 保留した更新は、今のバージョンのままです。再び許可するまで、ここでもリングでも、すべての更新をインストールするときに除かれます。
# Heading of the Apps card.
store-upd-card-apps = アプリ
# Plugins card: every plugin is up to date.
store-upd-plugins-current = すべてのプラグインが最新です。
# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
       *[other] { $n } 件のプラグインの更新が保留中です。
    }
# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
       *[other] { $n } 件のプラグインに更新があります
    }
# Heading of the Plugins card.
store-upd-card-plugins = プラグイン
# Held back card: nothing is held back.
store-upd-nothing-held = 保留中のものはありません。
# On a row of the Held back card for a thing released while the page is open.
store-upd-released = 保留を解除しました
# Note under the Held back card: what releasing does.
store-upd-release-why = 更新を再び許可すると、ここでもリングでも、ほかの更新と一緒にインストールされます。
# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = デバイスのファームウェア
# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = デバイス { $id } のファームウェア
# Heading of the Held back card.
store-upd-card-held = 保留中
# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } から { $to }
# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
       *[other] 更新 { $n } 件、まとめてインストール
    }
# Under Debian's rows: how many of them show.
store-upd-debian-showing = Debian の更新 { $total } 件のうち { $shown } 件を表示しています。
# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = このコンピューターには Trindle { $version } が入っています。
# Heading of the card about the daily security updates.
store-upd-card-security = セキュリティ更新
# On the security card: its state.
store-upd-security-on = オン、毎日
# On the security card: that holding back never stops these.
store-upd-never-held = 更新を保留しても、セキュリティ更新は止まりません。
# Firmware card: the computer has no updater.
store-upd-fw-none = このコンピューターにはファームウェアの更新ツールがありません。
# Firmware card: restart to finish.
store-upd-fw-restart = 再起動して完了
# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = バージョン { $version }
# Note on the firmware card when the updater is there.
store-upd-fw-note = 新しいファームウェアは、コンピューターの再起動時にインストールされます。
# Note on the firmware card when the updater is missing.
store-upd-fw-install = ファームウェアを最新に保つには、fwupd をインストールしてください。
# Heading of the Firmware card.
store-upd-card-firmware = ファームウェア
# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd は最新の一覧を取得できませんでした。
# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub に接続できませんでした。
# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Trindle は Debian の一覧を取得できませんでした。
# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Trindle は Debian の一覧を取得できませんでした。{ $why }
# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = 保留中の更新の一覧を保存できなかったため、Trindle は保留できませんでした: { $why }。
# The same, for a release.
store-upd-not-released-list = 保留中の更新の一覧を保存できなかったため、Trindle は更新を再び許可できませんでした: { $why }。
# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = アプリでは保留中ですが、Trindle は Flatpak 側を変更できませんでした。アプリを更新する別のプログラムが、インストールする可能性があります。
# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = アプリでは更新を再び許可しましたが、Trindle は Flatpak 側を変更できなかったため、Flatpak が除いたままの可能性があります。いったん保留してから、もう一度更新を許可してください。
# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak のメッセージ: { $why }
# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle

# Said when a security fix for a boxed app can't be downloaded. { $package } is the package, { $version } the fixed version.
store-patch-not-fetched = Trindle は { $package } { $version } をダウンロードできませんでした。
# Said when a security fix for a boxed app was downloaded but isn't there to use.
store-patch-not-found = Trindle はダウンロードしたパッケージを見つけられませんでした。

