# machine translated (Korean), not yet reviewed by a person.

# Settings, Bluetooth page: the adapter, pairing, connecting and forgetting devices.

## Bluetooth page (src/settings/panels/bluetooth.rs)

# Said in red under the Bluetooth switch when switching it on failed for no known reason.
settings-bluetooth-on-failed = Bluetooth를 켜지 못했어요.
# Said in red under the Bluetooth switch when switching it off failed for no known reason.
settings-bluetooth-off-failed = Bluetooth를 끄지 못했어요.
# Said in red when Bluetooth refuses something because it is not switched on; the first sentence says what could not be done.
settings-bluetooth-cause-not-ready = { $could_not } Bluetooth가 꺼져 있어요.
# Said in red when the PIN or number typed was wrong; the first sentence says what could not be done.
settings-bluetooth-cause-auth-failed = { $could_not } PIN이나 숫자가 맞지 않았어요. 다시 해 보세요.
# Said in red when the pairing was turned down; the first sentence says what could not be done.
settings-bluetooth-cause-turned-down = { $could_not } 여기서 또는 상대 장치에서 페어링을 거절했어요.
# Said in red when the other device did not answer in time; the first sentence says what could not be done.
settings-bluetooth-cause-timeout = { $could_not } 상대 장치가 제때 응답하지 않았어요. 페어링 모드로 바꾼 뒤 다시 해 보세요.
# Said in red when the same thing is already being done; the first sentence says what could not be done.
settings-bluetooth-cause-in-progress = { $could_not } Trindle이 이미 이 작업을 하고 있어요. 잠시 기다려 주세요.
# Said in red when the device did not answer; the first sentence says what could not be done.
settings-bluetooth-cause-no-answer = { $could_not } 장치가 응답하지 않았어요. 장치가 켜져 있고 가까이 있는지 확인한 뒤 다시 해 보세요.
# Said in red when the device is not ready to be used; the first sentence says what could not be done.
settings-bluetooth-cause-not-available = { $could_not } 장치를 쓸 준비가 안 됐어요. 페어링 모드로 바꾼 뒤 다시 해 보세요.
# The word for the system bus in "The system bus is not available".
settings-bluetooth-the-bus = 시스템 버스
# Headline of the page while Bluetooth is looked for.
settings-bluetooth-top-looking = Bluetooth를 확인하는 중…
# Headline of the page when the Bluetooth service is not running.
settings-bluetooth-top-gone = 이 컴퓨터에서 Bluetooth가 실행되고 있지 않아요.
# Headline of the page when Bluetooth is off.
settings-bluetooth-top-off = Bluetooth가 꺼져 있어요.
# Headline of the page when Bluetooth is off at a switch or key on the computer itself.
settings-bluetooth-top-hard = 컴퓨터의 스위치나 키로 Bluetooth가 꺼져 있어요.
# Headline of the page when the computer has no Bluetooth.
settings-bluetooth-top-none = 이 컴퓨터에는 Bluetooth가 없어요.
# Headline of the page when the computer has an adapter that Bluetooth has not finished setting up.
settings-bluetooth-top-starting = Bluetooth를 시작하는 중…
# Headline of the page when the adapter was there a moment ago and has gone.
settings-bluetooth-top-lost = Bluetooth 어댑터가 사라졌어요.
# Headline of the page when Bluetooth is on and searching.
settings-bluetooth-top-searching = 켜져 있고 장치를 찾는 중…
# Headline of the page when Bluetooth is on and nothing is connected.
settings-bluetooth-top-idle = 켜져 있지만 연결된 것이 없어요.
# Headline of the page when Bluetooth is on and devices are connected; the names are already joined into a list.
settings-bluetooth-top-connected = { $n ->
       *[other] 켜져 있고 { $names }이(가) 연결돼 있어요.
    }
