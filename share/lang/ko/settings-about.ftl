# machine translated (Korean), not yet reviewed by a person.

# The About this computer page of Settings: the computer's name, what it is made of, what runs on it and how long it takes to start.

## About (src/settings/panels/about.rs)

# Name of the About page in the Settings side bar and its title.
settings-about-title = 이 컴퓨터 정보
# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = 호스트 이름 컴퓨터 이름 네트워크 이름 정보 시스템 버전 hostname debian trindle 메모리 ram 프로세서 cpu 디스크 그래픽 화면 모델 커널 롤링 릴리스 활동 실행 중인 프로그램 작업 관리자 시스템 모니터 드라이브 usb 공간 업데이트 업그레이드 확인 자동 보안 수정
# Text behind the info button of the About page.
settings-about-help = 이 페이지에는 네트워크의 다른 컴퓨터에 보이는 이 컴퓨터의 이름, 컴퓨터의 구성, 시작하는 데 걸리는 시간이 나와요. 활동은 컴퓨터가 지금 무엇을 하는지, 어떤 프로그램이 메모리와 프로세서를 쓰는지 보여 줘요. 드라이브는 드라이브와 남은 공간을 보여 줘요. 각각 자기 창에서 열려요. 자세히 보기의 화살표를 눌러 여세요.
# What is shown when a fact about the computer cannot be found.
settings-about-unknown = 알 수 없음
# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
       *[other] { $model }, 스레드 { $cores }개
    }
# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Intel 그래픽
# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = AMD 그래픽
# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = NVIDIA 그래픽
# The graphics line inside a virtual machine.
settings-about-gpu-virtual = 가상 머신 그래픽
# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = 그래픽
# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height }, { $hz } Hz
# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } 사용 중
# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } 남음
# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = 전체 { $amount }
# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, { $on } 동안 켜져 있어요.
# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, { $on } 동안 켜져 있어요. 디스크가 거의 가득 찼어요.
# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n }초
# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n }분
# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
       *[other] { $n }분
    }
# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
       *[other] { $n }시간
    }
# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
       *[other] { $n }일
    }
# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = 문자, 숫자, 하이픈만 쓰세요. 하이픈으로 시작하거나 끝내지 마세요.
# Words under the computer name box while the password window is open.
settings-about-name-waiting = 비밀번호를 기다리는 중…
# Red words under the computer name box when renaming failed.
settings-about-name-failed = 컴퓨터 이름을 바꾸지 못했어요.
# Words under the computer name box after renaming worked.
settings-about-name-done = 이미 열려 있는 프로그램은 다음에 로그인한 뒤부터 새 이름을 써요.
# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = 컴퓨터 이름
# Tip of the save button beside the computer name box.
settings-about-rename = 이 컴퓨터 이름 바꾸기
# Bold line of the computer name setting.
settings-about-name = 컴퓨터 이름
# Heading of the card with the computer's name.
settings-about-name-group = 이름
# Text behind the info button of the Name card.
settings-about-name-about = 네트워크와 터미널의 프롬프트에 나오는 이 컴퓨터의 이름이에요. 문자, 숫자, 하이픈을 쓸 수 있지만 하이픈으로 시작하거나 끝내면 안 돼요. 바꾸려면 비밀번호를 입력해야 해요.
# Heading of the card about the computer's hardware.
settings-about-hardware = 컴퓨터
# Text behind the info button of the hardware card.
settings-about-hardware-about = 컴퓨터의 제조사와 모델, 프로세서, 그래픽이 나와요. 프로세서가 한 번에 돌릴 수 있는 스레드 수와, 사용 중인 화면마다 크기와 주사율도 보여 줘요.
# Heading of the card with the switch for looking for updates.
settings-about-updates = 업데이트
# Text behind the info button of the updates card.
settings-about-updates-about = Trindle은 로그인한 직후에, 그다음에는 몇 시간마다 앱, 플러그인, 펌웨어, Trindle 자체의 새 버전이 있는지 확인해요. 이것을 끄면 앱에서 업데이트 확인을 누르거나 링의 업데이트 버튼을 누를 때만 확인해요. Debian의 보안 수정은 매일 설치되고 별도의 스위치가 있어요. 그것을 끄면 저절로 설치되지 않아서, 알려진 보안 구멍이 업데이트할 때까지 그대로 남아요. 어느 스위치든 바꾸려면 관리자 비밀번호가 필요해요. 컴퓨터 전체가 내 선택을 따라요.
# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = 업데이트 확인
# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = 비밀번호를 기다리는 중…
# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = 보안 업데이트
# Warning line under the Security updates switch when it is off.
settings-about-security-off = 보안 수정이 더 이상 저절로 설치되지 않아요.
# Heading of the card about the software.
settings-about-software = 소프트웨어
# Text behind the info button of the software card.
settings-about-software-about = Trindle은 Debian 위에서 돌아가는 데스크톱이에요. Debian은 그 아래의 프로그램을 관리해요. 커널은 하드웨어와 함께 일하는 부분이에요. 데스크톱은 Trindle 자체 컴포지터가 Wayland로 그려요.
# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle 자체 컴포지터
# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
       *[other] 앱 { $n }개
    }
