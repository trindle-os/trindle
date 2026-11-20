# machine translated (Korean), not yet reviewed by a person.

# Settings: the Date and time page (src/settings/panels/datetime.rs).

## The Date and time page (src/settings/panels/datetime.rs)

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = 날짜 및 시간
# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = 시간대 시계 날짜 시간 time zone clock date
# Help line at the foot of the Date and time page.
settings-datetime-help = 시간대나 자동 시간을 바꾸려면 관리자 비밀번호가 필요해요.
# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = 현재 시각
# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24시간제
# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12시간제
# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24시간제, 초 포함
# Name of the box for the day when the date is set by hand.
settings-datetime-day = 일
# Name of the box for the month when the date is set by hand.
settings-datetime-month = 월
# Name of the box for the year when the date is set by hand.
settings-datetime-year = 년
# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = 시
# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = 분
# Red words when the time could not be set.
settings-datetime-no-time = 시간을 설정할 수 없었어요.
# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = 시계가 자동으로 맞춰져요. 위에서 자동 설정을 끄고 시간을 직접 설정하세요.
# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = 시스템 버스에 연결할 수 없었어요.
# Red words when the time service does not answer.
settings-datetime-not-answering = 시간 서비스가 응답하지 않아요.
# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = 자동 시간을 켤 수 없었어요.
# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = 자동 시간을 끌 수 없었어요.
# Red words when the time zone could not be changed.
settings-datetime-zone-failed = 시간대를 바꿀 수 없었어요.
# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = 존재하지 않는 날짜예요.
# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = { $date } { $time }예요.
# Headline of the page when the time service is not answering.
settings-datetime-service-down = 시간 서비스가 응답하지 않아서 여기서 시간을 바꿀 수 없어요.
# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = { $date } { $time }예요. 시계가 자동으로 맞춰져요.
# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = { $date } { $time }예요. 컴퓨터가 인터넷에 연결되면 시계가 자동으로 맞춰져요.
# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = { $date } { $time }예요. 시계를 직접 설정했어요.
# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }, { $offset }
# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }, { $region }, { $offset }
# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = { $zone }, { $date } { $time }예요.
# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = 가운데 링의 시계는 24시간제나 12시간제로, 초를 넣거나 빼고 보여 줄 수 있어요. 링의 시계만 바뀌어요. 프로그램의 날짜와 시간은 지역 및 언어에서 고른 형식을 따라요.
# Heading of the card with the clock settings.
settings-datetime-the-clock = 시계
# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = 시간 서비스가 응답하지 않아요
# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = 먼저 systemd-timesyncd를 설치하세요.
# Label of the switch for automatic time, and heading of its line.
settings-datetime-set-auto = 시간을 자동으로 설정
# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = 시간대
# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = 날짜와 시간 설정
# Heading of the line for setting the time by hand.
settings-datetime-by-hand = 시간을 직접 설정
# Help text of the clock card (the information button) when the time service can keep the time from the internet.
settings-datetime-about = 자동 시간을 켜면 컴퓨터가 연결될 때마다 인터넷의 시간 서버로 시계를 맞춰요. 끄면 날짜와 시간을 직접 입력해요. 시간대는 야간 모드가 해가 지는 때를 아는 데도 쓰여요. 둘 다 관리자 비밀번호가 필요해요. 가운데 링의 시계는 24시간제나 12시간제로, 초를 넣거나 빼고 보여 줄 수 있어요. 링의 시계만 바뀌어요. 프로그램의 날짜와 시간은 지역 및 언어에서 고른 형식을 따라요.
# Help text of the clock card (the information button) when the time service is not installed.
settings-datetime-about-no-ntp = 자동 시간을 켜면 컴퓨터가 연결될 때마다 인터넷의 시간 서버로 시계를 맞춰요. 끄면 날짜와 시간을 직접 입력해요. 시간대는 야간 모드가 해가 지는 때를 아는 데도 쓰여요. 둘 다 관리자 비밀번호가 필요해요. 인터넷으로 시간을 맞추려면 systemd-timesyncd가 필요해요: sudo apt install systemd-timesyncd. 가운데 링의 시계는 24시간제나 12시간제로, 초를 넣거나 빼고 보여 줄 수 있어요. 링의 시계만 바뀌어요. 프로그램의 날짜와 시간은 지역 및 언어에서 고른 형식을 따라요.
# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  ({ $what })
# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = 표시 형식
# Heading of the line for the ring's clock.
settings-datetime-rings-clock = 링의 시계
