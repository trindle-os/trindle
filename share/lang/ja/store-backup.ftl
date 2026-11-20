# machine translated (Japanese), not yet reviewed by a person.

# Apps: moving apps and settings to another computer, and the reasons a backup was refused (src/store/refusal.rs, backup.rs, put_back.rs, flat.rs).

## Why a backup was refused (src/store/refusal.rs)

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = { $why }ため、何も元に戻しませんでした。
# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = { $why }ため、Trindle はバックアップの一部だけを元に戻しました。
# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = バックアップに、戻す先のフォルダの外につながるリンクなどが入っている
# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = バックアップの一部を戻す場所に、このコンピューターのものがすでにある
# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = バックアップを展開する場所がない
# The reason when the file is not a Trindle backup.
store-because-not-trindle = そのファイルは Trindle のバックアップではない
# The reason when the backup's list is damaged or too big.
store-because-damaged-list = バックアップが壊れていて、Trindle が一覧を読み取れなかった
# The reason when the file is not a Flat backup.
store-because-not-flat = そのファイルは Flat のバックアップではないようだ
# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = そのファイルに Flat のアプリ一覧がない
# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = Trindle が読み取れない新しい版の Flat で作られている
# The reason when the file holds no apps.
store-because-no-apps = そのファイルにアプリがない
# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = バックアップを読み取れなかった：{ $why }
# The reason when the program gave none.
store-because-none = 理由が示されなかった
# The reason when the system said permission was denied.
store-because-permission = Trindle にはそこへ書き込む許可がない
# The reason when the disk is full.
store-because-space = このコンピューターに空きがない
# The reason when a part of the computer is read-only.
store-because-read-only = このコンピューターのその部分は読み取り専用である
# The reason when the person's disk quota is used up.
store-because-quota = このアカウントに許可された容量を使い切っている
# The reason when a file or program is missing.
store-because-missing = 必要なファイルまたはプログラムがない
# The reason when no reason could be read from the program's last line.
store-because-failed = 失敗した
# The reason when the backup has no name to be written under.
store-because-no-name = 名前がない
# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = バックアップ内のその設定が、アプリ自身のフォルダの外につながっている
# The reason when something is too big to be read whole.
store-because-too-big = 大きすぎて読み込めない
# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = バックアップ内のその設定が壊れているか、変更されている
# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = アカウントの画像以外はすべて元に戻しました。{ $why }
# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } アカウントの画像も元に戻せませんでした。{ $why }
# Said when the pkexec program could not be started.
store-refusal-pkexec = パスワードを求められませんでした：{ $why }
# Said when the backup could not be read at all.
store-refusal-not-read-from = Trindle はバックアップから読み取れませんでした：{ $why }。

## Making and putting back a backup (src/store/backup.rs)

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = { $why }ため、Trindle はバックアップを作りませんでした。
# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = まだありません
# Beside the size of a part of the backup: about how big it is.
store-backup-about = 約 { $size }
# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Trindle の設定と AppImage を元に戻しています…
# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Trindle は設定とデータを元に戻しましたが、Flatpak が次のアプリのインストールを断りました：{ $failed }。
# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Trindle は設定とデータを元に戻しましたが、次のアプリはインストールしませんでした：{ $refused }。

## Restoring from a Flat backup (src/store/flat.rs)

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
       *[other] 名前が正しくない { $n } 個のアプリを、Trindle は飛ばしました。
    }
# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = { $name } のソースは暗号化されていないため、Trindle は使いませんでした
# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = { $name } のソースはこのコンピューターになく、バックアップにも場所が書かれていません
# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = { $name } の設定（バックアップ内の設定がチェックに通らなかったため）
# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name }（ソースの名前が正しくないため）
# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what }（{ $why }ため）
# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name }（Flatpak がインストールできませんでした：{ $why }）
# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = すべて（バックアップを展開する場所がないため）
# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = すべてのアプリの権限
# Said among what could not be done when a backup's permissions asked for more than the app had: only ones that take access away are put back.
store-flat-grants-left-out = { $what }：Trindle は、アクセスを減らす権限だけを元に戻しました。
# Said among what could not be done: one app's settings.
store-flat-app-settings = { $name } の設定
# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }。以前あったものは { $path } として残してあります：{ $error }
# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = { $name } をインストールしています（{ $at } / { $total }）…
# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = { $name } の設定を元に戻しています（{ $at } / { $total }）…
# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
       *[other] { $n } 個のアプリをインストールしました。
    }
# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings =
    { $n ->
       *[other] { $n } 個のアプリの設定を元に戻しました。
    }
# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
       *[other] { $n } 個のアプリはすでにインストール済みでした。
    }
# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = 除外：{ $list }
