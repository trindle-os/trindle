# machine translated (Japanese), not yet reviewed by a person.

# The Help page's topics: titles, search words, paragraphs, tips, steps and picture captions. Part of Settings' own words (see settings-help.ftl).


## Topic: AI programs (src/settings/help_topics.rs)

# Title of the Help topic "AI programs".
settings-help-topics-ai-programs-title = AI プログラム
# Words people might search for the Help topic "AI programs" by, in lower case, separated by spaces.
settings-help-topics-ai-programs-keywords = ai プログラム claude codex cursor gemini aider コーディング エージェント フェンス 非公開 ファイル 共有 フォルダ ssh 鍵 sudo アシスタント mcp flathub 再起動 キャッシュ
# Help topic "AI programs": paragraph 1.
settings-help-topics-ai-programs-p1 = この機能は、AI プログラムをあなたの個人ファイルから遠ざけます。たとえば、だまされた AI プログラムが銀行の明細を集めて外へ送るのを防ぎます。守れる範囲には限りがあり、絶対ではありません。執拗なプログラム、コンピューターのソフトウェアの欠陥、あなたが許可した要求や共有したフォルダを通じて、何かが漏れる可能性はあります。
# Help topic "AI programs": paragraph 2.
settings-help-topics-ai-programs-p2 = フェンスの中で動くプログラムからは、ホームフォルダが空に見えます。あなたの書類、ssh 鍵、保存したパスワード、ブラウザのログイン情報は読めません。見えるのは、あなたが共有したフォルダだけで、場所もあなたと同じです。それ以外のシステムは見えるので、ログを読んだりコマンドを実行したりできます。プログラムが起動するものは、インストールするプログラムも含めて、すべてフェンスの中に残ります。
# Help topic "AI programs": step 1 of the numbered steps at block 3.
settings-help-topics-ai-programs-step3-1 = フォルダを共有するには、Folders で右クリックして「AI プログラムと共有」を選ぶか、AI プログラムのページの「フォルダを共有」を使います。
# Help topic "AI programs": step 2 of the numbered steps at block 3.
settings-help-topics-ai-programs-step3-2 = 隠しフォルダや、鍵、パスワード、メールが入っているフォルダは、共有できません。
# Help topic "AI programs": step 3 of the numbered steps at block 3.
settings-help-topics-ai-programs-step3-3 = ssh で別のコンピューターに接続するには、ページでそのコンピューターを追加します。Trindle は専用の鍵を使い、その鍵は一覧にあるコンピューターにしか使えません。
# Help topic "AI programs": step 4 of the numbered steps at block 3.
settings-help-topics-ai-programs-step3-4 = コマンドに管理者権限が必要なときは、Trindle がフェンスの外にウィンドウを開いて、あなたの操作を待ちます。
# Help topic "AI programs": paragraph 4.
settings-help-topics-ai-programs-p4 = フェンスの中で起動するのは、ページに一覧されているプログラムだけです。それ以外の方法で起動したプログラムは、フェンスで囲まれません。ページでフェンスをオフにすると、AI プログラムはあなたと同じものがすべて見えます。アシスタント接続 (trindle-mcp) をインストールした場合、接続したアシスタントは、確認なしで一部の情報を読めます。読めるのは、開いているウィンドウ、インストール済みのアプリ、実行中のプログラム、Trindle の設定、システムログ、最近のクラッシュの記録、デバイスです。メモリ、ディスク、バッテリー、ネットワークの状態も読めます。Flathub からインストールしたアプリについては、設定の名前は読めますが、設定の中身は読めません。何かを変更する前には、必ずあなたに確認します。Flathub のアプリの調子が悪いときは、アプリの再起動、設定の変更、キャッシュの消去ができますが、どれもあなたに確認してからです。変更した設定や消去したキャッシュは、元に戻すよう頼めます。ただし、再起動は元に戻せません。

## Topic: Welcome to Trindle (src/settings/help_topics.rs)

# Title of the Help topic "Welcome to Trindle".
settings-help-topics-welcome-title = Trindle へようこそ
# Words people might search for the Help topic "Welcome to Trindle" by, in lower case, separated by spaces.
settings-help-topics-welcome-keywords = はじめに 始める 初めて 新規 最初 紹介 とは 使い方 端 ウィンドウ 開く welcome start ring
# Help topic "Welcome to Trindle": paragraph 1.
settings-help-topics-welcome-p1 = Trindle は画面をすっきり保ちます。端にバーやメニューはなく、あるのはウィンドウと壁紙だけです。ほかのことはすべて、リングと呼ぶ 1 つの円にまとまっています。必要なときに現れ、使い終わると消えます。
# Help topic "Welcome to Trindle": caption and spoken name of picture 2 (ring-first.png).
settings-help-topics-welcome-pic2 = 中央に時刻と日付があるリング
# Help topic "Welcome to Trindle": paragraph 3.
settings-help-topics-welcome-p3 = 操作はすべて、マウスか指でできます。ウィンドウのある場所で、リングを開く方法です。
# Help topic "Welcome to Trindle": step 1 of the numbered steps at block 4.
settings-help-topics-welcome-step4-1 = ポインターを画面の端に動かして、ダブルクリックします。リングが開きます。
# Help topic "Welcome to Trindle": step 2 of the numbered steps at block 4.
settings-help-topics-welcome-step4-2 = Super キーでも開きます。
# Help topic "Welcome to Trindle": step 3 of the numbered steps at block 4.
settings-help-topics-welcome-step4-3 = 開いているウィンドウはリングの中にあります。クリックすると、そのウィンドウが手前に来ます。
# Help topic "Welcome to Trindle": paragraph 5.
settings-help-topics-welcome-p5 = Welcome は、最初にログインしたときに開く短い案内です。動く絵で、ここまでのことを見せます。どのくらい知っているかを聞いて、すでにできることは飛ばします。Welcome は、いつでも「すべてのアプリ」からもう一度開けます。
# Help topic "Welcome to Trindle": paragraph 6.
settings-help-topics-welcome-p6 = ごみ箱のように、何かを失うかもしれないボタンは、2 回クリックが必要です。1 回目で、ボタンがハイライト色になり、チェックマークが出ます。続けるには、もう一度クリックします。そのままにすると、3 秒後にボタンは元に戻ります。

## Topic: Opening the ring and finding apps (src/settings/help_topics.rs)

# Title of the Help topic "Opening the ring and finding apps".
settings-help-topics-ring-title = リングを開く、アプリを探す
# Words people might search for the Help topic "Opening the ring and finding apps" by, in lower case, separated by spaces.
settings-help-topics-ring-keywords = ランチャー トレイ アイコン 明るさ ツール アプリ アプリケーション スタート メニュー 開く プログラム 端 クリック お気に入り すべてのアプリ カテゴリ オフィス ゲーム 入力 検索 探す 絞り込み
# Help topic "Opening the ring and finding apps": paragraph 1.
settings-help-topics-ring-p1 = ポインターを画面のいちばん端、どの端でもよいので、そこへ動かしてダブルクリックします。リングが現れます。ウィンドウが画面いっぱいでも使えますが、全画面のゲームや動画の上では使えません。そこでは、すべてのクリックがそのウィンドウに届くからです。ポインターを端に動かしただけでは開かないので、うっかり出ることはありません。デスクトップのほかの場所を 1 回クリックしても、何も開きません。Super キーでもリングを開けます。全画面のウィンドウの上でも開きます。何もないデスクトップをダブルクリックしても開きます。
# Help topic "Opening the ring and finding apps": paragraph 2.
settings-help-topics-ring-p2 = 中央には、時刻と日付、そして小さなアイコンの列があります。アイコンは、サウンド、Bluetooth、Wi-Fi、バッテリー、そして画面をつけたままにするカップです。画面キーボードは、設定の「キーボードとマウス」でオンにすると、ここに出ます。コンピューターに画面のバックライトがあれば、明るさのアイコンもあります。その上でスクロールすると明るさが変わり、クリックするとスライダーが出ます。その下には、すべてのアプリ、あれば更新のボタン、そして電源があります。電源には、ロック、スクリーンセーバー、スリープ、ハイバネート (対応していれば)、ログアウト、再起動、シャットダウンがあります。
# Help topic "Opening the ring and finding apps": paragraph 3.
settings-help-topics-ring-p3 = 中央のサウンド、Bluetooth、Wi-Fi、バッテリーのアイコンをクリックすると、お気に入りの上に丸いパネルが開きます。ここでは、音量やマイクの調整、Bluetooth ヘッドホンの接続、Wi-Fi ネットワークへの接続、バッテリーのもちと劣化の確認ができます。パネルの歯車は、設定の全ページを開きます。×は、パネルを閉じます。
# Help topic "Opening the ring and finding apps": paragraph 4.
settings-help-topics-ring-p4 = お気に入りのアプリは、中央のまわりに並びます。最初は Folders と Drives が入っています。アプリのように見えるツールの円も 4 つあります。Screenshot (カメラ)、クリップボード、通知、拡大鏡です。アプリをクリックすると、開くか、ウィンドウが手前に来ます。開いているプログラムは、お気に入りの中ではなく、お気に入りのまわりの、開いているウィンドウのリングに出ます。
# Help topic "Opening the ring and finding apps": paragraph 5.
settings-help-topics-ring-p5 = チャットアプリやバックアップツールなど、ほかに動いているプログラムのアイコンは、中央の下に 1 列で並びます。クリックするとそのプログラムが開き、右クリックするとメニューが出ます。プラグインのアイコンもここにあります。クリックするとプラグインが始まり、右クリックすると、できることと設定が出ます。たとえば Flow なら、接続と切断です。
# Help topic "Opening the ring and finding apps": caption and spoken name of picture 6 (ring-all.png).
settings-help-topics-ring-pic6 = すべてのアプリ: すべてのアプリが外側に、種類ごとに並びます
# Help topic "Opening the ring and finding apps": step 1 of the numbered steps at block 7.
settings-help-topics-ring-step7-1 = 中央の「すべてのアプリ」をクリックします。まだお気に入りでないアプリがすべて、外側に種類ごとのセクションで並びます。インターネット、オフィス、グラフィックス、サウンドとビデオ、ゲームなどです。アプリが残っていない種類は、セクションが出ません。
# Help topic "Opening the ring and finding apps": step 2 of the numbered steps at block 7.
settings-help-topics-ring-step7-2 = セクションの中のアプリをクリックすると、開きます。
# Help topic "Opening the ring and finding apps": step 3 of the numbered steps at block 7.
settings-help-topics-ring-step7-3 = 探しているアプリが決まっていますか。名前の最初の数文字を入力してください。一致するアプリだけが残り、いちばん近いものが先頭に来ます。Enter キーで先頭のアプリが開きます。Esc キーで、入力した文字が消えます。
# Help topic "Opening the ring and finding apps": step 4 of the numbered steps at block 7.
settings-help-topics-ring-step7-4 = もう一度「すべてのアプリ」をクリックすると、セクションが隠れます。
# Help topic "Opening the ring and finding apps": tip 8, shown in a box of its own.
settings-help-topics-ring-tip8 = リングを閉じるには、Esc キーを押すか、円の外をクリックします。

## Topic: Keeping the screen awake (src/settings/help_topics.rs)

# Title of the Help topic "Keeping the screen awake".
settings-help-topics-awake-title = 画面をつけたままにする
# Words people might search for the Help topic "Keeping the screen awake" by, in lower case, separated by spaces.
settings-help-topics-awake-keywords = カップ カフェイン 画面 つけたまま オフ スリープ 暗く ロック 動画 プレゼン
# Help topic "Keeping the screen awake": paragraph 1.
settings-help-topics-awake-p1 = 何かを見ているときや、マウスに触れずに読んでいるときは、リングの中央にあるカップをクリックしてください。カップが光ります。もう一度クリックするか、ログアウトするまで、画面は暗くならず、ロックもオフもスリープもしません。
# Help topic "Keeping the screen awake": tip 2, shown in a box of its own.
settings-help-topics-awake-tip2 = 同じスイッチは、設定の「電源」の上にもあります。

## Topic: Keeping your favourite apps in the ring (src/settings/help_topics.rs)

# Title of the Help topic "Keeping your favourite apps in the ring".
settings-help-topics-favourites-title = お気に入りのアプリをリングに置く
# Words people might search for the Help topic "Keeping your favourite apps in the ring" by, in lower case, separated by spaces.
settings-help-topics-favourites-keywords = お気に入り 置く 追加 削除 右クリック ドラッグ 移動 順番
# Help topic "Keeping your favourite apps in the ring": paragraph 1.
settings-help-topics-favourites-p1 = リングか「すべてのアプリ」で、どのアプリでも右クリックして、「お気に入りに追加」を選びます。そのアプリは、リング中央のまわりにずっと残ります。外すには、右クリックして「お気に入りから削除」を選びます。中央には 12 個まで置けます。
# Help topic "Keeping your favourite apps in the ring": paragraph 2.
settings-help-topics-favourites-p2 = 最初は Folders と Drives が入っています。どちらも外せますし、「すべてのアプリ」から戻せます。
# Help topic "Keeping your favourite apps in the ring": paragraph 3.
settings-help-topics-favourites-p3 = お気に入りの順番を変えるには、アプリをほかのアプリの場所へドラッグします。ほかのアプリが回って場所をあけます。
# Help topic "Keeping your favourite apps in the ring": caption and spoken name of picture 4 (ring-menu.png).
settings-help-topics-favourites-pic4 = アプリを右クリックすると、ほかの選択肢が出ます
# Help topic "Keeping your favourite apps in the ring": paragraph 5.
settings-help-topics-favourites-p5 = 同じメニューから、アプリの別のウィンドウを開いたり、ウィンドウを閉じたりできます。Apps からインストールしたアプリは、アンインストールもできます。

## Topic: Switching between windows (src/settings/help_topics.rs)

