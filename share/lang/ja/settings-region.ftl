# machine translated (Japanese), not yet reviewed by a person.

# Settings: the Region and language page (src/settings/panels/region.rs).

## The Region and language page (src/settings/panels/region.rs)

# Name of the Region and language page, in the page list and as its heading.
settings-region-title = 地域と言語
# Words people may search for to find the Region and language page, separated by spaces.
settings-region-keywords = 言語 ロケール 国 形式 日付 通貨 language locale country formats date currency
# Help line at the foot of the Region and language page.
settings-region-help = 言語と形式は、次にログインしたときに反映されます。
# Said beside the Language and Formats lists once a choice is saved.
settings-region-later = 変更を見るには、もう一度ログインしてください。
# Shown while the page reads the computer.
settings-region-looking = 確認中…
# Under Money and paper: the paper and measures the chosen formats use (paper is a4, letter or other; measures is imperial or metric).
settings-region-paper-measures =
    { $paper ->
        [a4] A4 用紙
        [letter] レターサイズ用紙
       *[other] 用紙
    }、{ $measures ->
        [imperial] インチとポンド
       *[other] メートル法
    }
# Under Money and paper: { $money } is the money sign and { $paper } the paper and measures.
settings-region-money-and-paper = { $money }、{ $paper }
# Under A date and a number: a date and a number written the way the formats chosen write them.
settings-region-date-and-number = { $date } と { $number }
# Said beside the Add a language line once a language is installed.
settings-region-installed-done = インストール済み
# Headline when the language and the formats are the same: { $lang } is the language's name.
settings-region-everything-in = すべて { $lang } です。
# Headline when the formats differ from the language: the names are { $lang } and { $fmt }.
settings-region-menus-in = メニューは { $lang } です。日付と数字は { $fmt } の形式です。
# Headline when a change is saved but waits for the next login: { $words } is the sentence so far.
settings-region-with-wait = { $words } 変更を見るには、ログアウトしてからログインしてください。
# Said beside the Install another line while a language is made.
settings-region-installing = インストール中
# Name of the list of languages for menus and messages, heading of its line and of the card.
settings-region-language = 言語
# Name of the list of formats and heading of its line.
settings-region-formats = 形式
# Help text of the Language card (the information button).
settings-region-language-about = 言語は、メニューとメッセージに使われます。形式は、日付、時刻、数字、通貨、単位の書き方と、用紙サイズを決めます。形式は他の国の書き方にもできます。どちらも次にログインしたときに反映されます。
# Added to the Language card's i: when menu and message changes arrive.
settings-region-language-sub = メニューとメッセージは次のログインで変わります。
# Heading of the line that shows a date and a number in the chosen formats.
settings-region-date-number = 日付と数字
# Heading of the line that shows the money and paper of the chosen formats.
settings-region-money-paper = 通貨と用紙
# Heading of the line that shows the language now in use.
settings-region-in-use = 現在使用中
# Heading of the card for installing another language.
settings-region-add = 言語を追加
# Help text of the Add a language card (the information button).
settings-region-add-about = 別の言語をインストールして、メニューや形式に選べるようにします。インストールには少し時間がかかり、管理者のパスワードが必要です。
# Heading of the line shown when every language is already installed.
settings-region-all-installed = すべての言語がインストール済みです
# Heading of the line saying how many languages are installed.
settings-region-installed = インストール済み
# Under the Installed heading: how many languages are installed.
settings-region-count =
    { $n ->
       *[other] { $n } 個の言語がインストールされています。
    }
# Name of the list of languages that can be installed, and heading of its line.
settings-region-install-another = 別の言語をインストール
# Tip of the button that installs the chosen language.
settings-region-install-this = この言語をインストール
# Heading of the line saying how many more languages can be made.
settings-region-ready = 追加できる言語
# Under the Ready to add heading: how many more languages can be made.
settings-region-more =
    { $n ->
       *[other] あと { $n } 個の言語をインストールできます。
    }
settings-region-removing = 削除中
settings-region-removed-done = 削除しました
settings-region-in-use-stays = 使用中です。
settings-region-remove-language = この言語を削除
settings-region-kept = このコンピューターの言語
settings-region-kept-about = このコンピューターにインストールされているすべての言語です。使用中の言語以外で、使わないものは削除できます。削除には管理者のパスワードが必要です。
