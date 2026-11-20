# machine translated (Korean), not yet reviewed by a person.

# The Help page of Settings: its search, the cards that group the topics and the sentences at the top. The topics themselves are in settings-help-topics.ftl.

## Help page (src/settings/panels/help.rs)

# Name of the Help page in the Settings side bar and its title.
settings-help-title = 도움말
# Words the Settings search matches the Help page by, in lower case.
settings-help-keywords = 도움말 help 사용법 안내 질문 설명서 지원 how guide question manual support
# Heading of the card of Help topics for starting out.
settings-help-area-start = 시작하기
# Heading of the card of Help topics about apps.
settings-help-area-apps = 앱
# Heading of the card of Help topics about the screen and how Trindle looks.
settings-help-area-screen = 화면과 모양
# Heading of the card of Help topics about magnifying, screen reading and voice.
settings-help-area-access = 보기와 듣기
# Heading of the card of Help topics about networks, sound and connected devices.
settings-help-area-devices = 네트워크, 소리, 기기
# Heading of the card of Help topics about updates, snapshots and security.
settings-help-area-safe = 안전하게 지키기
# Heading of the card of Help topics about this computer.
settings-help-area-computer = 이 컴퓨터
# The sentence at the top of Help before anything is typed.
settings-help-start = Trindle이 처음이세요? 환영 페이지부터 보거나, 찾는 내용을 입력하세요.
# The sentence at the top of Help when a search finds nothing; typed is what was typed.
settings-help-found-none = “{ $typed }”와(과) 맞는 것이 없어요. 더 짧은 단어로 해 보세요.
# The sentence at the top of Help when a search finds topics; n is how many and typed is what was typed.
settings-help-found =
    { $n ->
       *[other] “{ $typed }”에 관한 주제가 { $n }개 있어요.
    }
# Tip of the arrow button that closes an open Help topic; title is the topic's title.
settings-help-close = { $title } 닫기
# Tip of the arrow button that opens a Help topic to read; title is the topic's title.
settings-help-read = { $title } 읽기
# Bold line in the Reading card of Help when no topic is open.
settings-help-nothing-open = 열린 것이 없어요
# Heading of the Help card that shows the open topics.
settings-help-reading = 읽는 중
# Placeholder and spoken name of the typing box that searches Help.
settings-help-search-placeholder = 도움말 검색
# Heading of the Help card with the search box.
settings-help-search = 도움말 검색
# Line and tip of the button that goes back to the Settings page Help was opened from; title is that page's title.
settings-help-back = { $title }(으)로 돌아가기

# The row in the search card that opens the Help program, which holds every page and a search of them.
settings-help-program = 도움말을 별도 창에서 보기
# The hint on that row's button.
settings-help-program-open = 도움말 열기
# Red words under the line "Help in its own window" when the Help program could not be started.
settings-help-program-not-opened = 도움말을 열 수 없어요.
