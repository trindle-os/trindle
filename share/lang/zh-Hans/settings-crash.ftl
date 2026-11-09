
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = 崩溃报告

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = 崩溃 报告 错误 故障 发送 隐私 遥测 crash reports errors bugs faults send sent privacy telemetry buildmorphic

# Help line at the foot of the Crash reports page.
settings-crash-help = Trindle 的某个程序崩溃时，崩溃报告附加组件会在这台电脑上保存一条简短的记录。在你运行它的发送命令并对每份报告说“是”之前，它什么也不会发送，而且会先显示完整的报告。只有你在这里输入了你的名字和 GitHub 名称，它们才会写入。绝不发送消息、文件名、你所在的位置或其他程序的任何信息。移除附加组件即可全部停止。

# Heading and label of the switch for sending crashes.
settings-crash-crashes = 崩溃

# Under the Crashes switch.
settings-crash-crashes-short = 在 Trindle 代码的哪个位置崩溃。

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = “崩溃”会发送程序、程序版本、崩溃类型，以及崩溃发生在 Trindle 代码的哪个位置。

# Heading and label of the switch for sending errors.
settings-crash-errors = 错误

# Under the Errors switch.
settings-crash-errors-short = 错误的固定名称及位置。

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = “错误”会发送程序、程序版本、Trindle 为该错误指定的固定名称，以及它在代码中的位置。

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = 关于这台电脑

# Under the About this computer switch.
settings-crash-computer-short = Linux、处理器、内存、显卡、屏幕。

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = “关于这台电脑”会在每份报告中加入：所运行的 Linux、处理器类型、向下取整的内存大小、Wayland 或 X11、显卡制造商，以及屏幕数量。

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = 发送的内容

# Help text of the What is sent card: { $switches } is what each switch sends.
settings-crash-what-about = { $switches } 报告不带名字，除非你在下面的框里填入你的 Buildmorphic 用户名，这只会让你获得署名。

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = 未做任何更改，因为{ $why }。

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = 未做任何更改，因为保存设置的文件不是 Trindle 写入时的样子。

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = 没有设置文件夹。

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = 无法创建设置文件夹。

# Red words when a setting could not be written to its file.
settings-crash-not-written = 无法写入该设置。

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = { $program } 中的崩溃（{ $signal }）

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = { $program } 中的错误 { $code }

# In the list of sent reports: the report was taken.
settings-crash-taken = 已收到

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = 未发送：问题追踪器已关闭

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = 未发送：报告过大

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = 未发送：被拒绝

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = 未发送：同时收到太多

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = 未发送

# Said beside the link line once it is saved.
settings-crash-saved = 已保存

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = 清空列表

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = 无法清空列表。

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = 已移除

# Heading of the line that holds the clear button.
settings-crash-kept-here = 仅保存在这台电脑上

# Under the Kept on this computer only heading.
settings-crash-newest-first = 最新的在最前，最多五十条。

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = 暂无

# Under the Nothing yet heading.
settings-crash-listed-here = 每份报告都会按发送时的原样列在这里。

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = 查看发送的确切内容

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }：{ $answer }
       *[other] { $when }，共 { $n } 次：{ $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = 已发送

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = 每份报告都按发送时的原样列出，最新的在最前，且仅保存在这台电脑上。“已收到”表示 Buildmorphic 已收到。“未发送：问题追踪器已关闭”表示 Trindle 的问题追踪器目前不接收报告；“被拒绝”表示因格式不正确而被拒绝；“同时收到太多”表示同一时间收到的报告太多。

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = 崩溃后会询问你是否发送，并显示完整的报告内容。

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = 不会发送任何内容，崩溃后也不会询问你。

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = 不会发送任何内容：崩溃和错误都已关闭。

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Trindle 程序的错误会以匿名方式报告。
        [both] Trindle 程序的崩溃和错误会以匿名方式报告。
       *[crashes] Trindle 程序的崩溃会以匿名方式报告。
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = 询问问题并分享报告

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = 关闭后会隐藏全部内容，并删除等待中的报告。

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = 已关闭。没有等待中的报告。

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
       *[other] 已关闭。已删除 { $n } 份等待中的报告。
    }

# Tip of the button that opens the community page.
settings-crash-about-community = 关于社区

# Heading of the line with the community button.
settings-crash-go-to-bm = 报告发送到 Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = 帮助构建 Trindle 的社区。

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = 用户名

# Heading of the line with the username box.
settings-crash-your-username = 你的 Buildmorphic 用户名

# Under the username heading.
settings-crash-username-sub = 可选。不要填电子邮件地址。

# Heading of a line about joining with an email address.
settings-crash-join = 只需一个电子邮件地址即可加入

# Under the Join heading.
settings-crash-join-sub = 首先报告问题可获得积分和等级。

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = 报告只包含固定的内容

# Under the fixed list heading.
settings-crash-fixed-sub = 你会先看到内容，未经你点击不会发送任何东西。

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = 你发送的报告

# Help text of the Reports you send card (the information button).
settings-crash-reports-about = 当 Trindle 的某个程序崩溃时，可能会打开一个窗口询问是否发送报告。窗口会先显示确切的文字：你写的话、电脑的型号和显卡、错误，以及崩溃前不久的日志。绝不会读取文件、密码和 Wi-Fi 名称，姓名和地址也会被抹去。你也可以随时通过圆环中的“报告问题”按钮打开同一个窗口。

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = github 名称

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = 你的 GitHub 名称

# Small words under that heading.
settings-crash-github-about = 可选。显示为“报告者”。

