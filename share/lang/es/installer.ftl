
# machine translated (Spanish), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = Instalar { $name }

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Bienvenida

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Disco

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Privacidad

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Contraseña

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = Tú

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Listo

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Instalando

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Hecho

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Preparando

# Red line at the foot when the installer's helper could not be started.
installer-no-start = El instalador no pudo iniciarse: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = No se pudo leer el plan.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = No se pudo leer el plan: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = La instalación se detuvo. No se ha escrito nada más.

# Name of the round button at the foot that goes back a page.
installer-back = Atrás

# Name of the main button at the foot that goes to the next page.
installer-next = Siguiente

# SAFETY: needs a human review before it ships
# Name of the main button at the foot of the Ready page that starts the install.
installer-install = Instalar

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = Reiniciar ahora

# Beside the keyboard test box on the first page.
installer-try-keyboard = Probar el teclado

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Escribe aquí para probar el teclado

# SAFETY: needs a human review before it ships
# Note beside Install after one press, when the second press writes beside another system.
installer-arm = Haz clic otra vez para instalar

# SAFETY: needs a human review before it ships
# Note beside Install after one press, when the second press erases a whole disk.
installer-arm-erase = Haz clic otra vez para borrar e instalar

# Note beside Next on the first page while no language is chosen.
installer-missing-language = Elige un idioma

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Elige un teclado

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Elige una hora

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Elige dónde irá Trindle

# SAFETY: needs a human review before it ships
# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = Escribe la contraseña del disco dos veces, igual las dos

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = Falta un nombre de usuario

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Ponle un nombre a este equipo

# SAFETY: needs a human review before it ships
# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = Escribe tu contraseña dos veces, igual las dos

# Title of the first page.
installer-welcome-title = Te damos la bienvenida a { $name }

# Sentence under the title of the first page.
installer-welcome-help = Elige tu idioma; el teclado y la hora lo siguen, y puedes cambiar cualquiera de los dos.

# Head of the card for choosing the language, and the name of its search box.
installer-language = Idioma

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Teclado

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Hora

# Under a card's head while nothing has been chosen in its list.
installer-select = Seleccionar

# Title of the disk page.
installer-disks-title = Dónde irá { $name }

# SAFETY: needs a human review before it ships
# Sentence under the title of the disk page.
installer-disks-help = Todo lo que hay en el disco que elijas se reemplaza, salvo que hagas sitio junto a lo que ya hay.

# Shown on the disk page when no disk is big enough.
installer-disks-none = No se encontró ningún disco con tamaño suficiente. Trindle necesita 20 GB.

# SAFETY: needs a human review before it ships
# Name and words of the choice to erase a whole disk.
installer-disks-whole = Usar el disco entero, reemplazando todo lo que contiene

# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = Conservar { $what } y compartir el disco

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = { $kept } para lo que hay ahora, { $mine } para Trindle

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = Espacio libre

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } y { $last }

# Under a disk's name: what is on it.
installer-disk-has = Contiene { $list }

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = Contiene { $list }, cifrado

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = Contiene un sistema cifrado

# Under a disk's name: there is nothing on it.
installer-disk-empty = Vacío

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = Contiene archivos, pero ningún sistema que Trindle reconozca

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = Un archivo de intercambio de { $gb } GB

# Name of the switch that makes a swap file.
installer-swap-switch = Crear un archivo de intercambio

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = El arranque seguro está activado, así que este equipo no puede hibernar, pero el archivo de intercambio ayuda cuando falta memoria. Solo se crea si el disco tiene espacio de sobra.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = Espacio para hibernar, que mantiene todo abierto con el equipo apagado, y un recurso para cuando falta memoria. Sin él, el equipo no puede hibernar. Solo se crea si el disco tiene espacio de sobra.

# Title of the page about encrypting the disk.
installer-privacy-title = Cifrar el disco

# Sentence under the title of that page.
installer-privacy-help = Nadie puede leer un disco cifrado si lo saca de este equipo.

# Head of the card with the three choices.
installer-privacy-locked = Cómo se bloquea el disco

# SAFETY: needs a human review before it ships
# Name and words of the choice not to encrypt the disk.
installer-privacy-none = No cifrar el disco

