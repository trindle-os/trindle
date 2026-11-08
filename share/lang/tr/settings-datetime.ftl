
# machine translated (Turkish), not yet reviewed by a person.

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = Tarih ve saat

# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = saat dilimi saat tarih zaman

# Help line at the foot of the Date and time page.
# SAFETY: needs a human review before it ships
settings-datetime-help = Saat dilimi ve otomatik saat için yöneticinin parolası gerekir.

# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = Şimdiki saat

# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24 saat

# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12 saat

# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24 saat, saniyeli

# Name of the box for the day when the date is set by hand.
settings-datetime-day = Gün

# Name of the box for the month when the date is set by hand.
settings-datetime-month = Ay

# Name of the box for the year when the date is set by hand.
settings-datetime-year = Yıl

# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = Saat

# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = Dakika

# Red words when the time could not be set.
settings-datetime-no-time = Saat ayarlanamadı.

# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = Saat şu anda kendini doğru tutuyor. Önce yukarıdan bunu kapatın, sonra saati elle ayarlayın.

# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = Sistem veriyoluna ulaşılamadı.

# Red words when the time service does not answer.
settings-datetime-not-answering = Saat hizmeti yanıt vermiyor.

# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = Saatin kendini doğru tutması sağlanamadı.

# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = Otomatik saat kapatılamadı.

# Red words when the time zone could not be changed.
settings-datetime-zone-failed = Saat dilimi değiştirilemedi.

# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = Bu geçerli bir tarih değil.

# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = { $date } tarihinde saat { $time }.

# Headline of the page when the time service is not answering.
settings-datetime-service-down = Saat hizmeti yanıt vermiyor, bu yüzden saat buradan değiştirilemez.

# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = { $date } tarihinde saat { $time } ve saat kendini doğru tutuyor.

# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = { $date } tarihinde saat { $time }. Saat, internete ulaşınca kendini denetler.

# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = { $date } tarihinde saat { $time }. Saat elle ayarlanmış.

# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }, { $offset }

# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }, { $region }, { $offset }

# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = { $date } tarihinde saat { $time }, { $zone }.

# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = Halkanın ortasındaki saat 24 ya da 12 saatlik biçimde, saniyeli ya da saniyesiz yazılır. Yalnızca halkanın saati buna uyar; programlardaki tarih ve saatler Bölge ve dil bölümünde seçilen biçimlere uyar.

# Heading of the card with the clock settings.
settings-datetime-the-clock = Saat

# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = Saat hizmeti yanıt vermiyor

# Under Set the time automatically when the internet can be used.
settings-datetime-from-internet = İnternetten, bağlıyken.

# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = systemd-timesyncd kurulu olmalı.

# Label of the switch for automatic time, and heading of its line.
settings-datetime-set-auto = Saati otomatik ayarla

# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = Saat dilimi

# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = Tarihi ve saati ayarla

# Heading of the line for setting the time by hand.
settings-datetime-by-hand = Saati elle ayarla

# Under the Set the time by hand heading.
settings-datetime-by-hand-sub = Önce otomatik saati kapatın, şu anda olduğu gibi.

# Help text of the clock card (the information button) when the time service can keep the time from the internet.
# SAFETY: needs a human review before it ships
settings-datetime-about = Otomatik ayarlandığında saat, bilgisayar bağlı olduğu sürece internetteki zaman sunucularıyla denetlenir ve kendini doğru tutar. Kapatıldığında tarih ve saat elle yazılır. Saat dilimi, gece ışığına güneşin ne zaman battığını da söyler. İkisi için de yöneticinin parolası gerekir. Halkanın ortasındaki saat 24 ya da 12 saatlik biçimde, saniyeli ya da saniyesiz yazılır. Yalnızca halkanın saati buna uyar; programlardaki tarih ve saatler Bölge ve dil bölümünde seçilen biçimlere uyar.

# Help text of the clock card (the information button) when the time service is not installed.
# SAFETY: needs a human review before it ships
settings-datetime-about-no-ntp = Otomatik ayarlandığında saat, bilgisayar bağlı olduğu sürece internetteki zaman sunucularıyla denetlenir ve kendini doğru tutar. Kapatıldığında tarih ve saat elle yazılır. Saat dilimi, gece ışığına güneşin ne zaman battığını da söyler. İkisi için de yöneticinin parolası gerekir. Saati internetten tutmak için systemd-timesyncd gerekir: sudo apt install systemd-timesyncd Halkanın ortasındaki saat 24 ya da 12 saatlik biçimde, saniyeli ya da saniyesiz yazılır. Yalnızca halkanın saati buna uyar; programlardaki tarih ve saatler Bölge ve dil bölümünde seçilen biçimlere uyar.

# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  ({ $what })

# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = Şöyle göster

# Heading of the line for the ring's clock.
settings-datetime-rings-clock = Halkanın saati

# Under The ring's clock heading.
settings-datetime-rings-clock-sub = Halkanın saati nasıl yazacağı.

