# machine translated (Simplified Chinese), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = 报告问题

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = 出了点问题。要发送报告吗？

# Under the title: what a report holds, and what it never holds.
# SAFETY: needs a human review before it ships
report-summary = 这会发送你写的文字、电脑信息、本次开机的错误和 Trindle 的日志。Trindle 收集的内容中，姓名和地址（电子邮件、网络、Wi-Fi）已被去掉。日志行中仍可能有文件名和网址。不会发送你的文件或密码。

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = 发生了什么？

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = 是哪个应用？

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = 不确定

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = 其他程序

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = 桌面

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
report-not-taken = 目前还不能发送报告。

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = 正在检查。

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = 同时收到的报告太多。

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
report-no-at = 请输入你的 Buildmorphic 用户名，而不是电子邮件地址。不要使用 @。

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = 用户名长度必须为 2 到 31 个字符，可使用小写字母、数字或短横线。

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = GitHub 名称只能包含字母、数字和短横线。

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle 无法保存报告留待以后发送：{ $reason }。

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = 无法保存：{ $reason }。

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = 没有权限

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = 驱动器已满

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = 文件夹不存在

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = 驱动器出错

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = 缺少 curl。

# Why a report could not be posted: the post could not be started.
report-not-posted = 无法发送。

# Why a report could not be posted: the computer is offline.
report-no-connection = 没有网络连接。

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = 服务器返回了 { $code }。

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = 请再多写几句，说明发生了什么。

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = 正在读取日志。

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = 没有可随报告发送的日志，所以无法发送。请改为保存。

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = 图片可选。只有项目所有者能看到。

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = 选择一张 PNG 图片

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = 截图

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = 移除图片

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = 你的图片，{ $width } 乘 { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = 选择一张 PNG 图片

report-picture-filter = PNG 图片

# Under the picture while it is being read.
report-picture-reading = 正在读取。

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = 无法截图。

report-picture-no-chooser = Trindle 无法打开文件选择器。

# Red words under the picture, each the reason it is refused.
report-picture-not-png = 不是 PNG 图片。

report-picture-too-big = 超过 { $megabytes } MB。

report-picture-too-large = 图片太大了。最宽 { $side } 像素，最高 { $side } 像素。

report-picture-damaged = 图片已损坏。

report-picture-unreadable = 无法读取。

report-sent-as-bug = 已发送。问题报告编号：{ $number }。谢谢。

# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle 目前处于测试阶段。报告是匿名的，其中的姓名、电子邮件地址和主文件夹都已去掉。用户名可填可不填。

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Buildmorphic 用户名（可选）

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle 未能发送报告。{ $why } 请按“发送”重试。
