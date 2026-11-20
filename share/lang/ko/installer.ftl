# machine translated (Korean), not yet reviewed by a person.

## The installer (src/installer): puts Trindle on a disk, and the welcome shown when it runs from a stick

## Window and steps along the top (src/installer/mod.rs)

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = { $name } 설치
# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = 환영합니다
# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = 디스크
# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = 개인정보
# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = 비밀번호
# Name of the fifth stop on that line, the page about the person.
installer-stop-you = 사용자
# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = 준비
# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = 설치 중
# Name of the last stop on that line, shown when the install is done.
installer-stop-done = 완료
# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = 준비하는 중
# Red line at the foot when the installer's helper could not be started.
installer-no-start = 설치 프로그램을 시작할 수 없어요: { $why }
# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = 계획을 읽을 수 없어요.
# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = 계획을 읽을 수 없어요: { $why }
# Red line at the foot when the helper stopped without saying why.
installer-stopped = 설치가 멈췄어요. 더 이상 기록된 내용은 없어요.

## Foot of every page

# Name of the round button at the foot that goes back a page.
installer-back = 뒤로
# Name of the main button at the foot that goes to the next page.
installer-next = 다음
# Name of the main button at the foot of the Ready page that starts the install.
installer-install = 설치
# Name of that button after one press, when a second press goes ahead.
installer-sure = 확실한가요?
# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = 지금 다시 시작
# Beside the keyboard test box on the first page.
installer-try-keyboard = 키보드 시험하기
# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = 여기에 입력해서 키보드를 시험해 보세요
# Note beside Install after one press, when the second press writes beside another system.
installer-arm = 한 번 더 누르면 설치해요
# Note beside Install after one press, when the second press erases a whole disk.
installer-arm-erase = 한 번 더 누르면 지우고 설치해요
# Note beside Next on the first page while no language is chosen.
installer-missing-language = 언어를 선택하세요
# Note beside Next on the first page while Trindle's language is not chosen.
installer-missing-trindle-language = Trindle 언어를 선택하세요
# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = 키보드를 선택하세요
# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = 시간대를 설정하세요
# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Trindle를 설치할 곳을 선택하세요
# Note beside Next on the disk page while the disks are read again, after one turned out not to be the disk shown.
installer-disks-rereading = 디스크를 다시 읽는 중
# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = 디스크 비밀번호를 두 번 입력하세요. 두 번 모두 같아야 해요
# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = 로그인 이름을 입력하세요
# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = 이 컴퓨터의 이름을 지어 주세요
# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = 비밀번호를 두 번 입력하세요. 두 번 모두 같아야 해요

## First page: language, keyboard and time

# Title of the first page.
installer-welcome-title = { $name }에 오신 것을 환영합니다
# Sentence under the title of the first page.
installer-welcome-help = 언어를 선택하면 나머지는 Trindle가 거기에 맞춰요.
# Head of the card for choosing the computer's language (dates, spelling, other programs), and the name of its search box.
installer-language = 언어
# Head of the card for choosing the language of Trindle's own words (only those Trindle has), and the name of its search box.
installer-trindle-language = Trindle 언어
# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = 키보드
# Head of the card for choosing the time zone, and the name of its search box.
installer-time = 시간
# Under a card's head while nothing has been chosen in its list.
installer-select = 선택

## Disk page