# Help text (i button) of the Bluetooth card.
settings-bluetooth-card-about = 스위치 아래 이름은 내 Bluetooth 어댑터의 이름이에요. 다른 장치에 보이기를 켜면 근처 장치가 이 컴퓨터를 찾을 수 있어요.
# Heading of the Bluetooth card, of its switch's line, and the name of its switch.
settings-bluetooth-title = Bluetooth
# Heading of the line with the switch that makes this computer visible, and the name of that switch.
settings-bluetooth-visible = 다른 장치에 보이기
# Heading of the card when Bluetooth is off at a switch on the computer itself.
settings-bluetooth-hard-line = 컴퓨터 자체에서 꺼져 있어요
# Small line under it.
settings-bluetooth-hard-line-sub = 켜려면 컴퓨터의 스위치나 키를 쓰세요.
# Heading of the card when the computer has no Bluetooth adapter.
settings-bluetooth-no-adapter = Bluetooth 어댑터 없음
# Help text (i button) of that card.
settings-bluetooth-no-adapter-about = Trindle이 이 컴퓨터에서 Bluetooth 어댑터를 찾지 못했어요. USB 단자에 어댑터를 꽂으면 바로 여기에 나타나요.
# Line in that card.
settings-bluetooth-none-found = 찾지 못함
# Line in the no-adapter card while Bluetooth finishes setting up the adapter.
settings-bluetooth-starting = 시작하는 중
# Line in the no-adapter card when the adapter was there a moment ago.
settings-bluetooth-lost = 어댑터가 사라졌어요
# Small line under it.
settings-bluetooth-lost-sub = Trindle이 계속 찾고, 돌아오면 여기에 보여 줘요. 돌아오지 않으면 컴퓨터를 다시 시작하세요.
# Shown beside the Bluetooth switch while the password is asked for.
settings-bluetooth-waiting = 비밀번호를 기다리는 중…
# Said in red when this computer could not be made visible to other devices.
settings-bluetooth-visible-failed = Trindle이 이 컴퓨터를 다른 장치에 보이게 하지 못했어요.
# Said in red when this computer could not be hidden from other devices.
settings-bluetooth-hidden-failed = Trindle이 이 컴퓨터를 다른 장치에서 숨기지 못했어요.
# Shown on the two device lists while Bluetooth is off.
settings-bluetooth-switch-on-first = 이것들을 쓰려면 Bluetooth를 켜세요.
# Line in the My devices card when nothing is paired.
settings-bluetooth-none-paired = 아직 페어링한 장치가 없어요
# Heading of the card that lists paired devices.
settings-bluetooth-mine = 내 장치
# Help text (i button) of the My devices card.
settings-bluetooth-mine-about = 이 컴퓨터와 페어링한 장치예요. 배터리 잔량을 읽을 수 있으면 함께 보여 줘요. 페이지를 열 때 연결돼 있던 장치가 먼저 나오고, 페이지를 열어 둔 동안은 순서가 바뀌지 않아요. 장치를 삭제하면 다시 페어링해야 해요.
# Line in the Nearby card while the search runs.
settings-bluetooth-looking = 장치를 찾는 중…
# Tip of the button that stops the search.
settings-bluetooth-stop-looking = 찾기 중지
# Heading of the line that starts the search, and tip of its button.
settings-bluetooth-look = 장치 찾기
# Heading of the card that lists devices nearby.
settings-bluetooth-nearby = 근처
# Help text (i button) of the Nearby card.
settings-bluetooth-nearby-about = 장치 찾기를 누르고, 상대 장치를 페어링 모드로 바꾼 뒤, 그 옆의 페어링을 누르세요. 페이지를 열면 신호가 가장 센 장치가 먼저 나오고, 새 장치는 끝에 추가돼요. 일부 장치는 여기 숫자와 맞는지 확인할 숫자를 보여 줘요. 일부는 PIN을 물어요. PIN은 설명서에 있는 경우가 많고, 아니면 0000이에요.
# Said in red when the search could not be stopped.
settings-bluetooth-stop-failed = Trindle이 장치 찾기를 중지하지 못했어요.
# Said in red when the search could not be started.
settings-bluetooth-start-failed = 장치를 찾지 못했어요.
# Small line of a device that is connected.
settings-bluetooth-connected = 연결됨
# Small line of a paired device that is not connected.
settings-bluetooth-not-connected = 연결 안 됨
# Part of a device's small line: its battery level in per cent.
settings-bluetooth-battery = 배터리 { $n }%
# Tip of the button that pairs a device, and the button of the same name in the pairing question.
settings-bluetooth-pair = 페어링
# Tip of the button that disconnects a device.
settings-bluetooth-disconnect = 연결 끊기
# Tip of the button that connects a device.
settings-bluetooth-connect = 연결
# Tip of the bin button that forgets a device.
settings-bluetooth-forget = 이 장치 잊기
# Said in red when a device could not be disconnected.
settings-bluetooth-disconnect-failed = 연결을 끊지 못했어요.
# Said in red when a device could not be forgotten.
settings-bluetooth-forget-failed = 잊지 못했어요.
# Shown under a device while it is being paired.
settings-bluetooth-pairing = 페어링하는 중…
# Said in red when a device could not be paired.
settings-bluetooth-pair-failed = 페어링하지 못했어요.
# Shown under a device while it is being connected.
settings-bluetooth-connecting = 연결하는 중…
# Said in red when a device could not be connected.
settings-bluetooth-connect-failed = 연결하지 못했어요.
# Said under a device that paired but could not be set to connect by itself.
settings-bluetooth-not-trusted = 연결됐지만 Trindle이 다음에 자동으로 연결되게 설정하지 못했어요. 쓰고 싶을 때 여기서 연결하세요.
# Said under a device that shows a code to be typed on it; the code is already written.
settings-bluetooth-type-code = 장치에 { $code }을(를) 입력한 뒤 장치에서 Enter를 누르세요.
# Question under a device that shows a number to compare with the one on this computer.
settings-bluetooth-compare = 상대 장치에 { $code }이(가) 보이나요?
# Tip of the button that answers no to a pairing question.
settings-bluetooth-no = 아니요
# Tip of the button that says the numbers match.
settings-bluetooth-yes-matches = 네, 같아요
# Question under a device that asks to be paired.
settings-bluetooth-pair-question = 이 장치와 페어링할까요?
# Placeholder of the box where a device's PIN is typed.
settings-bluetooth-pin = PIN
# Tip of the button that refuses to type a PIN.
settings-bluetooth-cancel = 취소
# Help line above the box where a device's PIN is typed.
settings-bluetooth-pin-help = 이 장치의 PIN을 입력하세요. 설명서에 있는 경우가 많아요. 없으면 0000을 해 보세요.
# Words the settings search looks for on this page (not shown).
settings-bluetooth-keywords = headphones mouse keyboard pair devices wireless 헤드폰 마우스 키보드 페어링 장치 무선
# Help text at the top of the page.
settings-bluetooth-help = 장치 찾기를 누르고, 상대 장치를 페어링 모드로 바꾼 뒤, 그 옆의 페어링을 누르세요.

