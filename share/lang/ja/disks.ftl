# machine translated (Japanese), not yet reviewed by a person.

# Words of Drives (trindle-drives), the window that shows the drives in or plugged into this computer and changes them: the page, the cards, the floating boxes of changes, and how every change ends. Sizes such as 1.5 GB, percentages and the names of file systems (exFAT, ext4, NTFS, FAT) are written by the program and passed in.

## The page (src/disks/mod.rs)

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = ドライブ

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = このページについて

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = このコンピューターの中や、つないだドライブをすべて表示します。各ドライブはパーティションごとに分けた棒で表します。棒の幅はパーティションの大きさ、色の濃さは使用量を示すので、いっぱいのものがすぐ分かります。

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = 図の下に、ドライブごとのカードがあります。パーティションを開く、マウントやアンマウントをする、ドライブを安全に取り外す、電源を切る、などができます。パーティションの鉛筆では、名前の変更、フォルダーの選択、起動時のマウント、サイズ変更、チェック、修復、フォーマット、削除ができます。空き領域の＋ボタンで新しいパーティションを作れます。

# Third paragraph of the longer words that open from the i beside the title.
disks-help-3 = システムが動いているドライブは見られますが、データが消えたり動いたりする変更はできません。どのドライブも、変更のたびに管理者のパスワードが必要です。目のボタンを押すと、ドライブやパーティションを Folders とリングから隠せます。隠しても、ここには薄く表示され、ロックはかかりません。

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = お使いのドライブ

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = 今はドライブを読み取れません。

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = もう一度試す

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = もう一度調べる

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = ドライブが見つかりません。ドライブをつなぐと、ここに表示されます。

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = ドライブサービス

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = システムバス

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } を使えません。インストールするには、次を実行してください: sudo apt install { $package }

## The sentence at the top (src/disks/mod.rs)

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = ドライブを調べています…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } が故障しかけています。残したいファイルを今すぐ別のドライブにコピーしてください。

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } を確認してください: { $troubles }。

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] ドライブとその空き容量です。
       *[other] ドライブとその空き容量です。つないだドライブは { $plugged } 台です。
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] このコンピューターはほぼいっぱいで、空きは { $free } だけです。早めにファイルを削除するか移動してください。
       *[other] このコンピューターはほぼいっぱいで、空きは { $free } だけです。つないだドライブは { $plugged } 台です。早めにファイルを削除するか移動してください。
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] このコンピューターは残り少なくなってきました。空きは { $free } です。
       *[other] このコンピューターは残り少なくなってきました。空きは { $free } です。つないだドライブは { $plugged } 台です。
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] このコンピューターには十分な空きがあります。空きは { $free } です。
       *[other] このコンピューターには十分な空きがあります。空きは { $free } です。つないだドライブは { $plugged } 台です。
    }

## How full the places are (src/disks/mod.rs)

# Title of the card with the ring for the disk the system is on.
disks-ring-system = このコンピューター

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = あなたのファイル

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] システムのディスクの空きは { $free } です。
       *[files] あなたのファイル用の空きは { $free } です。
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } 使用、{ $whole } 中 { $free } 空き

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = / { $whole }

## The drives together (src/disks/mod.rs)

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = ドライブからの健康状態の読み取りはまだありません

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = ドライブ自身のチェックで正常です

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
       *[other] { $n } 台すべてがドライブ自身のチェックに合格しました
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
       *[other] { $n } 台に確認が必要です。カードを見てください
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
       *[other] ドライブ { $n } 台、合計 { $total }。{ $health }。
    }

## The map of the drives (src/disks/mod.rs)

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = 空き { $size }

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = マウントされていません

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = 空き領域

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = ディスクやカードがありません

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = まだ何もありません

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }、USB または取り外し可能

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }、コンピューター内蔵

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }、USB または取り外し可能、非表示

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }、コンピューター内蔵、非表示

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }、{ $full } 使用、{ $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }、{ $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }、{ $words }: { $parts }。

## Parts and drives (src/disks/mod.rs)

# Name given to a part that has no label and no device name.
disks-part-name = パーティション

# Name given to a drive that gives no maker or model.
disks-drive-name = ドライブ

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }、{ $size }

# Kind of a part that is swap space.
disks-kind-swap = スワップ

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = 暗号化、ロック解除済み

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = 暗号化、ロック中

# Kind of a part that holds other parts.
disks-kind-container = パーティションを含む