# Title of the disk page.
installer-disks-title = { $name } 설치 위치
# Sentence under the title of the disk page.
installer-disks-help = 선택한 디스크의 모든 내용이 바뀌어요. 지금 있는 내용 옆에 설치하도록 고르면 예외예요.
# Shown on the disk page when no disk is big enough.
installer-disks-none = Trindle를 설치할 만큼 큰 디스크를 찾지 못했어요. 20 GB가 필요해요.
# Name and words of the choice to erase a whole disk.
installer-disks-whole = 디스크 전체를 사용하고 모든 내용 바꾸기
# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = { $what }을(를) 유지하고 디스크 나눠 쓰기
# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = 지금 있는 내용에 { $kept }, Trindle에 { $mine }을(를) 남겨요
# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }
# Name of a partition with nothing on it.
installer-empty-space = 빈 공간
# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } 및 { $last }
# Under a disk's name: what is on it.
installer-disk-has = { $list }이(가) 있어요
# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = { $list }이(가) 있고 암호화되어 있어요
# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = 암호화된 시스템이 있어요
# Under a disk's name: there is nothing on it.
installer-disk-empty = 비어 있음
# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = 파일이 있지만 Trindle가 아는 시스템은 없어요
# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = { $gb } GB 스왑 파일
# Name of the switch that makes a swap file.
installer-swap-switch = 스왑 파일 만들기
# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Secure Boot가 켜져 있어서 이 컴퓨터는 최대 절전 모드를 쓸 수 없지만, 메모리가 모자랄 때 스왑 파일이 도움이 돼요. 디스크에 여유 공간이 충분할 때만 만들어요.
# Under the swap file's head when the computer can hibernate.
installer-swap-note = 스왑 파일이 있으면 최대 절전 모드를 쓸 수 있어요. 전원이 꺼져 있어도 열어 둔 것이 그대로 남아요. 메모리가 모자랄 때 여분의 공간도 돼요. 없으면 최대 절전 모드를 쓸 수 없어요. 디스크에 여유 공간이 충분할 때만 만들어요.
# Under the swap file switch when the chosen disk or space is too small for one; the switch is then off and cannot be turned on.
installer-swap-too-small = 이 공간은 스왑 파일을 만들기에 너무 작아요.

## Privacy page

# Title of the page about encrypting the disk.
installer-privacy-title = 디스크 암호화
# Sentence under the title of that page.
installer-privacy-help = 암호화된 디스크는 이 컴퓨터에서 빼내도 아무도 읽을 수 없어요.
# Head of the card with the three choices.
installer-privacy-locked = 디스크를 잠그는 방법
# Name and words of the choice not to encrypt the disk.
installer-privacy-none = 디스크를 암호화하지 않기
# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = 암호화하고, 컴퓨터를 켤 때 비밀번호 묻기
# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = 암호화하고, 이 컴퓨터의 TPM 칩으로 열기
# Head of the card about the TPM chip.
installer-privacy-about = 칩에 대해
# In the card about the chip, on a computer that has one.
installer-privacy-chip = 이 컴퓨터에는 TPM 칩이 있어요. 설치 후 처음 켤 때 칩을 설정하는 동안 비밀번호를 한 번 물어봐요. 그 뒤에는 칩이 디스크를 열어 주어서 아무것도 입력하지 않아도 돼요. 칩을 바꾸거나 디스크를 다른 컴퓨터로 옮기면 필요하니 비밀번호를 잘 보관하세요.
# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = 이 컴퓨터에는 Trindle가 쓸 수 있는 TPM 칩이 없어서 켤 때마다 비밀번호를 물어봐요. 칩이 있는 컴퓨터에서는 칩이 디스크를 저절로 열 수 있어요.

## Disk password page

# Title of the disk password page.
installer-disk-title = 디스크 비밀번호
# Sentence under the title of the disk password page.
installer-disk-help = 컴퓨터를 켜면 가장 먼저 입력해요.
# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = 디스크 비밀번호
# Name of the round button that makes up a passphrase.
installer-disk-suggest = 암호 문구 추천받기
# Label and name of the second box of a password, where it is typed again.
installer-same-again = 한 번 더 입력
# Red line under the password boxes when the two do not match.
installer-mismatch = 두 비밀번호가 같지 않아요.
# Head of the card that tells the person to write the password down.
installer-disk-write-down = 적어 두세요
# Words in that card.
installer-disk-write-down-text = 이 비밀번호를 종이에 적어서 컴퓨터에서 떨어진 안전한 곳에 보관하세요. 비밀번호가 없으면 아무도 디스크를 열 수 없고, 본인도 마찬가지예요. 다시 들어갈 방법이 없어요. 흔한 단어 네 개로 된 암호 문구는 입력하기 쉽고 알아맞히기 어려워요.

## Account page

