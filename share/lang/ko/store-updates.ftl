# machine translated (Korean), not yet reviewed by a person.

# Apps: the Updates page (src/store/update_page/).

## The Updates page (src/store/update_page/)

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Debian 패키지
# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = { $have } 버전에서 보류 중, { $waits }
# On a row held back: the one that waits.
store-upd-held-waits = 보류 중, { $waits }
# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = { $have } 버전에서 보류 중
# On a thing held back with no update waiting.
store-upd-held = 보류 중
# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = { $have } 버전에서 보류 중이에요. 아직 더 새로운 버전은 없어요.
# On the Held back card: nothing newer waits.
store-upd-held-nothing = 보류 중, 아직 더 새로운 버전은 없어요
# How many updates there are, as part of a line.
store-upd-count = { $n ->
       *[other] 업데이트 { $n }개
    }
# How many updates are ready, as the start of the headline.
store-upd-ready-count = { $n ->
       *[other] 업데이트 { $n }개가 준비됐어요
    }
# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, 모두 { $size }
# How many updates are held back, as part of a line.
store-upd-held-clause = { $n ->
       *[other] { $n }개는 보류 중이에요
    }
# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }
# A line made into a sentence.
store-upd-sentence = { $text }.
# Two sentences one after the other.
store-upd-and-then = { $first } { $second }
# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = 다시 시작해서 펌웨어 업데이트를 마치세요.
# Headline while the check runs.
store-upd-checking = 업데이트를 확인하는 중이에요.
# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = { $name }을(를) 업데이트하는 중이에요. 설치하는 동안 다른 일을 해도 돼요.
# Headline while all the updates go in.
store-upd-installing-all = 업데이트를 설치하는 중이에요. 설치하는 동안 다른 일을 해도 돼요.
# Headline when the updates are in and the computer must start again.
store-upd-restart = 컴퓨터를 다시 시작해서 업데이트 설치를 마치세요.
# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = 다시 시작해서 설치를 마치세요.
# Added to the headline with no internet.
store-upd-no-more = 이 컴퓨터는 인터넷에 연결되어 있지 않아서 Trindle이 다른 업데이트를 찾을 수 없어요.
# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = 준비된 업데이트가 없어요
# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Trindle이 아는 대기 중인 업데이트가 없어요. 이 컴퓨터가 인터넷에 연결되어 있지 않아서 다른 업데이트를 찾을 수 없어요.
# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = 앱
# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle과 Debian
# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = 펌웨어
# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Trindle이 { $names }의 업데이트를 확인하지 못했어요.
# Headline when everything is up to date.
store-upd-current = 모두 최신 상태예요.
# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = 마지막 확인: { $time }.
# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = 자동 업데이트 확인이 꺼져 있어요.
# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = 확인 안 함
# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = 인터넷 없이는 확인할 수 없어요
# Red words under a card when a check took too long.
store-upd-too-long = 확인에 너무 오래 걸렸어요.
# Tip of the round button that checks again.
store-upd-check-again = 다시 확인
# Main button while the check runs.
store-upd-checking-main = 확인 중
# Main button while the updates go in.
store-upd-installing-main = 설치 중
# Main button that installs every update waiting.
store-upd-install-n = { $n ->
       *[other] 업데이트 { $n }개 설치
    }
# Main button that restarts the computer.
store-upd-restart-now = 지금 다시 시작
# Beside Restart now: the box that puts the restart off, then the ring's Updates button reminds. Its answers are the choices below.
store-upd-remind-later = 나중에 알림
# Red words under the headline when the Remind me later time could not be saved; the reason follows.
store-upd-remind-not-saved = 알림을 저장하지 못했어요: { $why }.
# One answer in the Remind me later box.
store-upd-remind-hours =
    { $n ->
       *[other] { $n }시간 뒤
    }
