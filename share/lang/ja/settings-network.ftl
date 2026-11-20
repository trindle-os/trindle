# machine translated (Japanese), not yet reviewed by a person.

# Settings, Network page and the radio words shared with Bluetooth.

## Radio switches (src/common/rfkill.rs)

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } は、コンピューター本体のスイッチかキーでオフになっています。オンにしてから、もう一度試してください。
# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = システムが { $radio } をブロックしていて、Trindle はブロックを解除できませんでした。

## Network page (src/settings/panels/network.rs)

# Said in red under a Wi-Fi network when its password was typed and not accepted.
settings-network-join-password-typed = パスワードが受け付けられませんでした。もう一度入力してください。
# Said in red under a Wi-Fi network when the password kept for it was not accepted.
settings-network-join-password-kept = このネットワークの保存済みパスワードが受け付けられませんでした。ごみ箱ボタンでネットワークを削除してから、もう一度接続してください。
# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Trindle はそのネットワークを見つけられませんでした。名前と、電波の届く範囲かを確認してください。
# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = このコンピューターはネットワークに接続しましたが、アドレスを受け取れませんでした。もう一度試すか、ルーターを再起動してください。
# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = このコンピューターはネットワークに接続できませんでした。ルーターに近づいて、もう一度試してください。
# Said in red when the password typed is one a Wi-Fi network cannot have.
settings-network-bad-password = Wi-Fi のパスワードとして使えません。確認して、もう一度試してください。
# Said in red when joining a network failed for no known reason.
settings-network-join-refused = ネットワークに接続できませんでした。
# Small line of a shared folder: who may open it with a password.
settings-network-access-open = パスワードで開けます
# Small line of a shared folder: who may open and change it with a password.
settings-network-access-change = パスワードで開いて変更できます
# Small line of a shared folder: anyone on the local network may open it.
settings-network-access-guests-open = 近くの誰でも開けます
# Small line of a shared folder: anyone on the local network may change it.
settings-network-access-guests-change = 近くの誰でも変更できます
# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }
# Tip of the round button that stops sharing a folder; the folder's name follows.
settings-network-stop-sharing = { $name } の共有を停止
# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = 管理対象外
# State of a network device that cannot be used.
settings-network-state-unavailable = 使用不可
# State of a network device that is not connected.
settings-network-state-disconnected = 未接続
# State of a network device that is getting ready to connect.
settings-network-state-preparing = 準備中
# State of a network device that is being set up.
settings-network-state-configuring = 設定中
# State of a network device that needs a password.
settings-network-state-needs-password = パスワードが必要です
# State of a network device that is getting an address.
settings-network-state-address = アドレスを取得中
# State of a network device whose connection is being checked.
settings-network-state-checking = 確認中
# State of a network device that is waiting.
settings-network-state-waiting = 待機中
# State of a network device that is connected.
settings-network-state-connected = 接続済み
# State of a network device that is disconnecting.
settings-network-state-disconnecting = 切断中
# State of a network device that failed.
settings-network-state-failed = 失敗
# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = ケーブルと、Wi-Fi の { $name } に接続しています。
# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] { $name } に接続しています。電波は強いです。
        [good] { $name } に接続しています。電波は良好です。
        [fair] { $name } に接続しています。電波はやや弱いです。
       *[other] { $name } に接続しています。電波は弱いです。
    }
# Headline of the page when connected by cable only.
settings-network-top-cable = ケーブルで接続しています。
# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi がオフなので、このコンピューターは接続されていません。
# Headline of the page when not connected.
settings-network-top-none = 未接続です。下からネットワークを選んでください。
# Headline of the page while the networks are looked for.
settings-network-top-looking = ネットワークを探しています…
# Headline of the page when NetworkManager is not running.
settings-network-top-gone = NetworkManager が動いていないため、Trindle はネットワークを表示できません。
# The word for the system bus in "The system bus is not available".
settings-network-the-bus = システムバス
# Name of the page in the list of settings pages and its heading.
settings-network-title = ネットワーク
# Words the settings search looks for on this page (not shown).
settings-network-keywords = ネットワーク wifi wi-fi 無線 インターネット イーサネット 有線 パスワード 接続 ファイアウォール 共有 プリンター フォルダ ethernet wireless firewall sharing share printer folder
# Help text at the top of the page.
settings-network-help = Wi-Fi のオンとオフ、ネットワークへの接続、不要なネットワークの削除、ほかのコンピューターからの接続の許可ができます。
# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] { $name } の電波は強いです。
        [good] { $name } の電波は良好です。
        [fair] { $name } の電波はやや弱いです。
       *[other] { $name } の電波は弱いです。
    }
# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Wi-Fi には、より新しい Linux が必要です。
# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = このコンピューターでは Wi-Fi を使えません。
# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = オフです。ネットワークに接続するには、オンにしてください。
# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = オンですが、ネットワークには接続していません。
# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Wi-Fi を使う
# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi
# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi とケーブル
# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi は、ケーブルなしでこのコンピューターをネットワークにつなぎます。オフにするとネットワークを探さなくなり、バッテリーを少し節約できます。名前を隠しているネットワークは、電波の届く範囲のネットワーク一覧に出ません。接続するには、大文字と小文字も含めて名前を正確に入力してください。Wi-Fi チップによっては、Trindle が使っている Linux にまだないドライバーが必要です。その場合は、より新しい Linux が必要です。ネットワークケーブルには専用の行があり、接続と切断のボタンがあります。
# Heading of the line with the switch that lets other computers connect, and the name of the switch.
settings-network-share-switch = ほかのコンピューターからの接続を許可
# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = 共有フォルダを探しています…
# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = 共有フォルダを読み取れませんでした
# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = まだ接続できません
# Small line under it.
settings-network-not-reachable-sub = 共有がオフの間は、ほかのコンピューターはこれらを開けません。
# Line in the Sharing card when no folder is shared.
settings-network-no-shared = 共有しているフォルダはありません
# Heading of the Sharing card.
settings-network-sharing = 共有
# Help text (i button) of the Sharing card.
settings-network-sharing-about = 「ほかのコンピューターからの接続を許可」がオフの間は、ネットワーク上のほかのコンピューターは、このコンピューターから先に接続した場合しか、このコンピューターに届きません。これで、同じ Wi-Fi にいる誰かが侵入しようとするのを防げます。家にあるほかのコンピューターとフォルダやプリンターを共有するには、オンにしてください。フォルダを共有するには、Folders でフォルダを右クリックして「ネットワークで共有」を選びます。共有フォルダごとに、誰が何をできるかが表示されます。パスワードあり: 共有パスワードを知っている人は、ファイルを開くか、開いて変更できます。家の誰でも: ネットワーク上の誰でも、パスワードなしでファイルを開くか、開いて変更できます。スイッチがオフの間は、家のほかのコンピューターはどれも開けません。
# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = 停止しています…
# Shown beside the sharing switch while the password is asked for.
settings-network-waiting = パスワードを待っています…
# Shown beside the sharing switch after other computers were allowed in.
settings-network-sharing-allowed = 共有はオンです
# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = ブロック中
# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi をオンにできませんでした。
# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi をオフにできませんでした。
# Name of a wired network device that has no name of its own.
settings-network-ethernet = イーサネット
# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = 切断
# Tip of the button that connects a network or a cable.
settings-network-connect = 接続
# Shown under a network or cable while it is being connected.
settings-network-connecting = 接続しています…
# Line in the Networks in range card when there are none.
settings-network-none-in-range = 届く範囲にネットワークはありません
# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = 届く範囲のネットワーク
# Help text (i button) of the Networks in range card.
settings-network-in-range-about = このコンピューターが受信できるすべての Wi-Fi ネットワークです。ページを開いたときは、使用中のネットワークが先頭、次に保存済みのネットワーク、その後は電波の強い順に並びます。ページを開いている間、順番は変わりません。矢印を押すとネットワークに接続します。保護されたネットワークは、最初にパスワードを求めます。ごみ箱ボタンは、保存済みのネットワークとそのパスワードを削除します。オープンなネットワークは暗号化されていないので、近くの人に送った内容を見られるおそれがあります。
# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = 届く範囲のネットワークを見るには、Wi-Fi をオンにしてください。
# Part of a network's small line: it is saved on this computer.
settings-network-saved = 保存済み
# Part of a network's small line: it needs a password.
settings-network-secured = 保護あり
# Part of a network's small line: it signs in with a username as well as a password, as work and university networks do.
settings-network-username = ユーザー名が必要
# Red words when someone tries to join a network that signs in with a username.
settings-network-username-unsupported = このネットワークにはパスワードのほかにユーザー名が必要です。Trindle はまだこの種類に接続できません。
# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = オープン、暗号化なし
# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s
# Tip of the bin button that forgets a saved network.
settings-network-forget = このネットワークを削除
# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = ネットワークを削除できませんでした。
# Placeholder of the box where a network's password is typed.
settings-network-password = パスワード
# Tip of the button that joins the network.
settings-network-join = 接続
# Tip of the button that closes the box without joining.
settings-network-cancel = キャンセル
# Shown under a network while it is being joined.
settings-network-joining = 接続しています…
# Said in red when a Wi-Fi password is too short.
settings-network-password-short = Wi-Fi のパスワードは 8 文字以上です。
# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = 非公開のネットワークに接続
# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = ネットワーク名
# Placeholder of the box where a hidden network's password is typed.
settings-network-hidden-password = パスワード (ある場合)
# Said in red when no name was typed for a hidden network.
settings-network-name-needed = ネットワーク名を入力してください。
# Shown under a network after it was joined.
settings-network-joined = 接続しました
# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } ネットワークは保存されたままです。もう一度接続する前に、ごみ箱ボタンで削除してください。
# Said in red when a network would not disconnect.
settings-network-disconnect-failed = 切断できませんでした。

# Heading of the card that lists the ports opened by hand.
settings-network-ports = 開いているポート

# Help text (i button) of the Open ports card.
settings-network-ports-about = このコンピューターから先に接続した場合を除き、すべてのポートは閉じています。ゲームサーバーや同期ツールのように、ほかからの接続を待つプログラムは、ここでポートを開くまで接続できません。ポートを開くと、このコンピューターと同じローカルネットワークにいるすべてのコンピューターが接続できますが、外のインターネットからは接続できません。カフェ、ホテル、職場のネットワークでは、その場にいるほかの全員が含まれます。ポートを開いたり閉じたりするには、管理者のパスワードが必要です。

# Line in the Open ports card when no port is open.
settings-network-ports-none = 開いているポートはありません



# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = { $port } を閉じる

# Heading of the line that opens a port.
settings-network-port-add = ポートを開く


# Placeholder in the box where the port number is typed.
settings-network-port-number = ポート番号

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = プロトコル

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = 両方

# Button that opens the typed port.
settings-network-port-open = 開く

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = 1 から 65535 の数字を入力してください。

# Red words when the port is already open.
settings-network-port-already = すでに開いています。

# Said under the line once the port is open.
settings-network-port-opened = 開きました