# Title of the account page.
installer-you-title = 사용자 정보
# Sentence under the title of the account page.
installer-you-help = 로그인할 때 쓰는 계정이에요.
# Head of the card with the name boxes.
installer-account = 내 계정
# Label and name of the box for the person's name.
installer-your-name = 이름
# Label and name of the box for the sign-in name.
installer-user-name = 로그인 이름
# Label and name of the box for the computer's name.
installer-host-name = 이 컴퓨터의 이름
# Head of the card with the password boxes.
installer-your-password = 내 비밀번호
# Label and name of the first password box.
installer-password = 비밀번호
# Label of the strength meter and the name a screen reader gives it.
installer-strength = 알아맞히기 어려운 정도
# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = 로그인 이름을 입력하세요.
# Red line under the account boxes: a name is too long.
installer-name-too-long = 이름이 너무 길어요.
# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = a-z 중 한 글자로 시작하세요.
# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = a-z, 0-9, - 및 _만 쓸 수 있어요.
# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = 시스템이 이미 쓰는 이름이에요.
# Red line under the account boxes: the computer's name is empty.
installer-host-needed = 이 컴퓨터의 이름을 지어 주세요.
# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = 글자, 숫자, -만 쓸 수 있어요.
# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = 컴퓨터가 바로 알아맞혀요.
# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
       *[other] 컴퓨터가 약 { $n }분 안에 알아맞혀요.
    }
# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
       *[other] 컴퓨터가 약 { $n }시간 안에 알아맞혀요.
    }
# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
       *[other] 컴퓨터가 약 { $n }일 안에 알아맞혀요.
    }
# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
       *[other] 컴퓨터가 알아맞히려면 약 { $n }년이 걸려요.
    }
# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = 컴퓨터가 알아맞히려면 기록된 역사 전체보다 오래 걸려요.

## Ready page

# Title of the Ready page.
installer-ready-title = 설치할 준비가 됐어요
# Head of the card that lists what will happen.
installer-ready-nothing-changed = 아직 바뀐 것은 없어요
# Head of the card about security updates.
installer-ready-security = 보안 업데이트
# In the card about security updates.
installer-ready-security-text = { $name }는 Debian의 보안 수정 사항을 매일 저절로 설치해요. 설정의 이 컴퓨터 정보에서 끌 수 있어요.
# Sentence under the title when a whole disk will be replaced.
installer-ready-replaced = { $disk }의 모든 내용이 바뀌어요.
# Sentence under the title when the install goes beside what is there.
installer-ready-kept = { $part }의 내용은 유지되지만 먼저 백업하세요. 크기를 줄이는 중에 정전이 되면 파일을 잃을 수 있어요.
# Sentence under the title when no disk has been chosen.
installer-ready-none = 아직 디스크를 선택하지 않았어요.
# Name of a row in the list: the whole disk will be used.
installer-ready-whole = { $disk } 전체
# Words of that row.
installer-ready-whole-text = 그 안의 모든 내용이 { $name }(으)로 바뀌어요.
# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = { $part } 옆
# Words of that row.
installer-ready-beside-text = { $part }의 크기가 { $size }(으)로 줄고, 비워진 공간에 { $name }을(를) 설치해요.
# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = 디스크 없음
# Words of that row.
installer-ready-no-disk-text = 디스크를 선택하세요.
# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = 암호화 안 함
# Words of that row.
installer-ready-plain-text = 디스크를 빼내면 누구나 읽을 수 있어요.
# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = 암호화됨
# Words of that row when a password is typed at each start.
installer-ready-locked-password = 컴퓨터를 켤 때마다 비밀번호를 입력해요.
# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = 비밀번호를 한 번 입력하면, 그 뒤에는 이 컴퓨터의 TPM 칩이 디스크를 열어요.
# Added to the end of the disk's row on the last page when a swap file is to be made, with its size in GB.
installer-ready-swap = 그리고 { $gb } GB 스왑 파일도 만들어요.
# Under the choice to let the TPM chip unlock the disk, on a computer with no usable chip.
installer-privacy-chip-off = Trindle가 쓸 수 있는 TPM 칩이 필요해요.
# In the card about the chip when the disk is not being encrypted, on a computer that has a chip.
installer-privacy-chip-none = 이 컴퓨터에는 TPM 칩이 있고, 암호화된 디스크를 열 수 있어요.
# In the card about the chip when the disk is not being encrypted, on a computer with no chip Trindle can use.
installer-privacy-no-chip-none = 이 컴퓨터에는 Trindle가 쓸 수 있는 TPM 칩이 없어요.
# Words of the language row: Trindle's language (its own name), the keyboard and the time zone chosen.
installer-ready-language = Trindle 언어 { $trindle }, 키보드 { $keyboard }, 시간대 { $zone }.