# Heading of the card with the ways to Activity and Drives.
settings-about-more = 자세히 보기
# Text behind the info button of the More detail card.
settings-about-more-about = 활동은 컴퓨터가 지금 무엇을 하는지, 어떤 프로그램이 메모리와 프로세서를 쓰는지 보여 줘요. 드라이브는 모든 드라이브와 사용량, 상태, USB 메모리를 보여 줘요.
# Bold line of the setting that opens Activity.
settings-about-activity = 실행 중인 것 보기
# Tip of the arrow that opens the Activity program.
settings-about-activity-name = 활동
# Bold line of the setting that opens Drives.
settings-about-drives = 드라이브와 공간
# Tip of the arrow that opens the Drives program.
settings-about-drives-name = 드라이브
# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle이 시작 시간을 다운로드 폴더에 저장하지 못했어요.
# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = 날짜와 시간,초
# File name of the saved start-up times, in Downloads.
settings-about-times-file = Trindle 시작 시간.csv
# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = 다운로드 폴더에 { $file } 이름으로 저장했어요
# Label of the word button that saves every start-up time.
settings-about-times-save = 저장
# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = 모든 시작 시간을 다운로드 폴더에 저장
# Bold line of the setting that saves the start-up times.
settings-about-times-keep = 시작 시간 저장
# Added after the words behind the i of its card.
settings-about-times-keep-sub = 스프레드시트로 다운로드 폴더에 저장돼요.
# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = 막대 하나가 한 번의 시작이에요. 가장 최근 것이 오른쪽에 있어요.
# Heading of the card with the last ten start-up times.
settings-about-times = 최근 시작 열 번
# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle은 전원 버튼을 누른 때부터 로그인 화면까지 걸린 시간을 매번 재요. 디스크 비밀번호를 입력한 시간은 세지 않아요. 가장 최근 것이 맨 위에 있어요.
# Heading of the memory tile.
settings-about-memory = 메모리
# Heading of the disk tile.
settings-about-disk = 디스크

## Problem reports card (src/settings/panels/report_card.rs)

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = 문제 보고서
# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = 보고서는 Trindle을 함께 만드는 커뮤니티인 Buildmorphic으로 가요. 이메일 주소만 있으면 참여할 수 있어요. 직접 고치지 못해도, 가장 먼저 보고한 문제에 대해 크레딧과 레벨을 얻어요. 보고서에는 정해진 항목의 정보만 담기고, 보내기를 누르기 전에는 아무것도 전송되지 않아요. 사용자 이름이나 GitHub 이름은 선택이에요. 사용자 이름은 이메일 주소가 아니에요.
# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = 문제가 생기면 보고서 보내기를 제안
# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Buildmorphic 사용자 이름
# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = GitHub 이름
# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = 커뮤니티 소개
# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
       *[other] 꺼짐. 대기 중이던 보고서 { $n }개를 삭제했어요.
    }
# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
       *[other] 꺼짐. 대기 중인 보고서 { $n }개를 삭제하지 못했어요.
    }
# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = 꺼졌지만 보고 추가 기능의 대기 중인 보고서는 삭제되지 않았어요.
