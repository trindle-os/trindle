# machine translated (Korean), not yet reviewed by a person.

# Settings: the AI programs page (src/settings/panels/ai_programs.rs).

## The AI programs page (src/settings/panels/ai_programs.rs)

# Name of the AI programs page, in the page list, as its heading and on the card listing the programs.
settings-ai-title = AI 프로그램
# Words people may search for to find the AI programs page, separated by spaces.
settings-ai-keywords = ai 프로그램 claude codex cursor gemini aider 에이전트 울타리 샌드박스 비공개 폴더 공유 ssh 키 토큰 sudo
# Help line at the foot of the AI programs page.
settings-ai-help = AI 코딩 프로그램은 개인 파일이 보이지 않는 울타리 안에서 실행돼요. 필요한 폴더만 공유하세요.
# Headline of the AI programs page.
settings-ai-note = 평소에는 AI 프로그램이 개인 파일에 닿지 못하게 막아요. 예를 들어 속임수에 넘어간 프로그램이 은행 내역을 모아서 밖으로 보내지 못하게 해요. 이 보호에는 한계가 있고 보장은 아니에요. 프로그램이 끈질기게 시도하거나, 컴퓨터 소프트웨어에 결함이 있거나, 요청을 승인하거나 폴더를 공유하면 뚫릴 수도 있어요.
# Added after the words behind the i of its card.
settings-ai-inside = AI 프로그램이 시작한 것은 설치한 프로그램이나 스크립트도 모두 함께 울타리 안에 있어요.
# Red words when a change to the fence could not be made.
settings-ai-not-changed = 바꿀 수 없었어요.
# Said beside a setting on the AI programs page once it is saved.
settings-ai-saved = 저장됨
# Title of the window for choosing a folder to share with AI programs.
settings-ai-choose-folder = AI 프로그램과 공유할 폴더 선택
# Red words when the key for AI programs to reach computers by ssh could not be made.
settings-ai-key-not-made = 키를 만들 수 없었어요.
# Label of the switch that limits the network of AI programs, and heading of its line.
settings-ai-limit-net = 아래 사이트만 허용
# Tip of the button that removes a site from the list AI programs may reach.
settings-ai-stop-site = AI 프로그램이 이 사이트에 접속하지 못하게 하기
# Example in the box for typing a site AI programs may reach.
settings-ai-eg-site = api.anthropic.com
# Tip of the button that adds a site AI programs may reach.
settings-ai-let-site = AI 프로그램이 이 사이트에 접속하게 하기
# Heading of the line for adding a site.
settings-ai-add-site = 사이트 추가
# Added after the words behind the i of its card.
settings-ai-add-site-sub = 사이트 이름을 입력하세요. 그 아래 모든 사이트는 *.example.com 으로 입력해요.
# Heading of the card that limits the network of AI programs.
settings-ai-network = 네트워크
# Help text of the Network card (the information button).
settings-ai-network-about = 꺼져 있으면 AI 프로그램이 여러분과 같은 방식으로 네트워크를 써요. 켜져 있으면 아래 사이트에만 보안(https) 연결로 접속할 수 있고, 울타리에는 자체 네트워크가 없어요. 프로그램의 도구는 받은 프록시 설정을 써야 해요. AI 서비스의 사이트와 설치에 필요한 패키지 저장소를 추가하세요.
# Label of the switch that keeps AI programs inside the fence, and heading of its line.
settings-ai-keep-inside = AI 프로그램을 울타리 안에 두기
# Under the fence switch when it is off.
settings-ai-fence-off = 울타리가 꺼져 있어요. 이제 AI 프로그램이 여러분이 보는 모든 것을 볼 수 있어요.
# Heading of the card with the fence switch.
settings-ai-the-fence = 울타리
# Help text of The fence card (the information button).
settings-ai-fence-about = AI 프로그램은 시스템을 여전히 보고 쓸 수 있어요. 대신 홈 폴더 말고 비어 있는 상태로 시작하는 비공개 폴더를 받고, 이 폴더는 실행 사이에 유지돼요. 공유한 폴더만 그 안에 나타나요. 다른 프로그램, 세션, 키, 비밀번호는 볼 수 없어요. 이 목록에 있는 이름이 아닌 다른 방법으로 시작한 프로그램은 울타리에 들어가지 않아요.
# Label of the switch that fences one AI program: its name is { $name }.
settings-ai-fence-name = { $name } 울타리 안에 두기
# Tip of the button that takes one program off the list: its name is { $name }.
settings-ai-take-off = 목록에서 { $name } 빼기
# Label of the switch for using a program's existing settings folder: its name is { $name }.
settings-ai-own-for = { $name }에 기존 설정 폴더 사용
# Heading of the line with the switch for using a program's existing settings folder.
settings-ai-own = 기존 설정 폴더 사용
# Added after the words behind the i of its card.
settings-ai-own-sub = 이 폴더에는 프로그램의 이전 대화가 있어요.
# Example in the box for typing the name of an AI program.
settings-ai-eg-program = claude
# Tip of the button that adds an AI program, and heading of its line.
settings-ai-add-program = 프로그램 추가
# Help text of the AI programs card (the information button).
settings-ai-programs-about = 이 프로그램들은 울타리 안에서 시작해요. 기존 설정 폴더 사용이 꺼져 있으면 프로그램은 새 비공개 폴더로 시작해요. 각 프로그램은 울타리 안에서 따로 로그인해요. 여러분의 로그인은 공유되지 않아요.
# Heading of the line shown when no folder is shared with AI programs.
settings-ai-nothing-shared = 공유하는 것이 없어요
# Label of the switch that lets AI programs only read a shared folder.
settings-ai-only-read = 이 폴더를 읽기만 하게 하기
# Tip of the button that stops sharing a folder with AI programs.
settings-ai-make-private = 이 폴더를 다시 비공개로 하기
# Under a shared folder's path, beside the switch that makes it read only.
settings-ai-only-read-short = 읽기만 허용
# Tip of the button that shares a folder, and heading of its line.
settings-ai-share-folder = 폴더 공유
# Added after the words behind the i of its card.
settings-ai-share-folder-sub = 같은 위치에 표시돼요. AI 프로그램이 읽고 쓸 수 있어요.
# Heading of the card listing folders shared with AI programs.
settings-ai-shared = AI 프로그램과 공유 중
# Help text of the shared folders card (the information button).
settings-ai-shared-about = 홈 폴더의 다른 모든 폴더는 AI 프로그램에 비공개로 남아요. 숨김 폴더나 키, 비밀번호, 메일이 있는 폴더는 공유할 수 없어요. 여기서 공유한 폴더는 프로그램을 다음에 시작할 때 쓸 수 있어요.
# Tip of the button that removes a computer from the list AI programs may reach by ssh.
settings-ai-stop-computer = AI 프로그램이 이 컴퓨터에 접속하지 못하게 하기
# Example in the box for typing a computer AI programs may reach by ssh.
settings-ai-eg-computer = nas.local
# Tip of the button that adds a computer AI programs may reach by ssh.
settings-ai-let-computer = AI 프로그램이 이 컴퓨터에 접속하게 하기
# Heading of the line for adding a computer.
settings-ai-add-computer = 컴퓨터 추가
# Note showing the public key to put on each computer: the key is { $key }.
settings-ai-put-key = 각 컴퓨터에 이 키를 넣으세요: { $key }
# Tip of the button that makes a key for AI programs.
settings-ai-make-key = 키 만들기
# Heading of the line for making a key for AI programs.
settings-ai-make-key-for = AI 프로그램용 키 만들기
# Added after the words behind the i of its card.
settings-ai-make-key-sub = AI 프로그램이 볼 수 있는 폴더 밖에 따로 보관하는 키예요.
# Heading of the card listing computers AI programs may reach by ssh.
settings-ai-ssh = ssh로 접속할 수 있는 컴퓨터
# Help text of the ssh card (the information button).
settings-ai-ssh-about = 울타리 안에서 ssh는 Trindle를 거쳐 연결돼요. Trindle는 자체 키를 쓰고, 여기 있는 컴퓨터에만 써요. AI 프로그램은 여러분의 키를 절대 볼 수 없어요.
# Tip of the button that stops giving AI programs a key.
settings-ai-stop-key = AI 프로그램에 이 키 주지 않기
# Example in the box for typing a key to give AI programs: a name, an equals sign and the value.
settings-ai-eg-key = NAME=value
# Tip of the button that gives AI programs a key.
settings-ai-give-key = AI 프로그램에 이 키 주기
# Heading of the line for adding a key.
settings-ai-add-key = 키 추가
# Heading of the card listing keys given to AI programs.
settings-ai-keys = AI 프로그램에 준 키
# Help text of the keys card (the information button).
settings-ai-keys-about = AI 프로그램은 세션에 있는 키와 토큰을 받지 못해요. 여기에 입력한 키만 받아요.
# Under the AI programs card, greyed, when none of the listed AI programs is on this computer.
settings-ai-none-installed = 이 컴퓨터에 AI 프로그램이 없어요.
# Under the box, when the program typed in to add is not on this computer.
settings-ai-not-installed = 그 프로그램은 이 컴퓨터에 없어요.