# SAFETY: needs a human review before it ships
# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = Cifrarlo y pedir la contraseña al iniciar el equipo

# SAFETY: needs a human review before it ships
# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = Cifrarlo y dejar que el chip TPM de este equipo lo desbloquee

# Head of the card about the TPM chip.
installer-privacy-about = Sobre el chip

# SAFETY: needs a human review before it ships
# In the card about the chip, on a computer that has one.
installer-privacy-chip = Este equipo tiene un chip TPM. El primer inicio tras instalar pide la contraseña una vez, mientras el chip aprende este equipo; después desbloquea el disco por sí solo y no se escribe nada. Guarda la contraseña: es la forma de entrar si se cambia el chip o se lleva el disco a otro equipo.

# SAFETY: needs a human review before it ships
# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = Este equipo no tiene un chip TPM que Trindle pueda usar, así que la contraseña se pedirá cada vez que inicie. En un equipo que lo tenga, el chip puede desbloquear el disco por sí solo.

# SAFETY: needs a human review before it ships
# Title of the disk password page.
installer-disk-title = La contraseña del disco

# Sentence under the title of the disk password page.
installer-disk-help = Se escribe antes que nada cuando el equipo arranca.

# SAFETY: needs a human review before it ships
# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = Contraseña del disco

# Name of the round button that makes up a passphrase.
installer-disk-suggest = Sugerir una frase de contraseña

# Label and name of the second box of a password, where it is typed again.
installer-same-again = Otra vez la misma

# SAFETY: needs a human review before it ships
# Red line under the password boxes when the two do not match.
installer-mismatch = Las dos contraseñas no coinciden.

# Head of the card that tells the person to write the password down.
installer-disk-write-down = Anótala

# SAFETY: needs a human review before it ships
# Words in that card.
installer-disk-write-down-text = Anota esta contraseña en papel y guárdala en un lugar seguro, lejos del equipo. Sin ella, el disco no se puede abrir, ni tú ni nadie, nunca. No hay forma de volver a entrar. Cuatro palabras corrientes forman una frase de contraseña fácil de escribir y difícil de adivinar.

# Title of the account page.
installer-you-title = Sobre ti

# Sentence under the title of the account page.
installer-you-help = Esta es la cuenta con la que inicias sesión.

# Head of the card with the name boxes.
installer-account = Tu cuenta

# Label and name of the box for the person's name.
installer-your-name = Tu nombre

# Label and name of the box for the sign-in name.
installer-user-name = Nombre de usuario

# Label and name of the box for the computer's name.
installer-host-name = Nombre de este equipo

# Head of the card with the password boxes.
installer-your-password = Tu contraseña

# Label and name of the first password box.
installer-password = Contraseña

# Label of the strength meter and the name a screen reader gives it.
installer-strength = Qué difícil es de adivinar

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = Falta un nombre de usuario.

# Red line under the account boxes: a name is too long.
installer-name-too-long = Ese nombre es demasiado largo.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Empieza con una letra minúscula.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Solo letras minúsculas, dígitos, - y _.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = Ese nombre ya lo usa el sistema.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Ponle un nombre a este equipo.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Solo letras, dígitos y -.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = Un ordenador adivinaría esto al instante.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
        [one] Un ordenador adivinaría esto en aproximadamente { $n } minuto.
        [many] Un ordenador adivinaría esto en aproximadamente { $n } minutos.
       *[other] Un ordenador adivinaría esto en aproximadamente { $n } minutos.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
        [one] Un ordenador adivinaría esto en aproximadamente { $n } hora.
        [many] Un ordenador adivinaría esto en aproximadamente { $n } horas.
       *[other] Un ordenador adivinaría esto en aproximadamente { $n } horas.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
        [one] Un ordenador adivinaría esto en aproximadamente { $n } día.
        [many] Un ordenador adivinaría esto en aproximadamente { $n } días.
       *[other] Un ordenador adivinaría esto en aproximadamente { $n } días.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
        [one] Un ordenador necesitaría aproximadamente { $n } año para adivinar esto.
        [many] Un ordenador necesitaría aproximadamente { $n } años para adivinar esto.
       *[other] Un ordenador necesitaría aproximadamente { $n } años para adivinar esto.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = Un ordenador necesitaría más tiempo que toda la historia registrada para adivinar esto.

