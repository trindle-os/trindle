# machine translated (Korean), not yet reviewed by a person.

# Settings, Blocking page: adverts and bad sites kept out for the whole computer, and private name lookups.

## Blocking page (src/settings/panels/blocking.rs)

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = 방금
# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
       *[other] { $n }분 전
    }
# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
       *[other] { $n }시간 전
    }
# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
       *[other] { $n }일 전
    }
# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
settings-blocking-waiting = 비밀번호를 기다리는 중…
# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = 차단 켜짐
# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = 차단 꺼짐
# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = 저장했어요
# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = example.com처럼 사이트 이름을 입력하세요.
# Headline of the page when blocking is off.
settings-blocking-top-off = 차단이 꺼져 있어요. 웹사이트는 전과 같이 네트워크의 서버로 찾아요.
# Headline of the page while blocking is starting.
settings-blocking-top-starting = 차단을 시작하는 중이에요.
# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = 광고와 위험한 사이트를 차단하고, 웹사이트는 { $provider }을(를) 통해 비공개로 찾아요.
# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Trindle이 이 네트워크에서는 비공개 서버에 연결할 수 없어서 지금은 네트워크의 서버를 쓰고 있어요. 차단은 계속 작동해요.
# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = 광고와 위험한 사이트를 차단해요. 웹사이트는 네트워크의 서버로 찾는데, 이 서버는 방문하는 모든 사이트를 볼 수 있어요.
# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = 광고와 위험한 사이트를 차단해요. 웹사이트는 VPN의 서버로 찾아요.
# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = 광고와 위험한 사이트를 차단해요.
# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = 시작 중
# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = 꺼짐
# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = 차단을 시작하는 중
# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = 차단이 꺼져 있음
# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = 오늘 차단
# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = 오늘 차단한 조회
# Name of the tile about the lists of sites.
settings-blocking-tile-lists = 목록
# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = 업데이트 중
# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = 아직 없음
# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = 사이트 목록
# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = 목록의 사이트, { $ago } 업데이트
# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = 아직 차단한 것이 없어요
# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
settings-blocking-let-through = 이 사이트를 항상 허용
# Heading of the card that lists recently blocked sites.
settings-blocking-recent = 최근 차단
# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = 사이트가 차단되면 여기에 최신순으로 나타나요. 더하기를 누르면 앞으로 그 사이트를 허용해요. Trindle은 허용한 사이트의 기록을 남기지 않아요.
# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = 차단을 켜면 무엇을 차단하는지 볼 수 있어요.
# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = example.com
# Heading of the line with the box for adding a site.
settings-blocking-add = 사이트 추가
# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = 이 사이트를 다시 차단
# Heading of the card that lists the sites that are let through.
settings-blocking-allowed = 항상 허용
# Help text (i button) of the Always let through card.
settings-blocking-allowed-about = 이 사이트와 그 아래의 모든 주소는 목록에 있어도 차단하지 않아요. 빼기를 누르면 사이트를 다시 차단해요.
# Name of the page in the list of settings pages and its heading.
settings-blocking-title = 방화벽과 차단
# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = block adverts ads trackers malware scams dns private lookups quad9 mullvad cloudflare allow 차단 광고 추적기 악성코드 사기 비공개 조회 허용
# Help text at the top of the page.
settings-blocking-help = 이 컴퓨터의 모든 브라우저와 앱에서 광고, 추적기, 알려진 위험한 사이트를 차단해요. 또 암호화된 연결로 웹사이트를 찾아서, 지금 쓰는 네트워크가 어떤 사이트를 찾는지 보거나 바꿀 수 없어요.
# Text of the switch that turns blocking on or off.
settings-blocking-switch = 광고와 위험한 사이트 차단
# Tip of the dropdown that picks where lookups go.
settings-blocking-where = 조회 보낼 곳
# Help text (i button) of the Blocking card.
settings-blocking-card-about = HaGeZi의 광고, 추적기, 알려진 위험한 사이트 목록을 쓰고 하루에 한 번 업데이트해요. 차단이 꺼져 있으면 아무것도 실행되지 않아요. 웹사이트는 여기서 고른 비공개 서버를 통해 암호화된 연결로 찾아요. Quad9은 누가 물었는지 기록하지 않아요. "내 네트워크"도 차단은 하지만 비공개는 아니에요. 네트워크의 서버가 찾는 모든 사이트를 보기 때문이에요.
# Heading of the line with the private server dropdown.
settings-blocking-private = 비공개 서버
# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = 내 네트워크

# Heading of the Firewall card, first on the Firewall and blocking page.
settings-firewall = 방화벽

# Help text (i button) of the Firewall card.
settings-firewall-about = 방화벽이 켜져 있으면 이 컴퓨터가 먼저 시작한 연결이 아닌 한 다른 컴퓨터는 연결할 수 없어요. 그래서 같은 Wi-Fi에 있는 누구도 침입을 시도할 수 없어요. 웹 브라우징, 업데이트, 프린터는 계속 작동해요. 게임 서버처럼 다른 컴퓨터의 연결을 기다리는 프로그램은 아래에서 포트를 열어야 해요. 방화벽이 꺼져 있으면 네트워크의 모든 컴퓨터가 연결을 기다리는 모든 프로그램에 접근할 수 있어요. 켜거나 끄려면 관리자 비밀번호가 필요해요.

# Name of the firewall's on and off switch.
settings-firewall-switch = 방화벽


# Small line under the switch while the firewall is off.
settings-firewall-off-sub = 다른 컴퓨터가 연결을 기다리는 프로그램에 접근할 수 있어요.

# Said beside the switch once the firewall is on.
settings-firewall-done-on = 방화벽 켜짐

# Said beside the switch once the firewall is off.
settings-firewall-done-off = 방화벽 꺼짐

# Heading of the card that blocks adverts and bad sites, on the Firewall and blocking page.
settings-blocking-card = 차단

# Small line under the firewall switch when it is on but its rules are not loaded.
settings-firewall-not-in-force = 방화벽이 켜져 있지만 작동하지 않아요. 컴퓨터를 시작할 때 규칙을 불러오지 못했기 때문이에요. 껐다가 다시 켜세요.
