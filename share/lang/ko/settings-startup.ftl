# machine translated (Korean), not yet reviewed by a person.

# Settings page "Start at login": apps that open by themselves when someone logs in.

## Page (src/settings/panels/startup.rs)

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = 로그인 시 시작
# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay 자동 시작 로그인 부팅 지연
# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = 로그인하면 저절로 열리는 앱이에요. 지연 시간을 더하면 앱이 네트워크나 다른 앱을 먼저 기다려요.

## Top sentence

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = 로그인할 때 저절로 시작하는 앱이 없어요.
# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = 로그인하면 { $name }이(가) 저절로 시작돼요.
# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
       *[other] 로그인하면 앱 { $n }개가 저절로 시작돼요.
    }
# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
       *[other] { $n }개는 더 꺼져 있어요.
    }

## Delays

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = 바로
# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = 1분 후
# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = 2분 후
# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
       *[other] { $n }초 후
    }

## Each app's line

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = { $name }의 지연 시간
# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = 로그인 시 시작
# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = 로그인 시 시작하지 않기
# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = Trindle이 삭제하지 못해서 로그인 시 계속 시작돼요.

# Under the name of a launcher that comes with the system, which can only be switched on or off; { $command } is what it runs, cut short.
settings-startup-system = 시스템에 포함됨: { $command }

## Adding

# Red words under the add card when something could not be added.
settings-startup-not-added = 추가하지 못했어요.
# Placeholder of the search box above the list of installed apps.
settings-startup-find = 앱 찾기
# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = 로그인 시 이것 시작
# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = 앱이 더 이상 설치되어 있지 않아서 Trindle이 추가하지 못했어요.
# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = syncthing --no-browser
# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = 프로그램 고르기
# Heading of the line for adding a command.
settings-startup-command = 명령
# Red words under the command line when nothing was typed.
settings-startup-command-empty = 명령을 입력하거나 프로그램을 고르세요.
# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = 로그인 시 시작할 프로그램 고르기

## Cards

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = 저절로 시작하는 것이 없어요
# Title of the card listing what starts at login.
settings-startup-list-title = 로그인 시 시작
# Words behind the i of the card listing what starts at login.
settings-startup-about = 앱마다 로그인 후 잠시 기다릴 수 있어서, 네트워크나 다른 앱이 먼저 준비돼요.
# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } 실행: { $command }
# Name of the button that opens the list of installed apps.
settings-startup-open-apps = 설치된 앱에서 고르기
# Name of the button that opens the box for typing a command.
settings-startup-open-command = 명령 입력 또는 프로그램 고르기
# Heading of the line for adding an installed app.
settings-startup-app-heading = 설치된 앱
# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = 명령 또는 프로그램
# Words behind the i of the Add an app card.
settings-startup-add-about = 설치된 앱 중에서 고르거나, 터미널에서처럼 실행할 명령을 입력하세요. 디스크에 있는 프로그램을 고를 수도 있어요. 다음 로그인 때 시작돼요. 목록에 들어가면 지연 시간을 고를 수 있어요.
# Title of the card for adding an app.
settings-startup-add-title = 앱 추가
