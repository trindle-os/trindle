# machine translated (Korean), not yet reviewed by a person.

# Settings, Network page and the radio words shared with Bluetooth.

## Radio switches (src/common/rfkill.rs)

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio }이(가) 컴퓨터의 스위치나 키로 꺼져 있어요. 켠 다음 다시 시도하세요.
# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = 시스템이 { $radio }을(를) 막고 있고, Trindle가 그 차단을 풀지 못했어요.

## Network page (src/settings/panels/network.rs)

# Said in red under a Wi-Fi network when its password was typed and not accepted.
settings-network-join-password-typed = 그 비밀번호는 받아들여지지 않았어요. 다시 입력해 보세요.
# Said in red under a Wi-Fi network when the password kept for it was not accepted.
settings-network-join-password-kept = 이 네트워크에 저장된 비밀번호가 받아들여지지 않았어요. 휴지통 버튼으로 네트워크를 잊은 다음 다시 연결하세요.
# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Trindle가 그 네트워크를 찾지 못했어요. 이름을 확인하고, 범위 안에 있는지 확인하세요.
# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = 이 컴퓨터가 네트워크에 연결됐지만 주소를 받지 못했어요. 다시 시도하거나 공유기를 다시 시작하세요.
# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = 이 컴퓨터가 네트워크에 연결하지 못했어요. 공유기에 더 가까이 가서 다시 시도하세요.
# Said in red when the password typed is one a Wi-Fi network cannot have.
settings-network-bad-password = Wi-Fi 비밀번호가 될 수 없는 값이에요. 확인한 뒤 다시 시도하세요.
# Said in red when joining a network failed for no known reason.
settings-network-join-refused = 네트워크에 연결하지 못했어요.
# Small line of a shared folder: who may open it with a password.
settings-network-access-open = 내 비밀번호로 열기
# Small line of a shared folder: who may open and change it with a password.
settings-network-access-change = 내 비밀번호로 열기와 바꾸기
# Small line of a shared folder: anyone on the local network may open it.
settings-network-access-guests-open = 가까이 있는 누구나 열 수 있어요
# Small line of a shared folder: anyone on the local network may change it.
settings-network-access-guests-change = 가까이 있는 누구나 바꿀 수 있어요
# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }
# Tip of the round button that stops sharing a folder; the folder's name follows.
settings-network-stop-sharing = { $name } 공유 끝내기
# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = 관리되지 않음
# State of a network device that cannot be used.
settings-network-state-unavailable = 쓸 수 없음
# State of a network device that is not connected.
settings-network-state-disconnected = 연결 끊김
# State of a network device that is getting ready to connect.
settings-network-state-preparing = 준비 중
# State of a network device that is being set up.
settings-network-state-configuring = 설정하는 중
# State of a network device that needs a password.
settings-network-state-needs-password = 비밀번호 필요
# State of a network device that is getting an address.
settings-network-state-address = 주소 받는 중
# State of a network device whose connection is being checked.
settings-network-state-checking = 확인하는 중
# State of a network device that is waiting.
settings-network-state-waiting = 기다리는 중
# State of a network device that is connected.
settings-network-state-connected = 연결됨
# State of a network device that is disconnecting.
settings-network-state-disconnecting = 연결 끊는 중
# State of a network device that failed.
settings-network-state-failed = 실패
# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = 케이블로 연결됐고, Wi-Fi로 { $name }에도 연결됐어요.
# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] { $name }에 연결됐어요. 신호가 강해요.
        [good] { $name }에 연결됐어요. 신호가 좋아요.
        [fair] { $name }에 연결됐어요. 신호가 보통이에요.
       *[other] { $name }에 연결됐어요. 신호가 약해요.
    }
