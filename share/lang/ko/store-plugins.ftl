# machine translated (Korean), not yet reviewed by a person.

# Apps: the Plugins page (src/store/plugin_page.rs).

## The Plugins page (src/store/plugin_page.rs)

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = 플러그인 설정 열기
# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = 실패했어요: { $why }
# Said on a plugin's row when its update asks for a new permission and so waits for the person.
store-plugins-waits-for-you = 이 업데이트에는 새 권한이 필요해서, 허용해 주기를 기다리고 있어요.
# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = { $new } 버전을 쓸 수 있어요. 지금은 { $mine } 버전이에요.
# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
store-plugins-waiting-more = 지금은 { $mine } 버전이에요. { $new } 버전에는 권한이 더 필요해요: { $more }. { $waits }
# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = 끝나면 버튼이 다시 나타나요.
# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = 플러그인을 찾는 중…
# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = 설치된 플러그인이 없고, 아직 받을 수 있는 것도 없어요.
# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
       *[other] 아직 설치된 플러그인이 없어요. { $n }개를 설치할 수 있어요.
    }
# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
       *[other] 플러그인 { $n }개가 설치되어 있어요
    }
# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
       *[other] { $n }개에 업데이트가 있어요
    }
# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
       *[other] 업데이트 { $n }개가 보류되어 있어요
    }
# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }. { $held }.
# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.
# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, 모두 최신이에요.
# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, 그리고 { $have }.
# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, 그리고 { $have }. { $held }.
# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = 아무 권한도 요청하지 않아요.
# On a plugin's card: what it asks for.
store-plugins-asks = 요청하는 권한: { $list }.
# Title of the Plugins page.
store-plugins-title = 플러그인
# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = { $mine } 버전이 설치되어 있어요. { $new } 버전을 쓸 수 있어요.
# On a plugin's card: its installed version.
store-plugins-version-installed = { $version } 버전이 설치되어 있어요.
# On a plugin's card: the version on offer.
store-plugins-version-offered = { $version } 버전.
# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.
# Tip of the round button that installs a plugin.
store-plugins-install = { $name } 설치
# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = { $name } 업데이트 허용
# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = { $version } 버전으로 업데이트
# Heading of the card that says where plugins come from.
store-plugins-where = 플러그인을 받는 곳
# Information on the card that says where plugins come from.
store-plugins-come-from = 플러그인은 Trindle 자체 목록에서만 와요. 이 페이지를 열 때마다 Trindle이 trindle.org에서 그 목록을 읽어요. 플러그인을 목록에 제출할 수 있어요. 목록에 나타나기 전에 하나하나 테스트해요.
# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = 설치는 되었지만 백그라운드에서 실행되는 부분이 실행되고 있지 않아요: { $why }
# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = 제거하지 못했어요: { $why }