# Said in red under the Bluetooth switch when restarting the sound service after Bluetooth came back on failed; $why is the system's own reason.
settings-bluetooth-sound-failed = Trindle이 Bluetooth 소리를 다시 시작하지 못해서 다시 로그인할 때까지 헤드폰 소리가 평소보다 나쁠 수 있어요. { $why }
# Tooltip of the help button beside the Bluetooth switch.
settings-bluetooth-sound-help = Bluetooth를 껐다 켜면 헤드폰 소리가 나빠지는 이유
# Said in red when a device is no longer in BlueZ's list; the first sentence says what could not be done.
settings-bluetooth-cause-gone = { $could_not } 이 장치는 더 이상 이 컴퓨터에 저장되어 있지 않아요. 다시 찾아보세요.
# Said in red when a paired device's saved pairing is refused; the first sentence says what could not be done.
settings-bluetooth-cause-old-pairing = { $could_not } 장치가 저장된 페어링을 더 이상 받아들이지 않아요. 장치를 잊고 다시 페어링하세요.
# Said in red under a device that connected and then dropped at once.
settings-bluetooth-dropped = 연결됐다가 바로 끊어졌어요. 저장된 페어링이 오래됐을 수 있어요.
# Tooltip of the button that forgets a paired device and looks for it afresh.
settings-bluetooth-pair-again = 잊고 다시 페어링
# Said under Look for devices after a device was forgotten to be paired afresh.
settings-bluetooth-forgotten-pair = 장치를 잊었어요. 페어링 모드로 바꾼 뒤 아래에 나타나면 페어링을 누르세요.
# Tooltip of the button that opens Help from a failed connection.
settings-bluetooth-more = 자세히 보기
