# machine translated (Korean), not yet reviewed by a person.

# Settings, Games page: what helps Steam games run well that Steam itself does not set.

## Games page (src/settings/panels/games.rs, src/common/games.rs)

# The page's name in the side bar.
settings-games-title = 게임
# Words a search finds the page by.
settings-games-keywords = 게임 steam proton 지직거림 끊김 소리 버퍼 드라이브 라이브러리 분할 잠금 컨트롤러 게임패드 조이스틱 패드 버튼 키 변경 재배치 교체 스틱 트리거 방향키 xbox playstation nintendo
# What the page is for, in the help.
settings-games-help = 게임이 잘 돌아가도록 돕는 설정이에요. Steam 자체 설정은 Steam에서 바꾸세요. 컨트롤러 버튼을 바꾸려면 그 버튼을 누르세요. 버튼이 목록 맨 위로 올라가고 그림에서 빛나요. 그다음 대신할 키보드 키를 누르세요. 버튼을 다른 버튼이나 없음으로 바꾸는 것도 목록에서 할 수 있어요. Trindle은 컨트롤러 제조사와 모델별로 변경 내용을 저장하고, 게임은 바뀐 컨트롤러를 써요. 시작한 변경을 취소하려면 Esc를 누르세요.
# The sentence at the top of the page.
settings-games-top = 소리가 지직거리거나, 끊기거나, 공간이 더 필요한 게임을 위한 도움말이에요.
# Shown beside a switch while the person is asked for the password.
settings-games-waiting = 비밀번호를 기다리는 중…
# The card holding the sound and split-lock switches.
settings-games-smooth = 더 부드러운 게임
# What that card is for.
settings-games-smooth-about = 소리가 지직거리거나 끊기는 게임을 고쳐요. 둘 다 처음에는 꺼져 있어요.
# The sound switch.
settings-games-sound = 더 큰 소리 버퍼
# Added after the words behind the i of its card.
settings-games-sound-sub = 소리가 지직거리는 게임용이에요. 소리가 조금 늦게 나와요.
# After the sound switch is turned on.
settings-games-sound-done-on = 큰 버퍼 켜짐
# After the sound switch is turned on, when the sound system could not be told at once.
settings-games-sound-next-login = 다음 로그인부터 켜져요
# After the sound switch is turned off.
settings-games-sound-done-off = 큰 버퍼 꺼짐
# The split-lock switch.
settings-games-split = 오래된 게임 끊김 해결
# Added after the words behind the i of its card.
settings-games-split-sub = 오래된 게임이 덜 끊기지만, 프로그램 하나가 나머지를 느리게 할 수 있어요.
# After the split-lock switch is turned on.
settings-games-split-done-on = 끊김 해결 켜짐
# After the split-lock switch is turned off.
settings-games-split-done-off = 끊김 해결 꺼짐
# The games drive card.
settings-games-drive-title = 게임 드라이브
# What that card is for.
settings-games-drive-about = 게임용 두 번째 드라이브예요. 시작할 때마다 준비돼요.
# A drive with no name of its own.
settings-games-drive-unnamed = 드라이브
# One drive in the list: its name, size and kind of filesystem.
settings-games-drive-name = { $name }, { $size }, { $fs }
# The button that makes a drive the games drive.
settings-games-drive-use = 게임용으로 사용
# Under the drive in use for games.
settings-games-drive-in-use = { $place }의 게임 드라이브
# The button that stops using a drive for games.
settings-games-drive-stop = 게임용 사용 중지
# After a drive is made the games drive.
settings-games-drive-done = 게임 드라이브 준비됨
# After a drive is no longer the games drive.
settings-games-drive-forgotten = 더 이상 게임용이 아니에요
# When there is no second drive.
settings-games-drive-none = 두 번째 드라이브 없음
# When the drives could not be listed.
settings-games-drive-unread = 드라이브 목록을 볼 수 없었어요
# When Steam could not be let reach the games drive.
settings-games-drive-steam-failed = Steam이 드라이브를 쓰게 할 수 없었어요
# Added after the words behind the i of its card.
settings-games-drive-steam = Steam에서 추가하세요
# Added after the words behind the i of its card.
settings-games-drive-steam-sub = Steam에서: 설정, 저장소, 드라이브 추가.
# Why a Windows-formatted drive cannot be used.
settings-games-drive-windows = 이 드라이브는 Windows용으로 포맷되어 있어서 게임이 제대로 작동하지 않아요. Linux용으로 포맷하세요.
# Why a locked drive cannot be used.
settings-games-drive-locked = 먼저 드라이브에서 잠금을 푸세요.
# Why a part that holds no files cannot be used.
settings-games-drive-not-files = 파일용이 아니에요
# Why an unknown kind of drive cannot be used.
settings-games-drive-unknown = 여기서는 게임을 실행할 수 없어요

## Steam games (src/settings/panels/games/steam.rs)

# The card listing the person's Steam games.
settings-games-steam-title = Steam 게임
# What that card is for.
settings-games-steam-about = 게임별 문제 해결이에요. 바꾸기 전에 Steam을 닫으세요.
# Shown, and beside every control, while Steam is open.
settings-games-steam-open = Steam이 열려 있어요
# Under that.
settings-games-steam-open-sub = 바꾸려면 Steam을 닫으세요.
# Beside a game's controls while the game runs.
settings-games-game-running = 게임이 실행 중이에요
# Under a game's name.
settings-games-game-sub = 새로 시작하면 설정과 로컬 저장 파일이 휴지통으로 이동해요.
# The switch for a game: use the stronger graphics chip.
settings-games-stronger = 더 빠른 그래픽 사용
# After that switch is turned on.
settings-games-stronger-on = 빠른 그래픽 켜짐
# After that switch is turned off.
settings-games-stronger-off = 빠른 그래픽 꺼짐
# The switch for a game: use the community Proton build.
settings-games-proton-one = 커뮤니티 Proton 사용
# The switch for all games.
settings-games-proton-all = 모든 게임에 커뮤니티 Proton
# Added after the words behind the i of its card.
settings-games-proton-all-sub = 게임이 처음 필요로 할 때 설치돼요.
# While the community Proton build is being installed.
settings-games-proton-installing = 커뮤니티 Proton 설치 중…
# After a game or all games are given community Proton.
settings-games-proton-on = 커뮤니티 Proton을 선택했어요
# After the choice is given back to Steam.
settings-games-proton-off = Steam의 선택 사용
# When community Proton was not found after installing.
settings-games-proton-missing = 커뮤니티 Proton을 찾을 수 없었어요.
# The button that gives a game a fresh start.
settings-games-fresh = 새로 시작
# After a fresh start.
settings-games-fresh-done = 설정이 휴지통에 있어요. 게임을 다음에 시작하면 Steam이 새로 만들어요.
# When the game had no Windows setup to move.
settings-games-fresh-none = 옮길 것이 없어요
# When no Steam games are installed.
settings-games-none = Steam 게임이 없어요
# Red words when community Proton could not be installed.
settings-games-proton-not-installed = Trindle이 커뮤니티 Proton을 설치할 수 없었어요.
# Red words when Steam's settings could not be changed for a game.
settings-games-proton-not-changed = Trindle이 Steam 설정을 바꿀 수 없었어요.
