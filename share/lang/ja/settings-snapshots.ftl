# machine translated (Japanese), not yet reviewed by a person.

# Settings: the Snapshots page (src/settings/panels/snapshots.rs).

## The Snapshots page (src/settings/panels/snapshots.rs)

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = スナップショット
# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = スナップショット 復元 元に戻す ロールバック バックアップ 更新 壊れた btrfs snapper ブートメニュー grub 以前のバージョン 古いバージョン 削除したファイル 戻す 復旧 ホーム コピー snapshots snapshot restore undo rollback roll back backup update broke boot menu earlier versions older version previous deleted file put back recover home copies
# Help line at the foot of the Snapshots page.
settings-snap-help = スナップショットは、ソフトウェアを変える前後と、自分で取ったときの、システムのコピーです。スナップショットを復元すると、次に再起動したとき、システムがそのときの状態に戻ります。ホームフォルダがディスクの別の領域にある場合は、あなたのファイルはそのままです。Trindle のインストーラーはそのように設定します。コンピューターが起動しないときは、ブートメニューで Trindle のスナップショットを選んでください。「以前のバージョン」は、みんなのファイルを毎日コピーして残します。システムのスナップショットとは別です。
# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = ディスクがほぼいっぱいなので、Trindle はコピーを取りませんでした。
# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = 今日 { $time }
# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = 昨日 { $time }
# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = { $date }
# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = コンピューターを再起動できませんでした。代わりにリングから再起動してください。
# Said beside the Take a snapshot line while it works.
settings-snap-taking = スナップショットを取っています
# Said beside the set aside line while the space is freed.
settings-snap-freeing = 空けています…
# Said beside the set aside line once the space is freed.
settings-snap-freed = 空けました
# Said beside a snapshot while it is restored.
settings-snap-restoring = 復元しています
# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
settings-snap-restored = 次に再起動すると、システムはスナップショット { $n } の状態になります。そのあとに変えたものは、消えずに別に取ってあります。
# Said beside a snapshot while it is deleted.
settings-snap-deleting = 削除しています
# Said beside a snapshot once it is deleted.
settings-snap-deleted = 削除しました
# Said beside a setting while the administrator's password is awaited.
settings-snap-waiting = パスワードを待っています…
# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = コピーを毎日取ります
# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = これ以上コピーは取りません
# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = 保存しました
# Said beside the earlier copies while they are cleared.
settings-snap-clearing = 消去しています
# Said beside the earlier copies once they are cleared.
settings-snap-cleared = 消去しました
# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] コピーはまだありません
       *[other] コピー { $n } 個
    }
# Under the number of earlier copies when there are none.
settings-snap-first-soon = 最初のコピーは1日以内に取ります。
# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = 使用量は { $size } です。測定日時: { $when }。
# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = 使用量は毎日測ります。
# Tip of the button that clears all earlier copies, and its label while working.
settings-snap-clear-all = 今すぐすべて消去
# Heading of the card about earlier versions of files.
settings-snap-earlier = 以前のバージョン
# Help text of the Earlier versions card (the information button).
settings-snap-earlier-about = Trindle は、みんなのファイルを毎日コピーして残せます。Folders から、ファイルの古いバージョンを開いたり、間違えて削除したファイルを取り戻したりできます。コピーでは、その日に見られたものだけが各自に見えるので、ほかの人に見せないホームフォルダは見えないままです。ほかの人が開けたファイルは、コピーが消されるまで、そのコピーの中でも開けます。このコピーはシステムのスナップショットとは別で、どちらも相手を変えません。ただしディスクの容量は使います。削除したり変更したりしたファイルも、それを含む最後のコピーが消されるまで容量を使います。コピーを消去すると、全員の容量がすぐに空きますが、元に戻せません。ディスクがほぼいっぱいの間は、コピーを取りません。これらを変更するには、管理者のパスワードが必要です。
# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = ファイルの以前のバージョンを残す
# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = 削除したファイルは { $time } 残り、そのあと空きになります。
# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = 毎日のコピーを残す期間
# Name of the list of how long each day's copy is kept.
settings-snap-how-long = 毎日のコピーを残す期間
# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = スナップショットを確認しています…
# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = スナップショットを読み込めませんでした。
# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = スナップショット { $n } から起動したので、今することは保存されません。
# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
       *[other] スナップショット { $n } 個に戻せます。いちばん新しいのは { $when } です。
    }
# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = スナップショットはまだありません。ソフトウェアを変える前に毎回取ります。
# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }、削除するまで保存
# Tip of the button that restores the system to a snapshot, and its label while working.
settings-snap-restore-this = このスナップショットを復元
# Tip of the button that deletes a snapshot.
settings-snap-delete-this = このスナップショットを削除
# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = 今すぐ再起動
# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = まだ設定されていません。
# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = スナップショットは設定されていません。trindle パッケージを再インストールすると設定されます: sudo apt install --reinstall trindle。
# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = 別の種類
# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = スナップショットには Btrfs が必要です。このコンピューターは { $fs } です。
# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = スナップショットを使うには、システムが Btrfs 上にある必要があります。このコンピューターは { $fs } です。スナップショットが必要なときは、Debian をインストールするときにシステムを Btrfs にしてください。
# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = このコンピューターにスナップショットはありません。
# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = スナップショットなし
# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = このコンピューターにスナップショットはありません
# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = 読み込み中…
# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = スナップショットを読み込めませんでした
# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = まだありません
# Help text of the Snapshots card (the information button).
settings-snap-list-about = スナップショットはそれぞれ、次に再起動したときに、システム全体をそのときの状態に戻せます。復元や削除には、管理者のパスワードが必要です。
# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = 今すぐスナップショットを取る
# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = 通常どおり再起動してから行ってください。
# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
       *[other] 復元のあと別に取ってある古いシステム { $n } 個
    }
# Tip of the button that frees the space the set aside systems use.
settings-snap-free-them = 使っている容量を空ける
# Heading of the card for taking a snapshot.
settings-snap-take-one = 取る
# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = システムファイルをロック
# Added to the Take one card's i.
settings-snap-locked-sub = インストールと更新のときだけ、ロックが解除されます。
# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = ロック中
# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = ロック解除中
# Help text of the Take one card (the information button).
settings-snap-take-about = あとで元に戻したくなりそうなことを試す前に、スナップショットを取ってください。自分で取ったスナップショットは、削除するまで残ります。スナップショットから起動した場合は、先に通常どおり再起動してください。そうしないと、新しいスナップショットは残りません。復元するたびに、その直前のシステムを別に取っておくので、何も失われません。システムファイルをロックすると、インストールと更新だけがそれを変えられ、そのたびに前後のスナップショットを取ります。ロックの変更には、管理者のパスワードが必要です。
# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = スナップショット { $n } から起動しました
# Under the Started from snapshot heading.
settings-snap-nothing-kept = 変更は保存されません。
# Heading of a line on the Before restoring card.
settings-snap-files-too = あなたのファイルもスナップショットに入っています
# Under the Your files are in the snapshots too heading.
settings-snap-files-back = 復元すると、あなたのファイルも元に戻ります。
# Heading of the card about what to know before restoring.
settings-snap-before = 復元する前に
# Help text of the Before restoring card (the information button).
settings-snap-before-about = スナップショットから起動したので、今することは保存されません。このままのシステムを残すには、一覧でそのスナップショットの横の「このスナップショットを復元」を押してから、再起動してください。このコンピューターでは、ホームフォルダがシステムの一部かもしれません。その場合、スナップショットを復元すると、あなたのファイルもそのときの状態に戻るので、新しいものは先にどこか安全な場所にコピーしてください。