# Kind of a part that has no filesystem on it.
disks-kind-none = ファイルシステムなし

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }、EFI、コンピューターの起動用パーティション
        [legacy] { $kind }、古いコンピューターの起動用パーティション
       *[loader] { $kind }、ブートローダー専用のパーティション
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = どのコンピューターでも

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = Linux 専用

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows 用

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = 古い機器用

## Health (src/disks/plan.rs and src/disks/mod.rs)

# Trouble a drive reports about its spare space.
disks-trouble-spare = 予備の領域が残りわずか

# Trouble a drive reports about its heat.
disks-trouble-temperature = 温度が高すぎる

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = 劣化している

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = 読み取り専用になった

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = メモリのバックアップが故障した

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = 一部が読み取り専用になった

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = 想定寿命を超えて劣化している

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
       *[other] 損傷箇所 { $n } か所
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
       *[other] 使用 { $hours } 時間
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = 故障しかけています。残したいものを今すぐコピーしてください

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = 要注意: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = 正常

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }、{ $readings }

## The card of a drive (src/disks/mod.rs)

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = 応答なし

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = ドライブのケースはつながっていますが、中のドライブが応答していません。いったん抜いて 10 秒待ち、付属のケーブルで青い USB ポートか USB-C ポートにつなぎ直してください。それでも変わらなければ、中のドライブが外れたか故障した可能性があります。

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = 安全に取り外す

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = 電源を切る

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
disks-erase-whole = ドライブ全体を消去

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] このドライブを表示
       *[partition] このパーティションを表示
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] このドライブを隠す。{ $lock }
       *[partition] このパーティションを隠す。{ $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Folders とリングから隠しています。{ $lock }

# Note on the card of the drive the system runs from.
disks-system-note = システムが動いているので、ここでは何も消去できません。

# Note on the card of a drive that has no partition table.
disks-no-table = このドライブにはパーティションテーブルがないので、全体が空きです。

## The rows of parts (src/disks/mod.rs)

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = マウントされていません

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = 起動時に { $dir } にマウント

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = { $dir } に表示

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = { $dir } 内

# Tip of the round button that opens the folder a part is mounted in.
disks-open = 開く

# Tip of the round button that mounts a part.
disks-mount = マウント

# Tip of the round button that unmounts a part.
disks-unmount = アンマウント

# Tip of the round button on a part that opens the box of changes.
disks-change-it = 変更する

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = 名前

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = 名前 (省略可)

# Tip of the button that saves a part's new name.
disks-rename-tip = 名前を変更

# Note under the Name row of a part.
disks-name-note = ここや他のコンピューターで表示される名前です。

# Placeholder of the typing box for a part's folder.
disks-folder-name = フォルダー名

# Tip of the button that saves a part's folder.
disks-use-folder = このフォルダーを使う

# Label of the row where a part's folder is chosen.
disks-folder = フォルダー

# Note under the Folder row of a part.
disks-folder-note = ファイルが表示される場所です。空にするとコンピューターが選びます。

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = 起動時にマウント

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = コンピューターを起動するたびにマウントします。見つからなくても、コンピューターは起動します。

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = サイズ (GB)

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = { $least } から { $most } まで。

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = { $least } から { $most } まで。どこまで小さくできるかは、マウントすると分かります。

# Tip of the button that resizes a part.
disks-resize-tip = サイズを変更

# Label of the row where a part is resized.
disks-size = サイズ

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
disks-format-erases = { $words } のデータがすべて消えます。元に戻せません。

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning = もう一度押すと、{ $drive } の { $name } を { $from } から { $to } に変更します

# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = もう一度押すと、{ $drive } の { $name } を { $from } から { $to } に変更します。縮小中に停電するとファイルを失うことがあるので、先にバックアップしてください。

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = { $name } の Windows は休止状態か高速スタートアップのままなので、小さくするとファイルを失うおそれがあります。Windows を起動して「再起動」を選び、もう一度試してください。

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = 新しいパーティション用の空き

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = 空きにするサイズ (GB)

# Tip of the button that makes room under a part that is in use.
disks-room-tip = 空きを作る

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
disks-room-warning = もう一度押すと、{ $drive } の { $name } から { $size } を空けます。先に Trindle がスナップショットを取ります。

# Note under the row that makes room.
disks-room-note = Trindle がスナップショットを取ってから、使用中のパーティションを小さくします。空いた領域は未使用のまま残ります。

# Tip of the button that checks a part for faults.
disks-check-tip = エラーをチェック

# Tip of the button that repairs a part.
disks-repair-tip = 修復

# Label of the row where a part is checked and repaired.
disks-faults = エラー

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
disks-repair-warning = もう一度押すと、{ $drive } の { $name } を修復します

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = 種類

# Label of the row where a part is formatted.
disks-format = フォーマット

# Tip of the button that formats a part.
disks-format-tip = フォーマット

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
disks-format-warning = もう一度押すと、{ $words } のデータをすべて消去します

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $drive } の { $name }、{ $size }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-delete-name = { $drive } の { $name }、{ $size } を削除

