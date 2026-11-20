# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Keyboard and mouse": layouts, Caps Lock, changed keys, programs' keyboards, game controllers, mouse and touchpad. The names of keys that a person types (Escape, Ctrl, Caps Lock) are read by the program and are not here.

## Page (src/settings/panels/keyboard.rs)

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = キーボードとマウス
# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = キーボード マウス 配列 言語 入力 キャップスロック 画面キーボード ボタン リング タッチパッド ポインタ 速度 タップ クリック スクロール ホイール 遅く 速く ナチュラル 左利き プラグイン ショートカット キー 割り当て 変更 入れ替え Ctrl Escape Super Alt Mac サイドボタン プログラム keyd layout caps lock keyboard mouse touchpad
# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = ここでの変更はすぐに反映されます。Alt と Shift を同時に押すと、2 つ目の配列に切り替えられます。

## Top sentence

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = { $layout } で入力しています。
# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt と Shift で { $layout } に切り替わります。
# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock は何もしません。
# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock は { $key } として動きます。
# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = もう 1 つの Ctrl
# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape
# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace
# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = このコンピューターには、キーボード配列の一覧がありません。
# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = キーボード
# Heading of the line in that card.
settings-keyboard-no-list-heading = キーボード配列の一覧を使えません
# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = 入れるには、次を実行してください: sudo apt install xkb-data
# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
       *[other] { $n } 個のキーとボタンを変更しています。
    }

## Layout

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = 配列
# Heading of the layout line, and the name of its list.
settings-keyboard-layout = 配列
# Heading of the variant line, and the name of its list.
settings-keyboard-variant = バリアント
# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = 標準
# First choice in the list of second layouts: no second layout.
settings-keyboard-none = なし
# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = 切り替え先
# Words behind the i of the layout card.
settings-keyboard-layout-about = 各キーで入力される文字を決めます。配列と、そのバリアントを選べます。Alt と Shift を同時に押すと 2 つ目の配列に切り替わり、もう一度押すと戻ります。変更はすぐに反映されます。
# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = 使用中

## Other keys

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock
# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = もう 1 つの Ctrl
# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape
# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace
# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = 何もしない
# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Caps Lock の動作
# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = リングのキーボードボタン
# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = その他のキー
# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock は、2 つ目の Ctrl や Escape など、別のキーとして使えます。キーボードボタンはリングの中央にあり、画面上のキーボードを開きます。マウスやタッチスクリーンで入力できます。次にリングを開いたときに表示されます。

## Keys and buttons

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = プログラムはこれを変更できません。
# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = 変更するキーかマウスボタンを押してください。
# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = 何も押されなかったので、追加していません。
# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = 代わりに使うキーを入力してください。たとえば Escape や Ctrl+C です。何もしないなら Nothing と入力します。
# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle はどのキーか判別できませんでした。代わりに一覧からキーを選んでください。
# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = すべてのキーボードとマウス
# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = このキーは下で自分で変更しているので、その変更が使われます。
# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = { $key } を変更する対象
# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = { $key } の変更後
# Name of the bin button that takes a change away.
settings-keyboard-take-away = この変更を取り消す
# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = { $n } 行目は無視しました: { $why }
# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = 変更したキーを読み取れませんでした
# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Trindle が変更を読み取れるまで、キーは通常どおり動きます。{ $why }
# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = 保存していません。変更したキーのファイルを読み取れなかったので、そのままにしています。
# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = 変更するキーかボタンを選ぶ
# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = 変更するキーかボタンを押す
# Heading of the line for changing a key or button.
settings-keyboard-keys-add = キーかボタンを変更
# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = { $key } はすでに変更しています。
# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = キーとボタン
# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = どのキーやマウスボタンにも、別の働きを持たせられます。別のキーやボタンにしたり、Ctrl+C のように同時に押すキーにしたり、H、I のように順に押すキーにしたりできます。何もしないようにもできます。軽く押したときと長押ししたときで別の働きにもできます。たとえば、軽く押すと Escape、長押しすると Ctrl です。これはログイン画面からどこでも使えます。Ctrl、Alt と F キーを同時に押すと、いつでもテキストコンソールが開きます。そこで trindle-settings --keys default と入力すると、すべてのキーが元に戻ります。

