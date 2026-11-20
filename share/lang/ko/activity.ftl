# machine translated (Korean), not yet reviewed by a person.

## Activity (src/activity): what the computer is doing right now, drawn as a room of bubbles, with four cards of figures.

## The window and its page (src/activity/mod.rs)

# Window title, and the heading at the top of the Activity window.
activity-title = 활동
# Tip of the i beside the title, which opens the longer words about this page.
activity-about-page = 이 페이지 안내
# Button under the sentence when the computer could not be read: asks again.
activity-try-again = 다시 시도
# Red words in place of the room when the computer could not be read at the first look.
activity-cannot-read = Trindle이 컴퓨터의 활동을 확인하지 못했어요.
# In place of the room when no program is running.
activity-no-programs = 실행 중인 프로그램이 아직 없어요.
# In place of the list when the room could not be read, so there is nothing to list.
activity-nothing-to-list = 컴퓨터의 활동을 확인할 수 있을 때까지 Trindle이 프로그램을 보여 줄 수 없어요.
# Heading of the card that lists the programs.
activity-programs = 프로그램
# Column heading of the list: the name of each program.
activity-column-program = 프로그램
# Column heading of the list, and the name of a card and a fact: the memory a program holds.
activity-memory = 메모리
# Column heading of the list, and the name of a card and a fact: the processor a program uses.
activity-processor = 프로세서
# Words the screen reader gives the room when no bubble is picked.
activity-room-spoken = 메모리 사용량에 따라 크기가 달라지는 프로그램 방울

## The longer words behind the i, one paragraph at a time (src/activity/mod.rs)

# First paragraph of the words behind the i: what the computer is doing.
activity-help-1 = 컴퓨터가 지금 무엇을 하고 있는지 보여 줘요. 프로그램마다 원 안에 방울 하나로 나타나요. 방울이 클수록 메모리를 많이 써요. 원이 가득 찰수록 메모리를 많이 쓰고 있다는 뜻이에요.
# Second paragraph of the words behind the i: how the colours and bars show the work.
activity-help-2 = 프로그램이 일하는 동안 방울이 빛나고 깜박여요. 깜박임이 빠르고 밝을수록 일을 많이 하는 거예요. 차가운 색은 일이 적다는 뜻이고, 주황색은 많이 쓴다는 뜻이고, 빨간색은 가득 찼다는 뜻이에요. 가장자리의 막대는 스레드 하나씩을 나타내요. 스레드는 프로세서에서 일을 처리하는 한 부분이에요. 막대가 길고 색이 따뜻할수록 일을 많이 해요. 막대가 모두 길고 빨갛다면 컴퓨터가 최대한 일하고 있는 거예요. 가장 밝은 방울이 일을 가장 많이 하는 프로그램이에요.
# Third paragraph of the words behind the i: how to look at a program and stop it, and what the two grey bubbles are.
activity-help-3 = 방울이나 목록의 프로그램을 누르면 무엇을 하는지 알기 쉬운 수치로 볼 수 있어요. 응답하지 않는 프로그램은 멈출 수도 있어요. 멈춤 버튼을 두 번 누르면 Trindle이 프로그램에 닫아 달라고 요청해요. 저장하지 않은 내용은 사라질 수 있어요. 회색 방울 두 개는 나머지 전부를 나타내요. Trindle 데스크톱은 화면, 링, 탭, 배경 화면을 그리는 부분이에요. Linux 및 기타 프로그램은 Linux 자체와 컴퓨터를 관리하는 프로그램이에요.
# Fourth paragraph of the words behind the i: how memory, the processor, the disk and the swap file are counted.
activity-help-4 = Trindle은 smem 같은 도구와 같은 방식으로 메모리를 꼼꼼하게 세어요. 두 프로그램이 함께 쓰는 메모리는 나누어 계산하고, 스왑 파일로 옮겨진 메모리도 포함하므로 중복되거나 빠지는 것이 없어요. 그림 옆의 카드는 메모리, 프로세서, 디스크, 스왑 파일이 얼마나 찼는지 보여 줘요. 링은 거의 가득 찼을 때만 주황색, 그다음 빨간색으로 바뀌어요.

## The sentence at the top (src/activity/mod.rs)

