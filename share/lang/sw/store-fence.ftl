
# machine translated (Swahili), not yet reviewed by a person.

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = Hilo si jina halali la programu.

# Red words when a fence setting could not be written to its file.
store-fence-not-saved = Hilo halikuhifadhiwa.

# Red words when the folder chosen to share does not exist.
store-fence-no-folder = Chagua folda iliyopo.

# Red words when the whole home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-whole-home = Folda yote ya nyumbani inabaki ya faragha.

# Red words when a folder that holds the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-holds-home = Folda hii ina folda yako ya nyumbani, kwa hiyo inabaki ya faragha.

# Red words when a folder outside the home folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-outside-home = Ni folda zilizo ndani ya folda yako ya nyumbani tu zinazoweza kushirikiwa.

# Red words when a hidden folder is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-hidden = Folda zilizofichwa hubaki za faragha daima.

# Red words when a folder that keeps passwords or mail is chosen to share.
# SAFETY: needs a human review before it ships
store-fence-passwords = Folda hii ina manenosiri au barua pepe, kwa hiyo inabaki ya faragha.

# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = Programu hiyo haina folda ya mipangilio ya kutumia.

# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = Andika jina la tovuti, kama api.example.com, au *.example.com kwa kila tovuti iliyo chini ya jina hilo.

# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = Andika jina la kompyuta, kama nas.local.

# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Andika kila moja kama NAME=value.

# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Andika kila moja kama NAME=value, kwa herufi kubwa.

# Words above the question when an AI program asks to use a folder.
# SAFETY: needs a human review before it ships
store-fence-ask-share = Programu ya AI inaomba kutumia folda hii. Haiwezi kuiona kwa sasa.

# Words above the question when an AI program asks to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-ask-admin = Programu ya AI inaomba kuendesha hili kama msimamizi. Itaendesha nje ya uzio na itaweza kuona kila kitu unachoweza kuona wewe.

# Small heading of the window that asks about an AI program.
store-fence-ai-program = Programu ya AI

# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
# SAFETY: needs a human review before it ships
store-fence-read-only = Iruhusu kusoma folda hii tu

# Note under the question about a folder.
store-fence-sees-next = Itaiona folda programu itakapoanza mara ijayo.

# Tip of the round button that refuses a folder.
# SAFETY: needs a human review before it ships
store-fence-no = Hapana

# Tip of the round button that shares a folder until the session ends.
# SAFETY: needs a human review before it ships
store-fence-share-session = Shiriki kwa kipindi hiki tu

# Main button that shares a folder for good.
# SAFETY: needs a human review before it ships
store-fence-share-always = Shiriki daima

# Tip of the round button that refuses to run something as the administrator.
# SAFETY: needs a human review before it ships
store-fence-dont-allow = Usiruhusu

# Main button that lets something run as the administrator.
# SAFETY: needs a human review before it ships
store-fence-allow = Ruhusu

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = Amri hiyo ni ndefu mno kuonyeshwa yote, kwa hiyo haikuendeshwa.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = Ombi jingine linasubiri ulijibu.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = Hukuruhusu hili.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } haipatikani ndani ya uzio.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = Haikuweza kuanzishwa nje ya uzio.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = Toa njia kamili ya folda.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = Hukuishiriki.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = Imeshirikiwa. Itakuwepo programu hii itakapoanza tena.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = Taja kompyuta ya kufikia.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = Programu za AI zilizo ndani ya uzio haziwezi kufikia kompyuta hiyo kwa ssh.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = Ufunguo wa programu za AI haukuweza kutengenezwa: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = Taja amri ya kuendesha.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } haipatikani hapa.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } haipatikani ndani ya uzio. Omba iendeshwe nje.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: mstari wa maombi wa uzio haupo.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: ombi halikutumwa.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock imekataa: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = Uzio haukuanza: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = Uzio haukuanza: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = Programu za AI huweka kuingia kwao hapa, kando na kwako. Ingia ndani ya programu inapoomba.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = Folda zako ni za faragha dhidi ya programu za AI. Shiriki moja kwenye Mipangilio, Programu za AI, ili ufanye kazi ndani yake.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = Uzio haukuanza: kichujio cha miito ya mfumo hakikuweza kutengenezwa.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = Uzio haukuanza: kikomo cha mtandao hakikuweza kuwekwa.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = Uzio haukuanza: bubblewrap haikukubali kuendesha: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: haipatikani nje ya uzio
