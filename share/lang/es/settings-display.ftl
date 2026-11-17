# machine translated (Spanish), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Pantalla

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = pantalla monitor resolución tamaño escala zoom más grande más pequeño girar rotar frecuencia actualización luz nocturna cálida azul noche ojos

# Introduction under the heading of the display page, and the words behind the page i.
settings-display-help = Los cambios se aplican al instante. Si la pantalla falla, no hagas nada. Vuelve sola al estado anterior a los 15 segundos, a menos que pulses Mantener. La luz nocturna calienta los colores de la pantalla por la noche.

# Question along the bottom of the page while a change of displays waits to be kept.
settings-display-foot-keep = ¿Mantener esta disposición?

# Sentence at the top of the page while the displays are being found.
settings-display-looking = Comprobando las pantallas…

# Sentence at the top of the page when no display was found.
settings-display-none-found = No se encuentra ninguna pantalla.

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = No se usa ninguna pantalla

# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = Se usa una pantalla

# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
        [one] Se usa { $n } pantalla
       *[other] Se usan { $n } pantallas
    }

# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = la luz nocturna está desactivada

# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = la luz nocturna calienta los colores de la pantalla de { $from } a { $to }

# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = la luz nocturna calienta los colores de la pantalla del atardecer al amanecer

# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }; { $warm }.

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = Normal

# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = Girada a la derecha

# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = Girada a la izquierda

# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = Boca abajo

# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = A la derecha de

# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = A la izquierda de

# Choice in the list of where a display sits: above the other.
settings-display-place-above = Encima de

# Choice in the list of where a display sits: below the other.
settings-display-place-below = Debajo de

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }

# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height } a { $hz } Hz

# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (recomendada)

# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz

# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent }%  (estándar)

# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = Resolución de { $display }

# Name of the list of sizes; { $display } is the display.
settings-display-size-on = Tamaño en { $display }

# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = Rotación de { $display }

# Heading of the resolution line.
settings-display-resolution = Resolución

# Heading of the size line.
settings-display-size = Tamaño

# Heading of the rotation line.
settings-display-rotation = Rotación

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Usar esta pantalla

# Small words under that line for the only display in use.
settings-display-use-only = No puedes apagar la única pantalla en uso.

# Words behind the i of a display's card.
settings-display-card-about = Los cambios se aplican al instante. Si la pantalla falla, no hagas nada. Vuelve sola al estado anterior a los 15 segundos, a menos que pulses Mantener. La resolución recomendada es la que corresponde a la pantalla. El tamaño estándar es el que Trindle eligió para esta pantalla.

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = Colocar { $display }

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Colócala

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display } respecto a { $other }

# Title of the card about where displays sit.
settings-display-arrangement-title = Disposición

# Words behind the i of that card.
settings-display-arrangement-about = Define dónde está cada pantalla respecto a las otras, para que el puntero pueda cruzar el borde que comparten.

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Necesitas dos pantallas para colocar una junto a la otra.

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = No se pudo cambiar eso.

# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
        [one] Las pantallas han cambiado. Volverán al estado anterior en { $seconds } segundo, a menos que pulses Mantener.
       *[other] Las pantallas han cambiado. Volverán al estado anterior en { $seconds } segundos, a menos que pulses Mantener.
    }

# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = No se pudo deshacer el cambio. { $why }

# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = La disposición está en uso ahora, pero Trindle no pudo guardarla para el próximo inicio de sesión.

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = No se cambió nada

# Small words under that line.
settings-display-refused-change = No se pudo hacer ese cambio.

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Los cambios se aplican al instante

# Title of the night light card.
settings-display-night-title = Luz nocturna

# Words behind the i of that card.
settings-display-night-about = La pantalla se vuelve más cálida por la noche, lo que cansa menos la vista. Sigue el atardecer y el amanecer, calculados a partir de tu zona horaria, o las horas que fijes tú. Si Trindle no puede saber dónde estás por tu zona horaria, usa las horas que fijaste.

# Heading of the night light line, and name of its switch.
settings-display-night = Luz nocturna

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Atardecer y amanecer en { $zone }.

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Trindle usa horas fijas porque no sabe cuándo atardece.

# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = Del atardecer al amanecer

# Choice in the When list: between times the person sets.
settings-display-when-times = Horas fijadas

# Heading of the When line, and name of its list.
settings-display-when = Cuándo

# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = Desde

# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = Hasta

# Heading of the warmth line, and name of its slider.
settings-display-warmth = Calidez

# Small word under the left end of the warmth slider.
settings-display-little = Poco

# Small word under the right end of the warmth slider.
settings-display-very = Muy cálida

# Spoken description of the strip of colours night light runs through.
settings-display-strip = De poco cálida a muy cálida

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = Ninguna pantalla en uso

# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }

# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
        [one] { $n } pantalla
       *[other] { $n } pantallas, según su disposición
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = Ninguna pantalla

# Name of the large picture of the displays.
settings-display-your-displays = Tus pantallas

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Trindle no encontró ninguna pantalla.

# Title of the card shown when no display was found.
settings-display-none-title = No se encontró ninguna pantalla

# Heading of the line in that card.
settings-display-none-heading = No se encontró ninguna pantalla

# Title of the card about the graphics card.
settings-display-graphics-title = Gráficos

# Words behind the i of that card.
settings-display-graphics-about = Este equipo tiene una tarjeta gráfica NVIDIA que usa el controlador abierto. Funciona, pero una tarjeta de 2022 en adelante se notará lenta, porque el controlador abierto no puede subir la velocidad de la tarjeta. En su lugar, puedes instalar el controlador propio de NVIDIA.

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = Tarjeta NVIDIA con el controlador abierto

# Small words under that line.
settings-display-nvidia-about = Las tarjetas de 2022 en adelante pueden ir lentas.

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = Qué hacer al respecto