# Label of the row where a part is deleted.
disks-delete-label = パーティションを削除

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = ここに新しいパーティションを作る

# Label of the row where a new part is made.
disks-new-part = 新しいパーティション

# Tip of the button that makes a new part.
disks-make-tip = 作成

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = もう一度押すと、{ $drive } に { $size } の新しいパーティションを作ります

## The floating box of changes (src/disks/mod.rs)

# Title of the box for erasing a whole drive. { $name } is the drive.
disks-sheet-erase = { $name } を消去

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $drive }、{ $size }

# Title of the box for making a new part in free space.
disks-sheet-new = 新しいパーティション

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $drive }、空き { $size }

# Tip of the round button that closes the box of changes.
disks-close = 閉じる

## How a change goes (src/disks/mod.rs)

# Words on a row while a part is being mounted.
disks-mounting = マウント中

# Words on a row when a part has been mounted.
disks-mounted = マウントしました

# Words on a row while a part is being unmounted.
disks-unmounting = アンマウント中

# Words on a row when a part has been unmounted.
disks-unmounted = アンマウントしました

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = 取り外し準備中

# Words on a drive's row when it is safe to remove.
disks-safe-remove = 安全に取り外せます

# Words on a drive's row while it is being powered off.
disks-powering-off = 電源を切っています

# Words on a drive's row when it has been powered off.
disks-off-safe = 安全に抜けます

# Words on a row while a part is being renamed.
disks-renaming = 名前を変更中

# Words on a row when a part has been renamed.
disks-renamed = 名前を変更しました

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = フォルダーは自動で選ばれます

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = { $dir } に表示されます

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = 起動時に { $dir } にマウントします

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = 起動時にマウントしません

# Words on a row while a folder or start-up choice is being saved.
disks-saving = 保存中

# Words on a row while a part is being resized.
disks-resizing = サイズを変更中

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = { $size } に変更しました

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Trindle がファイルを見つけられないため、空きを作れません。

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = スナップショットを取っています

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Trindle は、あとでスナップショットを見つけられませんでした。

# Red words when the snapshot, or switching off the swap file, before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } サイズは変更していません。

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Trindle がスナップショットを取っている間に変化があったため、サイズは変更していません。

# Words on a row while room is being made.
disks-making-room = 空きを作っています

# Words on a row while the swap file is switched off before room is made.
disks-swap-off-doing = スワップファイルをオフにしています

# Words on a row while the swap file is switched back on after room is made.
disks-swap-on-doing = スワップファイルをオンに戻しています

# Red words when the swap file couldn't be switched back on after making room. { $done } is how making room went, { $why } why the swap file is still off, both as sentences.
disks-swap-still-off = { $done } スワップファイルは再起動するまでオフのままです。{ $why }

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } の空きを作りました

# Words on a row while a part is being repaired.
disks-repairing = 修復中

# Words on a row when a part has been repaired.
disks-repaired = 修復しました

# Red words when some faults could not be put right.
disks-repair-failed = Trindle はエラーの一部を修復できませんでした。残したいファイルを別のドライブにコピーしてください。

# Words on a row while a part is being checked.
disks-checking = チェック中

# Words on a row when the check found no faults.
disks-no-faults = エラーは見つかりませんでした

# Red words when the check found faults.
disks-faults-found = エラーが見つかりました。レンチのボタンを押すと修復します。

# Words on a row while a part or drive is being formatted.
disks-formatting = フォーマット中

# Words on a row when a part has been formatted.
disks-formatted = フォーマットしました

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = ドライブ全体の消去

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = 新しいパーティションが見つかりません。

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = ドライブサービスに接続できませんでした。

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Trindle は新しいパーティションの識別子を読み取れませんでした。

# Words on a row while a part is being deleted.
disks-deleting = 削除中

# Words on a row when a part has been deleted.
disks-deleted = 削除しました

# Words on a row while a new part is being made.
disks-making-it = 作成中