# Title of the Help topic "Switching between windows".
settings-help-topics-switcher-title = ウィンドウを切り替える
# Words people might search for the Help topic "Switching between windows" by, in lower case, separated by spaces.
settings-help-topics-switcher-keywords = ウィンドウ 切り替え スイッチャー alt tab 開く リング デスクトップ 表示 最小化 クリップボード コピー 貼り付け 履歴 キーボード 画面
# Help topic "Switching between windows": paragraph 1.
settings-help-topics-switcher-p1 = リングを開くと、お気に入りのまわりに、開いているウィンドウがリング状に並びます。ウィンドウごとに自分の区画があり、プログラムのアイコンと名前が出ます。1 つにポインターを合わせると、中央にそのウィンドウの絵が出ます。クリックすると、そのウィンドウが手前に来ます。右クリックすると、最小化、最大化、閉じるが選べます。Alt キーと Tab キーを押しても、開いているウィンドウが順に手前に来ます。
# Help topic "Switching between windows": paragraph 2.
settings-help-topics-switcher-p2 = Super キーと D キーを押すと、すべてのウィンドウが隠れて、デスクトップが見えます。もう一度押すと、ウィンドウが戻ります。何も開いていなければ、画面はすっきりしています。
# Help topic "Switching between windows": paragraph 3.
settings-help-topics-switcher-p3 = クリップボードは、リングのお気に入りの中にある 4 つのツールの円の 1 つです。丸いパネルで開き、最近コピーしたものを新しい順に出します。フィルターに入力して項目を探し、項目をクリックするともう一度コピーし、残したいものはピン留めします。行の横の小さな×は、その 1 件だけを削除します。間違えてコピーしたパスワードを消すのに便利です。下のごみ箱は、ピン留めしたもの以外をすべて削除します。確認のため、2 回クリックします。行を選ぶと、クリップボードに戻るだけです。貼り付けはあなたが自分でするので、選んでいない入力欄に入ることはありません。設定の「リング」の「クリップボード」で、残りを消すタイミングを選べます。再起動のたび、1 日後、1 週間後、2 週間後、または消さない、です。
# Help topic "Switching between windows": paragraph 4.
settings-help-topics-switcher-p4 = 設定の「キーボードとマウス」の「リングのキーボードボタン」でオンにすると、画面キーボードの小さなアイコンが、リングの中央に出ます。画面の下に沿ってキーボードが開きます。同じアイコンで、閉じます。

## Topic: Notifications: what programs tell you (src/settings/help_topics.rs)

# Title of the Help topic "Notifications: what programs tell you".
settings-help-topics-notifications-title = 通知: プログラムからのお知らせ
# Words people might search for the Help topic "Notifications: what programs tell you" by, in lower case, separated by spaces.
settings-help-topics-notifications-keywords = 通知 お知らせ アラート メッセージ バナー ポップアップ ベル 静か 消音 サイレント 邪魔しない 不在 メール バッテリー 隅
# Help topic "Notifications: what programs tell you": paragraph 1.
settings-help-topics-notifications-p1 = 新しいメールやバッテリー残量の低下など、プログラムからお知らせがあると、画面の隅に小さな円が出ます。数秒たつと消えます。最大 3 つまで、縦に並んで出ます。隅に出るので、読んでいるものには重なりません。
# Help topic "Notifications: what programs tell you": paragraph 2.
settings-help-topics-notifications-p2 = リングの「通知」の円を開くと、すべての通知の一覧が出ます。古いものが上、新しいものが下で、それぞれ横に時刻があります。席を外していたときは数が多いかもしれないので、一覧は新しいものがある下から開きます。古いものを見るには、上へスクロールします。
# Help topic "Notifications: what programs tell you": paragraph 3.
settings-help-topics-notifications-p3 = 各行の横には、小さなボタンが 2 つあります。ピンは、その通知をずっと残します。ほかの通知より上に移り、すべて消去してもそのまま残ります。小さなごみ箱は、その 1 件だけを削除します。下には、後で説明するベルのボタンと、ピン留めしたもの以外をすべて削除するごみ箱があります。そのごみ箱は、確認のため 2 回クリックします。
# Help topic "Notifications: what programs tell you": paragraph 4.
settings-help-topics-notifications-p4 = 通知を出しておく時間は、設定の「リング」の「通知」で選べます。ちらっと見るなら 3 秒、読むなら 15 秒、または閉じるまでです。緊急の通知は、必ず閉じるまで残ります。警告を読む前に消えることはありません。
# Help topic "Notifications: what programs tell you": paragraph 5.
settings-help-topics-notifications-p5 = 通知をオフにするには、「通知」の円から一覧を開き、下のベルを押します。ベルに線が入っていれば、通知はオフです。画面には出なくなりますが、一覧には入るので、見逃しません。もう一度ベルを押すと、オンに戻ります。
# Help topic "Notifications: what programs tell you": tip 6, shown in a box of its own.
settings-help-topics-notifications-tip6 = 通知の×は、その通知を隠すだけです。一覧にはまだ残っています。

## Topic: Getting new apps (src/settings/help_topics.rs)

# Title of the Help topic "Getting new apps".
settings-help-topics-apps-title = 新しいアプリを入手する
# Words people might search for the Help topic "Getting new apps" by, in lower case, separated by spaces.
settings-help-topics-apps-keywords = インストール ストア アプリ ダウンロード ソフトウェア flathub ブラウザ firefox brave アンインストール 削除 更新 deb パッケージ chrome vs code zoom ドライバー ダブルクリック
# Help topic "Getting new apps": paragraph 1.
settings-help-topics-apps-p1 = Trindle は、アプリを勝手には選びません。最初にログインしたとき、Welcome がブラウザなど人気のアプリを数ページで見せます。ほしいものを選んで、「選んだアプリをインストール」を押してください。Welcome は、いつでも「すべてのアプリ」からもう一度開けます。
# Help topic "Getting new apps": paragraph 2.
settings-help-topics-apps-p2 = あとからは、リングから Apps を開きます。左で種類から選ぶか、上で検索します。アプリをクリックすると説明が読めます。ダウンロードの矢印を押すと、インストールします。星の評価は名前の下にあり、アプリの画像はインストールボタンの下を横にスクロールします。レビューは、さらに下の「レビュー」にあります。地球儀のボタンは、Flathub のそのアプリのページを開きます。
# Help topic "Getting new apps": caption and spoken name of picture 3 (store-app.png).
settings-help-topics-apps-pic3 = Apps のアプリのページ
# Help topic "Getting new apps": step 1 of the numbered steps at block 4.
settings-help-topics-apps-step4-1 = アプリを削除するには、Apps でそのアプリを見つけて、ごみ箱を 2 回押します。1 回目で、ごみ箱がハイライト色になり、チェックマークが出ます。2 回目で、アプリが削除されます。
# Help topic "Getting new apps": step 2 of the numbered steps at block 4.
settings-help-topics-apps-step4-2 = または、リングで右クリックして「アンインストール」を選びます。
# Help topic "Getting new apps": step 3 of the numbered steps at block 4.
settings-help-topics-apps-step4-3 = 更新は、Apps の「更新」に一覧されます。更新ボタンを押します。
# Help topic "Getting new apps": tip 5, shown in a box of its own.
settings-help-topics-apps-tip5 = アプリはあなた専用にインストールされるので、パスワードは要りません。
# Help topic "Getting new apps": paragraph 6.
settings-help-topics-apps-p6 = Flathub にないアプリもあります。そうしたアプリは、AppImage や Flatpak ファイルとして、GitHub で公開されています。Apps で、検索ボックスに名前を入力します。AppImageHub に載っている AppImage は、入力するにつれて出てきます。Enter キーを押すと、GitHub 全体を検索します。目のボタンは、アプリの最新リリースのファイルを表示します。ダウンロードの矢印は、すぐにインストールします。
# Help topic "Getting new apps": tip 7, shown in a box of its own.
settings-help-topics-apps-tip7 = Flathub は自分のアプリを確認していますが、GitHub のアプリは誰も確認していません。信頼できる人のアプリだけをインストールしてください。
# Help topic "Getting new apps": paragraph 8.
settings-help-topics-apps-p8 = 「GitHub を検索」カードのいちばん端にある小さな丸い鍵のボタンは、GitHub キーの入力欄を開きます。ほとんどの場合、キーは要りません。キーがないと、GitHub は 1 時間に、あなたのコンピューターからの 60 件の要求に答えます。キーがあれば 5,000 件に答えますが、これが役立つのは、とてもたくさん検索するときだけです。Trindle は GitHub の答えを数時間覚えているので、同じものをもう一度検索しても、要求は減りません。上限に達すると、GitHub がまた答えるようになる時刻がページに出ます。このキーで Trindle が見られるのは、公開ファイルだけです。あなたのアカウントには一切アクセスできず、キーを作るときに権限にチェックを入れる必要もありません。
# Help topic "Getting new apps": paragraph 9.
settings-help-topics-apps-p9 = メーカーによっては、プログラムを Debian パッケージで提供しています。これは、.deb で終わるファイルです。Google Chrome、VS Code、Zoom、多くのプリンタードライバーが、この形です。ダウンロードが終わったら、Folders でダブルクリックするか、ブラウザのダウンロードから開きます。小さなウィンドウに、中身、バージョン、すでに入っているかどうかが出ます。ダウンロードの矢印を押して、パスワードを入力します。ほかに必要なものも、一緒にインストールされます。アプリが含まれていれば、アプリはリングに出ます。
# Help topic "Getting new apps": tip 10, shown in a box of its own.
settings-help-topics-apps-tip10 = パッケージはコンピューターの何でも変えられるので、信頼できるメーカーのものだけをインストールしてください。

## Topic: Programs from their makers: Claude, Tailscale and more (src/settings/help_topics.rs)

# Title of the Help topic "Programs from their makers: Claude, Tailscale and more".
settings-help-topics-sources-title = メーカーから直接入手するプログラム: Claude、Tailscale など
# Words people might search for the Help topic "Programs from their makers: Claude, Tailscale and more" by, in lower case, separated by spaces.
settings-help-topics-sources-keywords = ソフトウェア ソース リポジトリ repo apt claude anthropic chatgpt openai codex ai tailscale syncthing jellyfin メディア サーバー node nodejs github gh docker deb 新しい
# Help topic "Programs from their makers: Claude, Tailscale and more": paragraph 1.
settings-help-topics-sources-p1 = Flathub にないプログラムが少しあります。それらは、メーカー自身の Debian ソースから入手するか、そちらのほうがずっと新しいバージョンです。Claude、Tailscale、Syncthing、Jellyfin、Node.js、GitHub CLI、Docker です。ブラウザを含め、Flathub にあるものは、いつも通り Apps から入手します。Apps で「ソフトウェアソース」を選び、プログラムの横の＋を押します。管理者のパスワードを入力すると、ソースが追加され、プログラムがインストールされます。以後は、Debian の更新と一緒に更新されます。
# Help topic "Programs from their makers: Claude, Tailscale and more": paragraph 2.
settings-help-topics-sources-p2 = ソースはコンピューター全体の更新をインストールできるので、信頼できるメーカーだけを追加してください。ごみ箱は、プログラムとそのソースを一緒に削除します。
# Help topic "Programs from their makers: Claude, Tailscale and more": paragraph 3.
settings-help-topics-sources-p3 = Tailscale は、インストールしたら「Tailscale にログイン」を押します。ブラウザで Tailscale のログインページが開き、接続できると Apps が知らせます。
# Help topic "Programs from their makers: Claude, Tailscale and more": tip 4, shown in a box of its own.
settings-help-topics-sources-tip4 = 「別のソースを追加」では、どのメーカーのソースも追加できます。そのメーカーのインストール手順にある、アドレス、スイート、コンポーネント、署名鍵を使います。

## Topic: Undoing an update with snapshots (src/settings/help_topics.rs)

# Title of the Help topic "Undoing an update with snapshots".
settings-help-topics-snapshots-title = スナップショットで更新を取り消す
# Words people might search for the Help topic "Undoing an update with snapshots" by, in lower case, separated by spaces.
settings-help-topics-snapshots-keywords = スナップショット 復元 取り消し ロールバック 戻す 更新 壊れた ブートメニュー grub btrfs
# Help topic "Undoing an update with snapshots": paragraph 1.
settings-help-topics-snapshots-p1 = システムが Btrfs の場合、Trindle は Debian やソフトウェアソースからのインストール、更新、削除など、ソフトウェアを変更する前と後に、スナップショット (システムの保存したコピー) を作ります。設定の「スナップショット」に一覧されます。カメラのボタンは、取り消したくなるかもしれないことを試す前に、今すぐ 1 つ作ります。
# Help topic "Undoing an update with snapshots": step 1 of the numbered steps at block 2.
settings-help-topics-snapshots-step2-1 = 設定を開き、「スナップショット」を選びます。
# Help topic "Undoing an update with snapshots": step 2 of the numbered steps at block 2.
settings-help-topics-snapshots-step2-2 = おかしくなる前のスナップショットの横にある復元の矢印を押し、確認のためもう一度押します。
# Help topic "Undoing an update with snapshots": step 3 of the numbered steps at block 2.
settings-help-topics-snapshots-step2-3 = 再起動します。システムは、そのときの状態に戻ります。直前の状態は、削除されずに取り分けられます。
# Help topic "Undoing an update with snapshots": paragraph 3.
settings-help-topics-snapshots-p3 = コンピューターが起動しないときは、ブートメニューで「Trindle snapshots」を選び、続けてスナップショットを選びます。そのときの状態で起動しますが、そこで行ったことは残りません。残すには、設定でそのスナップショットを復元します。
# Help topic "Undoing an update with snapshots": tip 4, shown in a box of its own.
settings-help-topics-snapshots-tip4 = ブートメニューは隠れています。コンピューターの起動時に Esc キーを 1 回軽く押すと、表示されます。押し続けると、メニューを飛ばして入力画面になります。そうなったら、normal と入力して Enter キーを押すと、メニューに戻ります。
# Help topic "Undoing an update with snapshots": tip 5, shown in a box of its own.
settings-help-topics-snapshots-tip5 = Trindle を専用のインストーラーでインストールした場合、ホームフォルダは専用のパーティション (ディスクの別の区画) にあり、スナップショットには含まれません。そのため、復元してもあなたのファイルは変わりません。このコンピューターの設定が違う場合は、設定の「スナップショット」でお知らせします。

