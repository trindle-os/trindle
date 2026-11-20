# machine translated (Japanese), not yet reviewed by a person.

# Settings, Tailscale page: a private network between the person's own devices.

## Tailscale page (src/settings/panels/tailscale.rs)

# What a device runs, shown in a device's small line, when it is an iPhone or iPad.
settings-tailscale-system-ios = iPhone または iPad
# In a device's small line: the device is on the network now.
settings-tailscale-online = オンライン
# In a device's small line: the device has never been seen.
settings-tailscale-not-seen = まだ見ていません
# In a device's small line: when the device was last online; the date is already written.
settings-tailscale-last-online = 最後のオンライン: { $day }
# Shown beside the Use Tailscale switch while the password is asked for, when Tailscale is already installed.
settings-tailscale-waiting = パスワードを待っています…
# Shown beside the Use Tailscale switch while the password is asked for, before Tailscale is installed.
settings-tailscale-waiting-install = Tailscale をインストールする前に、パスワードを待っています…
# Shown beside the switch after Tailscale is turned on.
settings-tailscale-done-on = Tailscale オン
# Shown beside the switch after Tailscale is turned off.
settings-tailscale-done-off = Tailscale オフ
# Shown under the Sign in line once the browser is being opened.
settings-tailscale-opening = ブラウザーでログインページを開いています…
# Red words under the Sign in line when Tailscale could not be started.
settings-tailscale-not-started = Tailscale を起動できませんでした。
# Red words under the Sign out line when signing out did not work.
settings-tailscale-sign-out-failed = Tailscale からログアウトできませんでした。
# Headline of the page while Tailscale is being asked how it is.
settings-tailscale-top-asking = Tailscale に確認しています…
# Headline of the page when Tailscale is off and not installed.
settings-tailscale-top-not-installed = Tailscale はインストールされていません。下のスイッチをオンにするとインストールします。
# Headline of the page when Tailscale is installed but off.
settings-tailscale-top-off = Tailscale はオフなので、メモリを使いません。
# Headline of the page when Tailscale is on but does not answer.
settings-tailscale-top-silent = Tailscale が応答しません。まだ起動中かもしれません。
# Headline of the page when connected: the name of the private network and of this computer.
settings-tailscale-top-connected = { $network }に{ $name }として接続しています。
# Name used in the headline for the private network when it has no name of its own.
settings-tailscale-network-fallback = Tailscale
# Headline of the page when Tailscale is on and has to be signed in to.
settings-tailscale-top-sign-in = Tailscale はオンです。下でログインすると、プライベートネットワークに参加できます。
# Headline of the page for any other state; the state is already written in lower case.
settings-tailscale-top-state = Tailscale は{ $state }です。
# Name of the page in the list of settings pages.
settings-tailscale-title = Tailscale
# Words the settings search looks for on this page (not shown).
settings-tailscale-keywords = tailscale vpn private network devices remote tailnet
# Help text at the top of the page.
settings-tailscale-help = Tailscale は、あなたのコンピューターやスマートフォンを、どこにあっても1つのプライベートネットワークにつなぎます。オンにするまで何もインストールされず、オフの間はメモリを使いません。
# Text of the switch that turns Tailscale on or off, and heading of its line.
settings-tailscale-switch = Tailscale を使う
# Help text (i button) of the Tailscale card when Tailscale is off or has not answered.
settings-tailscale-about-plain = 初めてオンにすると、Trindle は Tailscale 公式のソフトウェア配布元から Tailscale をインストールして起動します。Tailscale のバックグラウンドサービスは動いている間メモリを使うので、これがオンのときだけ動かします。オフの間は Tailscale は停止し、コンピューターの起動時にも始まりません。ファイアウォールは、あなた自身の Tailscale デバイスだけを通し、ほかはこれまでどおり防ぎます。
# Help text (i button) of the Tailscale card when Tailscale is on but not yet connected.
settings-tailscale-about-sign-in = { settings-tailscale-about-plain } ログインすると、ブラウザーで Tailscale のログインページが開きます。どのデバイスでも同じアカウントを使ってください。
# Help text (i button) of the Tailscale card when connected.
settings-tailscale-about-connected = { settings-tailscale-about-plain } ログアウトすると、もう一度ログインするまで、このコンピューターはプライベートネットワークから外れます。
# Line in the devices card when there are no other devices.
settings-tailscale-no-devices = ほかのデバイスはまだありません
# Shown on the devices card while Tailscale is off.
settings-tailscale-waits-off = デバイスを見るには、Tailscale をオンにしてください。
# Shown on the devices card while Tailscale has not answered.
settings-tailscale-waits-looking = デバイスを探しています…
# Shown on the devices card when Tailscale needs signing in to.
settings-tailscale-waits-sign-in = デバイスを見るには、ログインしてください。
# Shown on the devices card when Tailscale is on but not connected.
settings-tailscale-waits-connect = デバイスを見るには、接続してください。
# Small line under this computer when it has no address yet.
settings-tailscale-no-address = アドレスはまだありません。
# Small line under this computer: its address on the private network.
settings-tailscale-address = プライベートネットワークでのアドレス: { $address }
# Name of this computer's line when Tailscale gives it no name.
settings-tailscale-this-computer = このコンピューター
# Heading of the line that signs in to Tailscale, and tip of its round button.
settings-tailscale-sign-in = ログイン
# Heading of the line that joins the private network again, and tip of its round button.
settings-tailscale-connect = 接続
# Heading of the line that says the firewall lets the person's own devices in.
settings-tailscale-firewall = あなたのデバイスは入れます
# Small line under that heading.
settings-tailscale-firewall-sub = ほかはファイアウォールが防ぎます。
# Heading of the line that signs out, and tip of its button.
settings-tailscale-sign-out = ログアウト
# Heading of the card with the Tailscale switch.
settings-tailscale-card = Tailscale
# Heading of the card that lists the person's devices.
settings-tailscale-devices = あなたのデバイス
# Help text (i button) of the Your devices card.
settings-tailscale-devices-about = プライベートネットワークにログインしているすべてのデバイスです。ネットワーク上のアドレス、動かしているシステム、最後にオンラインだった時刻が分かります。デバイスを追加するには、スマートフォンや別のコンピューターに Tailscale をインストールし、同じアカウントでログインしてください。
