# machine translated (Japanese), not yet reviewed by a person.

# Settings, Blocking page: adverts and bad sites kept out for the whole computer, and private name lookups.

## Blocking page (src/settings/panels/blocking.rs)

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = たった今
# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
       *[other] { $n } 分前
    }
# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
       *[other] { $n } 時間前
    }
# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
       *[other] { $n } 日前
    }
# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
settings-blocking-waiting = パスワードを待っています…
# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = ブロックをオンにしました
# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = ブロックをオフにしました
# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = 保存しました
# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = example.com のように、サイトの名前を入力してください。
# Headline of the page when blocking is off.
settings-blocking-top-off = ブロックはオフです。ウェブサイトは、これまでどおりネットワーク側のサーバーで調べます。
# Headline of the page while blocking is starting.
settings-blocking-top-starting = ブロックを開始しています。
# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = 広告と危険なサイトをブロックしています。ウェブサイトは { $provider } を通して非公開で調べます。
# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = このネットワークではプライベートサーバーに接続できないため、今はネットワーク側のサーバーを使っています。ブロックは引き続き働きます。
# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = 広告と危険なサイトをブロックしています。ウェブサイトはネットワーク側のサーバーで調べるため、見たサイトがそのサーバーに分かります。
# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = 広告と危険なサイトをブロックしています。ウェブサイトはVPN側のサーバーで調べます。
# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = 広告と危険なサイトをブロックしています。
# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = 開始中
# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = オフ
# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = ブロックを開始しています
# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = ブロックはオフです
# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = 今日のブロック
# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = 今日ブロックした問い合わせ
# Name of the tile about the lists of sites.
settings-blocking-tile-lists = リスト
# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = 更新中
# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = まだありません
# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = サイトのリスト
# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = 件のサイトを登録、{ $ago }に更新
# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = まだブロックしたものはありません
# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
settings-blocking-let-through = このサイトを常に許可
# Heading of the card that lists recently blocked sites.
settings-blocking-recent = 最近ブロックしたもの
# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = ブロックしたサイトが新しい順にここに並びます。プラスを押すと、そのサイトを今後は許可します。Trindleは、許可したサイトの記録を残しません。
# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = ブロックをオンにすると、ブロックしたものが見られます。
# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = example.com
# Heading of the line with the box for adding a site.
settings-blocking-add = サイトを追加
# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = このサイトをもう一度ブロック
# Heading of the card that lists the sites that are let through.
settings-blocking-allowed = 常に許可
# Help text (i button) of the Always let through card.
settings-blocking-allowed-about = これらのサイトと、その下にあるすべてのアドレスは、リストに載っていても、ブロックされません。マイナスを押すと、そのサイトをもう一度ブロックします。
# Name of the page in the list of settings pages and its heading.
settings-blocking-title = ファイアウォールとブロック
# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = ブロック 広告 トラッカー マルウェア 詐欺 dns プライベート 問い合わせ quad9 mullvad cloudflare 許可 block adverts ads trackers malware scams dns private lookups quad9 mullvad cloudflare allow
# Help text at the top of the page.
settings-blocking-help = このコンピューターのすべてのブラウザーとアプリで、広告、トラッカー、既知の危険なサイトをブロックします。ウェブサイトの調べ物も暗号化した接続で行うため、使っているネットワークから、どのサイトを調べたかを見られたり変えられたりしません。
# Text of the switch that turns blocking on or off.
settings-blocking-switch = 広告と危険なサイトをブロック
# Tip of the dropdown that picks where lookups go.
settings-blocking-where = 問い合わせ先
# Help text (i button) of the Blocking card.
settings-blocking-card-about = HaGeZiの広告、トラッカー、既知の危険なサイトのリストを使い、1日に1回更新します。ブロックがオフの間は、何も動きません。ウェブサイトは、ここで選んだプライベートサーバーを通して、暗号化した接続で調べます。Quad9は、誰が調べたかの記録を残しません。「ネットワーク」でもブロックはしますが、ネットワーク側のサーバーが調べたサイトをすべて見られるため、非公開ではありません。
# Heading of the line with the private server dropdown.
settings-blocking-private = プライベートサーバー
# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = ネットワーク

# Heading of the Firewall card, first on the Firewall and blocking page.
settings-firewall = ファイアウォール

# Help text (i button) of the Firewall card.
settings-firewall-about = ファイアウォールがオンの間は、このコンピューターが始めた接続を除き、ほかのコンピューターから接続できません。同じWi-Fiにいる人が入り込もうとしても無理です。ブラウズ、更新、プリンターは今までどおり使えます。ゲームサーバーのように、ほかのコンピューターからの接続を待つプログラムは、下でポートを開く必要があります。ファイアウォールがオフだと、ネットワーク上のどのコンピューターも、接続を待っているすべてのプログラムに届きます。オンとオフの切り替えには、管理者のパスワードが必要です。

# Name of the firewall's on and off switch.
settings-firewall-switch = ファイアウォール


# Small line under the switch while the firewall is off.
settings-firewall-off-sub = ほかのコンピューターが、接続を待つプログラムに届きます。

# Said beside the switch once the firewall is on.
settings-firewall-done-on = ファイアウォールをオンにしました

# Said beside the switch once the firewall is off.
settings-firewall-done-off = ファイアウォールをオフにしました

# Heading of the card that blocks adverts and bad sites, on the Firewall and blocking page.
settings-blocking-card = ブロック

# Small line under the firewall switch when it is on but its rules are not loaded.
settings-firewall-not-in-force = ファイアウォールはオンですが、コンピューターの起動時にルールが読み込まれなかったため、働いていません。いったんオフにして、もう一度オンにしてください。