## Install page

# Title of the page shown while the install runs.
installer-installing-title = { $name } 설치 중
# Head of the card that holds the short tour.
installer-tour = 간단한 둘러보기

## Done page

# Title of the last page.
installer-done-title = { $name } 설치가 끝났어요
# Sentence under the title of the last page.
installer-done-help = USB 메모리를 빼고 다시 시작하세요.
# Head of the card that repeats the warning about the disk password.
installer-done-keep = 디스크 비밀번호를 안전하게 보관하세요
# In that card when the TPM chip unlocks the disk.
installer-keep-chip = 디스크가 암호화되어 있어요. 컴퓨터를 처음 켤 때 칩을 설정하는 동안 디스크 비밀번호를 한 번 물어봐요. 그 뒤에는 디스크가 저절로 열려요. 그 비밀번호를 종이에 적어서 이 컴퓨터에서 떨어진 안전한 곳에 보관하세요. 비밀번호가 없으면 아무도 디스크를 열 수 없고, 본인도 마찬가지예요.
# In that card when the password is typed at each start.
installer-keep-password = 디스크가 암호화되어 있어요. 아직 안 했다면 지금 디스크 비밀번호를 종이에 적어서 이 컴퓨터에서 떨어진 안전한 곳에 보관하세요. 비밀번호가 없으면 아무도 디스크를 열 수 없고, 본인도 마찬가지예요.

## Slides shown while the disk is written (src/installer/brand.rs)

# Title of the first slide.
installer-slide-ring = 모든 것이 하나의 원 안에
# Words of the first slide.
installer-slide-ring-text = 포인터를 화면 가장자리로 옮기고 두 번 클릭하면 링이 열려요. 프로그램은 바깥쪽에 있고, 시간과 스위치는 가운데에 있어요.
# Title of the second slide.
installer-slide-snapshots = 잘못된 업데이트 되돌리기
# Words of the second slide.
installer-slide-snapshots-text = Trindle는 소프트웨어를 바꾸기 전과 후마다 시스템 스냅샷을 만들어요. 설정에서 시스템 전체를 그중 어느 것으로든 되돌릴 수 있어요. 컴퓨터가 켜지지 않아도 부팅 메뉴에서 할 수 있어요.
# Title of the third slide.
installer-slide-apps = 방해하지 않는 앱
# Words of the third slide.
installer-slide-apps-text = Flathub 앱, Debian 자체 앱, AppImage, 소프트웨어 소스가 모두 한곳에 있고, 다른 컴퓨터로 가져갈 수 있는 백업도 있어요.
# Title of the fourth slide.
installer-slide-yours = 내 취향대로
# Words of the fourth slide.
installer-slide-yours-text = 밝은 모드나 어두운 모드를 고르고, 소리도 직접 고르고, 복사한 내용을 기억하는 클립보드를 쓰세요.
# Title of the fifth slide.
installer-slide-small = 작고 조용하게
# Words of the fifth slide.
installer-slide-small-text = 데스크톱이 메모리를 적게 쓰고 몇 초 만에 시작해서, 오래된 노트북도 새것처럼 느껴져요.

## The tour with pictures (src/installer/tour.rs)