## Topic: Moving to another computer (src/settings/help_topics.rs)

# Title of the Help topic "Moving to another computer".
settings-help-topics-moving-title = 別のコンピューターへ移る
# Words people might search for the Help topic "Moving to another computer" by, in lower case, separated by spaces.
settings-help-topics-moving-keywords = バックアップ 復元 移行 移す 新しい コンピューター 転送 コピー 設定 アプリ データ flat fmpack
# Help topic "Moving to another computer": paragraph 1.
settings-help-topics-moving-p1 = Apps で「アプリを移す」を選びます。持っていきたいものをオンにします。アプリ、アプリ自身のデータ、AppImage、Trindle の設定です。保存ボタンを押すと、Trindle がホームフォルダにバックアップファイルを 1 つ作ります。それを USB メモリかクラウドストレージにコピーしてください。
# Help topic "Moving to another computer": paragraph 2.
settings-help-topics-moving-p2 = 移り先のコンピューターで同じページを開き、開くボタンを押してファイルを選びます。Trindle が Flathub からアプリをダウンロードしてインストールし、アプリのデータと設定を戻します。すべての設定を反映するには、ログアウトして、もう一度ログインしてください。
# Help topic "Moving to another computer": paragraph 3.
settings-help-topics-moving-p3 = このバックアップは、Trindle Flat が作るものと同じ種類のファイルで、.fmpack で終わります。そのため、Trindle Flat ならどの Linux コンピューターでも開けますし、Trindle Flat で作ったバックアップやアプリ一覧をここで選ぶこともできます。ページの Trindle Flat という名前は、Web サイトへのリンクです。このコンピューターにすでにあるアプリは、自分の設定のままです。設定が戻るのは、バックアップがインストールしたアプリだけです。
# Help topic "Moving to another computer": tip 4, shown in a box of its own.
settings-help-topics-moving-tip4 = 書類、写真、そのほかのファイルはバックアップに入らないので、自分でコピーしてください。Wi-Fi のパスワードと GitHub キーも入りません。

## Topic: Connecting to Wi-Fi (src/settings/help_topics.rs)

# Title of the Help topic "Connecting to Wi-Fi".
settings-help-topics-wifi-title = Wi-Fiにつなぐ
# Words people might search for the Help topic "Connecting to Wi-Fi" by, in lower case, separated by spaces.
settings-help-topics-wifi-keywords = wifi wi-fi 無線 インターネット ネットワーク つなぐ 接続 パスワード 有線 ケーブル ethernet
# Help topic "Connecting to Wi-Fi": step 1 of the numbered steps at block 1.
settings-help-topics-wifi-step1-1 = リングを開いて、真ん中のネットワークのアイコンを押すか、設定を開いて「ネットワーク」を選びます。
# Help topic "Connecting to Wi-Fi": step 2 of the numbered steps at block 1.
settings-help-topics-wifi-step1-2 = Wi-Fiのスイッチがオンになっているか確かめます。
# Help topic "Connecting to Wi-Fi": step 3 of the numbered steps at block 1.
settings-help-topics-wifi-step1-3 = 一覧から自分のネットワークを見つけて、「つなぐ」を押します。
# Help topic "Connecting to Wi-Fi": step 4 of the numbered steps at block 1.
settings-help-topics-wifi-step1-4 = Wi-Fiのパスワードを入力して、「参加」を押します。
# Help topic "Connecting to Wi-Fi": caption and spoken name of picture 2 (settings-network.png).
settings-help-topics-wifi-pic2 = ネットワークのページ
# Help topic "Connecting to Wi-Fi": tip 3, shown in a box of its own.
settings-help-topics-wifi-tip3 = 前に参加したことのあるネットワークには、Trindleが自動でつなぎます。これを止めたいときは、「このネットワークを忘れる」を2回押します。

## Topic: Sound, speakers and microphone (src/settings/help_topics.rs)

# Title of the Help topic "Sound, speakers and microphone".
settings-help-topics-sound-title = 音、スピーカー、マイク
# Words people might search for the Help topic "Sound, speakers and microphone" by, in lower case, separated by spaces.
settings-help-topics-sound-keywords = 音 オーディオ 音量 スピーカー ヘッドホン マイク ミュート テスト 小さい 大きい 切り替え リング 入力 出力 sound audio volume
# Help topic "Sound, speakers and microphone": paragraph 1.
settings-help-topics-sound-p1 = 設定を開いて「オーディオ」を選びます。「スピーカーとヘッドホン」で使う機器を、「マイク」で使うマイクを選びます。
# Help topic "Sound, speakers and microphone": caption and spoken name of picture 2 (settings-audio.png).
settings-help-topics-sound-pic2 = オーディオのページ
# Help topic "Sound, speakers and microphone": step 1 of the numbered steps at block 3.
settings-help-topics-sound-step3-1 = 「テスト音を鳴らす」を押すと、選んだスピーカーから短いチャイムが聞こえます。
# Help topic "Sound, speakers and microphone": step 2 of the numbered steps at block 3.
settings-help-topics-sound-step3-2 = 話しながら「レベル」のバーを見ます。バーが動けば、マイクは声を拾っています。
# Help topic "Sound, speakers and microphone": step 3 of the numbered steps at block 3.
settings-help-topics-sound-step3-3 = 「音量」のスライダーをドラッグするか、「ミュート」をオンにします。
# Help topic "Sound, speakers and microphone": paragraph 4.
settings-help-topics-sound-p4 = 設定を開かずに音量やマイクを変えるには、リングの真ん中にある音のアイコンを押して、丸いパネルを使います。
# Help topic "Sound, speakers and microphone": tip 5, shown in a box of its own.
settings-help-topics-sound-tip5 = リングの音のアイコンの上でスクロールしても、音量を変えられます。

## Topic: Sounds for logging in, the charger and the battery (src/settings/help_topics.rs)

# Title of the Help topic "Sounds for logging in, the charger and the battery".
settings-help-topics-sounds-title = ログイン、充電器、バッテリーの音
# Words people might search for the Help topic "Sounds for logging in, the charger and the battery" by, in lower case, separated by spaces.
settings-help-topics-sounds-keywords = 音 チャイム 起動 ログイン ログアウト シャットダウン 充電器 接続 取り外し バッテリー 警告 オフ 静か 変更 sounds
# Help topic "Sounds for logging in, the charger and the battery": paragraph 1.
settings-help-topics-sounds-p1 = 設定を開いて「サウンド」を選びます。次のものには、それぞれ音があります。ログイン、ログアウト・再起動・シャットダウン、充電器をつないだとき、充電器を外したとき、バッテリーが少なくなったときです。
# Help topic "Sounds for logging in, the charger and the battery": step 1 of the numbered steps at block 2.
settings-help-topics-sounds-step2-1 = どれかの横にある一覧から、別の音を選びます。聞いて確かめられるように、音が鳴ります。
# Help topic "Sounds for logging in, the charger and the battery": step 2 of the numbered steps at block 2.
settings-help-topics-sounds-step2-2 = その音を鳴らしたくないときは、「オフ」を選びます。
# Help topic "Sounds for logging in, the charger and the battery": step 3 of the numbered steps at block 2.
settings-help-topics-sounds-step2-3 = スピーカーを押すと、もう一度その音が聞こえます。

## Topic: Bluetooth headphones, mice and keyboards (src/settings/help_topics.rs)

# Title of the Help topic "Bluetooth headphones, mice and keyboards".
settings-help-topics-bluetooth-title = Bluetoothのヘッドホン、マウス、キーボード
# Words people might search for the Help topic "Bluetooth headphones, mice and keyboards" by, in lower case, separated by spaces.
settings-help-topics-bluetooth-keywords = bluetooth ペアリング ヘッドホン イヤホン マウス キーボード スピーカー 接続
# Help topic "Bluetooth headphones, mice and keyboards": step 1 of the numbered steps at block 1.
settings-help-topics-bluetooth-step1-1 = 設定を開いて「Bluetooth」を選びます。Bluetoothがオンになっているか確かめます。
# Help topic "Bluetooth headphones, mice and keyboards": step 2 of the numbered steps at block 1.
settings-help-topics-bluetooth-step1-2 = 機器をペアリングモードにします(機器の説明書を見てください)。
# Help topic "Bluetooth headphones, mice and keyboards": step 3 of the numbered steps at block 1.
settings-help-topics-bluetooth-step1-3 = 「機器を探す」を押し、機器が出てきたら、その横の「ペアリング」を押します。
# Help topic "Bluetooth headphones, mice and keyboards": step 4 of the numbered steps at block 1.
settings-help-topics-bluetooth-step1-4 = 相手の機器に数字が出たら、同じ数字か確かめて、「はい、同じです」を押します。
# Help topic "Bluetooth headphones, mice and keyboards": caption and spoken name of picture 2 (settings-bluetooth.png).
settings-help-topics-bluetooth-pic2 = Bluetoothのページ

## Topic: Screen size and displays (src/settings/help_topics.rs)

# Title of the Help topic "Screen size and displays".
settings-help-topics-display-title = 画面の大きさとディスプレイ
# Words people might search for the Help topic "Screen size and displays" by, in lower case, separated by spaces.
settings-help-topics-display-keywords = 画面 ディスプレイ 大きさ 大きく 小さく 文字 ズーム 拡大 解像度 モニター 回転 夜間モード 暖かい ブルーライト 夜 目
# Help topic "Screen size and displays": paragraph 1.
settings-help-topics-display-p1 = 表示が小さすぎたり大きすぎたりするときは、設定を開いて「ディスプレイ」を選び、「サイズ」を変えます。100%が、お使いの画面の標準の大きさです。
# Help topic "Screen size and displays": caption and spoken name of picture 2 (settings-display.png).
settings-help-topics-display-pic2 = ディスプレイのページ
# Help topic "Screen size and displays": paragraph 3.
settings-help-topics-display-p3 = 変更すると、そのままにするか聞くメッセージが出ます。15秒間何もしないと、画面は元に戻ります。選び間違えても、困ることはありません。
# Help topic "Screen size and displays": paragraph 4.
settings-help-topics-display-p4 = 同じページの下にある「夜間モード」は、夜に画面を暖かい色にして、眠りを妨げるブルーライトを減らします。オンにすると日没と日の出に合わせて切り替わります。Trindleは時間帯の設定からそれを計算するので、場所を聞くことはありません。開始と終了の時刻を自分で決めたいときは、「時刻を指定」を選びます。「暖かさ」を右に動かすと、画面がもっと暖かい色になります。

## Topic: NVIDIA graphics, and making them faster (src/settings/help_topics.rs)

# Title of the Help topic "NVIDIA graphics, and making them faster".
settings-help-topics-nvidia-title = NVIDIAのグラフィックスを速くする
# Words people might search for the Help topic "NVIDIA graphics, and making them faster" by, in lower case, separated by spaces.
settings-help-topics-nvidia-keywords = nvidia グラフィックス グラフィックカード ドライバー nouveau オープン 遅い 速度 gpu セキュアブート 独自 geforce
# Help topic "NVIDIA graphics, and making them faster": paragraph 1.
settings-help-topics-nvidia-p1 = Trindleは、どのNVIDIAカードでも、まずLinuxに入っているオープンソースのドライバー「nouveau」で始まります。インストールも選択も要りません。最初の起動から、デスクトップは使えます。
# Help topic "NVIDIA graphics, and making them faster": paragraph 2.
settings-help-topics-nvidia-p2 = 古いカードなら、それで十分です。2022年ごろ以降のカードでは、オープンのドライバーはカードの速度を最低のところより上げられません。デスクトップは動きますが、もたつく感じがして、動画がコマ落ちすることもあります。NVIDIA独自のドライバーを入れると直ります。
# Help topic "NVIDIA graphics, and making them faster": tip 3, shown in a box of its own.
settings-help-topics-nvidia-tip3 = コンピューターの調子が良ければ、そのままにしてください。グラフィックスのドライバーを変えると、画面が映らなくなることがよくあります。
# Help topic "NVIDIA graphics, and making them faster": paragraph 4.
settings-help-topics-nvidia-p4 = NVIDIA独自のドライバーを入れるには、リングからターミナルを開いて、下の2行を1行ずつ入力します。aptが「インストールできない」という意味のメッセージを出したら、先にTrindleの更新を入れてから、2行をもう一度入力してください。
# Help topic "NVIDIA graphics, and making them faster": step 1 of the numbered steps at block 5.
settings-help-topics-nvidia-step5-1 = sudo apt update
# Help topic "NVIDIA graphics, and making them faster": step 2 of the numbered steps at block 5.
settings-help-topics-nvidia-step5-2 = sudo apt install nvidia-driver
# Help topic "NVIDIA graphics, and making them faster": step 3 of the numbered steps at block 5.
settings-help-topics-nvidia-step5-3 = まだ再起動しないでください。次の説明を先に読んでください。
# Help topic "NVIDIA graphics, and making them faster": paragraph 6.
settings-help-topics-nvidia-p6 = Debianは、インストールのときに、お使いのカーネルに合わせてドライバーをビルドします。数分かかり、ソフトウェアを作るためのツールであるコンパイラーも入ります。カーネルが更新されるたびに、ドライバーは自分で作り直されます。
# Help topic "NVIDIA graphics, and making them faster": paragraph 10.
settings-help-topics-nvidia-p10 = オープンのドライバーにいつでも戻すには、sudo apt remove nvidia-driver を実行してから、再起動します。