# Words on a row when a new part has been made.
disks-made = 作成しました

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = 保存できませんでした: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

## Decisions about names and sizes (src/disks/plan.rs)

# Red words under the folder name box when it is empty.
disks-name-empty = 先に名前を付けてください。

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
       *[other] 名前が長すぎます。{ $max } 文字以内にしてください。
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = 使えるのは、英字、数字、- と _ だけです (スペースは使えません)。

# Red words under the folder name box when it starts with a dash.
disks-name-start = 文字か数字で始めてください。

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } はすでに使われています。

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
       *[other] このパーティションの名前としては長すぎます。{ $max } 文字までにしてください。
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = FAT の名前には * ? . , ; : / \ | + = < > [ ] " は使えません。

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] 先にマウントしてください。この種類のパーティションは、マウント中だけ大きくできます。
       *[smaller] 先にマウントしてください。この種類のパーティションは、マウント中だけ小さくできます。
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] この種類は大きくできません。
       *[smaller] この種類は小さくできません。
    }

## What a change says and what became of it (src/disks/plan.rs)

# Red words when the password box was closed with no password. Said whole, as a sentence.
disks-no-password = パスワードが入力されなかったため、何も変更しませんでした。

# Red words when the password box was closed with no password, after some steps of a change were already done.
disks-no-password-part-way = パスワードが入力されなかったため、作業が途中で止まりました。

# Red words when another change is pressed while one is still being made.
disks-change-going = 変更を実行中です。
disks-one-at-a-time = 別の変更を実行中です。終わってからもう一度試してください。

# Red words when it could not be made sure that the password would be asked for.
disks-not-sure = パスワードが求められることを Trindle が確認できなかったため、何も変更しませんでした。

# Red words after how a change ended, when the password just given could not be taken back.
disks-standing = Trindle は今入力されたパスワードを取り消せませんでした。数分間、ドライブの別の変更でパスワードが求められない可能性があります。Trindle は再試行しています。

# Words after how a change ended, when the password just given has been taken back after all.
disks-taken-back = 入力されたパスワードは取り消されたので、次の変更ではパスワードが求められます。

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Trindle がドライブサービスに接続できなかったため、何も変更しませんでした。

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = この変更の実行中にドライブが閉じられました。

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = 結果を確認できるよう、これが終わるまでドライブは開いたままになります。

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }ため、何も変更しませんでした。

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }ため、作業が途中で止まりました。

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = ドライブサービスに問い合わせられなかった

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } 作業が途中で止まりました。

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }。

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = 完了: { $steps }。

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = 失敗: { $steps }。

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = 未実行: { $steps }。

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = 不明: { $steps }。

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } はこの変更のためにマウント解除され、今はマウントされていません。

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } はこの変更のためにマウント解除され、まだマウント解除のままです。その行からマウントしてください。

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } はマウント解除された可能性があります。その行を確認してください。

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = { $dev } はこの変更のためにマウント解除されました。その行からもう一度マウントし、求められたらパスワードを入力してください。

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } はこの変更のためにマウント解除され、再マウントできませんでした({ $why })。その行からマウントしてください。

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = フォーマットは完了しましたが、Trindle はフォルダー { $dir } を再び割り当てられませんでした。{ $why } その行からフォルダーをもう一度選んでください。

# Red words when a drive is in use and cannot be unmounted.
disks-busy = まだ使用中です。そのファイルを表示しているウィンドウやプログラムを閉じて、もう一度試してください。

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = そのサイズでは、すでにあるファイルに対して小さすぎます。

## Steps of a change, in the account of one that stopped part-way (src/disks/plan.rs)

# Step of a change. { $dev } is the part.
disks-step-unmount = { $dev } のマウント解除

# Step of a change. { $dev } is the part.
disks-step-mount = { $dev } の再マウント

# Step of a change.
disks-step-eject = ドライブを取り外せる状態にする

# Step of a change.
disks-step-power-off = ドライブの電源を切る

# Step of a change. { $dev } is the part.
disks-step-format = { $dev } のフォーマット

# Step of a change.
disks-step-create = パーティションの作成

# Step of a change. { $dev } is the part.
disks-step-delete = { $dev } の削除

# Step of a change. { $dev } is the part.
disks-step-resize-part = { $dev } のサイズ変更

# Step of a change. { $dev } is the part.
disks-step-resize = { $dev } の中身のサイズ変更

# Step of a change. { $dev } is the part.
disks-step-repair = { $dev } の修復