# Heading of the card that shows the room and its sentence, at the top of the room.
activity-right-now = 지금
# The sentence at the top while the first look is being taken.
activity-looking = 확인하는 중…
# The sentence at the top when the computer could not be read at all.
activity-unread = Trindle이 컴퓨터를 확인하지 못했어요.
# The sentence at the top when the whole processor is flat out, and one program is using most of it. $a is its name.
activity-flat-out-one = 컴퓨터가 최대한 일하고 있어요. { $a }이(가) { $threads }를 쓰고 있어요.
# The sentence at the top when the whole processor is flat out, and two programs are using most of it. $a and $b are their names.
activity-flat-out-two = 컴퓨터가 최대한 일하고 있어요. { $a }, { $b }이(가) { $threads }를 쓰고 있어요.
# The sentence at the top when the whole processor is flat out, and one program is the busiest. $a is its name.
activity-flat-out-busiest = 컴퓨터가 { $threads }로 최대한 일하고 있어요. { $a }이(가) 가장 바빠요.
# The sentence at the top when the whole processor is flat out and no one program stands out.
activity-flat-out = 컴퓨터가 { $threads }로 최대한 일하고 있어요.
# The sentence at the top when memory is nearly full, and one program holds the most. $b is its name.
activity-memory-full-named = 메모리가 거의 찼어요. { $b }이(가) 가장 많이 쓰고 있어요.
# The sentence at the top when memory is nearly full, and no program is named.
activity-memory-full = 메모리가 거의 찼어요.
# The sentence at the top when the computer is busy and one program is the busiest. $a is its name.
activity-ticking-busiest = 컴퓨터는 바쁘지 않아요. { $a }이(가) 가장 많이 일하고 있어요.
# The sentence at the top when the computer is busy and nothing is named.
activity-busy = 컴퓨터가 바빠요.
# The sentence at the top when the computer is only lightly busy and nothing is named.
activity-ticking-over = 컴퓨터는 바쁘지 않아요.
# The sentence at the top when one program is doing most of the work. $a is its name.
activity-doing-most = 지금은 { $a }이(가) 일을 대부분 하고 있어요.
# The sentence at the top when two programs are keeping the computer busy. $a and $b are their names.
activity-two-keeping-busy = { $a }, { $b }이(가) 컴퓨터를 바쁘게 하고 있어요.
# The sentence at the top when one program is working hard but the computer has plenty to spare. $a is its name.
activity-plenty-to-spare = { $a }이(가) 열심히 일하고 있지만 컴퓨터에는 여유가 충분해요.
# The sentence at the top when all is calm but memory is filling up. $b is the program holding the most.
activity-calm-memory-filling = 컴퓨터가 일을 많이 하지는 않지만 메모리 사용량이 늘고 있어요. { $b }이(가) 가장 많이 쓰고 있어요.
# The sentence at the top when all is calm and nothing is working hard.
activity-all-calm = 열심히 일하는 프로그램이 없어요.
# How many threads, as a phrase inside a sentence: all of them. $n is the number of threads.
activity-all-threads =
    { $n ->
       *[other] 스레드 { $n }개 전부
    }
# How many threads, as a phrase inside a sentence: less than one of them. $n is the number of threads.
activity-less-than-one-thread =
    { $n ->
       *[other] 스레드 { $n }개 중 1개 미만
    }
# How many threads, as a phrase inside a sentence: about some of them. $n is about how many, $total is how many there are.
activity-about-threads =
    { $total ->
       *[other] 스레드 { $total }개 중 약 { $n }개
    }
# The limit under the processor figure, in threads. $n is the number of threads.
activity-of-threads =
    { $n ->
       *[other] 스레드 { $n }개 중
    }

## Figures and words about a program (src/activity/mod.rs)

# A program's name when it has none that can be read.
activity-program-unnamed = 프로그램
# The percentage shown when a program or the processor is under one per cent.
activity-under-one-percent = 1% 미만
# A program's memory when it holds less than half a megabyte.
activity-size-under-mb = 1 MB 미만
# How a thread is described in its tip when it is resting.
activity-resting = 쉬는 중
# How a thread is described in its tip when it is busy. $percent is the share, such as 80%.
activity-busy-percent = { $percent } 사용 중
# How hard a program is working, when it is resting.
activity-effort-resting = 쉬는 중
# How hard a program is working, when it is working a little.
activity-effort-little = 조금 일하는 중
# How hard a program is working, when it is working flat out.
activity-effort-flat-out = 최대한 일하는 중
# How hard a program is working, when it is working hard.
activity-effort-hard = 열심히 일하는 중
# How long something has been running, when it is under a minute.
activity-under-a-minute = 1분 미만
# How long something has been running, in minutes. $n is the number of minutes.
activity-minutes =
    { $n ->
       *[other] { $n }분
    }
# How long something has been running, in hours. $n is the number of hours.
activity-hours =
    { $n ->
       *[other] { $n }시간
    }
# How long something has been running, in days. $n is the number of days.
activity-days =
    { $n ->
       *[other] { $n }일
    }