## Topic: Making the screen bigger where you are looking (src/settings/help_topics.rs)

# Title of the Help topic "Making the screen bigger where you are looking".
settings-help-topics-magnifier-title = 見ているところを大きくする
# Words people might search for the Help topic "Making the screen bigger where you are looking" by, in lower case, separated by spaces.
settings-help-topics-magnifier-keywords = 拡大鏡 拡大 ズーム 大きく 読む 老眼鏡 視力 小さい文字 ルーペ アクセシビリティ magnifier
# Help topic "Making the screen bigger where you are looking": paragraph 1.
settings-help-topics-magnifier-p1 = 拡大鏡は、押したところに拡大した丸い窓を出します。画面そのものは変えずに、小さい文字を読めます。
# Help topic "Making the screen bigger where you are looking": step 1 of the numbered steps at block 2.
settings-help-topics-magnifier-step2-1 = リングの「拡大鏡」の丸を押します。
# Help topic "Making the screen bigger where you are looking": step 2 of the numbered steps at block 2.
settings-help-topics-magnifier-step2-2 = 画面のどこかを、指かマウスのボタンで押し続けます。丸い窓が出て、その下にあるものが大きく見えます。
# Help topic "Making the screen bigger where you are looking": step 3 of the numbered steps at block 2.
settings-help-topics-magnifier-step2-3 = 押したまま動かします。丸い窓がついてくるので、1行ずつ読めます。
# Help topic "Making the screen bigger where you are looking": step 4 of the numbered steps at block 2.
settings-help-topics-magnifier-step2-4 = 離すと丸い窓は消えます。使い終わったら、拡大鏡のボタンをもう一度押します。
# Help topic "Making the screen bigger where you are looking": tip 3, shown in a box of its own.
settings-help-topics-magnifier-tip3 = 拡大鏡がオンの間、下の画面は動かず、押すこともできません。読んでいる途中で、うっかり何かを動かすことはありません。

## Topic: Picking a colour from the screen (src/settings/help_topics.rs)

# Title of the Help topic "Picking a colour from the screen".
settings-help-topics-eye-dropper-title = 画面から色を選ぶ
# Words people might search for the Help topic "Picking a colour from the screen" by, in lower case, separated by spaces.
settings-help-topics-eye-dropper-keywords = スポイト 色 カラー 選ぶ ピッカー 16進 コード コピー 見本 画面 eyedropper hex
# Help topic "Picking a colour from the screen": paragraph 1.
settings-help-topics-eye-dropper-p1 = スポイトは、画面のどの1点でも、その色を読み取って、#FBC711 のようなコードをコピーします。文書やデザインに貼り付けられます。
# Help topic "Picking a colour from the screen": step 1 of the numbered steps at block 2.
settings-help-topics-eye-dropper-step2-1 = リングの「スポイト」の丸を押します。スポイトの絵が付いているものです。
# Help topic "Picking a colour from the screen": step 2 of the numbered steps at block 2.
settings-help-topics-eye-dropper-step2-2 = 画面は止まり、ポインターのそばに拡大した丸い窓が出ます。それを見て、1点に狙いを定めます。
# Help topic "Picking a colour from the screen": step 3 of the numbered steps at block 2.
settings-help-topics-eye-dropper-step2-3 = 欲しい色の点を押します。やめるときは、代わりにEscキーを押します。
# Help topic "Picking a colour from the screen": step 4 of the numbered steps at block 2.
settings-help-topics-eye-dropper-step2-4 = コードはすぐにコピーされて、一覧の一番上に、新しい順に加わります。
# Help topic "Picking a colour from the screen": tip 3, shown in a box of its own.
settings-help-topics-eye-dropper-tip3 = 前に選んだ色は一覧に残ります。選び直さなくても、あとでもう一度コピーできます。

## Topic: Taking a picture of the screen (src/settings/help_topics.rs)

# Title of the Help topic "Taking a picture of the screen".
settings-help-topics-screenshots-title = 画面を写真のように撮る
# Words people might search for the Help topic "Taking a picture of the screen" by, in lower case, separated by spaces.
settings-help-topics-screenshots-keywords = スクリーンショット 画面 撮る 画像 キャプチャ print prtsc super shift s 範囲 一部 コピー 貼り付け クリップボード screenshot
# Help topic "Taking a picture of the screen": paragraph 1.
settings-help-topics-screenshots-p1 = Printキーを押します。キーボードにない場合は、Super、Shift、Sを押します。リングの「スクリーンショット」の丸(カメラの絵)を押してもできます。リングが先に閉じるので、写真には写りません。そのあと画面は止まって、少し暗くなります。撮りたいところをドラッグして囲むか、下にある2つの丸いボタンを使います。1つは画面全体用、もう1つは閉じる用です。ボタンの上にポインターを置くと、名前が見えます。
# Help topic "Taking a picture of the screen": step 1 of the numbered steps at block 2.
settings-help-topics-screenshots-step2-1 = 画面全体を撮るには、画面のボタンを押します。
# Help topic "Taking a picture of the screen": step 2 of the numbered steps at block 2.
settings-help-topics-screenshots-step2-2 = 一部を撮るには、点線の四角を押して、撮りたいところをドラッグします。指を離した瞬間に撮れます。
# Help topic "Taking a picture of the screen": step 3 of the numbered steps at block 2.
settings-help-topics-screenshots-step2-3 = やめたくなったら、バツを押すか、Escキーを押します。
# Help topic "Taking a picture of the screen": paragraph 3.
settings-help-topics-screenshots-p3 = Trindleは、写真を「ピクチャ」の中の「Screenshots」フォルダーに保存します。ファイル名には日付と時刻が入ります。Trindleは写真をコピーもするので、メールや文書にそのまま貼り付けられます。うまくいったときは何も表示されません。問題があったときだけメッセージが出ます。
# Help topic "Taking a picture of the screen": tip 4, shown in a box of its own.
settings-help-topics-screenshots-tip4 = ShiftとPrintを押すと、何も聞かずに画面全体をすぐ撮ります。

## Topic: Having the screen read aloud (src/settings/help_topics.rs)

# Title of the Help topic "Having the screen read aloud".
settings-help-topics-screen-reader-title = 画面を読み上げてもらう
# Words people might search for the Help topic "Having the screen read aloud" by, in lower case, separated by spaces.
settings-help-topics-screen-reader-keywords = スクリーンリーダー 目が見えない 読み上げ 話す 音声 声 視力 アクセシビリティ screen reader
# Help topic "Having the screen read aloud": paragraph 1.
settings-help-topics-screen-reader-p1 = スクリーンリーダーは入っていないので、設定の「外観」の「画面の読み上げ」にある「画面を読み上げる」スイッチは灰色になっていて、その旨のメモが付いています。
# Help topic "Having the screen read aloud": paragraph 2.
settings-help-topics-screen-reader-p2 = スクリーンリーダーが使えるようになったら、Super、Alt、Sを同時に押すと、どこからでもオンとオフを切り替えられます。コンピューターを初めて起動したときの、見るものも押すものもない状態でも使えます。それまでは、このキーを押すと、そのスイッチのある設定が開きます。

## Topic: Wallpaper (src/settings/help_topics.rs)

# Title of the Help topic "Wallpaper".
settings-help-topics-wallpaper-title = 壁紙
# Words people might search for the Help topic "Wallpaper" by, in lower case, separated by spaces.
settings-help-topics-wallpaper-keywords = 壁紙 背景 画像 写真 スライドショー 変更 タイル 画像 フォルダー 追加 削除 スキップ wallpaper
# Help topic "Wallpaper": paragraph 1.
settings-help-topics-wallpaper-p1 = 設定を開いて「壁紙」を選びます。Trindleに入っている画像は小さなタイルで表示され、今画面に出ているものは右の大きなプレビューに出ます。
# Help topic "Wallpaper": step 1 of the numbered steps at block 2.
settings-help-topics-wallpaper-step2-1 = タイルを押すと、その画像が画面に出ます。
# Help topic "Wallpaper": step 2 of the numbered steps at block 2.
settings-help-topics-wallpaper-step2-2 = もう一度押すと、その画像を使わないことにできます。タイルが暗くなり、それ以降その画像は飛ばされます。もう一度押すと元に戻ります。
# Help topic "Wallpaper": step 3 of the numbered steps at block 2.
settings-help-topics-wallpaper-step2-3 = 自分の画像は「画像を選ぶ」で、自分のフォルダーは「フォルダーを選ぶ」で追加できます。
# Help topic "Wallpaper": step 4 of the numbered steps at block 2.
settings-help-topics-wallpaper-step2-4 = 「ときどき画像を変える」をオンにして、間隔を選びます。使われるのは、外していない画像だけです。
# Help topic "Wallpaper": tip 3, shown in a box of its own.
settings-help-topics-wallpaper-tip3 = 画像を外しても、削除されることはありません。タイルは暗くなって残るので、気が変わっても大丈夫です。
# Help topic "Wallpaper": caption and spoken name of picture 4 (settings-wallpaper.png).
settings-help-topics-wallpaper-pic4 = 壁紙のページ

## Topic: Colours, icons and text (src/settings/help_topics.rs)

# Title of the Help topic "Colours, icons and text".
settings-help-topics-look-title = 色、アイコン、文字
# Words people might search for the Help topic "Colours, icons and text" by, in lower case, separated by spaces.
settings-help-topics-look-keywords = 色 カラー アクセント テーマ ダーク ライト モード 日差し 明るい フォント 文字 アイコン カーソル ポインター 見た目
# Help topic "Colours, icons and text": paragraph 1.
settings-help-topics-look-p1 = 設定を開いて「外観」を選びます。「テーマ」の下で、ダークとライトの両方の見た目があるテーマには、絵の横に「ダーク」と「ライト」の2つのボタンがあります。明るい日差しで画面が見にくいときは「ライト」を、あとでまた「ダーク」を押します。変更はすぐにすべてに反映されます。ただし「アプリ」から入れたアプリは、次に開いたときに変わります。ミッドナイト(ダーク)やペーパー(ライト)のように、見た目が1つだけのテーマには、このボタンはありません。ダークとライトを切り替えたいときは、別のテーマを選びます。
# Help topic "Colours, icons and text": paragraph 2.
settings-help-topics-look-p2 = ハイライトの色は、ボタンやスイッチ、今いる場所を示すために使われます。20色から選ぶか、自分で入力します。
# Help topic "Colours, icons and text": caption and spoken name of picture 3 (settings-appearance.png).
settings-help-topics-look-pic3 = 外観のページ
# Help topic "Colours, icons and text": paragraph 4.
settings-help-topics-look-p4 = 「アイコン」の下で、「丸」か「四角」を選びます。どちらもTrindleのアイコンです。アプリのマークを、そのアプリ専用の色の上に白で描いていて、丸か角の丸い四角で、ライトでもダークでも同じに見えます。Trindleは丸で始まります。そのアプリのアイコンがまだセットにないときは、アプリ自身の絵が白い丸か四角の上に出るので、並べても揃って見えます。
# Help topic "Colours, icons and text": paragraph 5.
settings-help-topics-look-p5 = 同じページで、フォントとポインターの大きさも変えられます。自分のフォントを追加することもできます。「自分のフォントを追加する」を見てください。
# Help topic "Colours, icons and text": paragraph 6.
settings-help-topics-look-p6 = テーマは、見た目全体を一度に変えます。「テーマ」を見てください。影、角、アプリの背景などをもっと細かく自分で変えたいときは、「スタイルシートで見た目を変える」を見てください。

## Topic: Adding fonts of your own (src/settings/help_topics.rs)

# Title of the Help topic "Adding fonts of your own".
settings-help-topics-themes-title = 自分のフォントを追加する
# Words people might search for the Help topic "Adding fonts of your own" by, in lower case, separated by spaces.
settings-help-topics-themes-keywords = フォント ttf otf ttc zip 追加 アップロード インストール 書体 文字
# Help topic "Adding fonts of your own": paragraph 1.
settings-help-topics-themes-p1 = 設定の「外観」で、ダウンロードしたフォントを追加できます。Trindleに入っているものだけを使う必要はありません。「文字」の下で追加すると、そこの「フォント」の一覧に出ます。
# Help topic "Adding fonts of your own": paragraph 2.
settings-help-topics-themes-p2 = フォントは、.ttf、.otf、.ttcのファイル、またはそれを入れたzipファイルです。Trindleは見つけたフォントをすべて、自分用のフォントフォルダーにコピーし、「フォント」の一覧に出します。コンピューターを使う全員にフォントを入れたいときは、「アプリ」を使ってください。
# Help topic "Adding fonts of your own": tip 3, shown in a box of its own.
settings-help-topics-themes-tip3 = 追加したものはどこにも送られず、すでにコンピューターにあるものが置き換わることもありません。

## Topic: Themes: a whole look from one file (src/settings/help_topics.rs)

