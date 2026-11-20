# machine translated (Korean), not yet reviewed by a person.

# Settings, Printers page, and the words of the CUPS helper (src/common/cups.rs) that the print window of the screenshot program also shows.

## Printers page (src/settings/panels/printers.rs)

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = 아직 프린터가 없어요. Trindle이 네트워크에서 찾은 프린터는 아래에 표시돼요.
# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } 프린터가 일시 중지돼서 지금은 인쇄할 수 없어요.
# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } 프린터가 인쇄하고 있어요.
# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = { $name } 프린터로 인쇄할 준비가 됐어요.
# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } 프린터 상태: { $state }.
# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } 프린터가 설정돼 있지만 기본 프린터를 고르지 않았어요.
# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = 프린터 { $n }대가 설정돼 있지만 기본 프린터를 고르지 않았어요.
# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = 그 작업은 더 이상 없어요.
# Shown beside a print job after it was cancelled.
settings-printers-cancelled = 취소됨
# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = 확인하지 못했어요.
# Red words when Trindle cannot reach CUPS at all, after trying twice.
printers-unreachable = 인쇄 서비스가 실행 중이 아니거나 연결할 수 없어요.
# Words in the ring while the network is searched for printers.
settings-printers-searching = 찾는 중
# Words in the ring when the search is finished.
settings-printers-search-done = 완료
# Shown beside a printer after it was made the one used first.
settings-printers-default-now = 이제 기본
# Shown beside a printer after a test page was sent.
settings-printers-sent = 보냈어요
# Shown beside a printer after it was removed.
settings-printers-removed = 삭제됨
# Red words under the By its address line when nothing was typed.
settings-printers-type-address = 먼저 프린터 주소를 입력하세요.
# Shown under the By its address line while the printer is added.
settings-printers-adding = 추가하는 중…
# Shown beside a printer or the address line after it was added.
settings-printers-added = 추가됨
# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = 드라이버를 찾는 중…
# Heading of a print job in the queue: its number and the printer.
settings-printers-job = { $printer }의 작업 { $number }
# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }
# Tip of the cross button that cancels a print job.
settings-printers-cancel-job = 이 작업 취소
# Heading of the card that lists waiting print jobs.
settings-printers-jobs = 인쇄 작업
# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = 모든 프린터에서 인쇄를 기다리는 작업이에요. 이 페이지가 열려 있는 동안 Trindle이 몇 초마다 목록을 새로 고쳐요. 이미 인쇄된 페이지는 취소할 수 없어요.
# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS가 응답하지 않아요
# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = 대기열을 읽지 못했어요.
# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = 찾는 중…
# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = 기다리는 작업 없음
# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = 일시 중지됨
# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = 대기 중
# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = 인쇄 중
# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = 기본, { $state }
# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = 기본 프린터로 지정
# Tip of the button that prints a test page.
settings-printers-test-page = 테스트 페이지 인쇄
# Tip of the bin button that removes a printer.
settings-printers-remove = 이 프린터 삭제
# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = 검색하려면 cups-filters가 필요해요
# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = 설치하려면 이 명령을 실행하세요: sudo apt install cups-filters
# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = 이 프린터 추가
# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = 새로 찾은 것이 없어요
# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50
# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = 추가
# Heading of the line where a printer's address is typed.
settings-printers-by-address = 주소로 추가
# Heading of the card for adding printers.
settings-printers-add-card = 프린터 추가
# Help text (i button) of the Add a printer card.
settings-printers-add-about = Trindle은 먼저 스스로 알리는 프린터를 찾고, 그다음 네트워크의 각 장치를 차례로 확인해요. 프린터는 켜져 있어야 하고 같은 네트워크에 있어야 해요. 스스로 알리지 않는 프린터도 있고, 알림을 전달하지 않는 네트워크도 있어요. 그럴 때는 192.168.1.50 같은 주소로 프린터를 추가하세요. 주소는 프린터 화면의 네트워크 설정에서 볼 수 있어요.
# Name of the page in the list of settings pages.
settings-printers-title = 프린터
# Words the settings search looks for on this page (not shown).
settings-printers-keywords = 프린터 인쇄 스캐너 printer print scanner
# Help text at the top of the page.
settings-printers-help = Trindle은 네트워크의 프린터를 스스로 찾아요. 사용하려면 프린터 옆의 프린터 추가 버튼을 누르세요.
# Headline of the page when printing is not installed.
settings-printers-top-not-installed = 이 컴퓨터에는 인쇄가 설치되어 있지 않아요.
# Headline of the page when printing has not answered yet.
settings-printers-top-looking = 찾는 중…
# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Trindle이 인쇄 서비스에 연결할 수 없어서 지금은 인쇄할 수 없어요.
# Heading of the card shown when printing is not installed.
settings-printers-printing = 인쇄
# Help text (i button) of that card.
settings-printers-printing-about = 인쇄하려면 CUPS가 필요한데 이 컴퓨터에 없어요. 설치하면 Trindle이 네트워크의 프린터를 스스로 찾아요.
# Line in that card.
settings-printers-unavailable = 인쇄를 사용할 수 없어요
# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = 설치하려면 이 명령을 실행하세요: sudo apt install cups
# Line in that card.
settings-printers-then-found = 프린터 찾기
# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = 인쇄 서비스
# Line in the Printers card when there are none.
settings-printers-none = 아직 프린터가 없어요
# Heading of the card that lists the printers.
settings-printers-card = 프린터
# Help text (i button) of the Printers card.
settings-printers-card-about = 이 컴퓨터에 설정된 모든 프린터예요. 변경하려면 계정이 lpadmin 그룹에 있어야 해요.