# Headline of the page when connected by cable only.
settings-network-top-cable = 케이블로 연결됐어요.
# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi가 꺼져 있어서 이 컴퓨터는 연결되지 않았어요.
# Headline of the page when not connected.
settings-network-top-none = 연결되지 않았어요. 아래에서 네트워크를 고르세요.
# Headline of the page while the networks are looked for.
settings-network-top-looking = 네트워크를 찾는 중…
# Headline of the page when NetworkManager is not running.
settings-network-top-gone = NetworkManager가 실행되고 있지 않아서 Trindle가 네트워크를 보여 줄 수 없어요.
# The word for the system bus in "The system bus is not available".
settings-network-the-bus = 시스템 버스
# Name of the page in the list of settings pages and its heading.
settings-network-title = 네트워크
# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi wireless internet ethernet wired password connect firewall sharing share printer folder 와이파이 무선 인터넷 이더넷 유선 비밀번호 연결 방화벽 공유 프린터 폴더
# Help text at the top of the page.
settings-network-help = Wi-Fi를 켜고 끄거나, 네트워크에 연결하거나, 필요 없는 네트워크를 잊거나, 다른 컴퓨터의 연결을 허용할 수 있어요.
# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] { $name }의 신호가 강해요.
        [good] { $name }의 신호가 좋아요.
        [fair] { $name }의 신호가 보통이에요.
       *[other] { $name }의 신호가 약해요.
    }
# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Wi-Fi를 쓰려면 더 새로운 Linux가 필요해요.
# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = 이 컴퓨터에는 작동하는 Wi-Fi가 없어요.
# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = 꺼짐. 네트워크에 연결하려면 켜세요.
# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = 켜져 있지만 네트워크에 연결되지 않았어요.
# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Wi-Fi 사용
# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi
# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi와 케이블
# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi는 케이블 없이 이 컴퓨터를 네트워크에 연결해요. 끄면 네트워크 찾기를 멈추고 배터리를 조금 아껴요. 이름을 숨기는 네트워크는 범위 안의 네트워크 목록에 나타나지 않아요. 연결하려면 대문자까지 포함해 이름을 정확히 입력하세요. 일부 Wi-Fi 칩은 Trindle가 쓰는 Linux 버전에 아직 없는 드라이버가 필요해요. 그런 경우에는 더 새로운 Linux 버전이 필요해요. 네트워크 케이블은 연결하거나 끊는 버튼이 있는 자신의 줄이 따로 있어요.
# Heading of the line with the switch that lets other computers connect, and the name of the switch.
settings-network-share-switch = 다른 컴퓨터의 연결 허용
# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = 공유 폴더를 찾는 중…
# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = 공유 폴더를 읽을 수 없어요
# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = 아직 접근할 수 없어요
# Small line under it.
settings-network-not-reachable-sub = 공유가 꺼져 있는 동안은 다른 컴퓨터가 이것들을 열 수 없어요.
# Line in the Sharing card when no folder is shared.
settings-network-no-shared = 공유된 폴더가 없어요
# Heading of the Sharing card.
settings-network-sharing = 공유
# Help text (i button) of the Sharing card.
settings-network-sharing-about = 다른 컴퓨터의 연결 허용이 꺼져 있으면, 네트워크의 다른 컴퓨터는 이 컴퓨터가 먼저 연락한 경우에만 이 컴퓨터에 닿을 수 있어요. 같은 Wi-Fi에 있는 누구도 들어오려고 시도하지 못해요. 집의 다른 컴퓨터와 폴더나 프린터를 공유하려면 켜세요. 폴더를 공유하려면 폴더에서 폴더를 오른쪽 클릭하고 네트워크에서 공유를 고르세요. 공유 폴더마다 누가 무엇을 할 수 있는지 표시돼요. 내 비밀번호로: 공유 비밀번호가 있는 사람은 파일을 열거나, 열고 바꿀 수 있어요. 집의 누구나: 네트워크의 누구나 비밀번호 없이 파일을 열거나, 열고 바꿀 수 있어요. 스위치가 꺼져 있는 동안은 집의 다른 컴퓨터가 어느 것도 열 수 없어요.
# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = 끝내는 중…
# Shown beside the sharing switch while the password is asked for.
settings-network-waiting = 비밀번호를 기다리는 중…
# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = 공유가 켜졌어요
# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = 차단됨
# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi를 켜지 못했어요.
# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi를 끄지 못했어요.
# Name of a wired network device that has no name of its own.
settings-network-ethernet = 이더넷
# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = 연결 끊기
# Tip of the button that connects a network or a cable.
settings-network-connect = 연결
# Shown under a network or cable while it is being connected.
settings-network-connecting = 연결하는 중…
# Line in the Networks in range card when there are none.
settings-network-none-in-range = 범위 안에 네트워크가 없어요
# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = 범위 안의 네트워크
# Help text (i button) of the Networks in range card.
settings-network-in-range-about = 이 컴퓨터가 잡을 수 있는 모든 Wi-Fi 네트워크예요. 페이지를 열면 쓰고 있는 네트워크가 맨 위에 오고, 그다음 저장된 네트워크, 그다음 나머지가 신호 세기 순으로 나와요. 페이지가 열려 있는 동안 순서는 바뀌지 않아요. 화살표를 눌러 네트워크에 연결하세요. 보안 네트워크는 처음 연결할 때 비밀번호를 물어요. 휴지통 버튼은 저장된 네트워크와 그 비밀번호를 잊어요. 열린 네트워크는 암호화되지 않아서 가까이 있는 사람이 보낸 내용을 볼 수 있어요.
# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = 범위 안의 네트워크를 보려면 Wi-Fi를 켜세요.
# Part of a network's small line: it is saved on this computer.
settings-network-saved = 저장됨
# Part of a network's small line: it needs a password.
settings-network-secured = 보안
# Part of a network's small line: it signs in with a username as well as a password, as work and university networks do.
settings-network-username = 사용자 이름 필요
# Red words when someone tries to join a network that signs in with a username.
settings-network-username-unsupported = 이 네트워크는 비밀번호와 함께 사용자 이름도 필요해요. Trindle는 아직 이런 네트워크에 연결할 수 없어요.
# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = 열림, 암호화 안 됨
# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s
# Tip of the bin button that forgets a saved network.
settings-network-forget = 이 네트워크 잊기
# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = 네트워크를 잊지 못했어요.
# Placeholder of the box where a network's password is typed.
settings-network-password = 비밀번호
# Tip of the button that joins the network.
settings-network-join = 연결
# Tip of the button that closes the box without joining.
settings-network-cancel = 취소
# Shown under a network while it is being joined.
settings-network-joining = 연결하는 중…
# Said in red when a Wi-Fi password is too short.
settings-network-password-short = Wi-Fi 비밀번호는 8자 이상이에요.
# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = 숨겨진 네트워크에 연결
# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = 네트워크 이름
# Placeholder of the box where a hidden network's password is typed.
settings-network-hidden-password = 비밀번호가 있으면 입력
# Said in red when no name was typed for a hidden network.
settings-network-name-needed = 네트워크 이름을 입력하세요.
# Shown under a network after it was joined.
settings-network-joined = 연결됨
# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } 네트워크는 그래도 저장됐어요. 다시 연결하기 전에 휴지통 버튼으로 잊으세요.
# Said in red when a network would not disconnect.
settings-network-disconnect-failed = 연결을 끊지 못했어요.