# Step of a change. { $dev } is the part.
disks-step-check = { $dev } のチェック

# Step of a change. { $dev } is the part.
disks-step-rename = { $dev } の名前変更

# Step of a change. { $dev } is the part.
disks-step-fstab = { $dev } のフォルダーと起動時の設定の保存

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = { $dev } での { $method } のステップ

## Making room under a part that is in use (src/disks/room.rs)

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = この { $kind } パーティションは使用中でないときだけ小さくできるため、Trindle はここから空きを作れません。

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = xfs パーティションは小さくできません。

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = この { $kind } パーティションは、使用中は小さくできません。

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Trindle がスナップショットを取れるのはシステム自身のディスクだけなので、ここでは空きを作れません。

# Red words under the Make room row.
disks-room-not-in-table = ドライブのパーティションテーブルにあるパーティションではないため、Trindle はここから空きを作れません。

# Red words under the Make room row.
disks-room-no-online-shrink = ドライブサービスは、ここで使用中の Btrfs を小さくできないため、Trindle は空きを作れません。

# Red words under the Make room row.
disks-room-holds-parts = 中に他のパーティションがあるため、Trindle はここから空きを作れません。

# Red words under the Make room row.
disks-room-not-gpt = ドライブが GPT パーティションテーブルではないため、Trindle はここから空きを作れません。

# Red words under the Make room row.
disks-room-mount-first = 先にマウントしてください。Btrfs はマウント中に小さくします。

# Red words under the Make room row.
disks-room-mounts-not-one = マウントされていないため、Trindle は空きを作れません。

# Red words under the Make room row.
disks-room-read-only = 読み取り専用でマウントされているため、Trindle は空きを作れません。

# Red words under the Make room row.
disks-room-read-only-unknown = 読み取り専用かどうかを Trindle が判断できなかったため、空きを作れません。

# Red words under the Make room row.
disks-room-many-drives = ファイルが複数のドライブにまたがっているため、Trindle は空きを作れません。

# Red words under the Make room row.
disks-room-drives-unknown = ファイルがいくつのドライブにまたがっているかを Trindle が判断できなかったため、空きを作れません。

# Red words under the Make room row.
disks-room-swap-unknown = スワップファイルがあるかどうかを Trindle が判断できなかったため、空きを作れません。

# Red words under the Make room row.
disks-room-used-unknown = どれだけ使われているかを Trindle が読み取れなかったため、空きを作れません。

# Red words under the Make room row.
disks-room-area-unknown = ドライブサービスがファイルシステムの大きさを返さないため、Trindle はチェックも空きの作成もできません。

# Red words under the Make room row.
disks-room-open-missing = ロックを解除したコピーを Trindle が見つけられなかったため、空きを作れません。

# Red words under the Make room row.
disks-room-encryption-sizes = 暗号化のサイズが一致しないため、Trindle は空きを作れません。

# Red words under the Make room row.
disks-room-sizes = 指定されたサイズが一致しないため、Trindle は空きを作れません。

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = { $used } が使われていて、それより多く残す必要があるため、空けられる容量がありません。

# Red words under the Make room row.
disks-room-sizes-unread = サイズを読み取れませんでした。

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = 作れる空きは { $least } から { $most } までです。

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }ため、Trindle は中止しました。

# What was found wrong while making room.
disks-room-check-unmounted = ファイルがもうマウントされていない

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = パーティションは { $is } ですが、{ $should } であるはずです

# What was found wrong while making room.
disks-room-check-open = ロックを解除した暗号化のコピーが、あるべきサイズになっていない

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = ファイルシステムは { $is } ですが、{ $should } であるはずです

# What was found wrong while making room.
disks-room-check-area-small = ファイルシステムが、使用中の量より小さい

# What was found wrong while making room.
disks-room-check-fit = ファイルシステムがもう収まらない

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [partition] ドライブサービスの一覧にパーティションがなくなったため、作業を中止しました。
        [copy] ドライブサービスの一覧にロックを解除したコピーがなくなったため、作業を中止しました。
       *[files] ドライブサービスの一覧にファイルシステムがなくなったため、作業を中止しました。
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = ファイルシステムを小さくする

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = ロックを解除した暗号化のコピーを小さくする

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = パーティションを縮小する

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = サイズをチェックする

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = { $dev } の空きを作る前({ $at })

# Red words when the swap file is back on after making room, but Trindle met a problem doing it. { $done } is how making room went, { $why } the problem, both as sentences.
disks-swap-on-but = { $done } { $why }