# Main button that checks for updates.
store-upd-check-for = 업데이트 확인
# Tip of the round button that releases a thing held back.
store-upd-release = { $name } 업데이트 허용
# Tip of the round button that holds a thing's updates back.
store-upd-hold = { $name } 보류
# Tip of the round button that ends an update that is running.
store-upd-stop = 중지
# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = 중지하는 중
# Under a row whose update was stopped.
store-upd-stopped = 중지했어요. 계속하려면 업데이트를 누르세요.
# Under the Apps card when Install all was stopped.
store-upd-stopped-all = 중지했어요. 계속하려면 다시 설치하세요.
# A card's state while it is being checked.
store-upd-status-checking = 확인 중
# A card's state while its updates go in.
store-upd-status-installing = 설치 중
# A card's state when its check failed; the reason follows.
store-upd-status-failed = 확인하지 못했어요. { $why }
# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }
# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } 보류 중
# A card's state when only held back updates are left.
store-upd-held-back-n = { $n }개 보류 중
# A card's state when nothing waits.
store-upd-up-to-date = 최신 상태
# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = 출처: { $from }, { $change }
# On a row of the Updates page: where an app comes from.
store-upd-from = 출처: { $from }
# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = 공유 부품, 출처: { $from }
# Note under the Apps card.
store-upd-steam-restarts = 실행 중인 게임이 없어지면 Steam이 다시 시작돼요.
store-upd-yours-alone = 이 앱들은 나만 쓰는 앱이라 비밀번호 없이 업데이트돼요. 앱을 업데이트하면 필요한 공유 부품도 함께 설치돼요.
# Note under the Apps card: what holding back does.
store-upd-hold-why = 보류한 업데이트는 현재 버전에 머물러요. 업데이트를 다시 허용할 때까지 모든 업데이트 설치에서 빠져요. 여기와 링에서 모두 그래요.
# Heading of the Apps card.
store-upd-card-apps = 앱
# Plugins card: every plugin is up to date.
store-upd-plugins-current = 모든 플러그인이 최신 상태예요.
# Plugins card: how many plugins have updates held back.
store-upd-plugins-held = { $n ->
       *[other] 플러그인 { $n }개의 업데이트가 보류 중이에요.
    }
# Plugins card: how many plugins have an update.
store-upd-plugins-have = { $n ->
       *[other] 플러그인 { $n }개에 업데이트가 있어요
    }
# Heading of the Plugins card.
store-upd-card-plugins = 플러그인
# Held back card: nothing is held back.
store-upd-nothing-held = 보류 중인 것이 없어요.
# On a row of the Held back card for a thing released while the page is open.
store-upd-released = 보류 안 함
# Note under the Held back card: what releasing does.
store-upd-release-why = 업데이트를 다시 허용하면 나머지와 함께 설치돼요. 여기와 링에서 모두 그래요.
# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = 장치의 펌웨어
# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = 장치 { $id }의 펌웨어
# Heading of the Held back card.
store-upd-card-held = 보류 중
# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from }에서 { $to }(으)로
# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count = { $n ->
       *[other] 업데이트 { $n }개, 함께 설치돼요
    }
# Under Debian's rows: how many of them show.
store-upd-debian-showing = Debian 업데이트 { $total }개 중 { $shown }개를 보여 줘요.
# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = 이 컴퓨터에는 Trindle { $version }이(가) 있어요.
# Heading of the card about the daily security updates.
store-upd-card-security = 보안 업데이트
# On the security card: its state.
store-upd-security-on = 켜짐, 매일
# On the security card: that holding back never stops these.
store-upd-never-held = 업데이트를 보류해도 보안 업데이트는 멈추지 않아요.
# Firmware card: the computer has no updater.
store-upd-fw-none = 이 컴퓨터에는 펌웨어 업데이터가 없어요.
# Firmware card: restart to finish.
store-upd-fw-restart = 다시 시작해서 마치기
# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = 버전 { $version }
# Note on the firmware card when the updater is there.
store-upd-fw-note = 새 펌웨어는 컴퓨터를 다시 시작할 때 설치돼요.
# Note on the firmware card when the updater is missing.
store-upd-fw-install = 펌웨어를 최신으로 유지하려면 fwupd를 설치하세요.
# Heading of the Firmware card.
store-upd-card-firmware = 펌웨어
# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd가 최신 목록을 가져오지 못했어요.
# Red words when Flathub could not be reached.
store-upd-no-flathub = Flathub에 연결할 수 없었어요.
# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Trindle이 Debian 목록을 가져오지 못했어요.
# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Trindle이 Debian 목록을 가져오지 못했어요. { $why }
# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = 보류한 업데이트 목록을 저장할 수 없어서 Trindle이 보류하지 못했어요: { $why }.
# The same, for a release.
store-upd-not-released-list = 보류한 업데이트 목록을 저장할 수 없어서 Trindle이 업데이트를 다시 허용하지 못했어요: { $why }.
# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = 앱에서는 보류됐지만 Trindle이 Flatpak에서 이를 바꾸지 못했어요. 앱을 업데이트하는 다른 프로그램이 설치할 수도 있어요.
# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = 앱에서는 업데이트가 다시 허용됐지만 Trindle이 Flatpak에서 이를 바꾸지 못해서 Flatpak이 계속 건너뛸 수 있어요. 보류했다가 업데이트를 다시 허용해 보세요.
# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak의 메시지: { $why }
# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle

# Said when a security fix for a boxed app can't be downloaded. { $package } is the package, { $version } the fixed version.
store-patch-not-fetched = Trindle이 { $package } { $version }을(를) 다운로드하지 못했어요.
# Said when a security fix for a boxed app was downloaded but isn't there to use.
store-patch-not-found = Trindle이 다운로드한 패키지를 찾지 못했어요.

