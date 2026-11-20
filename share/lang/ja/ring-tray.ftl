# machine translated (Japanese), not yet reviewed by a person.

# The ring's tray and the updates button: status buttons for sound, battery, network and Bluetooth, other programs' tray icons, and what the updates button says. Part of the ring's own words (see ring.ftl).

## Updates button (src/ring/updates.rs)

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
       *[other] アプリで確認が必要なプラグインの更新が { $n } 件あります
    }
# Answer to a spoken question about updates when none wait.
ring-updates-none = すべて最新です
# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
       *[other] 更新が { $n } 件あります。アプリの横にある更新ボタンでインストールできます。
    }
# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look =
    { $n ->
       *[other] 更新が { $n } 件あります。{ $look }。
    }
# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = 更新が { $n } 件あります。アプリの横にある更新ボタンで、そのうち { $put } 件をインストールできます。{ $look }。
# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
       *[other] ファームウェアの更新 { $n } 件を見る
    }
# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
       *[other] ファームウェアを含む更新 { $n } 件を見る
    }
# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
       *[other] 更新 { $n } 件をインストール
    }
# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
       *[other] 以前より多くの権限を求めるプラグインの更新 { $n } 件を見る
    }
# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
       *[other] 更新 { $put } 件をインストール、{ $look }
    }
# Name of the updates button when nothing waits.
ring-updates-up-to-date = 最新です
# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = 確認はオフです
# Name of the updates button when a restart to finish installing was put off with Remind me later and the time has come.
ring-updates-restart = 再起動してインストールを完了

## Tray (src/ring/tray.rs)

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = キーボード
# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = 画面の明るさ
# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = スリープ防止
# Name of the sound button when the sound is muted.
ring-tray-muted = ミュート中
# Name of the sound button with the volume level.
ring-tray-volume = 音量 { $pct }%
# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] バッテリー { $pct }%、充電中
        [onbattery] バッテリー { $pct }%、バッテリー駆動
        [full] バッテリー { $pct }%、満充電
        [pluggedin] バッテリー { $pct }%、電源接続中
       *[other] バッテリー { $pct }%
    }
# Name of the network button when networking is switched off.
ring-tray-net-off = ネットワークはオフです
# Name of the network button when nothing is connected.
ring-tray-not-connected = 接続されていません
# Name of the network button when the connection is through a VPN.
ring-tray-vpn = VPN経由で接続中
# Name of the network button when connected by cable.
ring-tray-wired = 有線
# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = 有線、インターネットなし
# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }、電波 { $strength }%
# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }、電波 { $strength }%、接続が制限されています
# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetoothはオフです
# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = { $names } に接続中
# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetoothはオンです

## Other programs' tray icons (src/ring/sni/)

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = トレイアイコン
# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = すべてのトレイアイコン
# Words on the panel of every tray icon when there are none.
ring-tray-none = トレイアイコンはありません
# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = リングに表示
# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = リングから外す
# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = リングにはすでにアイコンが4つあります。先に1つ外してください。