# Title of the tour slide about the ring.
installer-tour-ring = 링이 데스크톱 전체예요
# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = 포인터를 이 모서리에 두면 링이 열려요
# Note on the ring picture, at the time.
installer-tour-ring-time = 시간과 날짜는 가운데에 있어요
# Note on the ring picture, at the programs.
installer-tour-ring-programs = 프로그램은 바깥쪽에 있어요
# Note on the ring picture, at the switches.
installer-tour-ring-switches = 소리, 화면, 모든 앱, 절전 방지, 끄기 스위치
# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = 열려 있는 것 보기, 클립보드 쓰기, 버튼 하나로 화면 비우기
# Title of the tour slide about Settings.
installer-tour-settings = 쉬운 말로 된 설정
# Note on the Settings picture, at the search box.
installer-tour-settings-search = 여기에 입력해서 설정 찾기
# Note on the Settings picture, at Network.
installer-tour-settings-network = 네트워크에서 Wi-Fi 네트워크에 연결해요
# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = 밝게든 어둡게든 언제든지
# Note on the Settings picture, at the colours.
installer-tour-settings-colours = 색 스무 가지 중에서 고르거나 직접 입력하세요
# Title of the tour slide about Apps.
installer-tour-apps = 그 밖의 모든 것은 앱에서
# Note on the Apps picture, at the search box.
installer-tour-apps-search = 이름이나 하는 일로 프로그램 찾기
# Note on the Apps picture, at the list.
installer-tour-apps-many = Flathub와 Debian의 무료 프로그램 수천 개
# Note on the Apps picture, at updates.
installer-tour-apps-updates = Trindle를 포함한 모든 것의 업데이트
# Title of the tour slide about voice control.
installer-tour-voice = 컴퓨터에게 말하기
# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = 음성 제어는 설정의 링에 있어요
# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = 음성 제어 추가를 한 번 누르세요. 다운로드는 약 150 MB이고 몇 분 걸려요
# Note on the voice picture, about what to say.
installer-tour-voice-say = 그다음 링 가운데를 누르고 앱 이름, “소리 크게” 또는 “Wi-Fi 설정”이라고 말하세요
# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Trindle는 말을 이 컴퓨터에서 알아듣고, 다른 곳으로는 보내지 않아요
# Title of the tour slide about snapshots.
installer-tour-snapshots = 바뀌기 전으로 돌아가기
# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = Trindle는 업데이트 전마다 스냅샷을 만들어요
# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = 문제가 생기면 부팅 메뉴에서 이전 상태로 돌아갈 수 있어요

## The welcome shown when running from a stick (src/installer/welcome.rs)

# Name of the picture of the ring, and the head of its card.
installer-live-caption = 모든 것이 들어 있는 링
# Sentence under the title of the welcome from a stick.
installer-live-help = 메모리 스틱에서 실행 중이라서 이 컴퓨터에는 아무것도 바뀐 게 없어요.
# Head of the card with the three first things to do.
installer-live-begin = 시작하기
# Name of the first thing to do.
installer-live-open = 링 열기
# Line under it.
installer-live-open-line = 화면 가장자리에서 두 번 클릭하거나 Super 키를 누르세요.
# Name of the second thing to do.
installer-live-speak = 음성 사용하기
# Line under it when voice control is there.
installer-live-speak-line = 링 가운데를 누르고 원하는 것을 말하세요.
# Line under it when voice control has to be added.
installer-live-speak-add = Trindle를 설치한 뒤 설정의 링에서 음성 제어를 추가하세요.
# Name of the third thing to do.
installer-live-tab = 모두 링 안에
# Line under it.
installer-live-tab-line = 열려 있는 것, 알림, 클립보드를 거기서 찾을 수 있어요.
# The choice of the dark look.
installer-live-dark = 어둡게
# The choice of the light look.
installer-live-light = 밝게
# Name the screen reader gives to the choice between dark and light.
installer-live-look = 데스크톱 전체를 밝게 또는 어둡게
# Name of the round button that closes this window and opens the ring.
installer-live-around = 먼저 둘러보기
# Name of the main button that opens the installer.
installer-live-install = { $name } 설치

## Words the root half sends to the window (src/installer/steps.rs)

