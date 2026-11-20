# machine translated (Korean), not yet reviewed by a person.

# Apps: lines of the lists of apps, where things came from and how long is left (src/store/updates.rs, listing.rs, look.rs, origin.rs, progress.rs, reviews.rs, key.rs).

## Where an installed thing came from (src/store/updates.rs)

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Flatpak 파일
# On an installed thing's line: it came from a .deb file.
store-from-deb = .deb 파일
# On an installed thing's line: it is a plugin.
store-from-plugin = 플러그인
# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }
# The kind of a pre-release version written rc.
store-version-rc = 릴리스 후보

## A tile of the list of apps (src/store/listing.rs)

# Tip of the round button on an app's tile while it is being removed.
store-list-removing = { $name } 제거하는 중
# Tip of the round button on an app's tile while it is being installed.
store-list-installing = { $name } 설치하는 중
# Tip of the round button on an app's tile that opens the app.
store-list-open = { $name } 열기
# Tip of the round button on an app's tile that installs the app.
store-list-install = { $name } 설치
# Line on an installed app's tile when the app has no line of its own.
store-list-installed = 설치됨

## Tabs and times (src/store/look.rs, progress.rs)

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }
# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = 보기

## Where a download came from (src/store/origin.rs)

# Credit under a package's page for the list its site was checked against.
store-origin-credit = HaGeZi's Threat Intelligence 목록과 대조해서 확인했어요
# Where a download came from: the site and the page that led to it.
store-origin-both = { $referrer }에서 이동해서 { $site }에서 내려받았어요.
# Where a download came from: the site.
store-origin-site = { $site }에서 내려받았어요.
# Where a download came from: the page that led to it.
store-origin-referrer = { $referrer }에서 연결됐어요.
# Said when nothing tells where a download came from.
store-origin-unknown = Trindle에서 이것이 어디서 왔는지 알 수 없어요.
# Warning when the site is on the list; the first part says where the download came from.
store-origin-listed = { $from } { $name }은(는) 유해한 다운로드와 사기로 알려진 사이트의 HaGeZi's Threat Intelligence 목록에 있어요. 어디서 왔는지 확실할 때만 설치하세요.
# Said when the site is not on the list; the first part says where the download came from.
store-origin-clear = { $from } 이 사이트는 HaGeZi's Threat Intelligence 목록에 없어요. 이 목록이 완전하지는 않아서 안전하다는 뜻은 아니에요.
# Said when the list could not be fetched; the first part says where the download came from.
store-origin-not-checked = { $from } HaGeZi's Threat Intelligence 목록을 가져올 수 없어서 Trindle에서 사이트를 확인하지 못했어요.

## Reviews and the GitHub key's keyring (src/store/reviews.rs, key.rs)

# Name shown for a review whose writer gave none.
store-review-someone = 익명
# Red words when the keyring could not be reached.
store-key-unreached = Trindle에서 키링에 접근할 수 없었어요: { $why }.
# Red words when the key kept in the keyring is not text.
store-key-not-text = 저장된 키가 텍스트가 아니에요.
# Red words when a file that held the key could not be removed.
store-key-not-removed = { $path }을(를) 지울 수 없었어요: { $why }
# Red words when a file that held the key could not be read.
store-key-not-read = { $path }을(를) 읽을 수 없었어요: { $why }