## Ready-made swaps

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Ctrl と Caps Lock を入れ替える
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock: 軽く押すと Escape、長押しで Ctrl
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Escape と Caps Lock を入れ替える
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Alt と Super を入れ替える (Mac のキーボードと同じ)
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = マウスのサイドボタンでコピーと貼り付け

## Programs' keyboards and mice

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
settings-keyboard-made-switch = { $name } を信頼する
# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = 今のところありません
# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = プログラムが作ったキーボードとマウス
# Words behind the i of that card.
settings-keyboard-made-about = キー割り当てツールやゲームランチャーは、独自のキーボードやマウスを作ることがあります。そのキーやクリックで入力や操作はできますが、シャットダウンや許可のような保護されたボタンは押せず、あなた宛ての質問にも答えられません。あなた自身として扱うには、ここでオンにします。

## Game controllers

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = コントローラー { $vendor }:{ $product }
# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $title } の { $name }
# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = { $title } の動作
# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = A と B を入れ替える
# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = X と Y を入れ替える
# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = 右スティックの上下を逆にする

## Plugin shortcuts

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = プラグインのショートカット
# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } は使っていません: { $why }
# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = { $label } のキー
# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = 保存しました

## The screenshot and the shortcut list

# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = スクリーンショット、報告、ショートカット
# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = スクリーンショットを撮る
# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = ショートカット一覧を表示
# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = 今すぐスクリーンショットを撮る
# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = 問題を報告
# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = 今すぐ問題を報告
# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = 今すぐ一覧を表示

## Restarting the desktop

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = デスクトップを再起動
# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = リングと補助部分を再起動
# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = すべて再起動
# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = { $label } のキー
# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = 今すぐ再起動します。開いているウィンドウはそのままです。
# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = 確認してから、すべて再起動
# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = 再起動しました
# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } は、すでに別のショートカットが使っています。
# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = 保存していません。{ $why }

## Mouse and touchpad

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = タッチパッドにも適用されます。
# Title of the mouse card.
settings-keyboard-mouse-title = マウス
# Words behind the i of the mouse card.
settings-keyboard-mouse-about = ポインタの速さは、マウスを動かしたときにポインタが動く距離です。タッチパッドにも適用されます。スクロールの速さは、ホイールを 1 回転させたときにページが動く距離です。「遅く」「標準」「速く」から選ぶか、スライダーで自由に決められます。ナチュラルスクロールでは、スマートフォンのように、ホイールを回した向きにページが動きます。左利き用にすると、左右のボタンが入れ替わります。
# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = ポインタの速さ
# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = スクロールの速さ
# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = ナチュラルスクロール
# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = 左利き用
# Heading and switch name for the clicking helper.
settings-keyboard-clicker = クリック補助
# Small words under the clicking helper line.
settings-keyboard-clicker-about = 一定時間の静止やスイッチによるクリックは、あなた自身のクリックとして扱います。
# Title of the touchpad card.
settings-keyboard-touchpad-title = タッチパッド
# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = スクロールの速さは、2 本指でスクロールしたときにページが動く距離です。タップでクリックをオンにすると、軽くたたくだけでクリックになります。ナチュラルスクロールでは、スマートフォンのように、指を動かした向きにページが動きます。入力中はオフにすると、入力中に手のひらでポインタが動くのを防ぎます。
# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = タップでクリック
# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = 入力中はオフ
# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = 遅く
# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = 標準
# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = 速く

## USB ports

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB ポート
# Words behind the i of that card.
settings-keyboard-usb-about = USB ポートが起動に失敗すると、Trindle は一度オフにしてからオンにして、やり直します。同じポートで 2 回以上は行いません。
# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle は変更を元に戻すこともできませんでした: { $undo }