# Title of the Help topic "Themes: a whole look from one file".
settings-help-topics-theme-files-title = テーマ: 1つのファイルで見た目全体を変える
# Words people might search for the Help topic "Themes: a whole look from one file" by, in lower case, separated by spaces.
settings-help-topics-theme-files-keywords = テーマ 見た目 スキン ファイル css 追加 共有 作る 自分 ガラス ダーク ライト 保持 戻す 元に戻す 30秒 下 安全 標準 試す theme
# Help topic "Themes: a whole look from one file": paragraph 1.
settings-help-topics-theme-files-p1 = テーマは、色、角、大きさ、間隔などを含む見た目全体で、1つのファイルに入っています。設定の「外観」の「テーマ」では、どのテーマも、そのテーマ自身の色で描いた小さな絵で表示されます。Trindle自身の見た目である深い夜の青は、いつも一番上にあり、削除はできません。Trindleに入っている「ミッドナイト」は、青を使わない黒と灰色です。以前のTrindle自身の見た目は、これでした。
# Help topic "Themes: a whole look from one file": step 1 of the numbered steps at block 2.
settings-help-topics-theme-files-step2-1 = テーマを押すと、試せます。Trindleのすべてのウィンドウがすぐに変わります。ダークとライトの両方があるテーマでは、絵の横に「ダーク」と「ライト」が出ます。その見た目で試したいときは、代わりにそちらを押します。今使っているテーマでは、確認なしですぐに見た目が変わります。
# Help topic "Themes: a whole look from one file": step 2 of the numbered steps at block 2.
settings-help-topics-theme-files-step2-2 = ウィンドウの下に「保持」の欄が出ます。ページをどこまで下に動かしても、見える位置にあります。30秒以内に「保持」を押すと、新しい見た目が残ります。何もしないか、「今すぐ元に戻す」かEscキーを押すと、前の見た目に自動で戻ります。どこでもそうなり、設定を閉じていても同じです。
# Help topic "Themes: a whole look from one file": step 3 of the numbered steps at block 2.
settings-help-topics-theme-files-step2-3 = 誰かが共有したテーマを追加するには、「テーマを追加」を押して、その.cssファイルを選びます。Trindleは先に中身を調べます。使えないときは、どの行が、なぜだめなのかを表示します。
# Help topic "Themes: a whole look from one file": paragraph 3.
settings-help-topics-theme-files-p3 = テーマファイルは、Trindle自身のシートと同じ書き方です(「スタイルシートで見た目を変える」を見てください)。同じ名前のものを入れた :root { "{" } { "}" } ブロックがあり、たとえば --trindle-page: #101820; のように書きます。先頭には、--trindle-theme-name、--trindle-theme-about(1行)、--trindle-theme-author、--trindle-theme-version で、自分が何かを書けます。--trindle-theme-look: dark; light; または both; で、どの見た目があるかも書けます。書かなかったものは、Trindleの標準のままです。追加したテーマは、.configフォルダーの中のtrindleフォルダーにある、themesフォルダーに保存されます。そこから1つ取り除くと、一覧からもなくなります。
# Help topic "Themes: a whole look from one file": paragraph 4.
settings-help-topics-theme-files-p4 = 両方の見た目があるテーマは、1つのファイルに両方を入れます。ダークの値はいつもどおり :root { "{" } { "}" } に書き、ライトの値は2つ目のブロック @media (prefers-color-scheme: light) { "{" } :root { "{" } { "}" } { "}" } に書きます。Trindle自身のシートと同じ形です。ライトのブロックには、ダークが決めているすべての色と影を書いてください。書かないと、ライトの見た目にダークの値が混ざって出ます。すると、設定ではテーマの絵の横に「ダーク」と「ライト」が出ます。dark または light と書いたテーマは、その見た目だけで、いつもその見た目で描かれ、このボタンはありません。何も書かなかったときは、ライトのブロックがあれば両方の見た目があるものとして、なければダークとして扱います。ほかのプログラムには、テーマが実際に描かれている見た目が伝わるので、ライトのテーマならほかのプログラムもライトになります。
# Help topic "Themes: a whole look from one file": paragraph 5.
settings-help-topics-theme-files-p5 = テーマは、誰が作ったものでも、安全に試せます。Trindleはテーマを値の一覧として読むだけで、実行はしません。ほかのファイルを読み込んだり、インターネットに出たりもできません。Trindleは知っている値だけを受け取り、それぞれを決まった範囲に収めます。だからどんなテーマも、ボタンや質問や警告を隠したり、読めないほど小さな文字にしたりできません。テーマがどんな色を指定しても、文字は読みやすいままで、赤は赤のまま、ハイライトの色は自分のものです。テーマが変えるのは、自分のウィンドウだけです。ログイン画面は、いつもTrindle自身の見た目です。
# Help topic "Themes: a whole look from one file": tip 6, shown in a box of its own.
settings-help-topics-theme-files-tip6 = 見た目がおかしくなって戻れなくなったら、ターミナルか、Ctrl、Alt、F3で開くテキストのコンソールを開いて、次を入力してください: trindle-settings --theme default

## Topic: Changing the look with a style sheet (src/settings/help_topics.rs)

# Title of the Help topic "Changing the look with a style sheet".
settings-help-topics-theming-title = スタイルシートで見た目を変える
# Words people might search for the Help topic "Changing the look with a style sheet" by, in lower case, separated by spaces.
settings-help-topics-theming-keywords = テーマ スタイルシート css 色 影 角 丸く 大きく 文字サイズ ボタン 背景 ページ 見た目 カスタム 自分用 theme stylesheet
# Help topic "Changing the look with a style sheet": paragraph 1.
settings-help-topics-theming-p1 = Trindle が描くものはすべて、ウェブページと同じ書き方の 1 つのスタイルシートで決まります。色、影、角、物と物の間隔、ボタンの大きさ、文字の大きさと太さも含みます。自分で作ったシートで、どれでも変えられます。AI に頼んでも変えられます。
# Help topic "Changing the look with a style sheet": step 1 of the numbered steps at block 2.
settings-help-topics-theming-step2-1 = .config フォルダーの中の trindle フォルダーに、style.css というファイルを作ります。Folders では、隠しファイルを表示すると見えます。
# Help topic "Changing the look with a style sheet": step 2 of the numbered steps at block 2.
settings-help-topics-theming-step2-2 = 変えたいものだけを :root { "{" } { "}" } の中に書きます。たとえば --trindle-page: #101010; で全ウィンドウのページが黒に近くなり、--trindle-highlight: #4bae4f; でボタンが緑になり、--trindle-card-corner: 16; でカードの角が丸くなります。
# Help topic "Changing the look with a style sheet": step 3 of the numbered steps at block 2.
settings-help-topics-theming-step2-3 = 保存します。Trindle のすべてのウィンドウ、リング、タイトルバーがすぐに変わります。
# Help topic "Changing the look with a style sheet": paragraph 3.
settings-help-topics-theming-p3 = 使える名前はすべて、Trindle 自身のシート /usr/share/trindle/style.css に載っています。それぞれの横に、働きと (6 から 18) のような範囲のメモがあります。明るい見た目の変更は、Trindle のシートと同じく、@media (prefers-color-scheme: light) { "{" } :root { "{" } { "}" } { "}" } のブロックの中にだけ書きます。影はウェブページと同じ書き方で、たとえば 0 8px 20px 2px rgba(0, 0, 0, 0.8) のように書きます。カードには初めから軽い影があります。--trindle-card-shadow: none; で平らになり、--trindle-edge: transparent; で細い縁が消えます。
# Help topic "Changing the look with a style sheet": paragraph 4.
settings-help-topics-theming-p4 = 何を書いても、ウィンドウが壊れることはありません。Trindle は知らない名前を無視し、読めない値は自分の値に置き換え、大きさは範囲内に収めます。色は自由で、書いたとおりに残ります。Trindle は、ページ、カード、タイトルバーの上で読めるように、自分の文字の色を変えます。明るい見た目用の色は、明るい見た目のときだけ使われます。透ける色は、後ろに何が見えるか Trindle には分からないので、黒の上と白の上の両方で確かめます。どの色にも合う文字の色がない場合は、黒か白を使います。次の 3 つは必ず読めます。失敗を知らせる赤い文字、ハイライトの色の上の文字、そしてパスワード欄です。パスワード欄はいつも Trindle 自身の見た目です。色が読みにくいときは、設定の外観に「読みにくい色があります。」と出ますが、色は書いたままです。自分で決めた文字が読みにくくなったら、ファイルを削除すれば、Trindle の見た目がすぐに戻ります。ファイルは 256 KB までの普通のテキストファイルにしてください。そうでないと Trindle は使わず、設定の外観でお知らせします。
# Help topic "Changing the look with a style sheet": paragraph 5.
settings-help-topics-theming-p5 = 自分のシートは、選んだテーマの上に重なります。だから、小さな変更はテーマを変えても残ります。残したり共有したりできる見た目を丸ごと作るには、代わりにテーマを作ります。「テーマ」を見てください。
# Help topic "Changing the look with a style sheet": tip 6, shown in a box of its own.
settings-help-topics-theming-tip6 = シートに --trindle-highlight を書くと、設定の外観で選んだハイライトの色より優先されます。

## Topic: Voice control (src/settings/help_topics.rs)

# Title of the Help topic "Voice control".
settings-help-topics-voice-title = 音声操作
# Words people might search for the Help topic "Voice control" by, in lower case, separated by spaces.
settings-help-topics-voice-keywords = 音声 話す しゃべる マイク コマンド whisper 聞く 言う
# Help topic "Voice control": paragraph 1.
settings-help-topics-voice-p1 = 音声操作は追加の機能です。追加するには、設定を開き、リングを選んで、音声操作を追加を押します。数分かかり、約 150 MB 使います。
# Help topic "Voice control": paragraph 2.
settings-help-topics-voice-p2 = そのあとリングを開き、ボタンのない場所で真ん中を押して話します。話し終わると Trindle は聞くのをやめます。もう一度押してやめることもできます。指でもマウスでも使えます。次のように話せます。
# Help topic "Voice control": step 1 of the numbered steps at block 3.
settings-help-topics-voice-step3-1 = アプリの名前。たとえば「Firefox」や「folders」。
# Help topic "Voice control": step 2 of the numbered steps at block 3.
settings-help-topics-voice-step3-2 = アプリへ「切り替え」、または「ウィンドウを閉じる」。
# Help topic "Voice control": step 3 of the numbered steps at block 3.
settings-help-topics-voice-step3-3 = 「音を大きく」、「音を小さく」、「ミュート」。
# Help topic "Voice control": step 4 of the numbered steps at block 3.
settings-help-topics-voice-step3-4 = 「Wi-Fi の設定」、「バッテリーの設定」など、ほかの設定ページ。
# Help topic "Voice control": step 5 of the numbered steps at block 3.
settings-help-topics-voice-step3-5 = 「インストール」とアプリの名前。アプリで探します。
# Help topic "Voice control": step 6 of the numbered steps at block 3.
settings-help-topics-voice-step3-6 = 「次の壁紙」、「スクリーンショット」、「いま何時」。
# Help topic "Voice control": tip 4, shown in a box of its own.
settings-help-topics-voice-tip4 = Trindle はこのコンピューターの中で声を理解します。どこにも送られません。音声操作は、削除しなくても、リングのページでオフにできます。

## Topic: Folders, USB sticks and phones (src/settings/help_topics.rs)

