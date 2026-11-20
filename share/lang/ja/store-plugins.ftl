# machine translated (Japanese), not yet reviewed by a person.

# Apps: the Plugins page (src/store/plugin_page.rs).

## The Plugins page (src/store/plugin_page.rs)

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = 設定を開く
# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = 失敗しました: { $why }
# Said on a plugin's row when its update asks for a new permission and so waits for the person.
store-plugins-waits-for-you = この更新は新しい許可が必要なので、あなたが許可するのを待っています。
# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = { $new } の準備ができました。今は { $mine } です。
# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
store-plugins-waiting-more = 今はバージョン { $mine } です。バージョン { $new } は、さらに次の許可が必要です: { $more }。{ $waits }
# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = 終わると、ボタンが戻ります。
# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = プラグインを探しています…
# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = プラグインは入っていません。入れられるものもまだありません。
# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
       *[other] プラグインはまだ入っていません。{ $n } 個を入れられます。
    }
# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
       *[other] { $n } 個のプラグインが入っています
    }
# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
       *[other] { $n } 個に更新があります
    }
# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
       *[other] { $n } 個の更新を止めています
    }
# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }。{ $held }。
# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }。
# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }。すべて最新です。
# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }。そして { $have }。
# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }。そして { $have }。{ $held }。
# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = 何も求めません。
# On a plugin's card: what it asks for.
store-plugins-asks = 求めるもの: { $list }。
# Title of the Plugins page.
store-plugins-title = プラグイン
# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = バージョン { $mine } が入っています。バージョン { $new } の準備ができました。
# On a plugin's card: its installed version.
store-plugins-version-installed = バージョン { $version } が入っています。
# On a plugin's card: the version on offer.
store-plugins-version-offered = バージョン { $version }。
# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }。
# Tip of the round button that installs a plugin.
store-plugins-install = { $name } を入れる
# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = { $name } の更新を許可する
# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = { $version } に更新する
# Heading of the card that says where plugins come from.
store-plugins-where = プラグインの入手元
# Information on the card that says where plugins come from.
store-plugins-come-from = プラグインは、Trindle自身の一覧からだけ入手できます。Trindleは、このページを開くたびに、trindle.orgからその一覧を読みます。プラグインを一覧に応募することもできます。一覧に載る前に、1つずつ試験されます。
# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = 入りましたが、バックグラウンドで動く部分が動いていません: { $why }
# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = 取り除けませんでした: { $why }
