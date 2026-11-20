# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Game controllers": a picture of the controller that lights as it is pressed, a list of its buttons, and what each becomes.

# The page's name in the side bar.
settings-pads-title = ゲームコントローラー

# The sentence at the top of the page when a controller is plugged in.
settings-pads-top-idle = 変更したいボタンをコントローラーで押してください。
# The sentence at the top of the page when a button has been pressed; { $name } is the button.
settings-pads-top-picked = 次に、{ $name } に使うキーを押してください。

# Heading of the line shown when no game controller is plugged in.
settings-pads-none = 接続されていません
# Name of the list that chooses which controller is shown; { $title } is the one shown now.
settings-pads-which = どのコントローラーか
# What the picture is, for the screen reader.
settings-pads-picture = { $title } の画像。ボタンを押すと、そのボタンが光ります。

# Title of the card of the controller's buttons.
settings-pads-buttons-title = ボタン
# Words behind the i of that card.
settings-pads-buttons-about = すべてのボタンとトリガー、スティックと十字キーのすべての方向です。最後に押したものが一番上に出ます。ボタンをキーに変えると、コントローラーに対応していないゲームでそのキーが入力されます。
# Name of the round button that starts changing a button; { $name } is the button.
settings-pads-change = { $name } をキーに変更
# Name of the round button that stops changing a button; { $name } is the button.
settings-pads-stop = { $name } の変更をやめる
# Name of the list of what a button becomes; { $name } is the button.
settings-pads-choose = { $name } の動作
# What the list says while a button waits for its key: the clue, and a choice that does nothing.
settings-pads-waiting = キーを押すか、ボタンを選んでください
# The next choice in that list: the Escape key.
settings-pads-escape = Escape
# Where a choice in that list comes from; { $what } is the key, mouse button or controller button.
settings-pads-on-keyboard = キーボード: { $what }
settings-pads-on-mouse = マウス: { $what }
settings-pads-on-pad = ゲームコントローラー: { $what }
# The next choice in that list: nothing at all.
settings-pads-nothing = なし
# The next choices in that list: the mouse buttons.
settings-pads-left-click = 左クリック
settings-pads-right-click = 右クリック
settings-pads-middle-click = 中クリック
# Name of a way of the d-pad; { $way } is up, down, left or right.
settings-pads-dpad = 十字キー { $way }
# Name of a way of a stick; { $stick } is Left or Right and { $way } is up, down, left or right.
settings-pads-stick-way = { $stick }スティック { $way }
settings-pads-left = 左
settings-pads-right = 右
settings-pads-up = 上
settings-pads-down = 下
# The two sticks, at the start of a name such as Left stick up.
settings-pads-stick-left = 左
settings-pads-stick-right = 右

# Title of the card of the ready-made changes.
settings-pads-quick-title = よく使う変更
# Title of the card of changes that are not one button into another.
settings-pads-other-title = その他の変更
# Words behind the i of that card.
settings-pads-other-about = スティックの反転や入れ替え、デッドゾーンを設定できます。次のように入力します: Left Stick dead zone = 10%、または Right Stick Y = inverted。

# Title of the card that chooses a profile for each game.
settings-pads-profiles-title = プロファイル
# Words behind the i of that card.
settings-pads-profiles-about = プロファイルは変更のまとまりです。「標準」は何も変更しないので、独自にコントローラーに対応しているゲームは、コントローラーをそのまま受け取ります。ここでプロファイルを選ばない限り、Steam のゲームはすべて「標準」を使います。ゲームが始まると、Trindle はそのプロファイルに切り替えます。「その他のゲーム」は、Steam が起動していないすべてのゲームです。
# The one switch that turns every controller change off at once.
settings-pads-changes = コントローラーの変更
# The game whose profile is shown.
settings-pads-game = ゲーム
settings-pads-other-game = その他のゲーム
settings-pads-game-running = { $name }(実行中)
settings-pads-game-number = Steam ゲーム { $n }
# The profile the shown game uses.
settings-pads-profile = プロファイル
settings-pads-standard = 標準
settings-pads-profile-add = プロファイルを追加
settings-pads-profile-remove = { $name } を削除
settings-pads-profile-name = プロファイル名
# The name a new profile is given.
settings-pads-profile-new = プロファイル { $n }
settings-pads-profile-empty = プロファイル名を入力してください。
settings-pads-profile-standard = 「標準」という名前は使われています。
settings-pads-profile-taken = その名前は使われています。
settings-pads-profile-gone = そのプロファイルはなくなりました。
settings-pads-not-a-game = Steam アプリ番号ではありません。
# Why the buttons cannot be changed while the shown game uses Standard.
settings-pads-standard-why = 「標準」は何も変更しません。先にプロファイルを追加してください。
# The headline while the shown game uses Standard, and while all changes are off.
settings-pads-top-standard = 「標準」を使用中で、変更はありません。
settings-pads-top-off = コントローラーの変更はオフです。

# Heading of the line shown when the file of changed controller buttons could not be read.
settings-pads-unread = Trindle は変更したボタンを読み込めませんでした
# Small words under that line; { $why } is the reason.
settings-pads-unread-about = 読み込めるまでは、すべてのボタンが通常どおり動き、何も保存されません。{ $why }
# Shown beside a change that was not saved because the file of changed buttons could not be read.
settings-pads-not-saved-unread = 保存されませんでした。変更したボタンのファイルを読み込めなかったため、そのままにしてあります。