# Title of the Help topic "Folders, USB sticks and phones".
settings-help-topics-files-title = Folders、USB メモリ、スマートフォン
# Words people might search for the Help topic "Folders, USB sticks and phones" by, in lower case, separated by spaces.
settings-help-topics-files-keywords = フォルダー ファイル ファイルマネージャー usb メモリ ドライブ スマートフォン 携帯 カメラ nas ネットワーク 共有 samba ディスク 取り出し フォーマット 展開 解凍 zip 圧縮 アーカイブ rar 7z tar iso ディスクイメージ パスワード ターミナル f4 root 壁紙 コピー パス 場所 プレビュー スペースバー 絵文字 バッジ サイズ変更 回転 画像 列 音楽 写真 pdf キー ショートカット タブ 新しいウィンドウ
# Help topic "Folders, USB sticks and phones": paragraph 1.
settings-help-topics-files-p1 = Folders はファイルマネージャーで、リングの周りにあるアプリの 1 つです。つないだ USB メモリ、スマートフォン、カメラは左側に出ます。同じローカルネットワークの共有フォルダーも、ネットワークの下に出ます。
# Help topic "Folders, USB sticks and phones": paragraph 2.
settings-help-topics-files-p2 = Folders は、ホームフォルダーを見せる 1 つのペインで開きます。左には、ダウンロードとドキュメントが固定されています。左上の「シングル」と「デュアル」の切り替え (または F3) で、2 つ目のペインを追加できます。Folders の設定で別の場所を選ばない限り、2 つ目のペインには最近使ったものが出ます。設定は、上のバーの端にある歯車 (または Ctrl とカンマ) で開きます。ここでは、各ペインを始める場所、ファイルを選ぶためのチェックボックスを出すか、ファイルの拡張子を出すか、ドライブをつないだらすぐ開くか、を決められます。各ペインには、リストの上に自分のタブの列があります。上のバーには、フォルダーの見え方を変えるボタンと、元に戻すと新しいフォルダーだけがあります。選んだファイルへの操作は、名前の変更、切り取り、コピー、ゴミ箱へ移動、完全に削除、安全に消去など、すべて右クリックのメニューにあり、どれにもキーがあります。
# Help topic "Folders, USB sticks and phones": paragraph 3.
settings-help-topics-files-p3 = 右クリックのメニューの「別のアプリで開く」には、そのファイルを開けるアプリが並びます。1 つをダブルクリックするか、Enter を押すと、すぐそのアプリでファイルが開きます。チェックマークでも同じです。
# Help topic "Folders, USB sticks and phones": paragraph 4.
settings-help-topics-files-p4 = USB メモリやメモリーカードをつなぐと、リングに「開く」ボタンのある行が出ます。開くを押すと、Folders でファイルが見られます。ドライブにロックがかかっているときは、先にパスフレーズを聞かれます。開くを押さなくても、Folders はドライブの下にそのドライブを表示し、安全に取り外せる丸い取り出しボタンもあります。Folders に自動でドライブを開かせるには、Folders の設定のドライブにある「つないだらドライブを開く」をオンにします。画面がロックされている間につないだドライブは、ロックを解除するまで待ちます。
# Help topic "Folders, USB sticks and phones": paragraph 5.
settings-help-topics-files-p5 = Folders で右クリックすると、さらにできます。アーカイブにはここに展開とフォルダーに展開、.zip や .tar を作る圧縮、ターミナルで開く、root で開く、場所をコピー、画像には壁紙に設定があります。Folders はアーカイブを自分で開きます。.zip、.7z、.rar、.tar、ディスクイメージ (.iso)、ほとんどどんな種類でもダブルクリックすると、フォルダーのように開くので、中を見て、欲しいものだけ取り出せます。.zip や .7z にパスワードがかかっているときは、Folders が聞きます。パスワードは覚えないので、次もまた聞きます。パスワードのかかった .rar は開けません。そのときは Folders が知らせます。
# Help topic "Folders, USB sticks and phones": paragraph 6.
settings-help-topics-files-p6 = ファイルでスペースバーを押すと、開かずに中が見られます。もう一度押すとプレビューが閉じます。F4 で、いまのフォルダーでターミナルが開きます。フォルダーやファイルを右クリックして「リングに固定」を選ぶと、お気に入りのアプリと一緒にリングの真ん中の周りに置かれ、1 クリックで開けます。固定を外すには、そこで右クリックします。画像を右クリックすると、大きさを変えたり回したりできます。ファイルのプロパティでは、小さなバッジを付けられます。フォルダーのプロパティでは、同じローカルネットワークの別のコンピューターと共有できます (カフェや会社のネットワークでは、そこにいる全員です)。初めてのときは、Folders が Samba をオンにし、パスワードを聞きます。コンピューターを再起動すると共有が使えます。リスト表示では、列の見出しを右クリックすると、音楽、写真、PDF の情報を追加できます。
# Help topic "Folders, USB sticks and phones": paragraph 7.
settings-help-topics-files-p7 = 上のパスの各名前の後ろには、小さな矢印があります。矢印を押すとそのフォルダーの中のフォルダーが見え、1 つ選ぶとそこへ移動します。フォルダーを中クリックするか、Ctrl を押しながらダブルクリックすると、いまのタブの後ろの新しいタブで開きます。タブを中クリックすると閉じます。固定したフォルダーの間にフォルダーをドラッグすると、そこに固定されます。固定したフォルダーを上下にドラッグすると、好きな順に並べられます。検索ボックスの下には、種類、サイズ、変更日の 3 つのリストがあります。選ぶと、表示がすぐ絞られます。
# Help topic "Folders, USB sticks and phones": paragraph 8.
settings-help-topics-files-p8 = キーはファイルマネージャーでおなじみのものです。Alt と左で戻る、Alt と右で進む、Alt と上、または Backspace で 1 つ上のフォルダーへ行きます。Ctrl と L、または Alt と D で、場所を入力できます。Ctrl と T でタブを開き、Ctrl と W で閉じます (最後のタブならウィンドウも閉じます)。Ctrl と Tab で次のタブ、Ctrl と Shift と Tab で前のタブへ移ります。Ctrl と N で別のウィンドウを開きます。Ctrl と F で検索、F2 で名前の変更、F5 でフォルダーの再読み込みです。Delete でファイルをゴミ箱へ移し、Shift と Delete で完全に削除します。Ctrl と Shift と N でフォルダーを作ります。Ctrl と A ですべて選択、Ctrl と Shift と A で選択を解除、Ctrl と I で選択と非選択を入れ替えます。Alt と Enter でプロパティを開きます。F3 で 1 ペインと 2 ペインを切り替え、F4 でターミナルを開きます。
# Help topic "Folders, USB sticks and phones": paragraph 9.
settings-help-topics-files-p9 = ドライブには、すべてのドライブと空き容量が出ます。すべてのアプリと、設定のこのコンピューターについてにあります。ここから、USB メモリを開いたり、抜く前に安全に取り出したり、フォーマットしたりできます。システムが動いているドライブをフォーマットするよう勧めることは、Trindle にはありません。
# Help topic "Folders, USB sticks and phones": caption and spoken name of picture 10 (settings-disks.png).
settings-help-topics-files-pic10 = ドライブのウィンドウ

## Topic: Apps that come as an AppImage (src/settings/help_topics.rs)

# Title of the Help topic "Apps that come as an AppImage".
settings-help-topics-appimages-title = AppImage で配布されるアプリ
# Words people might search for the Help topic "Apps that come as an AppImage" by, in lower case, separated by spaces.
settings-help-topics-appimages-keywords = appimage アプリ イメージ ダウンロード ポータブル ウェブサイト インストール
# Help topic "Apps that come as an AppImage": paragraph 1.
settings-help-topics-appimages-p1 = アプリによっては、作った人のウェブサイトから、.AppImage で終わる 1 つのファイルとしてダウンロードします。アプリを開き、インストール済みを選び、AppImage を追加の横のプラスを押して、ファイルを選びます。Trindle はそれをホームフォルダーの .appimage フォルダーにコピーし、アプリの名前と絵がリングに出ます。
# Help topic "Apps that come as an AppImage": paragraph 2.
settings-help-topics-appimages-p2 = ほかのアプリと同じページに並びます。削除するには、ごみ箱を 2 回押します。ダウンロードしたファイルは元の場所に残ります。

## Topic: Older programs: Steam, Wine games and Java apps (src/settings/help_topics.rs)

# Title of the Help topic "Older programs: Steam, Wine games and Java apps".
settings-help-topics-older-programs-title = 古いプログラム: Steam、Wine のゲーム、Java アプリ
# Words people might search for the Help topic "Older programs: Steam, Wine games and Java apps" by, in lower case, separated by spaces.
settings-help-topics-older-programs-keywords = 古いプログラム x11 xwayland steam wine ゲーム java 古いアプリ 許可 切り替え 見る のぞく
# Help topic "Older programs: Steam, Wine games and Java apps": paragraph 1.
settings-help-topics-older-programs-p1 = Trindle のほとんどのアプリは、新しい方法でウィンドウを描きます。Steam とそのゲーム、Wine のゲーム、Java アプリなど、一部の古いプログラムは、Xwayland という助っ人を使う古い方法で描きます。ほかのデスクトップでは、古いプログラムがすべて 1 つの助っ人を共有するので、どれでもほかのウィンドウや、そこに入力する内容をのぞけます。
# Help topic "Older programs: Steam, Wine games and Java apps": paragraph 2.
settings-help-topics-older-programs-p2 = Trindle は古いアプリごとに専用の助っ人を付けるので、お互いに見えません。これは、自分のフォルダーにインストールしたものすべてと、必要とする Flathub のアプリすべてに当てはまります。コンピューター全体用にインストールしたプログラムは、1 つの助っ人を共有します。助っ人は、古いプログラムが開いたときだけ始まり、閉じると止まるので、それ以外の間はメモリを使いません。
# Help topic "Older programs: Steam, Wine games and Java apps": tip 3, shown in a box of its own.
settings-help-topics-older-programs-tip3 = 古いプログラムを使わないなら、設定の権限で「古いプログラムを許可」をオフにします。オフの間は開きません。

## Topic: Printing (src/settings/help_topics.rs)

# Title of the Help topic "Printing".
settings-help-topics-printers-title = 印刷
# Words people might search for the Help topic "Printing" by, in lower case, separated by spaces.
settings-help-topics-printers-keywords = 印刷 プリンター テストページ
# Help topic "Printing": paragraph 1.
settings-help-topics-printers-p1 = 設定を開き、プリンターを選びます。Trindle はローカルネットワークのプリンターを自動で見つけます。自分のプリンターの横の「このプリンターを追加」を押します。動くか確かめるには、「テストページを印刷」を押します。
# Help topic "Printing": caption and spoken name of picture 2 (settings-printers.png).
settings-help-topics-printers-pic2 = プリンターのページ

## Topic: People, passwords and pictures (src/settings/help_topics.rs)

# Title of the Help topic "People, passwords and pictures".
settings-help-topics-users-title = ユーザー、パスワード、写真
# Words people might search for the Help topic "People, passwords and pictures" by, in lower case, separated by spaces.
settings-help-topics-users-keywords = ユーザー アカウント パスワード 写真 アイコン 管理者 追加 削除
# Help topic "People, passwords and pictures": paragraph 1.
settings-help-topics-users-p1 = 設定を開き、ユーザーを選びます。パスワードの変更、ログイン画面に出る自分の写真の選択、ほかの人のアカウントの追加ができます。
# Help topic "People, passwords and pictures": caption and spoken name of picture 2 (settings-users.png).
settings-help-topics-users-pic2 = ユーザーのページ
# Help topic "People, passwords and pictures": tip 3, shown in a box of its own.
settings-help-topics-users-tip3 = アカウントを変えるには、管理者のパスワードが必要です。

## Topic: Language and keyboard (src/settings/help_topics.rs)

# Title of the Help topic "Language and keyboard".
settings-help-topics-language-title = 言語とキーボード
# Words people might search for the Help topic "Language and keyboard" by, in lower case, separated by spaces.
settings-help-topics-language-keywords = 言語 キーボード 配列 イギリス アメリカ 入力 地域 形式 日付 通貨 マウス タッチパッド ポインター 速度 タップ クリック スクロール ナチュラル 左利き
# Help topic "Language and keyboard": paragraph 1.
settings-help-topics-language-p1 = 設定のキーボードとマウスでは、キーボード配列と、切り替える 2 つ目の配列を選びます。設定の地域と言語では、メニューの言語と、日付、お金、数字の書き方を選びます。次にログインしたときから有効になります。
# Help topic "Language and keyboard": paragraph 2.
settings-help-topics-language-p2 = 同じキーボードとマウスのページで、ポインターとスクロールの速さ、スクロールでページが動く向き、左利き用にマウスのボタンを入れ替えるかどうかも決められます。ノートパソコンでは、タップでクリックと、入力中にタッチパッドをオフにする設定もあります。変更はすぐに働きます。
# Help topic "Language and keyboard": caption and spoken name of picture 3 (settings-keyboard.png).
settings-help-topics-language-pic3 = キーボードとマウスのページ
# Help topic "Language and keyboard": caption and spoken name of picture 4 (settings-region.png).
settings-help-topics-language-pic4 = 地域と言語のページ

## Topic: Battery, sleep and shutting down (src/settings/help_topics.rs)

# Title of the Help topic "Battery, sleep and shutting down".
settings-help-topics-power-title = バッテリー、スリープ、シャットダウン
# Words people might search for the Help topic "Battery, sleep and shutting down" by, in lower case, separated by spaces.
settings-help-topics-power-keywords = バッテリー 電源 充電 スリープ サスペンド シャットダウン 電源を切る 再起動 ログアウト 画面
# Help topic "Battery, sleep and shutting down": paragraph 1.
settings-help-topics-power-p1 = 設定の電源では、バッテリーと電源モードが見られます。画面が暗くなり、ロックされ、消えるまでの時間と、コンピューターがスリープするまでの時間も決められます。ロックをオンにすると (下を見てください)、Super と L を同時に押して、すぐ画面をロックできます。
# Help topic "Battery, sleep and shutting down": paragraph 2.
settings-help-topics-power-p2 = ロックは、初めはオフです。同じページのロックの下で、画面をロックをオンにします。次に、コンピューターを使わずにおいてからロックされるまでの時間を選びます。スリープのときだけ、または 1 分から 2 時間です。ロックはパスワードで解除します。よければ、4 桁の PIN も追加できます。PIN のほうが早く解除できます。ロックしてから PIN が使える時間は、15 分から 2 時間の間で選びます。その時間が過ぎるか、5 回間違えると、パスワードを聞かれます。使わずにおいたために画面がロックされたときは、戻るまで、時刻だけがゆらゆら動いて表示されます。戻ってロックを解除するまで、画面にほかのものは出ません。ログインには、いつもパスワードが必要です。
# Help topic "Battery, sleep and shutting down": caption and spoken name of picture 3 (settings-power.png).
settings-help-topics-power-pic3 = 電源のページ
# Help topic "Battery, sleep and shutting down": paragraph 4.
settings-help-topics-power-p4 = バッテリーが 10% まで減ると、画面の真ん中に警告が 1 回、音とともに出ます。その値を変えるには、「バッテリー残量がここまで減ったら警告」で別の値か、なしを選びます。充電器をつないだり抜いたりすると、数秒間カードが音とともに出るので、ケーブルが外れても気づけます。
# Help topic "Battery, sleep and shutting down": paragraph 5.
settings-help-topics-power-p5 = ログアウト、再起動、シャットダウンをするには、リングを開き、真ん中の電源ボタンをクリックします。選択肢がリングの真ん中に出ます。ロックとスクリーンセーバーはすぐ実行されます。スリープ、休止状態、ログアウト、再起動、シャットダウンは 2 回確かめます。1 回目のクリックでボタンがハイライトの色で塗られてチェックマークが出るので、うっかりクリックしても何も終わりません。バツを押すと選択肢が閉じます。

## Topic: Apps that start by themselves (src/settings/help_topics.rs)

# Title of the Help topic "Apps that start by themselves".
settings-help-topics-startup-title = 自動で始まるアプリ
# Words people might search for the Help topic "Apps that start by themselves" by, in lower case, separated by spaces.
settings-help-topics-startup-keywords = 自動起動 スタートアップ 起動 ログイン 自動 開始 遅らせる ログインしたとき開く
# Help topic "Apps that start by themselves": paragraph 1.
settings-help-topics-startup-p1 = 設定のログイン時に起動には、ログインしたときに自動で開くアプリが並びます。
# Help topic "Apps that start by themselves": step 1 of the numbered steps at block 2.
settings-help-topics-startup-step2-1 = 追加するには、グリッドのボタンを押してこのコンピューターにあるアプリから選ぶか、ターミナルのボタンを押してコマンドを入力するか、ディスクからプログラムを選びます。
# Help topic "Apps that start by themselves": step 2 of the numbered steps at block 2.
settings-help-topics-startup-step2-2 = どれにも遅れ時間があります。すぐ、または数秒後です。先にネットワークやほかのアプリの準備が要るアプリには、遅れ時間を使います。
# Help topic "Apps that start by themselves": step 3 of the numbered steps at block 2.
settings-help-topics-startup-step2-3 = アプリをオフにすると、リストに残したまま起動だけ止められます。ごみ箱で削除します。