# Title of the Ready page.
installer-ready-title = Listo para instalar

# Head of the card that lists what will happen.
installer-ready-nothing-changed = Todavía no se ha cambiado nada

# Head of the card about security updates.
installer-ready-security = Actualizaciones de seguridad

# In the card about security updates.
installer-ready-security-text = { $name } instala por sí solo las correcciones de seguridad de Debian cada día. Esto no se puede desactivar.

# SAFETY: needs a human review before it ships
# Sentence under the title when a whole disk will be replaced.
installer-ready-replaced = Todo lo que hay en { $disk } se reemplazará.

# SAFETY: needs a human review before it ships
# Sentence under the title when the install goes beside what is there.
installer-ready-kept = Todo lo que hay en { $part } se conserva, pero haz antes una copia de seguridad: reducirla puede hacer perder archivos si se va la luz.

# Sentence under the title when no disk has been chosen.
installer-ready-none = Todavía no se ha elegido ningún disco.

# Name of a row in the list: the whole disk will be used.
installer-ready-whole = Todo { $disk }

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-whole-text = Todo lo que contiene se reemplaza por { $name }.

# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = Junto a { $part }

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-beside-text = { $part } se reduce hasta { $size }, y { $name } ocupa el espacio liberado.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = Ningún disco

# Words of that row.
installer-ready-no-disk-text = No se ha elegido ningún disco.

# SAFETY: needs a human review before it ships
# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = Sin cifrar

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-plain-text = Cualquiera que saque el disco puede leerlo.

# SAFETY: needs a human review before it ships
# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = Cifrado

# SAFETY: needs a human review before it ships
# Words of that row when a password is typed at each start.
installer-ready-locked-password = Escribes su contraseña cada vez que inicia el equipo.

