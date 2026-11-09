
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Window title, and the heading at the top of the Activity window.
activity-title = 活动

# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = 关于此页面

# Button under the sentence when the computer could not be read: asks again.
activity-try-again = 重试

# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = 无法读取电脑正在做什么。

# In place of the room when no program is running.
activity-no-programs = 目前没有程序在运行。

# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = 在能读取电脑之前，没有可列出的内容。

# The one sentence above the room, saying what it shows.
activity-caption = 每个气泡代表一个程序，气泡越大，占用的内存越多。

# The one sentence above the list of programs.
activity-list-caption = 点按一个程序，即可在圆形区域中查看它。

# Heading of the card that lists the programs.
activity-programs = 程序

# Column heading of the list: the name of each program.
activity-column-program = 程序

# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = 内存

# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = 处理器

# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = 各个程序，每个的大小对应它占用的内存

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = 这台电脑此刻在做什么。每个程序是大圆里的一个气泡，气泡越大，占用的内存越多，所以圆越满，内存就越满。

# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = 正在工作的程序会发光并跳动，工作越吃力，跳得越快、越亮：冷色表示工作量小，橙色表示工作量大，红色表示已满负荷。圆的边缘上，处理器的每个线程对应一根条，越忙就越长、越暖，所以当所有条都又长又红时，说明电脑已满负荷，而最亮的气泡就是原因。

# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
activity-help-3 = 点按一个气泡或列表中的程序，即可用简单的数字查看它在做什么；如果它不再响应，也可以在这里停止它：连按两次停止按钮会请它关闭，其中未保存的内容可能会丢失。两个安静的灰色气泡包含其余部分：Trindle 桌面，即负责绘制屏幕、圆环、标签页和壁纸的合成器；以及 Linux 和其他程序，即 Linux 本身和照看电脑的各个程序。

# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = 内存按公平的方式计算，与 smem 等工具相同：两个程序共用的内存由它们平分，换出到交换文件的内存也计算在内，所以不会重复计算，也不会遗漏。图旁的卡片显示内存、处理器、磁盘和交换文件有多满；只有接近已满时，圆环才会变成橙色，然后变成红色。

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = 此刻

# The sentence at the top while the first look is being taken.
activity-looking = 正在查看…

# The sentence at the top when the computer could not be read at all.
activity-unread = 暂时无法读取电脑的情况。

# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = 你的电脑已满负荷：{ $a } 占用了 { $threads }。

# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = 你的电脑已满负荷：{ $a } 和 { $b } 占用了 { $threads }。

# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = 你的电脑在 { $threads } 上已满负荷；{ $a } 最忙。

# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = 你的电脑在 { $threads } 上已满负荷。

# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = 内存快满了；{ $b } 占用最多。

# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = 内存快满了。

# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = 电脑运行平稳；{ $a } 最忙。

# The sentence at the top when the computer is busy and nothing is named.
activity-busy = 电脑很忙。

# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = 电脑运行平稳。

# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = 此刻大部分工作由 { $a } 完成。

# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a } 和 { $b } 让电脑忙个不停。

# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } 在努力工作，但电脑还有很多余力。

# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = 一切平静，但内存正在变满；{ $b } 占用最多。

# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = 一切平静：没有程序在吃力地工作。

# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads = 全部 { $n ->
       *[other] { $n } 个线程
    }

# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread = { $n ->
       *[other] { $n } 个线程
    }中不到 1 个

# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads = { $total ->
       *[other] { $total } 个线程
    }中约 { $n } 个

# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads = 共 { $n ->
       *[other] { $n } 个线程
    }

# A program's name when it has none that can be read.
activity-program-unnamed = 某个程序

# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = 不到 1%

# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = 不到 1 MB

# How a thread is described in its tip when it is resting.
activity-resting = 空闲

# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = 忙碌度 { $percent }

# How hard a program is working, when it is resting.
activity-effort-resting = 空闲

# How hard a program is working, when it is working a little.
activity-effort-little = 略有工作

# How hard a program is working, when it is working flat out.
activity-effort-flat-out = 满负荷工作

# How hard a program is working, when it is working hard.
activity-effort-hard = 努力工作

# How long something has been running, when it is under a minute.
activity-under-a-minute = 不到一分钟

# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes = { $n ->
       *[other] { $n } 分钟
    }

# How long something has been running, in hours. $n is the number of hours.
activity-hours = { $n ->
       *[other] { $n } 小时
    }

# How long something has been running, in days. $n is the number of days.
activity-days = { $n ->
       *[other] { $n } 天
    }

# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }，{ $b }

# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = 工作最吃力

# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort }：{ $work }

# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort }：{ $work }，并且占用内存最多

# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = 占用内存最多

# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = 占用内存次多

# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = 占用 { $amount }，{ $how }

# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = 占内存的 { $share }

# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = 空闲内存 { $amount }

# The name of a program's bubble when it is Linux itself.
activity-linux-itself = Linux 本身

# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = Trindle 本身

# The name of the bubble for Trindle's own desktop: the compositor, the ring, the tab and the wallpaper.
activity-trindle-desktop = Trindle 桌面

# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux 和其他程序

# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }，{ $figure }，卡片已打开

# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }，{ $figure }

# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = 线程 { $n }：{ $busy }

# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = 关于 { $name } 的更多信息

# Words in a card when a program's figures have gone (it has closed).
activity-closed = 已关闭

# Button on a card that closes it.
activity-close = 关闭

# SAFETY: needs a human review before it ships
# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
activity-stop-again = 再点按一次以停止 { $title }

# SAFETY: needs a human review before it ships
# Tip of the stop button on a card. $title is the program.
activity-stop = 停止 { $title }

# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1 个部分

# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = { $n } 个独立部分

# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = 已运行

# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = 组成

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = 暂时无法读取。

# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = 已用 { $used }；其中 Trindle 本身占用 { $ours }。

# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = 过去两分钟里 { $threads } 的忙碌程度。

# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = 已用 { $used }；仍有 { $free } 可用。

# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = 共 { $amount }

# Name of the disk card.
activity-disk = 磁盘

# Name of the swap file card.
activity-swap-file = 交换文件

# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = 已用 { $used }；其余空间用于休眠。

# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = 温度

# Sentence under the temperature figure.
activity-temperature-sentence = 处理器此刻的温度。

# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = 满刻度 100°C

# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = 这台电脑没有交换文件，所以不会把内存中的内容换出。


# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = 被迫退出：{ $title } 没有响应

