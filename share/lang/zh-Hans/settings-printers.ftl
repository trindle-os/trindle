
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = 还没有打印机。Trindle 在网络中找到的打印机会列在下方。

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } 已暂停，暂时无法打印。

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } 正在打印。

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = 可以使用 { $name } 打印。

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } 状态：{ $state }。

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } 已设置，但你还没有选择默认打印机。

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = 已设置 { $n } 台打印机，但你还没有选择默认打印机。

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = 该任务已不存在。

# Shown beside a print job after it was cancelled.
settings-printers-cancelled = 已取消

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = 检查失败。

# Red words when Trindle cannot reach CUPS at all, after trying twice.
printers-unreachable = 打印服务没有运行，或无法连接。

# Words in the ring while the network is searched for printers.
settings-printers-searching = 查找中

# Words in the ring when the search is finished.
settings-printers-search-done = 完成

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = 现为默认

# Shown beside a printer after a test page was sent.
settings-printers-sent = 已发送

# Shown beside a printer after it was removed.
settings-printers-removed = 已移除

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = 请先输入打印机的地址。

# Shown under the By its address line while the printer is added.
settings-printers-adding = 正在添加…

# Shown beside a printer or the address line after it was added.
settings-printers-added = 已添加

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = 正在查找驱动程序…

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = { $printer } 上的任务 { $number }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }，{ $size }，{ $when }

# Tip of the cross button that cancels a print job.
# SAFETY: needs a human review before it ships
settings-printers-cancel-job = 取消此任务

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = 打印任务

# Help text (i button) of the Print jobs card.
# SAFETY: needs a human review before it ships
settings-printers-jobs-about = 所有打印机上等待打印的全部任务。此页面打开时，Trindle 每隔几秒会更新一次列表。已经打印出的页面无法取消。

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS 没有响应

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = 无法读取打印队列。

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = 查看中…

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = 没有等待的任务

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = 已暂停

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = 空闲

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = 正在打印

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = 默认，{ $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = 设为默认打印机

# Tip of the button that prints a test page.
settings-printers-test-page = 打印测试页

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = 移除这台打印机

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = 搜索需要 cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = 要安装它，请运行：sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = 添加这台打印机

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = 没有发现新的打印机

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = 添加

# Heading of the line where a printer's address is typed.
settings-printers-by-address = 按地址添加

# Heading of the card for adding printers.
settings-printers-add-card = 添加打印机

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Trindle 会先查找主动通告自己的打印机，然后逐台检查网络中的设备。打印机必须已开机，并且在同一网络中。有些打印机不会主动通告，有些网络也不会转发通告。这时请改用地址添加，例如 192.168.1.50。你可以在打印机自己的屏幕上，网络设置里找到这个地址。

# Name of the page in the list of settings pages.
settings-printers-title = 打印机

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = 打印机 打印 扫描仪

# Help text at the top of the page.
settings-printers-help = Trindle 会自动找到网络中的打印机。要使用某台打印机，请按它旁边的“添加这台打印机”。

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = 这台电脑没有安装打印功能。

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = 查看中…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Trindle 无法连接到打印服务，所以目前无法打印。

# Heading of the card shown when printing is not installed.
settings-printers-printing = 打印

# Help text (i button) of that card.
settings-printers-printing-about = 打印需要 CUPS，但这台电脑上没有。安装后，Trindle 会自动找到网络中的打印机。

# Line in that card.
settings-printers-unavailable = 无法使用打印功能

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = 请用以下命令安装：sudo apt install cups

# Line in that card.
settings-printers-then-found = 查找打印机

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = 打印服务

# Line in the Printers card when there are none.
settings-printers-none = 还没有打印机

# Heading of the card that lists the printers.
settings-printers-card = 打印机

# Help text (i button) of the Printers card.
# SAFETY: needs a human review before it ships
settings-printers-card-about = 这台电脑上设置的所有打印机。你的账户需要属于 lpadmin 组才能进行更改。

# Red words under a page range box that cannot be used.
printers-bad-range = 例如：1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = 1 到 99 份。

# Name of the choice to print on one side of the paper.
printers-sides-one = 单面

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = 双面，长边翻转

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = 双面，短边翻转

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Letter

# Name of the US Legal paper size.
printers-paper-legal = Legal

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = 纵向

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = 横向

# Red words when a printing error is a refusal because the account is not in the lpadmin group.
# SAFETY: needs a human review before it ships
printers-not-allowed = 你无权这样做。你的账户需要属于 lpadmin 组。之后请注销并重新登录。

# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS 拒绝了请求。

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = 不允许使用这个打印机名称。

# Red words when printing is not installed on the computer.
printers-not-installed = 这台电脑没有安装打印功能。

# Red words when the printer chosen has been removed since.
printers-gone = { $name } 已不再设置。

# Red words when the printer chosen is paused.
printers-paused = { $name } 已暂停。

# Red words when sending the document to CUPS took too long.
printers-too-slow = 文档发送时间过长。

# Red words when a print job could not be cancelled.
settings-printers-not-cancelled = Trindle 无法取消该打印任务。

# Red words when a printer could not be made the default.
settings-printers-not-default = Trindle 无法将其设为默认打印机。

# Red words when the test page could not be sent.
settings-printers-not-sent = Trindle 无法发送测试页。

# Red words when a printer could not be removed.
settings-printers-not-removed = Trindle 无法移除该打印机。

# Red words when a printer could not be added.
settings-printers-not-added = Trindle 无法添加该打印机。

# Red words when the list of printers could not be read.
settings-printers-not-read = Trindle 无法读取打印机列表。