# SAFETY: needs a human review before it ships
# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = La contraseña una vez, y después el chip TPM de este equipo lo desbloquea.

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = Teclado { $keyboard }, hora { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = Instalando { $name }

# Head of the card that holds the short tour.
installer-tour = Un recorrido breve

# Title of the last page.
installer-done-title = { $name } está instalado

# Sentence under the title of the last page.
installer-done-help = Saca la memoria USB y reinicia.

# SAFETY: needs a human review before it ships
# Head of the card that repeats the warning about the disk password.
installer-done-keep = Guarda a salvo la contraseña del disco

# SAFETY: needs a human review before it ships
# In that card when the TPM chip unlocks the disk.
installer-keep-chip = El disco está cifrado. El primer inicio pide la contraseña del disco una vez, mientras el chip aprende este equipo; después se abre por sí solo. Anota esa contraseña en papel y guárdala en un lugar seguro, lejos de este equipo. Sin ella, el disco no se puede abrir, ni tú ni nadie, nunca.

# SAFETY: needs a human review before it ships
# In that card when the password is typed at each start.
installer-keep-password = El disco está cifrado. Anota ahora en papel la contraseña del disco, si aún no lo has hecho, y guárdala en un lugar seguro, lejos de este equipo. Sin ella, el disco no se puede abrir, ni tú ni nadie, nunca.

# Title of the first slide.
installer-slide-ring = Todo en un círculo

# Words of the first slide.
installer-slide-ring-text = Mueve el puntero al borde de la pantalla y haz doble clic, y se abre el anillo: tus programas alrededor, la hora y los interruptores en el centro.

# Title of the second slide.
installer-slide-snapshots = Recupera lo que rompió una actualización

# Words of the second slide.
installer-slide-snapshots-text = Se toma una instantánea del sistema antes y después de cada cambio de software. Ajustes devuelve todo el sistema a cualquiera de ellas, y el menú de arranque puede hacerlo cuando el equipo no inicia.

# Title of the third slide.
installer-slide-apps = Aplicaciones que no estorban

# Words of the third slide.
installer-slide-apps-text = Aplicaciones de Flathub, las propias de Debian, AppImages y fuentes de software en un solo lugar, con copias de seguridad que puedes llevar a otro equipo.

# Title of the fourth slide.
installer-slide-yours = Tuyo, y rápido

# Words of the fourth slide.
installer-slide-yours-text = Claro u oscuro, tus propios sonidos y un portapapeles que recuerda lo que copiaste.

# Title of the fifth slide.
installer-slide-small = Pequeño y silencioso

# Words of the fifth slide.
installer-slide-small-text = El escritorio usa poca memoria y arranca en pocos segundos, así que un portátil antiguo vuelve a parecer nuevo.

# Title of the tour slide about the ring.
installer-tour-ring = El anillo es todo el escritorio

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = Pon el puntero en esta esquina y se abre el anillo

# Note on the ring picture, at the time.
installer-tour-ring-time = La hora y la fecha, en el centro

# Note on the ring picture, at the programs.
installer-tour-ring-programs = Tus programas están alrededor

# Note on the ring picture, at the switches.
installer-tour-ring-switches = Sonido, pantalla, todas tus aplicaciones, mantener activo y apagar

# Title of the tour slide about Settings.
installer-tour-settings = Ajustes, en palabras sencillas

# Note on the Settings picture, at the search box.
installer-tour-settings-search = Escribe aquí para encontrar un ajuste sin buscar a ciegas

# Note on the Settings picture, at Network.
installer-tour-settings-network = En Red es donde te unes a una red Wi-Fi

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Claro u oscuro, cuando quieras

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Veinte colores, o escribe el tuyo

# Title of the tour slide about Apps.
installer-tour-apps = Aplicaciones, para todo lo demás

# Note on the Apps picture, at the search box.
installer-tour-apps-search = Busca un programa por su nombre o por lo que hace

# Note on the Apps picture, at the list.
installer-tour-apps-many = Miles de programas, gratis, de Flathub y Debian

# Note on the Apps picture, at updates.
installer-tour-apps-updates = Actualizaciones para todo, incluido el propio Trindle

# Title of the tour slide about voice control.
installer-tour-voice = Puedes hablarle en su lugar

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = El control por voz está aquí: Ajustes, luego Anillo

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Pulsa Añadir voz una vez. Ocupa unos 150 MB y tarda unos minutos

# Note on the voice picture, about what to say.
installer-tour-voice-say = Después toca el centro del anillo y di el nombre de una aplicación, “más alto” o “ajustes de Wi-Fi”

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Lo que dices se entiende en este equipo y no va a ningún otro sitio

# Title of the tour slide about snapshots.
installer-tour-snapshots = Una vía de vuelta, antes de que algo cambie

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = El equipo toma una instantánea antes de cada actualización

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = Si algo se rompe, vuelve a como estaba, desde el menú de arranque

# Name of the picture of the ring, and the head of its card.
installer-live-caption = El anillo, donde vive todo

# Sentence under the title of the welcome from a stick.
installer-live-help = Estás usando la memoria USB, así que no se ha cambiado nada de este equipo.

# Head of the card with the three first things to do.
installer-live-begin = Para empezar

# Name of the first thing to do.
installer-live-open = Abrir el anillo

# Line under it.
installer-live-open-line = Haz doble clic en el borde de la pantalla, o pulsa Super.

# Name of the second thing to do.
installer-live-speak = Háblale

# Line under it when voice control is there.
installer-live-speak-line = Toca el centro del anillo y di lo que quieres.

# Line under it when voice control has to be added.
installer-live-speak-add = Añade la voz en Ajustes, Anillo, y luego toca el centro y habla.

# Name of the third thing to do.
installer-live-tab = Todo en el anillo

# Line under it.
installer-live-tab-line = Lo que está abierto, tus notificaciones y el portapapeles están ahí.

# The choice of the dark look.
installer-live-dark = Oscuro

# The choice of the light look.
installer-live-light = Claro

# Name the screen reader gives to the choice between dark and light.
installer-live-look = Claro u oscuro, para todo el escritorio

# Name of the round button that closes this window and opens the ring.
installer-live-around = Echar un vistazo primero

# Name of the main button that opens the installer.
installer-live-install = Instalar { $name }

# Foot of the install page while room is made on the disk.
installer-step-room = Haciendo sitio

# Foot of the install page while the disk is prepared.
installer-step-prepare = Preparando el disco

# Foot of the install page while a place is made beside another system.
installer-step-place = Creando un lugar para Trindle

# Foot of the install page while the disk is looked at.
installer-step-check = Comprobando lo que ya hay en el disco

# Foot of the install page while another system is made smaller.
installer-step-smaller = Reduciendo el otro sistema

# Foot of the install page while the disk is encrypted.
installer-step-lock = Bloqueando el disco

# Foot of the install page while the TPM chip is set up.
installer-step-chip = Configurando el chip del equipo

# Foot of the install page while the system is copied.
installer-step-copy = Copiando Trindle al disco

# Foot of the install page while settings are written.
installer-step-settings = Escribiendo los ajustes

# Foot of the install page while the person's account is made.
installer-step-account = Creando tu cuenta

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Quitando las partes del sistema en vivo

# Foot of the install page while the start-up files are made.
installer-step-startup = Creando los archivos de inicio

# Foot of the install page while the boot menu is set up.
installer-step-boot = Configurando el menú de arranque

# Foot of the install page while snapshots are set up.
installer-step-snapshots = Configurando las instantáneas

# Foot of the install page at the very end.
installer-step-finishing = Terminando

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program } no pudo iniciarse: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } no terminó. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = No se pudo escribir { $path }: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } no es un disco.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } no es un disco de este equipo.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = No se eligió ningún disco.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Trindle no puede usar ese nombre de usuario.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Trindle no puede usar ese nombre de equipo.