## The CUPS helper (src/common/cups.rs)

# Red words under a page range box that cannot be used.
printers-bad-range = 예: 1-3, 5
# Red words under a copies box that cannot be used.
printers-bad-copies = 1부에서 99부까지.
# Name of the choice to print on one side of the paper.
printers-sides-one = 한 면
# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = 양면, 긴 쪽 넘김
# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = 양면, 짧은 쪽 넘김
# Name of the A4 paper size.
printers-paper-a4 = A4
# Name of the A3 paper size.
printers-paper-a3 = A3
# Name of the A5 paper size.
printers-paper-a5 = A5
# Name of the US Letter paper size.
printers-paper-letter = Letter
# Name of the US Legal paper size.
printers-paper-legal = Legal
# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = 세로
# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = 가로
# Red words when a printing error is a refusal because the account is not in the lpadmin group.
printers-not-allowed = 이 작업은 허용되지 않아요. 계정이 lpadmin 그룹에 있어야 해요. 그런 다음 로그아웃했다가 다시 로그인하세요.
# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS가 거부했어요.
# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = 그 프린터 이름은 사용할 수 없어요.
# Red words when printing is not installed on the computer.
printers-not-installed = 이 컴퓨터에는 인쇄가 설치되어 있지 않아요.
# Red words when the printer chosen has been removed since.
printers-gone = { $name } 프린터는 더 이상 설정되어 있지 않아요.
# Red words when the printer chosen is paused.
printers-paused = { $name } 프린터가 일시 중지됐어요.
# Red words when sending the document to CUPS took too long.
printers-too-slow = 문서를 보내는 데 너무 오래 걸렸어요.
# Red words when a print job could not be cancelled.
settings-printers-not-cancelled = Trindle이 작업을 취소하지 못했어요.
# Red words when a printer could not be made the default.
settings-printers-not-default = Trindle이 기본 프린터로 지정하지 못했어요.
# Red words when the test page could not be sent.
settings-printers-not-sent = Trindle이 테스트 페이지를 보내지 못했어요.
# Red words when a printer could not be removed.
settings-printers-not-removed = Trindle이 프린터를 삭제하지 못했어요.
# Red words when a printer could not be added.
settings-printers-not-added = Trindle이 프린터를 추가하지 못했어요.
# Red words when the list of printers could not be read.
settings-printers-not-read = Trindle이 프린터 목록을 읽지 못했어요.
