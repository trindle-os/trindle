# machine translated (Korean), not yet reviewed by a person.

# Settings: the Permissions page (src/settings/panels/permissions.rs).

## The Permissions page (src/settings/panels/permissions.rs)

# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = 권한
# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = 권한 허용 개인정보 보호 flatpak flathub 샌드박스 인터넷 네트워크 파일 소리 마이크 카메라 bluetooth 알림 플러그인 deb x11 xwayland 오래된 프로그램 steam wine java 화면 터미널 컴퓨터 thunderbolt usb4 도킹 장치 외부 알림 다운로드 내려받은
# Help line at the foot of the Permissions page.
settings-permissions-help = 이 페이지에서 각 앱이 쓸 수 있는 것을 볼 수 있어요. Flathub 앱, 자체 폴더에 설치된 프로그램, 플러그인은 각자 격리 공간에서 실행되고, 기본으로 제한이 걸려 있어요. Flathub 앱을 바꾸면 다음에 앱이 시작될 때 적용돼요. 허용한 Thunderbolt와 USB4 장치는 묻지 않고 항상 연결돼요. 다시 묻게 하려면 그 장치를 잊게 하세요.
# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = 바꿀 수 없었어요.
# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = 저장했지만 Trindle이 플러그인을 다시 시작할 수 없었어요. 다음에 시작될 때 적용돼요.
# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = 저장됨
# Help text of the AppImages card (the information button).
settings-permissions-loose-images = 각 AppImage는 자기만의 홈 폴더가 있는 격리 공간에서 실행돼요. 바꾸면 다음에 AppImage가 시작될 때 적용돼요.
# Help text of the Started from Folders card (the information button).
settings-permissions-loose-programs = Folders에서 시작한 각 프로그램은 자기만의 홈 폴더가 있는 격리 공간에서 실행돼요. 바꾸면 다음에 프로그램이 시작될 때 적용돼요.
# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title }: { $words }
# Under the Allow older programs switch.
settings-permissions-older-why = 각각 따로 실행돼요. 끄면 열리지 않아요.
# Heading of the card with the switch for older programs.
settings-permissions-older = 오래된 프로그램
# Label and tip of the switch that lets older programs open.
settings-permissions-allow-older = 오래된 프로그램 허용
# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = 이 포트는 화면 전용이라서 어떤 장치도 이 컴퓨터의 메모리에 닿을 수 없어요. 물어볼 것이 없어요.
# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = 이 포트는 USB와 화면 전용이라서 어떤 장치도 이 컴퓨터의 메모리에 닿을 수 없어요. 물어볼 것이 없어요.
# Help text on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-lets-in-all = 이 포트는 모든 장치를 연결해 주기 때문에 물어볼 것이 없어요. 이것은 컴퓨터의 펌웨어에서 정해져요.
# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = 이 포트는 화면만 써요.
# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = 이 포트는 USB와 화면만 써요.
# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
settings-permissions-short-firmware = 펌웨어가 모든 장치를 연결해 줘요.
# Under a package's name: its version number is { $v }.
settings-permissions-version = 버전 { $v }
# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = 버전 { $v }. 요청하는 것이 없어요.
# Under a plugin's name: its version number is { $v }, and { $notes } lists what it may do that matters most, such as "it can act as you and it can type into your windows".
settings-permissions-version-notes = 버전 { $v }: { $notes }.
# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }
# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = 잊게 할 수 없었어요.
# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt와 USB4
# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
settings-permissions-thunder-about = 허용한 Thunderbolt와 USB4 장치는 묻지 않고 항상 연결돼요. 다시 묻게 하려면 그 장치를 잊게 하세요. { $about }
# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = 포트
# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = 포트를 확인하는 중…
# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = 허용한 장치
# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = 불러오는 중…
# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = 메모리 보호
# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = Thunderbolt나 USB4 포트 없음
# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = 그래서 물어볼 것이 없어요.
# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = 이 방식으로 연결되는 장치가 없어요.
# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = 물어볼 것 없음
# Under Devices allowed when the ports let every device in themselves.
settings-permissions-every-one = 모두 허용돼요. 포트가 정해요.
# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
settings-permissions-iommu = 이 컴퓨터는 장치가 자기 것이 아닌 메모리에 닿는 것도 막아 줘서 위험이 더 작아요. 장치를 잊게 해도 뽑기 전까지는 계속 연결돼 있어요.
# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
settings-permissions-no-iommu = 장치를 잊게 해도 뽑기 전까지는 계속 연결돼 있어요. 다음에 다시 꽂으면 다시 물어봐요.
# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
settings-permissions-asked-before = 새 장치는 물어봐요
# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = Trindle이 허용한 장치 목록을 읽을 수 없었어요: { $why }
# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = 묻지 않고 연결하도록 허용한 장치가 아직 없어요.
# Tip of the button that forgets an allowed Thunderbolt or USB4 device.
settings-permissions-forget-device = 이 장치 잊기
# Name given to a Thunderbolt or USB4 device that has none.
settings-permissions-a-device = Thunderbolt 장치
# Under an allowed device's name when its maker is not known.
settings-permissions-may-always = 항상 연결 가능
# Under an allowed device's name: the maker is { $vendor }.
settings-permissions-made-by = 제조사: { $vendor }. 항상 연결 가능.
# Headline while the list of apps is being read.
settings-permissions-reading-apps = 앱이 쓸 수 있는 것을 확인하는 중…
# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
       *[other] 앱 { $n }개가 쓸 수 있는 것을 확인하는 중…
    }
# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = 자체 격리 공간에서 실행되는 앱이나 플러그인을 설치하지 않았어요.
# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = 앱 한 개가 자체 격리 공간에서 실행되고, 여기서 허용한 것만 쓸 수 있어요.
# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
       *[other] 앱과 플러그인 { $n }개가 각자 격리 공간에서 실행되고, 여기서 허용한 것만 쓸 수 있어요.
    }
# Heading of the card for apps from Flathub.
settings-permissions-flathub-apps = Flathub 앱
# Heading of the card for programs installed in their own folder.
settings-permissions-own-folder = 프로그램
# Heading of the card for plugins.
settings-permissions-plugins = 플러그인
# Heading of the card for AppImages.
settings-permissions-appimages = AppImage
# Heading of the card for programs started from Folders.
settings-permissions-from-folders = Folders에서 시작
settings-permissions-other = 기타 설정
# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = 설치된 것이 없어요.
# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = 아직 없어요.
settings-permissions-not-added = Folders에서 열었고, 앱에는 없음
settings-permissions-kind-tip = 앱 종류
# Heading of the card that says nothing is installed of any kind.
# Help text of the Flathub apps card (the information button).
settings-permissions-flathub-about = Flathub의 각 앱은 자체 격리 공간에서 실행되고, 여기서 허용한 것만 쓸 수 있어요. 바꾸면 다음에 앱이 시작될 때 적용돼요.
# Help text of the In their own folder card (the information button).
settings-permissions-debs-about = 터미널처럼 Trindle과 함께 온 프로그램과 .deb 파일로 설치한 프로그램이에요. 각각 자기 폴더에서 실행되고, 기본으로 제한이 걸려 있어요.
# Help text of the Plugins card (the information button).
settings-permissions-plugins-about = 플러그인은 제한된 상태로 시작하고, 플러그인이 요청한 것만 허용할 수 있어요. 백그라운드에서 실행되는 플러그인은 무언가 바꾸면 바로 다시 시작돼요.
# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = 입력 코드
settings-permissions-codes-about = 입력할 때 Trindle이 글로 바꿔 주는 짧은 코드예요. 각 코드는 어느 플러그인의 것인지 보여 줘요.
settings-permissions-codes-none = 켜진 코드가 없어요
# Line on the typing codes card when the plugins folder could not be read: the list may be missing codes.
settings-permissions-codes-unread-line = 플러그인 폴더를 읽을 수 없었어요
# Red words under that line.
settings-permissions-codes-unread = Trindle이 플러그인 폴더를 읽을 수 없어서 일부 코드가 빠졌을 수 있어요.
# Button on a plugin's row: stops that plugin's codes.
settings-permissions-codes-off = { $name }의 코드 끄기
settings-permissions-codes-count =
    { $n ->
       *[other] 코드 { $n }개, { $id }
    }
# A code's second line: what it types and who owns it.
settings-permissions-code-types = { $name }의 "{ $words }" 입력
settings-permissions-codes-more =
    { $n ->
       *[other] 코드 { $n }개가 더 켜져 있어요
    }

## Programs outside a box, and downloaded files (src/settings/panels/outside_card.rs)

# Heading of the card for programs that are not in a box.
settings-outside-heading = 격리 공간 밖의 프로그램
# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindle은 몇 초마다 확인하고 찾은 것을 알려 줘요. 이 프로그램들은 막을 수 없고, 잠깐만 실행되는 것은 놓칠 수도 있어요. 격리 공간 안의 프로그램은 그 공간이 허용하는 것만 쓸 수 있어요.
# Label of the switch for the notice about the network.
settings-outside-network = 프로그램이 네트워크를 쓰면 알려 주기
# Label of the switch for the notice about the person's files.
settings-outside-files = 프로그램이 내 파일을 열면 알려 주기
# Under those two switches.
settings-outside-once = 프로그램마다 한 번만요.
# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = { $name } 무시 그만하기
# Under a program marked fine for the network.
settings-outside-fine-network = 네트워크를 써도 괜찮아요.
# Under a program marked fine for the person's files.
settings-outside-fine-files = 파일을 열어 놓아도 괜찮아요.
# The line in the card when no program is marked fine.
settings-outside-no-fine = 괜찮다고 표시한 프로그램이 없어요.
# Heading of the card for downloaded files.
settings-downloads-heading = 내려받은 파일
# Help text of that card (the information button).
settings-downloads-about = 다운로드 폴더의 파일과 브라우저가 내려받은 파일로 표시한 파일은 격리 공간에서 열려요. 파일을 여는 프로그램은 그 파일만 받아요. 읽을 수만 있고, 인터넷이나 내 폴더는 쓸 수 없어요. 브라우저의 열기 버튼은 프로그램을 직접 시작하므로 여기에 포함되지 않아요.
# Label of the switch that opens downloaded files in a box.
settings-downloads-box = 내려받은 파일을 격리 공간에서 열기
# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = { $kind } 파일을 일반 방식으로 열기
# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = { $kind } 파일
# Under a file type's switch.
settings-downloads-normal-sub = 일반 방식으로, 인터넷과 함께 열려요.
# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = 아직 파일 종류가 없어요. 내려받은 파일이 격리 공간에서 열리면 그 종류가 여기에 나타나요.
