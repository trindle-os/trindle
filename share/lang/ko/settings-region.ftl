# machine translated (Korean), not yet reviewed by a person.

# Settings: the Region and language page (src/settings/panels/region.rs).

## The Region and language page (src/settings/panels/region.rs)

# Name of the Region and language page, in the page list and as its heading.
settings-region-title = 지역 및 언어
# Words people may search for to find the Region and language page, separated by spaces.
settings-region-keywords = 언어 로케일 나라 국가 형식 날짜 통화
# Help line at the foot of the Region and language page.
settings-region-help = 언어와 형식은 다음에 로그인할 때 적용돼요.
# Said beside the Language and Formats lists once a choice is saved.
settings-region-later = 다시 로그인하면 보여요.
# Shown while the page reads the computer.
settings-region-looking = 확인하는 중…
# Under Money and paper: the paper and measures the chosen formats use (paper is a4, letter or other; measures is imperial or metric).
settings-region-paper-measures =
    { $paper ->
        [a4] A4 용지
        [letter] Letter 용지
       *[other] 용지
    }, { $measures ->
        [imperial] 인치와 파운드
       *[other] 미터법
    }
# Under Money and paper: { $money } is the money sign and { $paper } the paper and measures.
settings-region-money-and-paper = { $money }, { $paper }
# Under A date and a number: a date and a number written the way the formats chosen write them.
settings-region-date-and-number = { $date }, { $number }
# Said beside the Add a language line once a language is installed.
settings-region-installed-done = 설치됨
# Headline when the language and the formats are the same: { $lang } is the language's name.
settings-region-everything-in = 모두 { $lang }(으)로 표시돼요.
# Headline when the formats differ from the language: the names are { $lang } and { $fmt }.
settings-region-menus-in = 메뉴는 { $lang }(으)로 표시돼요. 날짜와 숫자는 { $fmt } 방식을 따라요.
# Headline when a change is saved but waits for the next login: { $words } is the sentence so far.
settings-region-with-wait = { $words } 바뀐 내용을 보려면 로그아웃했다가 다시 로그인하세요.
# Said beside the Install another line while a language is made.
settings-region-installing = 설치하는 중
# Name of the list of languages for menus and messages, heading of its line and of the card.
settings-region-language = 언어
# Name of the list of formats and heading of its line.
settings-region-formats = 형식
# Help text of the Language card (the information button).
settings-region-language-about = 언어는 메뉴와 메시지에 쓰여요. 형식은 날짜, 시간, 숫자, 통화, 단위를 쓰는 방식과 용지 크기를 정해요. 다른 나라의 방식을 따를 수도 있어요. 둘 다 다음에 로그인할 때 적용돼요.
# Added to the Language card's i: when menu and message changes arrive.
settings-region-language-sub = 메뉴와 메시지는 다음 로그인 때 바뀌어요.
# Heading of the line that shows a date and a number in the chosen formats.
settings-region-date-number = 날짜와 숫자
# Heading of the line that shows the money and paper of the chosen formats.
settings-region-money-paper = 통화와 용지
# Heading of the line that shows the language now in use.
settings-region-in-use = 지금 사용 중
# Heading of the card for installing another language.
settings-region-add = 언어 추가
# Help text of the Add a language card (the information button).
settings-region-add-about = 다른 언어를 설치하면 메뉴나 형식에 고를 수 있어요. 설치하는 데 시간이 조금 걸리고 관리자 비밀번호가 필요해요.
# Heading of the line shown when every language is already installed.
settings-region-all-installed = 모든 언어가 이미 설치되어 있어요
# Heading of the line saying how many languages are installed.
settings-region-installed = 설치됨
# Under the Installed heading: how many languages are installed.
settings-region-count =
    { $n ->
       *[other] 언어 { $n }개가 설치되어 있어요.
    }
# Name of the list of languages that can be installed, and heading of its line.
settings-region-install-another = 다른 언어 설치
# Tip of the button that installs the chosen language.
settings-region-install-this = 이 언어 설치
# Heading of the line saying how many more languages can be made.
settings-region-ready = 추가할 수 있는 언어
# Under the Ready to add heading: how many more languages can be made.
settings-region-more =
    { $n ->
       *[other] 언어 { $n }개를 더 설치할 수 있어요.
    }
settings-region-removing = 지우는 중
settings-region-removed-done = 지웠어요
settings-region-in-use-stays = 사용 중이에요.
settings-region-remove-language = 이 언어 지우기
settings-region-kept = 이 컴퓨터의 언어
settings-region-kept-about = 이 컴퓨터에 설치된 모든 언어예요. 쓰고 있는 언어를 뺀 나머지는 쓰지 않으면 지울 수 있어요. 언어를 지우려면 관리자 비밀번호가 필요해요.
