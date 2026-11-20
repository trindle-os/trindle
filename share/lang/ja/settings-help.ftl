# machine translated (Japanese), not yet reviewed by a person.

# The Help page of Settings: its search, the cards that group the topics and the sentences at the top. The topics themselves are in settings-help-topics.ftl.

## Help page (src/settings/panels/help.rs)

# Name of the Help page in the Settings side bar and its title.
settings-help-title = ヘルプ
# Words the Settings search matches the Help page by, in lower case.
settings-help-keywords = ヘルプ 使い方 ガイド 質問 マニュアル サポート help
# Heading of the card of Help topics for starting out.
settings-help-area-start = はじめに
# Heading of the card of Help topics about apps.
settings-help-area-apps = アプリ
# Heading of the card of Help topics about the screen and how Trindle looks.
settings-help-area-screen = 画面と見た目
# Heading of the card of Help topics about magnifying, screen reading and voice.
settings-help-area-access = 見ること、聞くこと
# Heading of the card of Help topics about networks, sound and connected devices.
settings-help-area-devices = ネットワーク、サウンド、機器
# Heading of the card of Help topics about updates, snapshots and security.
settings-help-area-safe = 安全に使う
# Heading of the card of Help topics about this computer.
settings-help-area-computer = このコンピューター
# The sentence at the top of Help before anything is typed.
settings-help-start = Trindle は初めてですか。まず「ようこそ」を見るか、探したい言葉を入力してください。
# The sentence at the top of Help when a search finds nothing; typed is what was typed.
settings-help-found-none = 「{ $typed }」に合うものはありません。もっと短い言葉を試してください。
# The sentence at the top of Help when a search finds topics; n is how many and typed is what was typed.
settings-help-found =
    { $n ->
       *[other] 「{ $typed }」に関する項目が { $n } 件あります。
    }
# Tip of the arrow button that closes an open Help topic; title is the topic's title.
settings-help-close = { $title } を閉じる
# Tip of the arrow button that opens a Help topic to read; title is the topic's title.
settings-help-read = { $title } を読む
# Bold line in the Reading card of Help when no topic is open.
settings-help-nothing-open = 開いている項目はありません
# Heading of the Help card that shows the open topics.
settings-help-reading = 閲覧中
# Placeholder and spoken name of the typing box that searches Help.
settings-help-search-placeholder = ヘルプを検索
# Heading of the Help card with the search box.
settings-help-search = ヘルプを検索
# Line and tip of the button that goes back to the Settings page Help was opened from; title is that page's title.
settings-help-back = { $title } に戻る

# The row in the search card that opens the Help program, which holds every page and a search of them.
settings-help-program = ヘルプを別ウィンドウで開く
# The hint on that row's button.
settings-help-program-open = ヘルプを開く
# Red words under the line "Help in its own window" when the Help program could not be started.
settings-help-program-not-opened = ヘルプを開けませんでした。
