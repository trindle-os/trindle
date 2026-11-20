# machine translated (Korean), not yet reviewed by a person.

# Settings, Tailscale page: a private network between the person's own devices.

## Tailscale page (src/settings/panels/tailscale.rs)

# What a device runs, shown in a device's small line, when it is an iPhone or iPad.
settings-tailscale-system-ios = iPhone 또는 iPad
# In a device's small line: the device is on the network now.
settings-tailscale-online = 지금 온라인
# In a device's small line: the device has never been seen.
settings-tailscale-not-seen = 아직 확인된 적 없음
# In a device's small line: when the device was last online; the date is already written.
settings-tailscale-last-online = 마지막 온라인 { $day }
# Shown beside the Use Tailscale switch while the password is asked for, when Tailscale is already installed.
settings-tailscale-waiting = 비밀번호를 기다리는 중…
# Shown beside the Use Tailscale switch while the password is asked for, before Tailscale is installed.
settings-tailscale-waiting-install = Tailscale을 설치하기 전에 비밀번호를 기다리는 중…
# Shown beside the switch after Tailscale is turned on.
settings-tailscale-done-on = Tailscale 켜짐
# Shown beside the switch after Tailscale is turned off.
settings-tailscale-done-off = Tailscale 꺼짐
# Shown under the Sign in line once the browser is being opened.
settings-tailscale-opening = 브라우저에서 로그인 페이지를 여는 중…
# Red words under the Sign in line when Tailscale could not be started.
settings-tailscale-not-started = Tailscale을 시작하지 못했어요.
# Red words under the Sign out line when signing out did not work.
settings-tailscale-sign-out-failed = Tailscale에서 로그아웃하지 못했어요.
# Headline of the page while Tailscale is being asked how it is.
settings-tailscale-top-asking = Tailscale에 상태를 묻는 중…
# Headline of the page when Tailscale is off and not installed.
settings-tailscale-top-not-installed = Tailscale이 설치되어 있지 않아요. 아래에서 켜면 설치돼요.
# Headline of the page when Tailscale is installed but off.
settings-tailscale-top-off = Tailscale이 꺼져 있어서 메모리를 쓰지 않아요.
# Headline of the page when Tailscale is on but does not answer.
settings-tailscale-top-silent = Tailscale이 응답하지 않아요. 아직 시작하는 중일 수 있어요.
# Headline of the page when connected: the name of the private network and of this computer.
settings-tailscale-top-connected = { $network }에 { $name }(으)로 연결되어 있어요.
# Name used in the headline for the private network when it has no name of its own.
settings-tailscale-network-fallback = Tailscale
# Headline of the page when Tailscale is on and has to be signed in to.
settings-tailscale-top-sign-in = Tailscale이 켜져 있어요. 아래에서 로그인하면 내 비공개 네트워크에 들어가요.
# Headline of the page for any other state; the state is already written in lower case.
settings-tailscale-top-state = Tailscale 상태: { $state }.
# Name of the page in the list of settings pages.
settings-tailscale-title = Tailscale
# Words the settings search looks for on this page (not shown).
settings-tailscale-keywords = tailscale vpn 비공개 네트워크 기기 원격 tailnet private network devices remote
# Help text at the top of the page.
settings-tailscale-help = Tailscale은 내 컴퓨터와 휴대폰을 어디에 있든 하나의 비공개 네트워크로 묶어 줘요. 켜기 전에는 아무것도 설치되지 않고, 꺼져 있으면 메모리를 쓰지 않아요.
# Text of the switch that turns Tailscale on or off, and heading of its line.
settings-tailscale-switch = Tailscale 사용
# Help text (i button) of the Tailscale card when Tailscale is off or has not answered.
settings-tailscale-about-plain = 처음 켜면 Trindle이 Tailscale 자체 소프트웨어 저장소에서 Tailscale을 설치하고 시작해요. Tailscale의 백그라운드 서비스는 실행 중에 메모리를 쓰기 때문에 이 설정이 켜져 있을 때만 실행돼요. 꺼져 있으면 Tailscale은 멈춰 있고 컴퓨터를 켤 때 시작하지 않아요. 방화벽은 내 Tailscale 기기만 들어오게 하고 나머지는 전과 같이 막아요.
# Help text (i button) of the Tailscale card when Tailscale is on but not yet connected.
settings-tailscale-about-sign-in = { settings-tailscale-about-plain } 로그인하면 브라우저에서 Tailscale 로그인 페이지가 열려요. 모든 기기에서 같은 계정을 쓰세요.
# Help text (i button) of the Tailscale card when connected.
settings-tailscale-about-connected = { settings-tailscale-about-plain } 로그아웃하면 다시 로그인할 때까지 이 컴퓨터는 내 비공개 네트워크에서 빠져요.
# Line in the devices card when there are no other devices.
settings-tailscale-no-devices = 아직 다른 기기가 없어요
# Shown on the devices card while Tailscale is off.
settings-tailscale-waits-off = 기기를 보려면 Tailscale을 켜세요.
# Shown on the devices card while Tailscale has not answered.
settings-tailscale-waits-looking = 내 기기를 찾는 중…
# Shown on the devices card when Tailscale needs signing in to.
settings-tailscale-waits-sign-in = 기기를 보려면 로그인하세요.
# Shown on the devices card when Tailscale is on but not connected.
settings-tailscale-waits-connect = 기기를 보려면 연결하세요.
# Small line under this computer when it has no address yet.
settings-tailscale-no-address = 아직 주소가 없어요.
# Small line under this computer: its address on the private network.
settings-tailscale-address = 내 비공개 네트워크에서 { $address }
# Name of this computer's line when Tailscale gives it no name.
settings-tailscale-this-computer = 이 컴퓨터
# Heading of the line that signs in to Tailscale, and tip of its round button.
settings-tailscale-sign-in = 로그인
# Heading of the line that joins the private network again, and tip of its round button.
settings-tailscale-connect = 연결
# Heading of the line that says the firewall lets the person's own devices in.
settings-tailscale-firewall = 내 기기는 들어올 수 있어요
# Small line under that heading.
settings-tailscale-firewall-sub = 방화벽은 나머지를 계속 막아요.
# Heading of the line that signs out, and tip of its button.
settings-tailscale-sign-out = 로그아웃
# Heading of the card with the Tailscale switch.
settings-tailscale-card = Tailscale
# Heading of the card that lists the person's devices.
settings-tailscale-devices = 내 기기
# Help text (i button) of the Your devices card.
settings-tailscale-devices-about = 내 비공개 네트워크에 로그인한 모든 기기와 그 네트워크에서의 주소, 실행 중인 시스템, 마지막 온라인 시간을 보여 줘요. 기기를 추가하려면 휴대폰이나 다른 컴퓨터에 Tailscale을 설치하고 같은 계정으로 로그인하세요.