# A program's own figures in a card: how hard it works and how much it is using. $a is the effort, $b the figure.
activity-two-parts = { $a }, { $b }
# Heading of the card beside the room for the program working hardest.
activity-note-working-hardest = 가장 열심히 일하는 프로그램
# The words under that heading: how hard it works. $effort is how hard, $work is its share.
activity-working-hardest = { $effort }: { $work }
# The words under that heading when it also holds the most memory. $effort is how hard, $work is its share.
activity-working-hardest-most = { $effort }: { $work }, 메모리도 가장 많이 사용
# Heading of the card beside the room for the program holding the most memory.
activity-note-most-memory = 메모리를 가장 많이 사용
# Heading of the card beside the room for the program holding the next most memory.
activity-note-next-memory = 메모리를 두 번째로 많이 사용
# The words under a heading for memory. $amount is the memory, $how is its share of the whole.
activity-holding = { $amount } 사용, { $how }
# The words under the memory figure of a card: how much a program holds, as a share of the whole.
activity-share-of-memory = 전체 메모리의 { $share }
# The words beside the room saying how much memory is free. $amount is the memory.
activity-memory-free = 남은 메모리 { $amount }
# The name of a program's bubble when it is Linux itself.
activity-linux-itself = Linux 자체
# The name of a program's bubble when it is Trindle itself.
activity-trindle-itself = Trindle 자체
# The name of the bubble for Trindle's own desktop: the compositor, the ring and the wallpaper.
activity-trindle-desktop = Trindle 데스크톱
# The name of the bubble for Linux and the other programs the computer runs.
activity-linux-and-other = Linux 및 기타 프로그램
# The words read out for a bubble that is open in its card. $title is its name, $figure its memory.
activity-spoken-open = { $title }, { $figure }, 카드 열림
# The words read out for a bubble. $title is its name, $figure its memory.
activity-spoken = { $title }, { $figure }
# The tip for a thread of the processor. $n is its number, $busy says how busy it is.
activity-thread-tip = 스레드 { $n }: { $busy }
# The tip for a bubble or a note when the pointer rests on it and the note says more. $name is the program.
activity-more-about = { $name } 자세히 보기

## The card beside a picked bubble (src/activity/mod.rs)

# Words in a card when a program's figures have gone (it has closed).
activity-closed = 닫힘
# Button on a card that closes it.
activity-close = 닫기
# Tip of the stop button once it has been pressed once: pressing again stops the program. $title is the program.
activity-stop-again = 한 번 더 누르면 { $title }을(를) 멈춰요
# Tip of the stop button on a card. $title is the program.
activity-stop = { $title } 멈추기
# Short line in the Activity window after a program that did not answer Stop within five seconds was ended by force. $title is the program.
activity-forced = { $title }이(가) 응답하지 않아 Trindle이 강제로 종료했어요
# A card's fact: how many parts a program is made of, when it is one part.
activity-one-part = 1개
# A card's fact: how many separate parts a program is made of. $n is the number.
activity-separate-parts = 따로 { $n }개
# A card's fact: the name of the facts' heading for how long a program has been running.
activity-running-for = 실행 시간
# A card's fact: the name of the facts' heading for what a program is made of.
activity-made-of = 구성

## The four figure cards (src/activity/rings.rs)

# Words under a memory card when it could not be read at the first look.
activity-ring-not-read = 아직 읽을 수 없어요.
# Sentence under the memory figure: how much memory is in use, and how much of it Trindle itself holds. $used and $ours are sizes.
activity-memory-sentence = { $used } 사용 중. Trindle 자체는 { $ours }을(를) 써요.
# Sentence under the processor figure: how busy all the threads have been. $threads is "all 16 threads" or similar.
activity-processor-sentence = 최근 2분 동안 { $threads }의 사용률이에요.
# Sentence under the disk figure: how much is used and how much is still free. $used and $free are sizes.
activity-disk-sentence = { $used } 사용, { $free } 남음.
# Limit under a memory, disk or swap figure. $amount is the whole.
activity-of-size = 전체 { $amount }
# Name of the disk card.
activity-disk = 디스크
# Name of the swap file card.
activity-swap-file = 스왑 파일
# Sentence under the swap file figure: how much is in use, and the rest is room to hibernate. $used is a size.
activity-swap-sentence = { $used } 사용 중.
# Name of the temperature card, shown when the computer has a temperature to read.
activity-temperature = 온도
# Sentence under the temperature figure.
activity-temperature-sentence = 프로세서 온도
# Limit under the temperature figure. The degree sign is written as it is.
activity-of-hundred-degrees = 최대 100°C
# Sentence under the swap file card when the computer has no swap file.
activity-no-swap = 이 컴퓨터에는 스왑 파일이 없어서 메모리에서 밖으로 옮겨지는 것이 없어요.