## Topic: Keyboard shortcuts (src/settings/help_topics.rs)

# Title of the Help topic "Keyboard shortcuts".
settings-help-topics-keys-title = キーボードショートカット
# Words people might search for the Help topic "Keyboard shortcuts" by, in lower case, separated by spaces.
settings-help-topics-keys-keywords = キー キーボード ショートカット super windows キー alt tab スクリーンショット print 音量 明るさ ロック ターミナル 閉じる 最大化 スクリーンリーダー デスクトップ 表示
# Help topic "Keyboard shortcuts": paragraph 1.
settings-help-topics-keys-p1 = すべてマウスでできます。これらのキーは、少し早くするためのものです。Super は、ほとんどのキーボードで Windows のロゴがあるキーです。
# Help topic "Keyboard shortcuts": step 1 of the numbered steps at block 2.
settings-help-topics-keys-step2-1 = Super だけ、または Super と Space: リングを開く、または閉じる。
# Help topic "Keyboard shortcuts": step 2 of the numbered steps at block 2.
settings-help-topics-keys-step2-2 = Super と Enter、または Ctrl と Alt と T: ターミナルを開く。
# Help topic "Keyboard shortcuts": step 3 of the numbered steps at block 2.
settings-help-topics-keys-step2-3 = Super と I: 設定。
# Help topic "Keyboard shortcuts": step 4 of the numbered steps at block 2.
settings-help-topics-keys-step2-4 = Super と L: 画面をロックする。設定の電源でロックをオンにしたあと使えます。
# Help topic "Keyboard shortcuts": step 5 of the numbered steps at block 2.
settings-help-topics-keys-step2-5 = Super と Alt と S: 画面の読み上げをオンまたはオフにする。
# Help topic "Keyboard shortcuts": step 6 of the numbered steps at block 2.
settings-help-topics-keys-step2-6 = Super と Q: いちばん手前のウィンドウを閉じる。
# Help topic "Keyboard shortcuts": step 7 of the numbered steps at block 2.
settings-help-topics-keys-step2-7 = Super と上: ウィンドウを画面いっぱいにする、または元の大きさに戻す。
# Help topic "Keyboard shortcuts": step 8 of the numbered steps at block 2.
settings-help-topics-keys-step2-8 = Super と下: ウィンドウを最小化する。リングの開いているウィンドウから戻せます。
# Help topic "Keyboard shortcuts": step 9 of the numbered steps at block 2.
settings-help-topics-keys-step2-9 = Super と D: すべてのウィンドウを隠してデスクトップを見せる、また元に戻す。
# Help topic "Keyboard shortcuts": step 10 of the numbered steps at block 2.
settings-help-topics-keys-step2-10 = Super と N: キーボード操作を画面上の通知へ移す。Tab でボタンの間を移り、Enter か Space で押し、Escape で戻ります。
# Help topic "Keyboard shortcuts": step 11 of the numbered steps at block 2.
settings-help-topics-keys-step2-11 = Alt と Tab: 開いているウィンドウを順に手前に出す。
# Help topic "Keyboard shortcuts": step 12 of the numbered steps at block 2.
settings-help-topics-keys-step2-12 = Print、または Super と Shift と S: 画面、または画面の一部を撮る。Shift と Print: 画面全体を一度に撮る。
# Help topic "Keyboard shortcuts": step 13 of the numbered steps at block 2.
settings-help-topics-keys-step2-13 = 音量、ミュート、明るさのキーは、表示どおりに働きます。

## Topic: Drives and USB sticks (src/settings/help_topics.rs)

# Title of the Help topic "Drives and USB sticks".
settings-help-topics-disks-title = ドライブとUSBメモリー
# Words people might search for the Help topic "Drives and USB sticks" by, in lower case, separated by spaces.
settings-help-topics-disks-keywords = ドライブ ディスク usb メモリー メモリーカード フォーマット 消去 取り出し 電源 空き容量 ストレージ パーティション サイズ変更 縮小 拡大 状態 smart 点検 修復 マウント 起動時 fstab 名前 フォルダー mnt drive disk stick format wipe erase eject partition resize mount startup
# Help topic "Drives and USB sticks": paragraph 1.
settings-help-topics-disks-p1 = ドライブは、すべてのアプリにある独立したウィンドウです。設定の「このコンピューターについて」からも開けます。上部の1行で空き容量が分かり、使用中のパーティションごとにリングが付きます。その下に、このコンピューターの中や接続中のすべてのドライブと、各パーティションの使用量が出ます。ドライブが対応していれば、状態と温度も出ます。
# Help topic "Drives and USB sticks": step 1 of the numbered steps at block 2.
settings-help-topics-disks-step2-1 = USBメモリーを差し込むと、一覧に出ます。
# Help topic "Drives and USB sticks": step 2 of the numbered steps at block 2.
settings-help-topics-disks-step2-2 = フォルダーを押すと中のファイルを見られます。再生ボタンでマウント(使える状態にする)、停止ボタンでマウント解除します。
# Help topic "Drives and USB sticks": step 3 of the numbered steps at block 2.
settings-help-topics-disks-step2-3 = 抜く前に取り出しを押してください。データが失われません。電源ボタンは、電源を切れるUSBドライブの電源を切り、そのまま抜ける状態にします。
# Help topic "Drives and USB sticks": paragraph 3.
settings-help-topics-disks-p3 = パーティションの横の鉛筆で、変更できる項目が開きます。
# Help topic "Drives and USB sticks": step 1 of the numbered steps at block 4.
settings-help-topics-disks-step4-1 = 名前: ここと他のコンピューターで表示される名前です。
# Help topic "Drives and USB sticks": step 2 of the numbered steps at block 4.
settings-help-topics-disks-step4-2 = フォルダー: ファイルが現れる場所です。番号の名前のフォルダーの代わりに、/mnt/backup などにできます。空にすると、コンピューターが選びます。
# Help topic "Drives and USB sticks": step 3 of the numbered steps at block 4.
settings-help-topics-disks-step4-3 = 起動時にマウント: コンピューターの起動時にパーティションが使える状態になります。ドライブがなくても、コンピューターは起動します。
# Help topic "Drives and USB sticks": step 4 of the numbered steps at block 4.
settings-help-topics-disks-step4-4 = サイズ: 小さくできます。直後の空き領域まで大きくもできます。
# Help topic "Drives and USB sticks": step 5 of the numbered steps at block 4.
settings-help-topics-disks-step4-5 = エラー: エラーを調べて直します。実行中はドライブを使えません。
# Help topic "Drives and USB sticks": step 6 of the numbered steps at block 4.
settings-help-topics-disks-step4-6 = フォーマットと、パーティションの削除です。
# Help topic "Drives and USB sticks": paragraph 5.
settings-help-topics-disks-p5 = ドライブの横の3つの点は、ドライブ全体を消去し、1つのパーティションとして作り直します。空き領域にも鉛筆があり、そこに新しいパーティションを作れます。WindowsやMacと共有するドライブにはexFAT、Linuxだけで使うドライブにはext4、小さなメモリーや古い機器にはFATを選んでください。
# Help topic "Drives and USB sticks": tip 6, shown in a box of its own.
settings-help-topics-disks-tip6 = フォーマット、サイズ変更、削除は、どれも2回のクリックとパスワードが必要です。確認できるように、ドライブの名前とサイズが表示されます。コンピューターが動いているドライブには、Trindleはこれらを一切出しません。消去は元に戻せません。残したいものは先にコピーしてください。

## Topic: Which app opens what (src/settings/help_topics.rs)

# Title of the Help topic "Which app opens what".
settings-help-topics-defaults-title = どのアプリで開くか
# Words people might search for the Help topic "Which app opens what" by, in lower case, separated by spaces.
settings-help-topics-defaults-keywords = 既定のアプリ ブラウザー メール 開く pdf 写真 音楽 動画 リンク default browser email open with pictures music video links
# Help topic "Which app opens what": paragraph 1.
settings-help-topics-defaults-p1 = 設定の「既定のアプリ」で、ウェブのリンク、メール、写真、音楽、動画、PDFファイル、フォルダーを開くアプリを選べます。選べるのは、インストール済みで、その種類のファイルを開けるアプリだけです。
# Help topic "Which app opens what": tip 2, shown in a box of its own.
settings-help-topics-defaults-tip2 = Trindleにはウェブブラウザーが付いていません。先にアプリで入手すると、ここで選べます。

## Topic: Keeping the computer up to date (src/settings/help_topics.rs)

# Title of the Help topic "Keeping the computer up to date".
settings-help-topics-updates-title = コンピューターを最新に保つ
# Words people might search for the Help topic "Keeping the computer up to date" by, in lower case, separated by spaces.
settings-help-topics-updates-keywords = 更新 アップデート アップグレード セキュリティ 新バージョン debian trindle インストール ファームウェア bios fwupd update updates upgrade security firmware
# Help topic "Keeping the computer up to date": paragraph 1.
settings-help-topics-updates-p1 = 更新はすべて1か所で行います。アプリの「更新」ページです。
# Help topic "Keeping the computer up to date": step 1 of the numbered steps at block 2.
settings-help-topics-updates-step2-1 = アプリを開いて「更新」を選びます。アプリ、Debian、Trindle本体が一覧に出ます。更新待ちの数は、リングのアプリボタンに表示されます。
# Help topic "Keeping the computer up to date": step 2 of the numbered steps at block 2.
settings-help-topics-updates-step2-2 = 各カードの更新ボタンを押します。DebianとTrindleの更新にはパスワードが必要です。アプリは不要です。
# Help topic "Keeping the computer up to date": step 3 of the numbered steps at block 2.
settings-help-topics-updates-step2-3 = Trindleを更新したら、新しいバージョンで動くように、一度ログアウトしてからログインし直してください。
# Help topic "Keeping the computer up to date": paragraph 3.
settings-help-topics-updates-p3 = コンピューターのメーカーや、ドックやタッチパッドなどの部品のメーカーが、新しいファームウェアを公開することがあります。ファームウェアは、ハードウェア自体に組み込まれたソフトウェアです。お使いのコンピューター向けのものがあると、同じページに「ファームウェア」カードが現れ、部品ごとに新しいバージョンが表示されます。更新ボタンを押すとパスワードを求められます。完了するにはコンピューターの再起動が必要な場合があるので、作業を保存し、ノートパソコンでは充電器をつないだままにしてください。新しいものがないときは、カードは表示されません。
# Help topic "Keeping the computer up to date": tip 4, shown in a box of its own.
settings-help-topics-updates-tip4 = 勝手にインストールされるものはありません。更新は、リングまたはアプリで、あなたがインストールを選んだときだけ入ります。設定、アプリ、ファイルは、どの更新でもそのまま残ります。

## Topic: About Trindle (src/settings/help_topics.rs)

# Title of the Help topic "About Trindle".
settings-help-topics-about-title = Trindleについて
# Words people might search for the Help topic "About Trindle" by, in lower case, separated by spaces.
settings-help-topics-about-keywords = について バージョン debian コンポジター wayland smithay rust ライセンス gpl 無保証 リスク 作者 about version what is compositor licence license warranty risk who made
# Help topic "About Trindle": paragraph 1.
settings-help-topics-about-p1 = Trindleは、Debian 13向けの小さなデスクトップです。独自の部分はすべてRustで書かれています。画面にすべてのウィンドウを表示するコンポジターと、それ以外を担う1つのプログラムです。後者には、リング、設定、アプリ、ログイン画面、ロック画面が含まれます。
# Help topic "About Trindle": paragraph 2.
settings-help-topics-about-p2 = GTKのプログラムは、自分でタイトルバーを描きます。それ以外のターミナル、Qtのプログラム、古いX11のプログラムなどは、コンポジターがタイトルバーを描きます。右側にはGTKのウィンドウと同じ丸いボタンが付きます。ウィンドウを隠す線、画面いっぱいにする四角、閉じるバツです。
# Help topic "About Trindle": paragraph 3.
settings-help-topics-about-p3 = Trindleはローリングリリースなので、年に2回の大きな新バージョンといった予定はありません。改良は、できたものから、それぞれ番号を付けて届きます。設定の「このコンピューターについて」に、その番号と、プロセッサー、メモリー、グラフィックスなどが表示されます。ここでコンピューターの名前も変えられます。このページには、電源ボタンを押してからログイン画面までの直近10回の起動時間も残り、ワンクリックでダウンロードフォルダーに一覧ごと保存できます。コンピューターがいま何をしているかは、独立したウィンドウの「アクティビティ」で分かります。
# Help topic "About Trindle": paragraph 4.
settings-help-topics-about-p4 = WaylandではなくX11向けの古いプログラムも動きます。必要なXサーバーのXwaylandは、最初のプログラムを開いた瞬間に起動し、それまではメモリーを使いません。
# Help topic "About Trindle": paragraph 5.
settings-help-topics-about-p5 = GNU General Public License バージョン3のフリーソフトウェアなので、使う、変更する、共有することができます。いかなる保証もなく、ご自身の責任でお使いください。大切なものは必ずバックアップを取ってください。
# Help topic "About Trindle": paragraph 6.
settings-help-topics-about-p6 = ウェブサイトはtrindle.org、コードはtrindle.org/codeにあります。
# Help topic "About Trindle": paragraph 7.
settings-help-topics-about-p7 = 設定の「謝辞」に、Trindleの土台になっているプロジェクトが並んでいます。それぞれホームページへのリンクがあり、寄付を受け付けているものにはハートが付いています。

## Topic: What is running, and why it is busy (src/settings/help_topics.rs)

