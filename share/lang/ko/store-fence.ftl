# machine translated (Korean), not yet reviewed by a person.

# Apps: the fence AI programs run in, and the window that asks the person (src/store/fence.rs, fence_window.rs).

## The fence's settings (src/store/fence.rs)

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = 올바른 프로그램 이름이 아니에요.
# Red words when a fence setting could not be written to its file.
store-fence-not-saved = 저장되지 않았어요.
# Red words when the folder chosen to share does not exist.
store-fence-no-folder = 있는 폴더를 고르세요.
# Red words when the whole home folder is chosen to share.
store-fence-whole-home = 홈 폴더 전체는 비공개로 유지돼요.
# Red words when a folder that holds the home folder is chosen to share.
store-fence-holds-home = 이 폴더에 홈 폴더가 들어 있어서 비공개로 유지돼요.
# Red words when a folder outside the home folder is chosen to share.
store-fence-outside-home = 홈 폴더 안의 폴더만 공유할 수 있어요.
# Red words when a hidden folder is chosen to share.
store-fence-hidden = 숨김 폴더는 항상 비공개로 유지돼요.
# Red words when a folder that keeps passwords or mail is chosen to share.
store-fence-passwords = 이 폴더에 비밀번호나 메일이 있어서 비공개로 유지돼요.
# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = 그 프로그램에는 쓸 설정 폴더가 없어요.
# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = 사이트 이름을 입력하세요. 예: api.example.com, 또는 *.example.com은 그 이름 아래의 모든 사이트예요.
# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = 컴퓨터 이름을 입력하세요. 예: nas.local.
# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = 각각 NAME=value 형식으로 쓰세요.
# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = 각각 대문자로 NAME=value 형식으로 쓰세요.

## The window that asks about an AI program (src/store/fence_window.rs)

# Words above the question when an AI program asks to use a folder.
store-fence-ask-share = AI 프로그램이 이 폴더를 쓰게 해 달라고 해요. 지금은 이 폴더가 보이지 않아요.
# Words above the question when an AI program asks to run something as the administrator.
store-fence-ask-admin = AI 프로그램이 이것을 관리자로 실행하게 해 달라고 해요. 울타리 밖에서 실행되고 내가 볼 수 있는 모든 것을 볼 수 있어요.
# Small heading of the window that asks about an AI program.
store-fence-ai-program = AI 프로그램
# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
store-fence-read-only = 이 폴더를 읽기만 하게 하기
# Note under the question about a folder.
store-fence-sees-next = 프로그램을 다음에 시작할 때 이 폴더가 보여요.
# Tip of the round button that refuses a folder.
store-fence-no = 아니요
# Tip of the round button that shares a folder until the session ends.
store-fence-share-session = 이번 세션 동안만 공유
# Main button that shares a folder for good.
store-fence-share-always = 항상 공유
# Tip of the round button that refuses to run something as the administrator.
store-fence-dont-allow = 허용 안 함
# Main button that lets something run as the administrator.
store-fence-allow = 허용

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = 그 명령은 너무 길어서 전체를 보여 줄 수 없어서 실행하지 않았어요.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = 답을 기다리는 다른 요청이 있어요.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = 사용자가 허용하지 않았어요.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name }은(는) 울타리 안에서 쓸 수 없어요.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = 울타리 밖에서 시작할 수 없었어요.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = 폴더의 전체 경로를 알려 주세요.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = 사용자가 공유하지 않았어요.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = 공유했어요. 이 프로그램을 다음에 시작할 때 보여요.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = 연결할 컴퓨터의 이름을 알려 주세요.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = 울타리 안의 AI 프로그램은 그 컴퓨터에 ssh로 연결할 수 없어요.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = AI 프로그램용 키를 만들 수 없었어요: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = 실행할 명령을 알려 주세요.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name }은(는) 여기서 쓸 수 없어요.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name }은(는) 울타리 안에서 쓸 수 없어요. 울타리 밖에서 실행해 달라고 요청하세요.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: 울타리의 요청 통로가 없어요.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: 요청을 보내지 못했어요.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock이 거부했어요: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = 울타리가 시작되지 않았어요: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = 울타리가 시작되지 않았어요: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = AI 프로그램은 여기서 내 로그인과 따로 자기 로그인을 가져요. 프로그램이 요청하면 프로그램 안에서 로그인하세요.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = 내 폴더는 AI 프로그램에게 비공개예요. 폴더에서 작업하려면 설정의 AI 프로그램에서 공유하세요.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = 울타리가 시작되지 않았어요: 시스템 호출 필터를 만들 수 없었어요.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = 울타리가 시작되지 않았어요: 네트워크 제한을 설정할 수 없었어요.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = 울타리가 시작되지 않았어요: bubblewrap이 실행되지 않았어요: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: 울타리 밖에서 찾을 수 없어요
