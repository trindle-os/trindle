# machine translated (Japanese), not yet reviewed by a person.

# Settings, Printers page, and the words of the CUPS helper (src/common/cups.rs) that the print window of the screenshot program also shows.

## Printers page (src/settings/panels/printers.rs)

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = プリンターはまだありません。Trindle がネットワークで見つけたプリンターは、下に表示されます。
# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } は一時停止中のため、今は印刷されません。
# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } で印刷中です。
# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = { $name } で印刷できます。
# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } の状態: { $state }
# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } は設定済みですが、標準のプリンターが選ばれていません。
# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } 台のプリンターが設定済みですが、標準のプリンターが選ばれていません。
# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = そのジョブはもうありません。
# Shown beside a print job after it was cancelled.
settings-printers-cancelled = キャンセルしました
# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = 確認できませんでした。
# Red words when Trindle cannot reach CUPS at all, after trying twice.
printers-unreachable = 印刷サービスが動いていないか、接続できません。
# Words in the ring while the network is searched for printers.
settings-printers-searching = 検索中
# Words in the ring when the search is finished.
settings-printers-search-done = 完了
# Shown beside a printer after it was made the one used first.
settings-printers-default-now = 標準にしました
# Shown beside a printer after a test page was sent.
settings-printers-sent = 送信しました
# Shown beside a printer after it was removed.
settings-printers-removed = 削除しました
# Red words under the By its address line when nothing was typed.
settings-printers-type-address = 先にプリンターのアドレスを入力してください。
# Shown under the By its address line while the printer is added.
settings-printers-adding = 追加中…
# Shown beside a printer or the address line after it was added.
settings-printers-added = 追加しました
# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = ドライバーを探しています…
# Heading of a print job in the queue: its number and the printer.
settings-printers-job = ジョブ { $number } ({ $printer })
# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }、{ $size }、{ $when }
# Tip of the cross button that cancels a print job.
settings-printers-cancel-job = このジョブをキャンセル
# Heading of the card that lists waiting print jobs.
settings-printers-jobs = 印刷ジョブ
# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = どのプリンターでも、印刷待ちのジョブがすべて表示されます。このページを開いている間、Trindle は数秒ごとに一覧を更新します。印刷済みのページはキャンセルできません。
# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS が応答していません
# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = 待ち行列を読み取れませんでした。
# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = 確認中…
# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = 待っているジョブはありません
# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = 一時停止中
# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = 待機中
# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = 印刷中
# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = 標準、{ $state }
# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = 標準にする
# Tip of the button that prints a test page.
settings-printers-test-page = テストページを印刷
# Tip of the bin button that removes a printer.
settings-printers-remove = このプリンターを削除
# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = 検索には cups-filters が必要です
# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = インストールするには、次を実行します: sudo apt install cups-filters
# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = このプリンターを追加
# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = 新しいものは見つかりませんでした
# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50
# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = 追加
# Heading of the line where a printer's address is typed.
settings-printers-by-address = アドレスで追加
# Heading of the card for adding printers.
settings-printers-add-card = プリンターを追加
# Help text (i button) of the Add a printer card.
settings-printers-add-about = Trindle はまず、自分から知らせてくるプリンターを探し、次にネットワーク上の機器を1台ずつ調べます。プリンターは電源が入っていて、同じネットワークにつながっている必要があります。自分から知らせないプリンターや、知らせを通さないネットワークもあります。その場合は、192.168.1.50 のように、アドレスで追加してください。アドレスは、プリンター本体の画面のネットワーク設定にあります。
# Name of the page in the list of settings pages.
settings-printers-title = プリンター
# Words the settings search looks for on this page (not shown).
settings-printers-keywords = プリンター 印刷 スキャナー printer print scanner
# Help text at the top of the page.
settings-printers-help = Trindle は、ネットワーク上のプリンターを自動で見つけます。使うには、見つかったプリンターの横の「このプリンターを追加」を押してください。
# Headline of the page when printing is not installed.
settings-printers-top-not-installed = このコンピューターには印刷機能が入っていません。
# Headline of the page when printing has not answered yet.
settings-printers-top-looking = 確認中…
# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Trindle が印刷サービスに接続できないため、今は印刷できません。
# Heading of the card shown when printing is not installed.
settings-printers-printing = 印刷
# Help text (i button) of that card.
settings-printers-printing-about = 印刷には CUPS が必要ですが、このコンピューターには入っていません。インストールすると、Trindle がネットワーク上のプリンターを自動で見つけます。
# Line in that card.
settings-printers-unavailable = 印刷は使えません
# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = 次のコマンドでインストールします: sudo apt install cups
# Line in that card.
settings-printers-then-found = プリンターを探します
# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = 印刷サービス
# Line in the Printers card when there are none.
settings-printers-none = プリンターはまだありません
# Heading of the card that lists the printers.
settings-printers-card = プリンター
# Help text (i button) of the Printers card.
settings-printers-card-about = このコンピューターに設定されているプリンターがすべて表示されます。変更するには、アカウントが lpadmin グループに入っている必要があります。

## The CUPS helper (src/common/cups.rs)

# Red words under a page range box that cannot be used.
printers-bad-range = 例: 1-3, 5
# Red words under a copies box that cannot be used.
printers-bad-copies = 1 から 99 部です。
# Name of the choice to print on one side of the paper.
printers-sides-one = 片面
# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = 両面、長辺とじ
# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = 両面、短辺とじ
# Name of the A4 paper size.
printers-paper-a4 = A4
# Name of the A3 paper size.
printers-paper-a3 = A3
# Name of the A5 paper size.
printers-paper-a5 = A5
# Name of the US Letter paper size.
printers-paper-letter = レター
# Name of the US Legal paper size.
printers-paper-legal = リーガル
# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = 縦向き
# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = 横向き
# Red words when a printing error is a refusal because the account is not in the lpadmin group.
printers-not-allowed = この操作は許可されていません。アカウントが lpadmin グループに入っている必要があります。入れたあとで、ログアウトしてもう一度ログインしてください。
# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS に拒否されました。
# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = そのプリンター名は使えません。
# Red words when printing is not installed on the computer.
printers-not-installed = このコンピューターには印刷機能が入っていません。
# Red words when the printer chosen has been removed since.
printers-gone = { $name } はもう設定されていません。
# Red words when the printer chosen is paused.
printers-paused = { $name } は一時停止中です。
# Red words when sending the document to CUPS took too long.
printers-too-slow = 書類の送信に時間がかかりすぎました。
# Red words when a print job could not be cancelled.
settings-printers-not-cancelled = Trindle はジョブをキャンセルできませんでした。
# Red words when a printer could not be made the default.
settings-printers-not-default = Trindle はそのプリンターを標準にできませんでした。
# Red words when the test page could not be sent.
settings-printers-not-sent = Trindle はテストページを送信できませんでした。
# Red words when a printer could not be removed.
settings-printers-not-removed = Trindle はプリンターを削除できませんでした。
# Red words when a printer could not be added.
settings-printers-not-added = Trindle はプリンターを追加できませんでした。
# Red words when the list of printers could not be read.
settings-printers-not-read = Trindle はプリンターの一覧を読み取れませんでした。