# Red line at the foot: the language cannot be used.
installer-bad-language = Trindle no puede usar ese idioma.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Trindle no puede usar ese teclado.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Trindle no puede usar esa zona horaria.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = Ese nombre contiene algo que Trindle no puede guardar.

# SAFETY: needs a human review before it ships
# Red line at the foot: a password has a line break in it.
installer-bad-password = Una contraseña no puede contener un salto de línea.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = La segunda mitad del instalador debe ejecutarse como root.

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = Este disco no tiene una partición de arranque que compartir, así que Trindle no puede ir junto a lo que ya hay.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = No se encontró la nueva parte del disco.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = No se pudo leer el tamaño de { $part }.

# SAFETY: needs a human review before it ships
# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle no puede reducir una partición { $fs }.

# SAFETY: needs a human review before it ships
# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle no puede reducir una partición desconocida.

# SAFETY: needs a human review before it ships
# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = Hace falta una contraseña para cifrar el disco.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = No se pudo copiar el sistema: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = No se pudo copiar el sistema. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = No hay espacio para un archivo de intercambio.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = No se pudo leer dónde está el archivo de intercambio en el disco.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = No se pudieron identificar las particiones nuevas.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = No se pudo establecer la zona horaria: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = No se pudo instalar el menú de arranque en este equipo.

# Name of that button after one press, when a second press goes ahead.
installer-sure = ¿Seguro?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = Lo que está abierto, el portapapeles y un botón que despeja la pantalla

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } es la memoria USB desde la que se está ejecutando Trindle.

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } está en uso. Cierra lo que lo esté usando e inténtalo de nuevo.

# Said beside how far the install has got, once it has started.
installer-started = Instalación iniciada

# The button that ends an install under way.
installer-halt = Detener

# Beside Stop after its first press.
installer-halt-sure = ¿Detener la instalación? El contenido anterior del disco ya se ha perdido.

# While an install is being stopped.
installer-halting = Deteniendo

# When Stop could not let go of the disk.
installer-halt-stuck = No se pudo liberar el disco. Reinicia el equipo para intentarlo de nuevo.

# The copy is done and the person is still answering.
installer-step-waiting = Copiado. Esperando tus respuestas.



installer-already-running = Ya hay otro instalador en marcha en este equipo.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle necesita al menos 1 GB de memoria, y este equipo tiene menos.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = Un disco solo se puede cifrar en un equipo que arranque con UEFI, y este no lo hace.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = Windows en { $part } se hibernó o se dejó en inicio rápido, así que reducirla podría hacer que pierda sus archivos. Inicia Windows, elige Reiniciar y vuelve a intentarlo.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle está instalado, pero no se pudieron configurar las instantáneas: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle está instalado, pero no se pudo hacer que el menú de arranque muestre las instantáneas: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = No todo se configuró

