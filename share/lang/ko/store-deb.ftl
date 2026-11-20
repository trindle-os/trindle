# machine translated (Korean), not yet reviewed by a person.

# Apps: the window for a .deb file, and programs kept in a box (src/store/deb.rs, boxed/, parts.rs, loose.rs).

## The window for a .deb file (src/store/deb.rs)

# Window title of the window opened for a .deb file.
store-deb-title = 패키지 설치
# Words at the top of the window while the package is read.
store-deb-reading = 패키지를 읽는 중…
# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = 이 파일은 설치할 수 있는 Debian 패키지가 아니에요. 다운로드가 끝나지 않았을 수 있어요.
# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name }을(를) 자체 폴더에 설치할 수 있어요.
# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = 이 패키지는 { $arch } 컴퓨터용이라서 이 컴퓨터에는 설치할 수 없어요.
# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = 이것은 Trindle 자체예요. 파일이 아니라 앱의 업데이트 페이지에서 업데이트하세요.
# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name }은(는) 드라이버나 백그라운드 서비스예요. 자체 폴더에서는 실행할 수 없어서 여기에 설치할 수 없어요.
# Words on the caution card above every package.
store-deb-caution = Debian과 Trindle 밖에서 온 패키지는 자체 폴더에 설치돼요. 그곳에서 제한되지만 완전히 차단되지는 않아요. 믿을 수 있는 제작자의 패키지만 설치하세요.
# Words at the top of the window once the package is installed.
store-deb-installed = { $name }이(가) 자체 폴더에 설치되었어요.
# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = 출처를 확인하는 중…
# Heading in the card that says where the file came from.
store-deb-where = 출처
# Tip and spoken name of the round button that removes the package and what it saved.
store-deb-remove = 패키지와 데이터 제거
# Word beside the ring while the package goes in.
store-deb-installing = 설치 중
# Main button that installs the package.
store-deb-install = 설치
# Main button that installs the same version again.
store-deb-reinstall = 다시 설치
# Main button that brings the installed package up to this version.
store-deb-update = 업데이트
# Heading of the card for a package not installed yet.
store-deb-goes-in = 제한된 자체 폴더에 설치돼요
# Under that heading.
store-deb-goes-in-note = 비밀번호는 필요 없어요. 허용하기 전에는 인터넷이나 소리를 쓸 수 없고 내 파일도 볼 수 없어요.
# Heading of the card for a package installed at this version.
store-deb-same-version = 버전 { $version }이(가) 자체 폴더에 설치되어 있어요
# Heading of the card for a package installed at another version.
store-deb-other-version = 버전 { $version }이(가) 자체 폴더에 설치되어 있어요
# Under that heading: the version this brings.
store-deb-other-version-note = 버전 { $version }(으)로 업데이트하고, 저장된 내용은 그대로 둬요.
# Facts line: how big the package is once installed.
store-deb-fact-size = 설치 후 크기
# Facts line: the computer kind it is made for.
store-deb-fact-arch = 대상
# Text of the About card when the package has no description.
store-deb-no-description = 이 패키지에는 설명이 없어요.
# Words at the top of the window while the package goes in.
store-deb-installing-note = 자체 폴더에 설치하는 중이에요. 필요한 부분을 내려받느라 몇 분 걸릴 수 있어요.
# Words at the top of the window once the package is installed.
store-deb-done = 자체 폴더에 설치되었어요. 링에서 찾을 수 있어요.
# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = 자체 폴더에 설치되었지만 다음 부분이 없어서 시작되지 않을 수 있어요: { $parts }.
# Words at the top of the window once the package is removed.
store-deb-removed = 자체 폴더에서 저장된 모든 내용과 함께 제거되었어요.

## Programs kept in a box (src/store/boxed/, parts.rs, loose.rs)

# Red words when a file is not a Debian package.
store-boxed-not-deb = 이 파일은 설치할 수 있는 Debian 패키지가 아니에요.
# Step shown while a package is unpacked.
store-boxed-unpacking = { $name } 푸는 중
# Step shown while a part a package needs is added.
store-boxed-adding = { $part } 추가하는 중
# Red words when a file is not a Debian package.
store-parts-not-deb = { $path }은(는) 설치할 수 있는 Debian 패키지가 아니에요.
# Red words when a package could not be unpacked.
store-parts-not-unpacked = { $path }을(를) 풀 수 없었어요.
# Red words when the program is not one kept in a box.
store-loose-not-boxed = 그 프로그램은 격리 공간에서 실행되지 않아요.
# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = 인터넷 없이 열렸어요
# Body of that notice: why, and what the program was given.
store-downloaded-why = 다운로드한 파일이라서 프로그램은 그 파일 하나만 읽을 수 있어요.
# Name of Trindle's own terminal in the ring.
store-boxed-terminal = 터미널
# Title of the notice when older programs are off.
store-boxed-x-off = 오래된 프로그램이 꺼져 있어요
# Body of that notice: where to turn them on.
store-boxed-x-off-where = 설정, 권한
# Title of the notice when too many older programs are open.
store-boxed-x-many = 오래된 프로그램이 너무 많아요
# Body of that notice.
store-boxed-x-many-close = 하나를 닫고 다시 시도하세요
# Title of the notice when the desktop could not be asked for a display for an older program.
store-boxed-x-unasked = 오래된 프로그램을 열 수 없어요
# Body of that notice: what to do.
store-boxed-x-unasked-why = 로그아웃했다가 다시 로그인한 뒤 다시 시도하세요
# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app }을(를) 열 수 없었어요
# Body of the notice the first time a terminal opens outside its box.
store-boxed-terminal-told = 터미널은 다른 앱보다 내 컴퓨터의 더 많은 부분에 접근할 수 있어요. 이해하는 명령, 믿을 수 있는 사람의 명령만 입력하세요.
# Title of the notice when a plugin allowed the local network has none.
store-boxed-home-off = 로컬 네트워크 없음
# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app }에 네트워크가 없어요: { $why }
# Why a plugin allowed the local network has none: Trindle could not find its own program.
store-boxed-home-why-self = Trindle의 프로그램이 없어요
# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare가 설치되어 있지 않아요
# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta가 설치되어 있지 않아요
# The same: the network room was not made.
store-boxed-home-why-not-made = 네트워크가 설정되지 않았어요
# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = 네트워크가 이미 있었어요
# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = Trindle이 안전 필터를 설정할 수 없었어요
# The same: the room's rules would not load.
store-boxed-home-why-rules = 네트워크 규칙을 불러올 수 없었어요
# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program }을(를) 시작할 수 없었어요: { $why }
# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta의 응답: { $why }
# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta가 연결할 수 없었어요

# Said when an app can't be installed because its package has a link as a top folder. { $name } is the folder.
store-parts-link = 최상위 폴더 { $name }이(가) 링크라서 이 패키지를 쓸 수 없어요.
# Said when an app can't be installed in a box because it is a driver or a background service. { $name } is the app.
store-boxed-driver = { $name }은(는) 드라이버나 백그라운드 서비스예요. 이런 것은 자체 폴더에서 작동하지 않아서 설치하지 않았어요.