# Foot of the install page while room is made on the disk.
installer-step-room = 공간을 만드는 중
# Foot of the install page while the disk is prepared.
installer-step-prepare = 디스크를 준비하는 중
# Foot of the install page while a place is made beside another system.
installer-step-place = Trindle를 위한 공간을 만드는 중
# Foot of the install page while the disk is looked at.
installer-step-check = 디스크에 이미 있는 내용을 확인하는 중
# Foot of the install page while another system is made smaller.
installer-step-smaller = 다른 시스템의 크기를 줄이는 중
# Foot of the install page while the disk is encrypted.
installer-step-lock = 디스크를 암호화하는 중
# Foot of the install page while the TPM chip is set up.
installer-step-chip = TPM 칩을 설정하는 중
# Foot of the install page while the system is copied.
installer-step-copy = Trindle를 디스크에 복사하는 중
# Foot of the install page while settings are written.
installer-step-settings = 설정을 기록하는 중
# Foot of the install page while the person's account is made.
installer-step-account = 계정을 만드는 중
# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = 라이브 전용 요소를 제거하는 중
# Foot of the install page while the start-up files are made.
installer-step-startup = 시작 파일을 만드는 중
# Foot of the install page while the boot menu is set up.
installer-step-boot = 부팅 메뉴를 설정하는 중
# Foot of the install page while snapshots are set up.
installer-step-snapshots = 스냅샷을 설정하는 중
# Foot of the install page at the very end.
installer-step-finishing = 마무리하는 중
# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program }을(를) 시작할 수 없어요: { $why }
# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program }이(가) 끝나지 않았어요. { $why }
# Red line at the foot: a file could not be written.
installer-write-failed = { $path }을(를) 기록할 수 없어요: { $why }
# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev }은(는) 디스크가 아니에요.
# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev }은(는) 이 컴퓨터의 디스크가 아니에요.
# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev }은(는) Trindle를 실행 중인 스틱이에요.
# Red words under the disk, back on the disk page: something on the chosen disk is mounted, swapped to or open.
installer-disk-in-use = { $dev }을(를) 사용 중이에요. 쓰고 있는 것을 닫은 뒤 다시 선택하세요.
# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = 디스크를 선택하지 않았어요.
# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Trindle가 그 로그인 이름을 쓸 수 없어요.
# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Trindle가 그 컴퓨터 이름을 쓸 수 없어요.
# Red line at the foot: the language cannot be used.
installer-bad-language = Trindle가 그 언어를 쓸 수 없어요.
# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Trindle가 그 키보드를 쓸 수 없어요.
# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Trindle가 그 시간대를 쓸 수 없어요.
# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = 그 이름에 Trindle가 쓸 수 없는 글자가 있어요.
# Red line at the foot: a password has a line break in it.
installer-bad-password = 비밀번호에는 줄바꿈을 넣을 수 없어요.
# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = 설치 프로그램의 두 번째 부분은 root로 실행해야 해요.
installer-already-running = 이 컴퓨터에서 다른 설치 프로그램이 이미 실행 중이에요.
# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = 이 디스크에는 함께 쓸 부팅 파티션이 없어서 지금 있는 내용 옆에 Trindle를 설치할 수 없어요.
# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = Trindle가 새 파티션을 찾지 못했어요.
# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = { $part }의 크기를 읽을 수 없어요.
# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle는 { $fs } 파티션의 크기를 줄일 수 없어요.
# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle는 종류를 알 수 없는 파티션의 크기를 줄일 수 없어요.
# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = 디스크를 암호화하려면 비밀번호가 필요해요.
# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = 시스템을 복사할 수 없어요: { $why }
# Red line at the foot: the copy of the system failed.
installer-copy-failed = 시스템을 복사할 수 없어요. { $why }
# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = 스왑 파일을 둘 공간이 없어요.
# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = Trindle가 디스크에서 스왑 파일의 위치를 읽을 수 없었어요.
# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Trindle가 새 파티션을 식별할 수 없었어요.
# Red line at the foot: the time zone could not be set.
installer-timezone-failed = 시간대를 설정할 수 없어요: { $why }
# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Trindle가 이 컴퓨터에 부팅 메뉴를 설치할 수 없었어요.
# Said beside how far the install has got, once it has started.
installer-started = 설치 중
# The button that ends an install under way.
installer-halt = 중지
# Beside Stop after its first press.
installer-halt-sure = 설치를 중지할까요? 디스크의 이전 내용은 이미 사라졌어요.
# While an install is being stopped.
installer-halting = 중지하는 중
# When Stop could not let go of the disk.
installer-halt-stuck = Trindle가 디스크를 놓을 수 없었어요. 컴퓨터를 다시 시작한 뒤 다시 해 보세요.
# The copy is done and the person is still answering.
installer-step-waiting = 복사했어요. 답을 기다리는 중이에요.

# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle에는 메모리가 1 GB 이상 필요한데, 이 컴퓨터는 그보다 적어요.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = 디스크 암호화는 UEFI로 시작하는 컴퓨터에서만 할 수 있는데, 이 컴퓨터는 아니에요.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = { $part }의 Windows가 최대 절전 모드이거나 빠른 시작으로 종료되었어요. 지금 크기를 줄이면 파일을 잃을 수 있어요. Windows를 시작해서 다시 시작을 선택한 뒤 다시 해 보세요.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle가 설치되었지만 스냅샷을 설정할 수 없었어요: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle가 설치되었지만 부팅 메뉴에 스냅샷을 추가할 수 없었어요: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = 모두 설정되지는 않았어요
# Red line under the disk password boxes: the computer gave no random numbers, so no passphrase could be made.
installer-suggest-failed = Trindle가 암호 문구를 만들 수 없었어요. 직접 입력하세요.
# Red words on the disk page: the disk was swapped for another after the list was read.
installer-disk-changed = { $dev }은(는) 보여 드린 디스크가 아니에요. 디스크를 다시 선택하세요.
# Red line at the foot: a disk with an old-style partition table cannot be shared with.
installer-no-gpt = { $dev }은(는) GPT 파티션 테이블을 쓰지 않아서 지금 있는 내용 옆에 Trindle를 설치할 수 없어요.
# Red line at the foot: the boot partition on the disk is too small to share.
installer-boot-partition-small = 이 디스크의 부팅 파티션이 함께 쓰기에 너무 작아서 지금 있는 내용 옆에 Trindle를 설치할 수 없어요.
# Note on the last page: the swap file was not made, with why.
installer-swap-failed = Trindle가 설치되었지만 스왑 파일을 만들 수 없었어요: { $why }
# Red line at the foot: a partition's identity could not be read after it was made.
installer-uuid-missing = Trindle가 설정한 뒤 { $dev }을(를) 식별할 수 없었어요.
# Words on the last page: the disk that will be replaced, with what it holds.
installer-ready-replaced-known = { $disk }의 모든 내용이 바뀌어요 ({ $holds }).
# A disk named with its model and size, then its place such as /dev/sda.
installer-disk-named = { $what }, { $dev }
# Red line at the foot: a program the installer runs stopped before it had all it was given.
installer-run-stopped = { $program }이(가) 필요한 것을 다 받기 전에 멈췄어요: { $why }
# Red words on the last page: the TPM chip could not be set up to open the disk, so the disk password is asked for at every start.
installer-chip-not-set-up = TPM 칩을 설정할 수 없어서 컴퓨터를 켤 때마다 디스크 비밀번호를 물어봐요.
# Red words on the last page: the keyboard for the disk password screen could not be set, so it may be the American one.
installer-start-keyboard-failed = Trindle가 설치되었지만 디스크 비밀번호 화면이 미국식 키보드를 쓸 수 있어요.
# Red words on the last page: the login screen could not be switched on; { $why } is the reason.
installer-login-screen-failed = Trindle가 설치되었지만 로그인 화면을 켤 수 없었어요: { $why }
# Red words on the last page: Trindle could not be set to take its own updates; { $why } is the reason.
installer-updates-held = Trindle가 설치되었지만 아직 자체 업데이트를 받지 않아요: { $why }
# Red words on the last page: earlier versions of people's files (home snapshots) could not be switched on.
installer-home-snapshots-failed = Trindle가 설치되었지만 파일의 이전 버전 기능이 꺼져 있어요. 설정의 스냅샷에서 켤 수 있어요.
# Red words on the last page: Trindle could not be added to the computer's own start-up list; { $why } is the reason.
installer-firmware-entry-failed = Trindle가 설치되었지만 컴퓨터의 시작 목록에 추가할 수 없었어요: { $why }
# Red line: the account the stick ran as could not be removed from the new system, so the install stopped.
installer-live-account-left = 스틱의 계정 { $user }을(를) 새 시스템에서 제거할 수 없었어요.

# Under the swap file's switch when a size smaller than memory is chosen.
installer-swap-no-hibernate = 이 컴퓨터의 메모리보다 작아서 최대 절전 모드를 쓸 수 없어요.

# In the swap size list: the size of the computer's memory, which hibernating needs.
installer-swap-size-hibernate = { $gb } GB, 최대 절전 모드에 충분해요

# In the swap size list: a smaller size.
installer-swap-size = { $gb } GB

# Tip of the swap size list.
installer-swap-size-tip = 스왑 파일의 크기

# Why logind holds off sleep and shutting down while the installer changes the disk (shown by systemd-inhibit --list).
installer-power-held = Trindle 설치 중
