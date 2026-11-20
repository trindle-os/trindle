# machine translated (Japanese), not yet reviewed by a person.

# Settings: the Permissions page (src/settings/panels/permissions.rs).

## The Permissions page (src/settings/panels/permissions.rs)

# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = 権限
# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = 権限 許可 プライバシー flatpak flathub サンドボックス インターネット ネットワーク ファイル 音 マイク カメラ bluetooth 通知 プラグイン deb x11 xwayland 古いプログラム steam wine java 画面 ターミナル コンピューター thunderbolt usb4 ドック デバイス 外 お知らせ ダウンロード ダウンロードしたファイル
# Help line at the foot of the Permissions page.
settings-permissions-help = このページでは、各アプリが使えるものを確認できます。Flathubのアプリ、専用のフォルダーに入れたプログラム、プラグインは、それぞれ別の箱の中で動き、初期状態で制限がかかっています。Flathubアプリの変更は、次に起動したときに反映されます。許可したThunderboltとUSB4のデバイスは、いつも確認なしで接続されます。もう一度確認してほしいときは、そのデバイスを忘れてください。
# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = 変更できませんでした。
# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = 保存しましたが、Trindleはプラグインを再起動できませんでした。変更は次に起動したときに反映されます。
# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = 保存しました
# Help text of the AppImages card (the information button).
settings-permissions-loose-images = AppImageはそれぞれ、専用のホームフォルダーを持つ別の箱の中で動きます。変更は次にAppImageを起動したときに反映されます。
# Help text of the Started from Folders card (the information button).
settings-permissions-loose-programs = Foldersから起動したプログラムはそれぞれ、専用のホームフォルダーを持つ別の箱の中で動きます。変更は次にそのプログラムを起動したときに反映されます。
# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title }：{ $words }
# Under the Allow older programs switch.
settings-permissions-older-why = それぞれ別々に動きます。オフにすると、開けなくなります。
# Heading of the card with the switch for older programs.
settings-permissions-older = 古いプログラム
# Label and tip of the switch that lets older programs open.
settings-permissions-allow-older = 古いプログラムを許可
# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = これらのポートは画面専用なので、デバイスがこのコンピューターのメモリに触れることはありません。確認することはありません。
# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = これらのポートはUSBと画面専用なので、デバイスがこのコンピューターのメモリに触れることはありません。確認することはありません。
# Help text on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-lets-in-all = これらのポートは、すべてのデバイスを接続します。そのため、確認することはありません。これはコンピューターのファームウェアで決まります。
# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = これらのポートは画面だけが使います。
# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = これらのポートはUSBと画面だけが使います。
# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
settings-permissions-short-firmware = ファームウェアが、すべてのデバイスを接続します。
# Under a package's name: its version number is { $v }.
settings-permissions-version = バージョン { $v }
# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = バージョン { $v }。何も求めません。
# Under a plugin's name: its version number is { $v }, and { $notes } lists what it may do that matters most, such as "it can act as you and it can type into your windows".
settings-permissions-version-notes = バージョン { $v }：{ $notes }。
# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }。{ $detail }
# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = 忘れることができませんでした。
# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = ThunderboltとUSB4
# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
settings-permissions-thunder-about = 許可したThunderboltとUSB4のデバイスは、いつも確認なしで接続されます。もう一度確認してほしいときは、そのデバイスを忘れてください。{ $about }
# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = ポート
# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = ポートを確認しています…
# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = 許可したデバイス
# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = 読み込み中…
# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = メモリの保護
# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = ThunderboltとUSB4のポートがありません
# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = そのため、確認することはありません。
# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = この方法で接続するデバイスはありません。
# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = 確認はありません
# Under Devices allowed when the ports let every device in themselves.
settings-permissions-every-one = すべてです。ポートが決めます。
# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
settings-permissions-iommu = このコンピューターは、デバイスが自分のものではないメモリに触れることも防ぐので、危険は小さくなります。デバイスを忘れても、取り外すまでは接続されたままです。
# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
settings-permissions-no-iommu = デバイスを忘れても、取り外すまでは接続されたままです。次に接続したときに、もう一度確認します。
# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
settings-permissions-asked-before = 新しいデバイスは確認します
# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = Trindleは許可したデバイスの一覧を読み込めませんでした：{ $why }
# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = 確認なしで接続を許可したデバイスは、まだありません。
# Tip of the button that forgets an allowed Thunderbolt or USB4 device.
settings-permissions-forget-device = このデバイスを忘れる
# Name given to a Thunderbolt or USB4 device that has none.
settings-permissions-a-device = Thunderboltデバイス
# Under an allowed device's name when its maker is not known.
settings-permissions-may-always = いつでも接続できます
# Under an allowed device's name: the maker is { $vendor }.
settings-permissions-made-by = 製造元：{ $vendor }。いつでも接続できます。
# Headline while the list of apps is being read.
settings-permissions-reading-apps = アプリが使えるものを確認しています…
# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
       *[other] { $n } 個のアプリが使えるものを確認しています…
    }
# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = 専用の箱の中で動くアプリやプラグインは、まだインストールされていません。
# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = 1つのアプリが専用の箱の中で動き、ここで許可したものだけを使えます。
# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
       *[other] { $n } 個のアプリとプラグインがそれぞれ専用の箱の中で動き、ここで許可したものだけを使えます。
    }
# Heading of the card for apps from Flathub.
settings-permissions-flathub-apps = Flathubのアプリ
# Heading of the card for programs installed in their own folder.
settings-permissions-own-folder = プログラム
# Heading of the card for plugins.
settings-permissions-plugins = プラグイン
# Heading of the card for AppImages.
settings-permissions-appimages = AppImage
# Heading of the card for programs started from Folders.
settings-permissions-from-folders = Foldersから起動
settings-permissions-other = その他の設定
# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = インストールされていません。
# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = まだありません。
settings-permissions-not-added = Foldersから開いたもので、アプリにはありません
settings-permissions-kind-tip = アプリの種類
# Heading of the card that says nothing is installed of any kind.
# Help text of the Flathub apps card (the information button).
settings-permissions-flathub-about = Flathubの各アプリは専用の箱の中で動き、ここで許可したものだけを使えます。変更は次にアプリを起動したときに反映されます。
# Help text of the In their own folder card (the information button).
settings-permissions-debs-about = ターミナルなどTrindleに付いてきたプログラムと、.debファイルからインストールしたプログラムです。それぞれ専用のフォルダーの中で動き、初期状態で制限がかかっています。
# Help text of the Plugins card (the information button).
settings-permissions-plugins-about = プラグインは制限がかかった状態で始まり、許可できるのはプラグインが求めたものだけです。バックグラウンドで動くプラグインは、何かを変えるとすぐに再起動します。
# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = 入力コード
settings-permissions-codes-about = 入力中にTrindleが言葉に置き換える短いコードです。それぞれ、元のプラグインが表示されます。
settings-permissions-codes-none = 有効なコードはありません
# Line on the typing codes card when the plugins folder could not be read: the list may be missing codes.
settings-permissions-codes-unread-line = プラグインのフォルダーを読み込めませんでした
# Red words under that line.
settings-permissions-codes-unread = Trindleはプラグインのフォルダーを読み込めなかったため、表示されないコードがあるかもしれません。
# Button on a plugin's row: stops that plugin's codes.
settings-permissions-codes-off = { $name } のコードをオフにする
settings-permissions-codes-count =
    { $n ->
       *[other] { $n } 個のコード、{ $id }
    }
# A code's second line: what it types and who owns it.
settings-permissions-code-types = { $name } 用に「{ $words }」と入力します
settings-permissions-codes-more =
    { $n ->
       *[other] ほかに { $n } 個のコードが有効です
    }

## Programs outside a box, and downloaded files (src/settings/panels/outside_card.rs)

# Heading of the card for programs that are not in a box.
settings-outside-heading = 箱の外のプログラム
# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindleは数秒ごとに確認し、見つけたものをお知らせします。これらのプログラムを止めることはできず、一瞬だけ動くものは見逃すことがあります。箱の中のプログラムは、その箱が許すものだけを使えます。
# Label of the switch for the notice about the network.
settings-outside-network = プログラムがネットワークを使うときに知らせる
# Label of the switch for the notice about the person's files.
settings-outside-files = プログラムが自分のファイルを開くときに知らせる
# Under those two switches.
settings-outside-once = プログラムごとに1回です。
# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = { $name } を無視するのをやめる
# Under a program marked fine for the network.
settings-outside-fine-network = ネットワークの利用は問題ありません。
# Under a program marked fine for the person's files.
settings-outside-fine-files = ファイルを開いていても問題ありません。
# The line in the card when no program is marked fine.
settings-outside-no-fine = 問題なしにしたプログラムはありません。
# Heading of the card for downloaded files.
settings-downloads-heading = ダウンロードしたファイル
# Help text of that card (the information button).
settings-downloads-about = ダウンロードフォルダー内のファイルと、ブラウザーがダウンロードしたと記録したファイルは、箱の中で開きます。開くプログラムには、そのファイルだけが渡されます。読むことしかできず、インターネットやフォルダーは使えません。ブラウザー自身の「開く」ボタンは対象外です。そのボタンはプログラムを直接起動します。
# Label of the switch that opens downloaded files in a box.
settings-downloads-box = ダウンロードしたファイルを箱の中で開く
# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = { $kind } ファイルを通常どおり開く
# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = { $kind } ファイル
# Under a file type's switch.
settings-downloads-normal-sub = 通常どおり、インターネットを使えるまま開きます。
# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = 種類はまだありません。ダウンロードしたファイルを箱の中で開くと、その種類がここに表示されます。
