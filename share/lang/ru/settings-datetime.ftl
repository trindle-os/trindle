# machine translated (Russian), not yet reviewed by a person.

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = Дата и время

# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = часовой пояс часы дата время

# Help line at the foot of the Date and time page.
# SAFETY: needs a human review before it ships
settings-datetime-help = Для часового пояса и автоматической установки времени нужен пароль администратора.

# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = Текущее время

# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24 часа

# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12 часов

# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24 часа, с секундами

# Name of the box for the day when the date is set by hand.
settings-datetime-day = День

# Name of the box for the month when the date is set by hand.
settings-datetime-month = Месяц

# Name of the box for the year when the date is set by hand.
settings-datetime-year = Год

# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = Час

# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = Минута

# Red words when the time could not be set.
settings-datetime-no-time = Не удалось установить время.

# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = Сейчас часы подстраиваются сами. Выключите это выше, затем установите время вручную.

# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = Не удалось связаться с системной шиной.

# Red words when the time service does not answer.
settings-datetime-not-answering = Служба времени не отвечает.

# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = Не удалось включить автоматическую подстройку часов.

# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = Не удалось выключить автоматическое время.

# Red words when the time zone could not be changed.
settings-datetime-zone-failed = Не удалось сменить часовой пояс.

# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = Такой даты не существует.

# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = Сейчас { $time }, { $date }.

# Headline of the page when the time service is not answering.
settings-datetime-service-down = Служба времени не отвечает, поэтому часы здесь изменить нельзя.

# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = Сейчас { $time }, { $date }, часы подстраиваются сами.

# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = Сейчас { $time }, { $date }. Часы сверятся, как только появится интернет.

# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = Сейчас { $time }, { $date }. Часы установлены вручную.

# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }, { $offset }

# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }, { $region }, { $offset }

# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = Сейчас { $time }, { $date }, { $zone }.

# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = Часы в середине кольца показывают 24 или 12 часов, с секундами или без. Это относится только к часам кольца; даты и время в программах следуют форматам, выбранным в разделе «Регион и язык».

# Heading of the card with the clock settings.
settings-datetime-the-clock = Часы

# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = Служба времени не отвечает

# Under Set the time automatically when the internet can be used.
settings-datetime-from-internet = Из интернета, когда есть подключение.

# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = Нужна установленная systemd-timesyncd.

# Label of the switch for automatic time, and heading of its line.
# SAFETY: needs a human review before it ships
settings-datetime-set-auto = Устанавливать время автоматически

# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = Часовой пояс

# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = Установить дату и время

# Heading of the line for setting the time by hand.
settings-datetime-by-hand = Установить время вручную

# Under the Set the time by hand heading.
settings-datetime-by-hand-sub = Сначала выключите автоматическое время, как сейчас.

# Help text of the clock card (the information button) when the time service can keep the time from the internet.
# SAFETY: needs a human review before it ships
settings-datetime-about = Если время устанавливается автоматически, часы сверяются с серверами времени в интернете, когда компьютер подключён, и подстраиваются сами. Если выключить, дату и время вводят вручную. По часовому поясу ночной свет узнаёт, когда садится солнце. Для обоих нужен пароль администратора. Часы в середине кольца показывают 24 или 12 часов, с секундами или без. Это относится только к часам кольца; даты и время в программах следуют форматам, выбранным в разделе «Регион и язык».

# Help text of the clock card (the information button) when the time service is not installed.
# SAFETY: needs a human review before it ships
settings-datetime-about-no-ntp = Если время устанавливается автоматически, часы сверяются с серверами времени в интернете, когда компьютер подключён, и подстраиваются сами. Если выключить, дату и время вводят вручную. По часовому поясу ночной свет узнаёт, когда садится солнце. Для обоих нужен пароль администратора. Чтобы брать время из интернета, нужна systemd-timesyncd: sudo apt install systemd-timesyncd Часы в середине кольца показывают 24 или 12 часов, с секундами или без. Это относится только к часам кольца; даты и время в программах следуют форматам, выбранным в разделе «Регион и язык».

# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  ({ $what })

# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = Показывать так

# Heading of the line for the ring's clock.
settings-datetime-rings-clock = Часы кольца

# Under The ring's clock heading.
settings-datetime-rings-clock-sub = Как кольцо показывает время.

