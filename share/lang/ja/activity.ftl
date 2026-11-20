# machine translated (Japanese), not yet reviewed by a person.

## Activity (src/activity): what the computer is doing right now, drawn as a room of bubbles, with four cards of figures.

## The window and its page (src/activity/mod.rs)

# Window title, and the heading at the top of the Activity window.
activity-title = アクティビティ
# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = このページについて
# Button under the sentence when the computer could not be read: asks again.
activity-try-again = もう一度試す
# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = Trindle はコンピューターの動作状況を確認できませんでした。
# In place of the room when no program is running.
activity-no-programs = 実行中のプログラムはまだありません。
# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = コンピューターの動作状況を確認できるまで、Trindle はプログラムを表示できません。
# Heading of the card that lists the programs.
activity-programs = プログラム
# Column heading of the list: the name of each program.
activity-column-program = プログラム
# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = メモリ
# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = プロセッサー
# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = メモリ使用量で大きさを変えたプログラムのバブル

## The longer words behind the i, one paragraph at a time (src/activity/mod.rs)

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = コンピューターがいま何をしているかを表示します。プログラムは円の中のバブルで表されます。バブルが大きいほど、メモリを多く使っています。円がいっぱいに近いほど、メモリの使用量が多いことを示します。
# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = 作業中のプログラムのバブルは光って脈打ちます。脈が速く明るいほど、作業が多いことを示します。冷たい色は作業が少ないこと、オレンジは多いこと、赤は限界であることを示します。縁にあるバーは、プロセッサーで作業を処理する部分であるスレッドを1つずつ表します。バーが長く色が暖かいほど、作業が多いことを示します。すべてのバーが長く赤いときは、コンピューターが全力で動いています。いちばん明るいバブルが、最も多く作業しているプログラムです。
# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
activity-help-3 = バブルまたは一覧のプログラムをクリックすると、いまの状況を数値で確認できます。応答しなくなったプログラムを止めることもできます。停止ボタンを2回押すと、Trindle がプログラムに終了を求めます。保存していない内容は失われることがあります。2つの灰色のバブルは、それ以外のすべてを表します。Trindle デスクトップは、画面、リング、タブ、壁紙を描く部分です。Linux とその他のプログラムは、Linux 本体と、コンピューターを管理するプログラムです。
# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = Trindle は smem などのツールと同じ方法で、メモリを正確に数えます。2つのプログラムが共有するメモリは分けて数え、スワップファイルに移したメモリも含めるので、重複や漏れはありません。絵の横のカードには、メモリ、プロセッサー、ディスク、スワップファイルの使用状況が表示されます。リングは、ほぼいっぱいになったときだけオレンジになり、さらに進むと赤になります。

## The sentence at the top (src/activity/mod.rs)

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = いま
# The sentence at the top while the first look is being taken.
activity-looking = 確認中…
# The sentence at the top when the computer could not be read at all.
activity-unread = Trindle はコンピューターを確認できませんでした。
# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = コンピューターは全力で動いています。{ $a } が { $threads } を使っています。
# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = コンピューターは全力で動いています。{ $a } と { $b } が { $threads } を使っています。
# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = コンピューターは { $threads } で全力で動いています。いちばん忙しいのは { $a } です。
# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = コンピューターは { $threads } で全力で動いています。
# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = メモリがほぼいっぱいです。最も多く使っているのは { $b } です。
# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = メモリがほぼいっぱいです。
# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = コンピューターは忙しくありません。最も作業しているのは { $a } です。
# The sentence at the top when the computer is busy and nothing is named.
activity-busy = コンピューターは忙しい状態です。
# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = コンピューターは忙しくありません。
# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = いまは { $a } が作業のほとんどを行っています。
# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a } と { $b } がコンピューターを忙しくしています。
# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a } は多く作業していますが、コンピューターにはまだ余裕があります。
# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = コンピューターの作業は多くありませんが、メモリ使用量が増えています。最も多く使っているのは { $b } です。
# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = 多く作業しているプログラムはありません。
# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads = すべての { $n ->
       *[other] { $n } スレッド
    }
# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread = { $n ->
       *[other] { $n } スレッド
    }のうち1未満
# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads = { $total ->
       *[other] { $total } スレッド
    }のうち約 { $n }
# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads = / { $n ->
       *[other] { $n } スレッド
    }

## Figures and words about a program (src/activity/mod.rs)

# A program's name when it has none that can be read.
activity-program-unnamed = プログラム
# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = 1%未満
# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = 1 MB未満
# How a thread is described in its tip when it is resting.
activity-resting = 待機中
# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = 使用率 { $percent }
# How hard a program is working, when it is resting.
activity-effort-resting = 待機中
# How hard a program is working, when it is working a little.
activity-effort-little = 少し作業中
# How hard a program is working, when it is working flat out.
activity-effort-flat-out = 全力で作業中
# How hard a program is working, when it is working hard.
activity-effort-hard = たくさん作業中
# How long something has been running, when it is under a minute.
activity-under-a-minute = 1分未満
# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes = { $n ->
       *[other] { $n }分
    }
# How long something has been running, in hours. $n is the number of hours.
activity-hours = { $n ->
       *[other] { $n }時間
    }
# How long something has been running, in days. $n is the number of days.
activity-days = { $n ->
       *[other] { $n }日
    }
# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }、{ $b }
# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = 最も作業中
# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort }: { $work }
# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort }: { $work }。メモリも最も多く使用中
# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = メモリ使用量が最大
# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = メモリ使用量が2番目
# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = { $amount } を使用中、{ $how }
# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = メモリの { $share }
# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = 空きメモリ { $amount }
# The name of a program's bubble when it is Linux itself.
activity-linux-itself = Linux 本体
# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = Trindle 本体
# The name of the bubble for Trindle's own desktop: the compositor, the ring and the wallpaper.
activity-trindle-desktop = Trindle デスクトップ
# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux とその他のプログラム
# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }、{ $figure }、カードが開いています
# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }、{ $figure }
# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = スレッド { $n }: { $busy }
# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = { $name } の詳細

## The card beside a picked bubble (src/activity/mod.rs)

# Words in a card when a program's figures have gone (it has closed).
activity-closed = 終了しました
# Button on a card that closes it.
activity-close = 閉じる
# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
activity-stop-again = もう一度クリックすると { $title } を停止します
# Tip of the stop button on a card. $title is the program.
activity-stop = { $title } を停止
# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = { $title } が応答しなかったため、Trindle が強制終了しました
# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1個
# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = 別々の { $n } 個
# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = 実行時間
# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = 構成

## The four figure cards (src/activity/rings.rs)

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = まだ読み取れません。
# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = { $used } を使用中。うち Trindle 本体は { $ours } です。
# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = 直近2分間の { $threads } の使用状況です。
# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = { $used } を使用中、{ $free } が空いています。
# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = / { $amount }
# Name of the disk card.
activity-disk = ディスク
# Name of the swap file card.
activity-swap-file = スワップファイル
# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = { $used } を使用中。
# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = 温度
# Sentence under the temperature figure.
activity-temperature-sentence = プロセッサーの温度
# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = / 100°C
# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = このコンピューターにはスワップファイルがないので、メモリから移されるものはありません。