# Heading of the card that lists the ports opened by hand.
settings-network-ports = 열린 포트

# Help text (i button) of the Open ports card.
settings-network-ports-about = 이 컴퓨터가 먼저 연락하지 않은 컴퓨터에 대해서는 모든 포트가 닫혀 있어요. 게임 서버나 동기화 도구처럼 다른 컴퓨터의 연결을 기다리는 프로그램은 여기서 포트를 열기 전에는 접근할 수 없어요. 포트를 열면 이 컴퓨터와 같은 로컬 네트워크의 모든 컴퓨터가 들어올 수 있지만, 넓은 인터넷에서는 들어올 수 없어요. 카페, 호텔, 사무실 네트워크에서는 거기 있는 모든 사람이 해당돼요. 포트를 열거나 닫으려면 관리자 비밀번호가 필요해요.

# Line in the Open ports card when no port is open.
settings-network-ports-none = 열린 포트가 없어요



# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = { $port } 닫기

# Heading of the line that opens a port.
settings-network-port-add = 포트 열기


# Placeholder in the box where the port number is typed.
settings-network-port-number = 포트 번호

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = 프로토콜

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = 둘 다

# Button that opens the typed port.
settings-network-port-open = 열기

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = 1에서 65535 사이의 숫자를 입력하세요.

# Red words when the port is already open.
settings-network-port-already = 이미 열려 있어요.

# Said under the line once the port is open.
settings-network-port-opened = 열렸어요