# Title of the Help topic "What is running, and why it is busy".
settings-help-topics-activity-title = 何が動いていて、なぜ忙しいか
# Words people might search for the Help topic "What is running, and why it is busy" by, in lower case, separated by spaces.
settings-help-topics-activity-keywords = アクティビティ タスクマネージャー システムモニター 実行中 プログラム プロセス プロセッサー cpu メモリー ram 忙しい 遅い ファン 熱い スレッド 停止 閉じる 固まった 応答なし バブル スワップ 温度 activity task manager running busy slow hot frozen swap temperature
# Help topic "What is running, and why it is busy": paragraph 1.
settings-help-topics-activity-p1 = アクティビティは、コンピューターがいま何をしているかを見せる独立したウィンドウです。すべてのアプリの「システム」の中か、設定の「このコンピューターについて」から開きます。
# Help topic "What is running, and why it is busy": paragraph 2.
settings-help-topics-activity-p2 = 上部の1行に、状態が出ます。たとえば「負荷の高いプログラムはありません」や、コンピューターを忙しくしているプログラムの名前です。その下はコンピューターの絵です。大きな円がメモリーで、各プログラムはその中のバブルです。使っているメモリーが多いほど大きくなります。作業中のプログラムは光って脈打ち、負荷が高いほど速く明るくなります。少ない作業は青、高い負荷はオレンジ、全力は赤です。縁には、プロセッサーのスレッドごとに棒があり、忙しくなると伸びて暖かい色になります。
# Help topic "What is running, and why it is busy": paragraph 3.
settings-help-topics-activity-p3 = Trindleはプログラムのすべての部分を合計します。そのため、Firefoxはタブがいくつあっても1つのバブルです。残りは、静かな灰色のバブル2つで示されます。デスクトップとして動くためにTrindleが実行するものすべての「Trindleデスクトップ」と、「Linuxとその他のプログラム」です。設定やアプリなどTrindleのアプリは、デスクトップの一部ではなく、プログラムとして数えます。
# Help topic "What is running, and why it is busy": paragraph 4.
settings-help-topics-activity-p4 = 横には4つのリングがあり、メモリー、プロセッサー、ディスク、スワップファイルを示します。スワップファイルがないコンピューターでは、最後のリングはプロセッサーの温度を示します。メモリーのリングでは、Trindle自身の分が強調色、あなたのプログラムとLinuxが青です。
# Help topic "What is running, and why it is busy": step 1 of the numbered steps at block 5.
settings-help-topics-activity-step5-1 = プログラムが応答しなくなったら、そのバブルをクリックします。横にカードが開き、何をしているかが分かりやすい数字で出ます。
# Help topic "What is running, and why it is busy": step 2 of the numbered steps at block 5.
settings-help-topics-activity-step5-2 = カードの停止ボタンを押します。強調色で塗られ、チェックマークが出ます。
# Help topic "What is running, and why it is busy": step 3 of the numbered steps at block 5.
settings-help-topics-activity-step5-3 = 3秒以内にもう一度押します。Trindleがプログラムに閉じるよう求めます。保存していない内容は失われることがあります。
# Help topic "What is running, and why it is busy": tip 6, shown in a box of its own.
settings-help-topics-activity-tip6 = Trindle自身のプログラムとシステムのプログラムには、停止ボタンがありません。

## Topic: Trying Trindle in a virtual machine (src/settings/help_topics.rs)

# Title of the Help topic "Trying Trindle in a virtual machine".
settings-help-topics-virtual-machine-title = 仮想マシンでTrindleを試す
# Words people might search for the Help topic "Trying Trindle in a virtual machine" by, in lower case, separated by spaces.
settings-help-topics-virtual-machine-keywords = 仮想マシン vm virtualbox boxes qemu kvm vmware hyper-v parallels utm 遅い 3d アクセラレーション グラフィックス virtual machine slow acceleration graphics
# Help topic "Trying Trindle in a virtual machine": paragraph 1.
settings-help-topics-virtual-machine-p1 = Trindleは仮想マシンでも動き、仮想マシンで起動したときはそう知らせます。USBメモリーのウェルカムウィンドウと、インストール後の通知で1回見られます。実際のコンピューターでは、何も出ません。
# Help topic "Trying Trindle in a virtual machine": paragraph 2.
settings-help-topics-virtual-machine-p2 = 3Dグラフィックスなしで作った仮想マシンには、Trindleが使えるグラフィックチップがないため、すべてをプロセッサーが描きます。ウィンドウ、動画、ウェブページは、実際のコンピューターより遅く感じられ、Trindleは助けになるようアニメーションを省きます。仮想マシンのディスプレイ設定で3Dアクセラレーションをオンにすると、大きく改善します。VirtualBoxでは「3Dアクセラレーションを有効化」です。GNOME Boxesとvirt-managerでは「3Dアクセラレーション」または「Virtio with OpenGL」です。
# Help topic "Trying Trindle in a virtual machine": tip 3, shown in a box of its own.
settings-help-topics-virtual-machine-tip3 = Trindleの速さは、実際のコンピューターで判断してください。動かす場所はそちらだからです。

## Topic: If something goes wrong (src/settings/help_topics.rs)

# Title of the Help topic "If something goes wrong".
settings-help-topics-trouble-title = うまくいかないとき
# Words people might search for the Help topic "If something goes wrong" by, in lower case, separated by spaces.
settings-help-topics-trouble-keywords = 問題 ヘルプ 動かない 固まった クラッシュ ログイン エラー 壊れた 復旧 ターミナル problem help stuck frozen crash login error broken recover terminal
# Help topic "If something goes wrong": paragraph 1.
settings-help-topics-trouble-p1 = アプリが応答しなくなったら、リングでそのアプリを右クリックして「ウィンドウを閉じる」を選びます。それでもだめなとき、または遅い理由が分からないときは、アクティビティを開いてください。忙しいプログラムが分かり、停止ボタンでプログラムを閉じられます。
# Help topic "If something goes wrong": paragraph 2.
settings-help-topics-trouble-p2 = ログイン画面が出ないとき、またはログインしてもまたログイン画面に戻るときは、Ctrl、Alt、F2を同時に押してください。通常の文字だけのログインが出るので、ログインして問題を直せます。Ctrl、Alt、F7で、グラフィックの画面に戻ります。
# Help topic "If something goes wrong": tip 3, shown in a box of its own.
settings-help-topics-trouble-tip3 = Trindleは、各セッションの記録を、ホームフォルダーの .local/state/trindle/session.log に残します。何が悪かったのかを調べる助けになります。

## Topic: A chip with no driver (src/settings/help_topics.rs)

# Title of the Help topic "A chip with no driver".
settings-help-topics-hardware-no-driver-title = ドライバーやファームウェアがないチップ
# Words people might search for the Help topic "A chip with no driver" by, in lower case, separated by spaces.
settings-help-topics-hardware-no-driver-keywords = ドライバー ファームウェア wifi wi-fi bluetooth サウンド グラフィックス チップ ない 動かない mediatek mt7902 usb メモリー driver firmware sound graphics chip missing no driver not working usb stick
# Help topic "A chip with no driver": paragraph 1.
settings-help-topics-hardware-no-driver-p1 = チップが動くには2つ必要です。1つは、Linuxの中心であるカーネルの中のドライバーです。多くのチップには、起動時にチップが読み込む小さなプログラムであるファームウェアファイルも必要です。設定を開くと、Trindleはコンピューターのチップを調べ、どちらもないチップがあればここで知らせます。
# Help topic "A chip with no driver": paragraph 2.
settings-help-topics-hardware-no-driver-p2 = とても新しいチップには、Debianのカーネルにドライバーがないことがあります。MediaTek MT7902がその1つです。このWi-Fiドライバーが入るのはLinux 7.1からで、BluetoothファームウェアはDebianのfirmware-mediatek 20260916に入っています。TrindleはDebianの安定版カーネルだけを使うので、そのカーネルにドライバーが入るまで、このチップではWi-Fiが使えません。
# Title of the Help topic "A Bluetooth device will not connect".
settings-help-topics-bluetooth-connect-title = Bluetooth機器につながらない
# Words people might search for the Help topic "A Bluetooth device will not connect" by, in lower case, separated by spaces.
settings-help-topics-bluetooth-connect-keywords = bluetooth 接続 つながらない 削除 ペアリング もう一度 ヘッドホン 動かない 以前 ペア 古い キー 認証 connect connecting forget pair again headphones not working paired before old pairing key authentication
# Help topic "A Bluetooth device will not connect": paragraph 1.
settings-help-topics-bluetooth-connect-p1 = 機器の電源を入れ、近くに置いてください。以前ペアリングしたのにつながらない、またはつながってもすぐ切れるときは、このコンピューターに保存されたペアリングが、機器側のものと合っていない可能性があります。機器をリセットしたり、別のコンピューターとペアリングしたりした後に起こることがあります。
# Help topic "A Bluetooth device will not connect": paragraph 2.
settings-help-topics-bluetooth-connect-p2 = 機器の横の「削除して再ペアリング」を押します。再ペアリングしないで削除するだけなら、ごみ箱を押します。その後、説明書のとおりに機器をペアリングモードにして、近くの機器に出たら横の「ペアリング」を押してください。
# Help topic "A Bluetooth device will not connect": paragraph 3.
settings-help-topics-bluetooth-connect-p3 = それでもだめなときは、機器の下の1行に理由が出ます。Bluetoothがオフ、機器が応答しなかった、数字が拒否された、機器の音の準備ができていない、などです。30秒たっても機器が応答しないときは、機器の電源を切って入れ直し、もう一度試してください。
# Title of the Help topic "Headphones sound basic".
settings-help-topics-bluetooth-codec-title = ヘッドホンの音が悪い
# Words people might search for the Help topic "Headphones sound basic" by, in lower case, separated by spaces.
settings-help-topics-bluetooth-codec-keywords = bluetooth 音質 コーデック ヘッドホン 基本 sbc オフ オン sound quality codec headphones basic switched off on
# Help topic "Headphones sound basic": paragraph 1.
settings-help-topics-bluetooth-codec-p1 = Bluetoothをオフにしてからオンにすると、サウンドシステムがヘッドホンへのより良い音の送り方を見逃し、基本的な方式に戻ることがあります。Bluetoothが戻ると少ししてから、Trindleがサウンドシステムの Bluetooth の部分を自動で再起動します。
# Help topic "Headphones sound basic": paragraph 2.
settings-help-topics-bluetooth-codec-p2 = 再起動でうまくいかなかったときは、一度ログアウトしてログインし直すと、良い音質に戻ります。
# Help topic "A chip with no driver": paragraph 3.
settings-help-topics-hardware-no-driver-p3 = それまでは、USBのWi-Fiスティックなら、どのコンピューターでも設定なしで使えます。差し込むと「ネットワーク」に出ます。このメッセージで、コンピューターの設定が変わることはありません。

## Topic: Playing games (src/settings/help_topics.rs)

# Title of the Help topic "Playing games".
settings-help-topics-games-title = ゲームを遊ぶ
# Words people might search for the Help topic "Playing games" by, in lower case, separated by spaces.
settings-help-topics-games-keywords = ゲーム steam proton windows ランチャー ubisoft コントローラー ゲームパッド チートガード vanguard 黒い画面 起動しない クラッシュ alt-tab hdr games game launcher controller gamepad anti-cheat black screen will not start crash
# Help topic "Playing games": paragraph 1.
settings-help-topics-games-p1 = Steamは使えます。アプリからインストールしてください。Flatpakとして、他のアプリとは分けて動き、Protonで Windows のゲームを遊べます。Ubisoft Connectは確認済みです。他のランチャーも、たいていSteamの中で動きますが、それぞれの更新で動かなくなることがあります。Linux向けのゲーム、DirectX 12のゲーム、古い32ビットのゲームは、Trindleではまだ確認していません。
# Help topic "Playing games": paragraph 2.
settings-help-topics-games-p2 = Trindleは、大きなゲームに必要なメモリーの上限とファイルの上限を引き上げます。マウスをつかんだゲームは、AltとTabを押すまで離しません。全画面のゲームは、切り替えた先のウィンドウの後ろで全画面のままです。全画面のゲームが前面にある間、画面は消えません。
# Help topic "Playing games": paragraph 3, before the numbered steps.
settings-help-topics-games-p3 = ゲームが起動しないときは、次を順に試してください。
# Help topic "Playing games": step 1 of the numbered steps at block 4.
settings-help-topics-games-step4-1 = Steamを完全に閉じて、開き直します。Steamが何日も動き続けていると、ゲームが起動しないことがあります。
# Help topic "Playing games": step 2 of the numbered steps at block 4.
settings-help-topics-games-step4-2 = 待ちます。ゲームを初めて起動するときはシェーダーを作るため、画面が数分間、黒いままのことがあります。
# Help topic "Playing games": step 3 of the numbered steps at block 4.
settings-help-topics-games-step4-3 = ゲームの「プロパティ」の「互換性」で、Protonのバージョンを確認し、別のものを試します。
# Help topic "Playing games": step 4 of the numbered steps at block 4.
settings-help-topics-games-step4-4 = Ubisoft Connectなど、そのゲーム専用のランチャーにログインします。ランチャーのウィンドウが全画面のゲームの後ろに隠れることがあります。AltとTabを押して探してください。
# Help topic "Playing games": paragraph 5.
settings-help-topics-games-p5 = Vanguardのように、カーネルの中で動くチートガードは、Linuxでは一切動きません。Easy Anti-CheatとBattlEyeは、発売元がLinuxを許可したゲームでだけ動きます。
# Help topic "Playing games": paragraph 6.
settings-help-topics-games-p6 = キーボードとマウスは使えます。PlayStation、Switch、多くのXboxのコントローラー、およびSteam Inputは、まだゲームで動かないことがあります。Trindleにはゲームコントローラー用の小さなサービスがありますが、それを使う部分は未完成です。
# Help topic "Playing games": tip 7.
settings-help-topics-games-tip7 = HDRと可変リフレッシュレートは、まだ対応していません。
