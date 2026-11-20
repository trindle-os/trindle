# machine translated (Japanese), not yet reviewed by a person.

# Settings: the Your plugins page (src/settings/panels/plugins.rs).

## The Your plugins page (src/settings/panels/plugins.rs)

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = 自分のプラグイン
# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = プラグイン 追加機能 拡張機能 インストール済み 削除 設定 plugins add-ons extensions installed remove settings
# Help line at the foot of the Your plugins page.
settings-plugins-help = プラグインは Trindle に機能を足します。リングのアイコン、ここの設定、通知、ログイン時に始まるものなどを足せます。プラグインはそれぞれ専用の箱の中で動き、「権限」で許可するまで何も使えません。ページ題名の横のボタンで「権限」が開きます。プラグインを削除するには、カードのごみ箱を 2 回押してください。
# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = プラグインが使えるものを尋ねるウィンドウを、Trindle は開けませんでした。
# Under About it: the plugin's version number is { $v }.
settings-plugins-version = バージョン { $v }。
# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = バージョン { $v }。{ $settled }
# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } は何も求めていないため、何も許可されないまま、専用の箱の中で動きます。
# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
settings-plugins-asks-for =
    { $asks ->
       *[other] { $name } は { $list } を求めています。{ $asks } 件のうち { $allowed } 件を許可しています。許可したものだけを使い、専用の箱の中で動きます。
    }
# Heading of the line about a plugin on its card.
settings-plugins-about-it = 説明
# Tip of the button that changes what a plugin may use.
settings-plugins-change = 使えるものを変更
# Tip of the button that removes a plugin.
settings-plugins-remove = このプラグインを削除
# Red words when a plugin could not be removed.
settings-plugins-not-removed = 削除できませんでした。
# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = ここに入力
# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } には、専用の設定がありません。
# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } には、今表示できる設定がありません。
# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] インストールされているプラグインはありません。
       *[other] { $n } 個のプラグインがインストールされています: { $list }。
    }
# Heading of the card shown when no plugin is installed.
settings-plugins-group = プラグイン
# Heading of the line shown when no plugin is installed.
settings-plugins-none = インストールされていません
# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = 権限を開く

## Cards drawn from a plugin's rows file (src/settings/panels/plugin_rows.rs)

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = { $name } に問い合わせています…
# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } が応答しません。サービスが止まっている可能性があります。
# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = 保存しました。
# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } がその変更を受け付けませんでした。
# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } はそれを実行できませんでした。
# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } はそれを実行できませんでした: { $why }。
