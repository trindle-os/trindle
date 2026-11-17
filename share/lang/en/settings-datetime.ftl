# Settings: the Date and time page (src/settings/panels/datetime.rs).

## The Date and time page (src/settings/panels/datetime.rs)

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = Date and time
# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = time zone clock date
# Help line at the foot of the Date and time page.
settings-datetime-help = Changing the time zone or automatic time needs an administrator's password.
# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = The time now
# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24-hour
# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12-hour
# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24-hour, with seconds
# Name of the box for the day when the date is set by hand.
settings-datetime-day = Day
# Name of the box for the month when the date is set by hand.
settings-datetime-month = Month
# Name of the box for the year when the date is set by hand.
settings-datetime-year = Year
# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = Hour
# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = Minute
# Red words when the time could not be set.
settings-datetime-no-time = Couldn't set the time.
# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = The clock sets itself automatically. Switch that off above, then set the time by hand.
# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = Couldn't reach the system bus.
# Red words when the time service does not answer.
settings-datetime-not-answering = The time service isn't responding.
# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = Couldn't switch on automatic time.
# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = Couldn't switch off automatic time.
# Red words when the time zone could not be changed.
settings-datetime-zone-failed = Couldn't change the time zone.
# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = That isn't a real date.
# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = It is { $time } on { $date }.
# Headline of the page when the time service is not answering.
settings-datetime-service-down = The time service isn't responding, so you can't change the time here.
# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = It is { $time } on { $date }. The clock sets itself automatically.
# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = It is { $time } on { $date }. The clock will set itself once the computer is connected to the internet.
# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = It is { $time } on { $date }. The clock is set by hand.
# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }, { $offset }
# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }, { $region }, { $offset }
# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = It is { $time } on { $date }, { $zone }.
# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = The clock in the middle of the ring can show 24 or 12 hours, with or without seconds. This only changes the ring's clock. Dates and times in programs follow the formats you choose in Region and language.
# Heading of the card with the clock settings.
settings-datetime-the-clock = The clock
# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = The time service isn't responding
# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = Install systemd-timesyncd first.
# Label of the switch for automatic time, and heading of its line.
settings-datetime-set-auto = Set the time automatically
# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = Time zone
# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = Set the date and time
# Heading of the line for setting the time by hand.
settings-datetime-by-hand = Set the time by hand
# Help text of the clock card (the information button) when the time service can keep the time from the internet.
settings-datetime-about = When automatic time is on, the clock checks itself against time servers on the internet whenever the computer is connected. When it is off, you type in the date and time by hand. The time zone also tells night light when the sun sets. Both need an administrator's password. The clock in the middle of the ring can show 24 or 12 hours, with or without seconds. This only changes the ring's clock. Dates and times in programs follow the formats you choose in Region and language.
# Help text of the clock card (the information button) when the time service is not installed.
settings-datetime-about-no-ntp = When automatic time is on, the clock checks itself against time servers on the internet whenever the computer is connected. When it is off, you type in the date and time by hand. The time zone also tells night light when the sun sets. Both need an administrator's password. To set the time from the internet, you need systemd-timesyncd: sudo apt install systemd-timesyncd. The clock in the middle of the ring can show 24 or 12 hours, with or without seconds. This only changes the ring's clock. Dates and times in programs follow the formats you choose in Region and language.
# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  ({ $what })
# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = Show it as
# Heading of the line for the ring's clock.
settings-datetime-rings-clock = The ring's clock
