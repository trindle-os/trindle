# machine translated (Korean), not yet reviewed by a person.

# The ring's tray and the updates button: status buttons for sound, battery, network and Bluetooth, other programs' tray icons, and what the updates button says. Part of the ring's own words (see ring.ftl).

## Updates button (src/ring/updates.rs)

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
       *[other] 플러그인 업데이트 { $n }개를 앱에서 확인해야 해요
    }
# Answer to a spoken question about updates when none wait.
ring-updates-none = 모두 최신 상태예요
# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
       *[other] 업데이트 { $n }개가 기다리고 있어요. 앱 옆의 업데이트 버튼으로 설치하세요.
    }
# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look =
    { $n ->
       *[other] 업데이트 { $n }개가 기다리고 있어요. { $look }.
    }
# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = 업데이트 { $n }개가 기다리고 있어요. 앱 옆의 업데이트 버튼으로 그중 { $put }개를 설치하세요. { $look }.
# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
       *[other] 펌웨어 업데이트 { $n }개 보기
    }
# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
       *[other] 펌웨어 포함 업데이트 { $n }개 보기
    }
# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
       *[other] 업데이트 { $n }개 설치
    }
# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
       *[other] 이전보다 더 많은 권한을 요청하는 플러그인 업데이트 { $n }개 보기
    }
# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
       *[other] 업데이트 { $put }개 설치, { $look }
    }
# Name of the updates button when nothing waits.
ring-updates-up-to-date = 최신 상태
# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = 확인 꺼짐
# Name of the updates button when a restart to finish installing was put off with Remind me later and the time has come.
ring-updates-restart = 다시 시작해서 설치 끝내기

## Tray (src/ring/tray.rs)

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = 키보드
# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = 화면 밝기
# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = 깨어 있게 하기
# Name of the sound button when the sound is muted.
ring-tray-muted = 음소거됨
# Name of the sound button with the volume level.
ring-tray-volume = 음량 { $pct }%
# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] 배터리 { $pct }%, 충전 중
        [onbattery] 배터리 { $pct }%, 배터리 사용 중
        [full] 배터리 { $pct }%, 완전 충전
        [pluggedin] 배터리 { $pct }%, 전원 연결됨
       *[other] 배터리 { $pct }%
    }
# Name of the network button when networking is switched off.
ring-tray-net-off = 네트워크가 꺼져 있어요
# Name of the network button when nothing is connected.
ring-tray-not-connected = 연결 안 됨
# Name of the network button when the connection is through a VPN.
ring-tray-vpn = VPN으로 연결됨
# Name of the network button when connected by cable.
ring-tray-wired = 유선
# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = 유선, 인터넷 없음
# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, 신호 { $strength }%
# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, 신호 { $strength }%, 제한된 접속
# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth가 꺼져 있어요
# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = { $names }에 연결됨
# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth가 켜져 있어요

## Other programs' tray icons (src/ring/sni/)

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = 트레이 아이콘
# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = 모든 트레이 아이콘
# Words on the panel of every tray icon when there are none.
ring-tray-none = 트레이 아이콘이 없어요
# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = 링에 표시
# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = 링에서 빼기
# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = 링에 이미 아이콘 네 개가 있어요. 먼저 하나를 빼세요.
