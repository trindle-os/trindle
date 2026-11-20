# machine translated (Korean), not yet reviewed by a person.

# Settings: the Your plugins page (src/settings/panels/plugins.rs).

## The Your plugins page (src/settings/panels/plugins.rs)

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = 내 플러그인
# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = 플러그인 추가 기능 확장 설치됨 제거 설정
# Help line at the foot of the Your plugins page.
settings-plugins-help = 플러그인은 Trindle에 기능을 더해 줘요. 플러그인은 링에 아이콘, 여기에 설정, 알림, 로그인할 때 시작되는 것을 추가할 수 있어요. 플러그인은 각자 자기 상자 안에서 실행되고, 권한에서 허용하기 전에는 아무것도 쓸 수 없어요. 페이지 제목 옆 버튼을 누르면 권한이 열려요. 플러그인을 제거하려면 카드의 휴지통을 두 번 누르세요.
# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = Trindle이 플러그인이 쓸 수 있는 것을 묻는 창을 열지 못했어요.
# Under About it: the plugin's version number is { $v }.
settings-plugins-version = 버전 { $v }.
# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = 버전 { $v }. { $settled }
# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name }은(는) 아무것도 요청하지 않아서, 허용된 것 없이 자기 상자 안에서 실행돼요.
# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
settings-plugins-asks-for =
    { $asks ->
       *[other] { $name }이(가) { $list }을(를) 요청해요. 요청 { $asks }개 중 { $allowed }개를 허용했어요. 허용한 것만 가지고 자기 상자 안에서 실행돼요.
    }
# Heading of the line about a plugin on its card.
settings-plugins-about-it = 정보
# Tip of the button that changes what a plugin may use.
settings-plugins-change = 쓸 수 있는 것 바꾸기
# Tip of the button that removes a plugin.
settings-plugins-remove = 이 플러그인 제거
# Red words when a plugin could not be removed.
settings-plugins-not-removed = 제거하지 못했어요.
# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = 여기에 입력
# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name }에는 자체 설정이 없어요.
# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name }에는 지금 보여 줄 설정이 없어요.
# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] 설치된 플러그인이 없어요.
       *[other] 플러그인 { $n }개가 설치되어 있어요: { $list }.
    }
# Heading of the card shown when no plugin is installed.
settings-plugins-group = 플러그인
# Heading of the line shown when no plugin is installed.
settings-plugins-none = 설치된 것 없음
# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = 권한 열기

## Cards drawn from a plugin's rows file (src/settings/panels/plugin_rows.rs)

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = { $name }에 묻는 중…
# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name }이(가) 응답하지 않아요. 서비스가 멈췄을 수 있어요.
# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = 저장했어요.
# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name }이(가) 그 변경을 받아들이지 않았어요.
# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name }이(가) 그렇게 할 수 없었어요.
# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name }이(가) 그렇게 할 수 없었어요: { $why }.
