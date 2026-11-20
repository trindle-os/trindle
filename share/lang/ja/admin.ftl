# machine translated (Japanese), not yet reviewed by a person.

# Words of the administrator's helper (trindle-admin), which Settings and Apps show when a step behind the administrator's password fails or is refused. The 'usage:' lines it prints to its maker, and the words it hands to other programs to read, are not here.

## Accounts and passwords (src/admin/mod.rs)

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = ログイン名として使えません。

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = ログイン名が長すぎます。

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = { $name } というユーザーはいません。

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } はシステムのアカウントなので、Trindle は変更しませんでした。

# Red words in Settings when no password reached the helper that sets it.
admin-password-none = パスワードが入力されていません。

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
admin-password-short =
    { $n ->
       *[other] パスワードは { $n } 文字以上にしてください。
    }

# Red words in Settings when a new password has a line break in it.
admin-password-line-break = パスワードに改行は使えません。

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } はすでにあります。

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } は唯一の管理者なので、管理者のままにする必要があります。

# Red words in Settings when a person tries to delete the account they are signed in to.
admin-delete-self = 自分のアカウントは削除できません。

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = Trindle は { $name } のアカウントを削除しましたが、ログイン画面の写真を削除できませんでした: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = その写真は大きすぎます。

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = その写真は Trindle が作ったものではありません。

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle はログイン画面から写真のフォルダーを使えるようにできませんでした: { $why }

## Language, computer name and charging (src/admin/mod.rs)

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = 言語の名前として使えません。

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle は Debian の言語一覧を読み込めませんでした: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Debian は言語 { $code } を知りません。

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = コンピューター名に使えるのは、英字、数字、ハイフンだけです。63 文字までで、ハイフンで始めることも終わることもできません。

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = 充電の上限は今はオンですが、コンピューターの電源を切ると失われます: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = 満充電は今はオンですが、次にコンピューターを起動すると上限が戻ります: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

## Sharing and plugins through the firewall (src/admin/mod.rs)

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = Trindle が共有をオフにできなかったため、ほかのコンピューターはまだ接続できます: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = プラグインの ID として使えません。

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = そのような人はいません。

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = その人のフォルダーがありません。

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Trindle は許可したプラグインの一覧を保存できませんでした: { $why }。

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = そのようなプラグインはありません。

## Updates, apps, firmware and Tailscale (src/admin/mod.rs)

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = 何も更新されませんでした。Trindle は自分のインストール済みバージョンを確認できず、自分の更新を除外できませんでした。

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = アップデートをインストールしています

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = 後片付けをしています

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Trindle をインストールしています

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = パッケージ名として使えません。

# Red words when the package to remove is not installed.
admin-package-not-installed = そのパッケージはインストールされていません。

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = fwupd が知らないデバイスです。

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Trindle は誰が Tailscale を使いたいのか判断できません。

## Blocking sites and tools it runs (src/admin/mod.rs)

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = ブロックが時間内に始まりませんでした。

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = ブロックは完全にはオフになっていません: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle は { $path } を元に戻せませんでした: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = サイト名として使えません。

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = そのようなサーバーはありません。

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } が必要なものをすべて受け取る前に止まったため、作業は終わっていません: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } は失敗しました。

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path } を削除できませんでした: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Trindle が { $path } を読み込めなかったため、何も変更していません: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle は元の状態に戻すこともできませんでした: { $problem }

## Earlier versions of the home folder (src/admin/home_snapshots.rs)

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = 以前の版を残す期間として使えません。

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = 以前の版が見つかりません。

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = 以前の版のフォルダーがあるはずの場所にファイルがあります。

