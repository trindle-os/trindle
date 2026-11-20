# machine translated (Korean), not yet reviewed by a person.

# Settings: words shared by the pages (src/settings/ui.rs).

## Shared by the pages (src/settings/ui.rs)

# Said beside a line once its change is saved.
settings-common-saved = 저장했어요
# Said beside a button that worked, when nothing more is known.
settings-common-done = 완료
# Words that only say "that worked", in lower case and separated by bars. A line saying one of them under a control that has already moved is left out, as the glow says it. Translate each word, and keep the bars.
settings-common-quiet-words = 저장했어요|완료|지금 사용 중|변경했어요|설정했어요|사용 중|추가했어요|비웠어요|제거했어요|저장했어요|설치했어요|업데이트했어요|참여했어요|복사했어요|이제 기본값|재생했어요|보냈어요|연결했어요|연결 중|켜짐|꺼짐|잊었어요|80%에서 멈춤|완전히 충전|로그인 때 시작|보관하지만 시작하지 않음|최신 상태
# Said beside a button that failed, when nothing more is known.
settings-common-not-done = 실패했어요
# Tip of the information button at the top of every Settings page.
settings-common-about-page = 이 페이지 정보
# Tip of the information button on a card: { $name } is the card's heading as it is written.
settings-common-about-card = { $name } 정보
# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name }이(가) 이미 있어요
# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
       *[other] 나머지도 같게 하기(+{ $n })
    }
# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = 나머지도 같게 하기
# Tip of the button that keeps the one already there.
settings-common-skip = 건너뛰고 기존 것 유지
# Button that replaces the one already there.
settings-common-replace = 바꾸기
# Spoken name of the Replace button.
settings-common-replace-spoken = 기존 것을 새것으로 바꾸기
# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = 새것
# Heading of the line that shows the one already there.
settings-common-already-line = 이미 있는 것
# Heading of the line with the Skip and Replace buttons.
settings-common-replace-question = 이미 있는 것을 바꿀까요?
# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
       *[other] { $left }초 뒤에 되돌려요.
    }
# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
       *[other] { $left }초 뒤에 되돌려요. { $more }
    }
# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = 지금 되돌리기
# Button at the foot of the window that keeps the change on trial.
settings-common-keep = 유지
# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what }을(를) 쓸 수 없어요
# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = 설치하려면 다음을 실행하세요: sudo apt install { $package }
# Red words when a web page could not be opened.
settings-common-no-browser = Trindle이 웹 브라우저를 시작하지 못해서 웹 페이지를 열 수 없었어요.
# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name }을(를) 열 수 없었어요.
# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = 저장하지 못했어요.
# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = 시스템의 일부가 응답하지 않아서 Trindle이 할 수 없었어요. 컴퓨터를 다시 시작한 뒤 다시 시도하세요.
# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = 로그인의 일부가 응답하지 않아서 Trindle이 할 수 없었어요. 로그아웃했다가 다시 로그인한 뒤 다시 시도하세요.
# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Trindle이 파일이나 폴더를 고르는 창을 열 수 없었어요. 로그아웃했다가 다시 로그인한 뒤 다시 시도하세요.
# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = 디스크가 가득 찼어요.
# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Trindle이 지금 디스크에 쓸 수 없어요.
# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = 이 일을 하는 시스템 부분이 실행 중이 아니에요.
# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = 응답이 너무 오래 걸렸어요. 다시 시도하세요.
# Cause given in red words after an action failed: the password was not given.
settings-common-why-password = 비밀번호를 입력하지 않았어요.
# Cause given in red words after an action failed: the account is not allowed.
settings-common-why-not-allowed = 이 계정은 할 수 없어요.
# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = 필요한 것이 이 컴퓨터에 없어요.
# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }
# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } 및 { $last }
# A list of choices ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-or = { $rest } 또는 { $last }
# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

## The picture of the ring (src/settings/pictures.rs)

# Word drawn in the middle of that picture.
settings-common-ring-name = 링

# Words shared by Settings and other programs, from src/common: the charge limit and the virtual machine note.

## Charge limit (src/common/charge.rs)

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Trindle이 노트북의 충전 제어에 접근할 수 없었어요: { $error }
# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = 노트북이 충전 한도를 받아들이지 않았어요.
# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = 이 컴퓨터는 충전을 일찍 멈추는 기능을 지원하지 않아요.
# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = { $path }을(를) 설정할 수 없었어요: { $error }

## Virtual machine note (src/common/graphics.rs)

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = 가상 머신에서 Trindle 체험하기
# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle이 { $machine }에서 실행 중이에요. 잘 작동하지만 가상 머신은 실제 컴퓨터만큼 빠르거나 부드럽지 않아요. 속도는 실제 하드웨어에서 판단하세요.
# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle이 3D 그래픽 없이 { $machine }에서 실행 중이에요. 원래 그래픽 칩이 그리는 것을 모두 프로세서가 그려야 해요. 창, 동영상, 웹 페이지가 실제 컴퓨터보다 느리게 느껴지고, 그래서 Trindle은 애니메이션을 뺐어요. 이 가상 머신을 돌리는 컴퓨터가 지원하면 가상 머신의 디스플레이 설정에서 3D 가속을 켜세요. 차이가 아주 커요. 실제 컴퓨터에 Trindle을 설치하면 이 내용은 해당되지 않아요.
