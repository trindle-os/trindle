# machine translated (Japanese), not yet reviewed by a person.

# Settings, Games page: what helps Steam games run well that Steam itself does not set.

## Games page (src/settings/panels/games.rs, src/common/games.rs)

# The page's name in the side bar.
settings-games-title = ゲーム
# Words a search finds the page by.
settings-games-keywords = ゲーム steam proton 音割れ カクつき 音 バッファ ドライブ ライブラリ スプリットロック コントローラー ゲームパッド ジョイスティック パッド ボタン キー 変更 割り当て 入れ替え スティック トリガー 十字キー xbox playstation nintendo games crackle stutter sound buffer drive split lock gamepad controller remap
# What the page is for, in the help.
settings-games-help = ゲームを快適に動かすための設定です。Steam 自身の設定は Steam で変えてください。コントローラーのボタンを変えるには、そのボタンを押します。ボタンが一覧の先頭に移り、図の中で光ります。次に、そのボタンにしたいキーボードのキーを押します。ほかのボタンや「なし」にすることも、ボタンの一覧からできます。Trindle は変更をコントローラーのメーカーと型番ごとに覚え、ゲームは変更後のコントローラーを使います。始めた変更をやめるには Escape を押します。
# The sentence at the top of the page.
settings-games-top = 音割れ、カクつき、容量不足に悩むゲームの助けになります。
# Shown beside a switch while the person is asked for the password.
settings-games-waiting = パスワードを待っています…
# The card holding the sound and split-lock switches.
settings-games-smooth = ゲームをなめらかに
# What that card is for.
settings-games-smooth-about = 音割れやカクつきを直します。どちらも最初はオフです。
# The sound switch.
settings-games-sound = 大きめのサウンドバッファ
# Added after the words behind the i of its card.
settings-games-sound-sub = 音が割れるゲーム向けです。音が少し遅れて鳴ります。
# After the sound switch is turned on.
settings-games-sound-done-on = 大きめのバッファ オン
# After the sound switch is turned on, when the sound system could not be told at once.
settings-games-sound-next-login = 次のログインからオン
# After the sound switch is turned off.
settings-games-sound-done-off = 大きめのバッファ オフ
# The split-lock switch.
settings-games-split = 古いゲームのカクつき対策
# Added after the words behind the i of its card.
settings-games-split-sub = 古いゲームのカクつきが減りますが、1つのプログラムがほかを遅くすることがあります。
# After the split-lock switch is turned on.
settings-games-split-done-on = カクつき対策 オン
# After the split-lock switch is turned off.
settings-games-split-done-off = カクつき対策 オフ
# The games drive card.
settings-games-drive-title = ゲーム用ドライブ
# What that card is for.
settings-games-drive-about = ゲーム用の2つ目のドライブです。起動するたびに使える状態になります。
# A drive with no name of its own.
settings-games-drive-unnamed = ドライブ
# One drive in the list: its name, size and kind of filesystem.
settings-games-drive-name = { $name }、{ $size }、{ $fs }
# The button that makes a drive the games drive.
settings-games-drive-use = ゲームに使う
# Under the drive in use for games.
settings-games-drive-in-use = { $place } のゲーム用ドライブ
# The button that stops using a drive for games.
settings-games-drive-stop = ゲームに使うのをやめる
# After a drive is made the games drive.
settings-games-drive-done = ゲーム用ドライブの準備ができました
# After a drive is no longer the games drive.
settings-games-drive-forgotten = ゲームには使いません
# When there is no second drive.
settings-games-drive-none = 2つ目のドライブがありません
# When the drives could not be listed.
settings-games-drive-unread = ドライブの一覧を出せませんでした
# When Steam could not be let reach the games drive.
settings-games-drive-steam-failed = Steam にドライブを渡せませんでした
# Added after the words behind the i of its card.
settings-games-drive-steam = Steam に追加する
# Added after the words behind the i of its card.
settings-games-drive-steam-sub = Steam で：設定、ストレージ、ドライブを追加。
# Why a Windows-formatted drive cannot be used.
settings-games-drive-windows = このドライブは Windows 用にフォーマットされていて、ゲームが正しく動きません。Linux 用にフォーマットしてください。
# Why a locked drive cannot be used.
settings-games-drive-locked = 先にドライブで解除してください。
# Why a part that holds no files cannot be used.
settings-games-drive-not-files = ファイル用ではありません
# Why an unknown kind of drive cannot be used.
settings-games-drive-unknown = ここではゲームを動かせません

## Steam games (src/settings/panels/games/steam.rs)

# The card listing the person's Steam games.
settings-games-steam-title = Steam のゲーム
# What that card is for.
settings-games-steam-about = ゲームごとの修正です。変える前に Steam を閉じてください。
# Shown, and beside every control, while Steam is open.
settings-games-steam-open = Steam が開いています
# Under that.
settings-games-steam-open-sub = これらを変えるには Steam を閉じてください。
# Beside a game's controls while the game runs.
settings-games-game-running = ゲームが動いています
# Under a game's name.
settings-games-game-sub = 「やり直し」は、設定とローカルのセーブをごみ箱に移します。
# The switch for a game: use the stronger graphics chip.
settings-games-stronger = 速いグラフィックスを使う
# After that switch is turned on.
settings-games-stronger-on = 速いグラフィックス オン
# After that switch is turned off.
settings-games-stronger-off = 速いグラフィックス オフ
# The switch for a game: use the community Proton build.
settings-games-proton-one = コミュニティ版 Proton を使う
# The switch for all games.
settings-games-proton-all = すべてのゲームでコミュニティ版 Proton
# Added after the words behind the i of its card.
settings-games-proton-all-sub = ゲームが最初に必要としたときにインストールされます。
# While the community Proton build is being installed.
settings-games-proton-installing = コミュニティ版 Proton をインストールしています…
# After a game or all games are given community Proton.
settings-games-proton-on = コミュニティ版 Proton を選びました
# After the choice is given back to Steam.
settings-games-proton-off = Steam の選択を使う
# When community Proton was not found after installing.
settings-games-proton-missing = コミュニティ版 Proton が見つかりませんでした。
# The button that gives a game a fresh start.
settings-games-fresh = やり直し
# After a fresh start.
settings-games-fresh-done = 設定をごみ箱に移しました。次にゲームを始めると、Steam が新しく作ります。
# When the game had no Windows setup to move.
settings-games-fresh-none = 移すものがありません
# When no Steam games are installed.
settings-games-none = Steam のゲームが見つかりません
# Red words when community Proton could not be installed.
settings-games-proton-not-installed = Trindle はコミュニティ版 Proton をインストールできませんでした。
# Red words when Steam's settings could not be changed for a game.
settings-games-proton-not-changed = Trindle は Steam の設定を変えられませんでした。