## Shared folders on other computers (src/admin/shares.rs)

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = 誰が求めたのか判断できません。

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = これは pkexec 経由でのみ実行できます。

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = コンピューター名として使えません。

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = 共有フォルダーの名前として使えません。

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = コンピューターのアドレスとして使えません。

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = 共有フォルダーのパスとして使えません。

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = ここで閉じられるのは、自分の共有フォルダーだけです。

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = その場所で開いている共有フォルダーはありません。

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = Trindle が設定した後で何かが { $path } を変更したため、そこには何も接続されていません。

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } は Trindle が残した状態ではありません。

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = 開いている共有フォルダーが多すぎます。

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } がインストールされていません。Debian の { $package } パッケージに含まれています。

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = ログイン情報を読み込めませんでした。

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Trindle はこのコンピューターにマウントされているものを読み込めませんでした: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = プログラムがこのフォルダーの中をまだ使っています。プログラムを閉じてから、もう一度試してください。

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = そのコンピューターが応答しません。共有フォルダーはログアウトすると閉じます。

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = ユーザー ID として使えません。

## Sharing folders from this computer (src/admin/sharing.rs)

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = sambashare グループを作れませんでした。

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Samba を取得しています

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = 共有を設定しています

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = 共有はまだ設定されていません。先にフォルダーを共有してください。

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = このコンピューターはまだネットワーク上のほかの人に見えています: { $why }

## Snapshots of the system (src/admin/snapshots.rs)

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = スナップショットの番号として使えません。

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = 設定で作成

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle はシステムがどのディスクにあるか判断できませんでした。

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = システムが専用の Btrfs サブボリュームにないため、Trindle はスナップショットと入れ替えられません。

## Extra software sources and their programs (src/admin/sources.rs)

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle は { $url } から署名鍵を取得できませんでした。

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = そのアドレスは署名鍵を返しませんでした。

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Trindle が { $path } を読み込めなかったため、何も変更していません: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = 署名鍵を取得しています

# Second line of the progress while a source is added.
admin-step-adding-source = 追加して一覧を読み込んでいます

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = この入手元は使えませんでした。{ $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle はこの入手元を使えなかったため、削除しました。{ $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } はまだオンとオフを切り替えられません: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle は { $user } 用に起動できませんでした: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = Trindle は { $user } が仮想マシンを使えるようにできませんでした。Cowork に必要です: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
admin-source-docker = Trindle は { $user } がパスワードなしで使えるようにできませんでした: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } はインストールされましたが、Trindle は設定を終えられませんでした: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = インストールしています

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = 用意された入手元ではありません。

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } はこの種類のコンピューター ({ $arch }) 用ではありません。

# Last line of the progress while a program is installed.
admin-step-setting-up = 設定しています

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = アドレスは https:// で始まる普通のウェブアドレスにしてください。

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = スイートとコンポーネントは普通の単語にしてください。

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = パッケージ名として使えません。

# First line of the progress while a source and its program are taken away.
admin-step-removing = 削除しています

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = 一覧を読み込み直しています

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = ここで削除できるのは、Trindle が追加した入手元だけです。

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = 入手元はまだあります: { $why }

