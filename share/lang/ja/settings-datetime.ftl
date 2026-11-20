# machine translated (Japanese), not yet reviewed by a person.

# Settings: the Date and time page (src/settings/panels/datetime.rs).

## The Date and time page (src/settings/panels/datetime.rs)

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = 日付と時刻
# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = 時刻 タイムゾーン 時計 日付 時間 日時
# Help line at the foot of the Date and time page.
settings-datetime-help = タイムゾーンや自動時刻の変更には、管理者のパスワードが必要です。
# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = 現在の時刻
# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24時間
# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12時間
# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24時間(秒つき)
# Name of the box for the day when the date is set by hand.
settings-datetime-day = 日
# Name of the box for the month when the date is set by hand.
settings-datetime-month = 月
# Name of the box for the year when the date is set by hand.
settings-datetime-year = 年
# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = 時
# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = 分
# Red words when the time could not be set.
settings-datetime-no-time = 時刻を設定できませんでした。
# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = 時計は自動で合わせています。上でオフにしてから、手動で設定してください。
# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = システムバスにつながりませんでした。
# Red words when the time service does not answer.
settings-datetime-not-answering = 時刻サービスが応答していません。
# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = 自動時刻をオンにできませんでした。
# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = 自動時刻をオフにできませんでした。
# Red words when the time zone could not be changed.
settings-datetime-zone-failed = タイムゾーンを変更できませんでした。
# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = 実在しない日付です。
# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = { $date }、{ $time }です。
# Headline of the page when the time service is not answering.
settings-datetime-service-down = 時刻サービスが応答していないため、ここでは時刻を変更できません。
# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = { $date }、{ $time }です。時計は自動で合っています。
# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = { $date }、{ $time }です。コンピューターがインターネットにつながると、時計は自動で合います。
# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = { $date }、{ $time }です。時計は手動で設定されています。
# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }、{ $offset }
# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }、{ $region }、{ $offset }
# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = { $date }、{ $time }、{ $zone }です。
# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = リングの中央の時計は、24時間または12時間で、秒の有無も選べます。変わるのはリングの時計だけです。プログラム内の日付と時刻は、地域と言語で選んだ形式になります。
# Heading of the card with the clock settings.
settings-datetime-the-clock = 時計
# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = 時刻サービスが応答していません
# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = 先にsystemd-timesyncdをインストールしてください。
# Label of the switch for automatic time, and heading of its line.
settings-datetime-set-auto = 時刻を自動で設定
# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = タイムゾーン
# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = 日付と時刻を設定
# Heading of the line for setting the time by hand.
settings-datetime-by-hand = 時刻を手動で設定
# Help text of the clock card (the information button) when the time service can keep the time from the internet.
settings-datetime-about = 自動時刻がオンのときは、コンピューターがつながっている間、時計がインターネットの時刻サーバーに合わせます。オフのときは、日付と時刻を手で入力します。タイムゾーンは、夜間モードが日没を知るためにも使われます。どちらも管理者のパスワードが必要です。リングの中央の時計は、24時間または12時間で、秒の有無も選べます。変わるのはリングの時計だけです。プログラム内の日付と時刻は、地域と言語で選んだ形式になります。
# Help text of the clock card (the information button) when the time service is not installed.
settings-datetime-about-no-ntp = 自動時刻がオンのときは、コンピューターがつながっている間、時計がインターネットの時刻サーバーに合わせます。オフのときは、日付と時刻を手で入力します。タイムゾーンは、夜間モードが日没を知るためにも使われます。どちらも管理者のパスワードが必要です。インターネットから時刻を合わせるには、systemd-timesyncdが必要です: sudo apt install systemd-timesyncd。リングの中央の時計は、24時間または12時間で、秒の有無も選べます。変わるのはリングの時計だけです。プログラム内の日付と時刻は、地域と言語で選んだ形式になります。
# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  ({ $what })
# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = 表示形式
# Heading of the line for the ring's clock.
settings-datetime-rings-clock = リングの時計
