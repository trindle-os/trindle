# machine translated (Simplified Chinese), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = 报告问题

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = 出了点问题。要发送报告吗？

# SAFETY: needs a human review before it ships
# Under the title: what a report holds, and what it never holds.
report-summary = 这会发送错误、电脑摘要和日志的最后几行，其中的姓名、电子邮件地址、网络地址和 Wi-Fi 名称已被去掉。日志行中仍可能有文件名和网址。不会发送你的文件或密码。

# Typing box at the top of the window, shown quietly while empty.
report-what-doing = 您当时在做什么？（可选）

# SAFETY: needs a human review before it ships
# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = 发送

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when the person opened it themselves.
report-cancel = 取消

# SAFETY: needs a human review before it ships
# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = 不发送

# Button that opens the exact text of the report.
report-show-details = 显示详情

# Button that closes the exact text of the report.
report-hide-details = 隐藏详情

# Button under the shown report text that copies it.
report-copy = 复制

# Button under the shown report text that saves it as a file in Downloads.
report-save = 保存

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = 目前暂不接收报告。

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = 正在检查。


# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = 同时收到的报告太多。

# Written beside the Send button once the report has been sent.
report-already-sent = 已发送。

# Written under the typing box when Save is pressed a second time.
report-already-saved = 已保存。

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = 正在等待网络连接。

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = 正在等待网络连接。{ $why }

# Under the typing box after the report was sent.
report-sent = 已发送。谢谢。

# Under the typing box while the report is being posted.
report-sending = 正在发送。

# Under the typing box after Save. $name is the file's name.
report-saved-as = 已保存为 Downloads/{ $name }。

# Red words when the Buildmorphic username holds an email address.
report-no-at = 请使用您的 Buildmorphic 用户名，而不是电子邮件地址，所以不要有 @。

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = 用户名为 2 到 31 个小写字母、数字或短横线。

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = GitHub 名称只能包含字母、数字和短横线。

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = 无法留待以后发送：{ $reason }。

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = 无法保存：{ $reason }。

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = 没有权限

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = 驱动器已满

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = 文件夹不存在

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = 驱动器拒绝写入

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = 缺少 curl。

# Why a report could not be posted: the post could not be started.
report-not-posted = 无法发送。

# Why a report could not be posted: the computer is offline.
report-no-connection = 没有网络连接。

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = 该地址返回了 { $code }。

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = 请再多写几句，说明发生了什么。

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = 正在读取日志。

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = 没有可随报告发送的日志，所以无法发送。请改为保存。



report-sent-as-bug = 已发送。已记录为问题 { $number }。谢谢。