## Restoring a snapshot (src/admin/snapshots.rs)

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = スナップショット { $n } はディスクにありません。

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = Trindle がスナップショットを準備できなかったため、何も変更していません。{ $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Trindle は現在のシステムを脇に置けませんでした: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = 現在のシステムの名前が { $sub } ではなく { $aside } になっています。{ $sub } に戻るまで、コンピューターは起動しません: { $why }

# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = スナップショットは所定の場所にあり、コンピューターはそこから起動します。Trindle は古いシステムの後片付けを終えられませんでした。次にコンピューターを起動したときに、もう一度試します: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Trindle はスナップショットを移せませんでした: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Trindle はスナップショットを所定の場所に置けませんでした: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Trindle は脇に置いたシステムの一覧を更新できませんでした: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = { $names } を削除できませんでした

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = 何も必要としなくなったため、Trindle は次を削除しました: { $list }。

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = すべてのアップデートがインストールされたわけではありません。次は見送られました: { $list }。

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off. Keep "DKMS) will not build": Help finds its page by those words.
admin-driver-off-known = ドライバー ({ $driver }, DKMS) が新しい Linux カーネルではまだビルドできないため、{ $feature }はオフです。
admin-driver-off = ドライバー ({ $driver }, DKMS) が新しい Linux カーネルではまだビルドできません。必要な機能はオフです。
# Said once after the drivers that are off.
admin-driver-off-rest = カーネルとアップデートのそれ以外の部分はインストールされました。
# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = 仮想カメラ

admin-language-in-use = { $code } はこのコンピューターが起動する言語なので、削除できません。
# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = このコンピューターには、オフにできるスプリットロックの低速化はありません。
# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle はそのドライブを見つけられませんでした。または、システムのドライブです。
# The drive could not be closed where it was open.
admin-games-drive-busy = ドライブが使用中です。ドライブ上のファイルとプログラムを閉じてから、もう一度試してください: { $why }
# Another software change has the read-only system open.
admin-system-busy = ほかのソフトウェアの変更が進行中です。終わってからもう一度試してください。
# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Trindle がシステムのロックを解除できなかったため、何も変更していません: { $why }
# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle はシステムを再びロックできませんでした。次の起動時にロックされます: { $why }
# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle は /usr/local を書き込み可能なままにできませんでした: { $why }
# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = 実行できるのは root だけで、pkexec 経由はできません。
# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Btrfs が必要です。

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = 開けるポートは最大 { $most } 個です。

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle は開いているポートの一覧を保存できませんでした: { $why }。

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = ファイアウォールはまだオンです: { $why }
# Red line when a plugin now asks for other network ports than the ones the administrator's password was typed for.
admin-plugin-ports-changed = このプラグインは、許可したポートとは別のネットワークポートを求めています。開くには、もう一度許可してください。
# Red words when a program is asked to be set up for someone other than the person asking.
admin-source-not-you = プログラムを追加できるのは、求めた本人に対してだけです。
# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = 少し前に別の復元が同じ名前を使いました。数秒待ってから、もう一度試してください。

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = { $path } を切断できませんでした: { $error }
# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
       *[other] { $n } 個の共有フォルダーが応答しなかったため、まだ接続されています。コンピューターをシャットダウンするときに切断されます。
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = スワップファイル { $path } をオフにできませんでした: { $why }
# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = スワップファイル { $path } をオンに戻せませんでした: { $why }
# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } はシステムのスワップファイルではありません。
# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = ほかの人が { $why } を変更または読み取れるため、Trindle はスワップファイル { $path } を使いません。
# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = スワップファイルの今の場所が見つからなかったため、休止状態からの復帰が働かないかもしれません。
# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = 再起動すると、休止状態がまた使えるようになります。
# Red words in Settings, Hardware, when Use the chip again is pressed while it is already running.
admin-chip-busy = チップはすでに設定中です。終わるまで待ってください。
# Red words in Settings, Hardware, when the disk password typed for Use the chip again is wrong.
admin-chip-wrong-password = ディスクのパスワードではありません。
# Red words in Settings, Hardware, after several wrong disk passwords in a row.
admin-chip-wait = パスワードの間違いが多すぎます。1 分待ってから、もう一度試してください。
# Red words in Settings, Hardware, when the chip unlocked the disk at the last start after all.
admin-chip-not-needed = 前回の起動ではチップがディスクのロックを解除したので、することはありません。
# Red words in Settings, Hardware, when the disk is not the one the start-up screen noted.
admin-chip-changed = コンピューターの起動後にディスクが変わりました。再起動してから、もう一度試してください。
# Red words in Settings, Hardware, when the computer's TPM security chip can't be found.
admin-chip-none = セキュリティチップが見つかりませんでした。BIOS でオンにしてから、もう一度試してください。
# Red words in Settings, Hardware, when clevis, the tool that sets up the chip, is missing.
admin-chip-no-clevis = チップを設定するツール clevis がインストールされていません。
# Red words in Settings, Hardware, when the disk has no room left for another key.
admin-chip-no-slot = ディスクに、もう1つ鍵を入れる空きがありません。
# Red words in Settings, Hardware, when the chip couldn't be set up. The disk still opens with its password.
admin-chip-not-sealed = チップを設定できませんでした。ディスクは引き続きパスワードで開きます。
# Red words in Settings, Hardware, when the chip works again but its old key couldn't be removed.
admin-chip-old-kept = チップは再び使えますが、古い鍵を削除できませんでした。
