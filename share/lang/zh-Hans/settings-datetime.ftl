
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = 日期和时间

# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = 时区 时钟 日期 时间 time zone clock date

# Help line at the foot of the Date and time page.
# SAFETY: needs a human review before it ships
settings-datetime-help = 时区和自动时间需要管理员密码。

# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = 当前时间

# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24 小时制

# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12 小时制

# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24 小时制，含秒

# Name of the box for the day when the date is set by hand.
settings-datetime-day = 日

# Name of the box for the month when the date is set by hand.
settings-datetime-month = 月

# Name of the box for the year when the date is set by hand.
settings-datetime-year = 年

# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = 时

# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = 分

# Red words when the time could not be set.
settings-datetime-no-time = 无法设置时间。

# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = 时钟目前会自动校准。请先在上方关闭自动时间，再手动设置时间。

# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = 无法连接系统总线。

# Red words when the time service does not answer.
settings-datetime-not-answering = 时间服务没有响应。

# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = 无法让时钟自动校准。

# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = 无法关闭自动时间。

# Red words when the time zone could not be changed.
settings-datetime-zone-failed = 无法更改时区。

# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = 这不是一个有效的日期。

# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = 现在是 { $date } { $time }。

# Headline of the page when the time service is not answering.
settings-datetime-service-down = 时间服务没有响应，因此无法在这里更改时钟。

# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = 现在是 { $date } { $time }，时钟会自动校准。

# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = 现在是 { $date } { $time }。连接到互联网后，时钟会自行校准。

# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = 现在是 { $date } { $time }。时钟为手动设置。

# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }，{ $offset }

# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }，{ $region }，{ $offset }

# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = 现在是 { $date } { $time }，{ $zone }。

# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = 圆环中间的时钟可以用 24 或 12 小时制显示，可带秒或不带秒。只有圆环的时钟遵循此设置；程序中的日期和时间遵循“地区和语言”中选择的格式。

# Heading of the card with the clock settings.
settings-datetime-the-clock = 时钟

# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = 时间服务没有响应

# Under Set the time automatically when the internet can be used.
settings-datetime-from-internet = 联网时从互联网获取。

# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = 需要安装 systemd-timesyncd。

# Label of the switch for automatic time, and heading of its line.
settings-datetime-set-auto = 自动设置时间

# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = 时区

# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = 设置日期和时间

# Heading of the line for setting the time by hand.
settings-datetime-by-hand = 手动设置时间

# Under the Set the time by hand heading.
settings-datetime-by-hand-sub = 请先关闭自动时间，目前它是开启的。

# Help text of the clock card (the information button) when the time service can keep the time from the internet.
# SAFETY: needs a human review before it ships
settings-datetime-about = 设为自动后，只要电脑联网，时钟就会与互联网上的时间服务器核对并自动校准。关闭后，日期和时间需要手动输入。时区还会告诉夜间模式太阳何时落山。两者都需要管理员密码。圆环中间的时钟可以用 24 或 12 小时制显示，可带秒或不带秒。只有圆环的时钟遵循此设置；程序中的日期和时间遵循“地区和语言”中选择的格式。

# Help text of the clock card (the information button) when the time service is not installed.
# SAFETY: needs a human review before it ships
settings-datetime-about-no-ntp = 设为自动后，只要电脑联网，时钟就会与互联网上的时间服务器核对并自动校准。关闭后，日期和时间需要手动输入。时区还会告诉夜间模式太阳何时落山。两者都需要管理员密码。通过互联网校准时间需要 systemd-timesyncd：sudo apt install systemd-timesyncd 圆环中间的时钟可以用 24 或 12 小时制显示，可带秒或不带秒。只有圆环的时钟遵循此设置；程序中的日期和时间遵循“地区和语言”中选择的格式。

# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  （{ $what }）

# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = 显示为

# Heading of the line for the ring's clock.
settings-datetime-rings-clock = 圆环的时钟

# Under The ring's clock heading.
settings-datetime-rings-clock-sub = 圆环显示时间的方式。

