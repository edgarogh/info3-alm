SHELL=bash

add8bit.ec: q1.lus
	lv6 -ec -node add8bit -o $@ $<

# lv6 requiert que le fichier ait une extension, on ne peut donc pas directement lui passer
# /dev/stdin, d'où la création d'un symlink avec la bonne extension
TMP_STDIN=.stdin.lus

# règle pour générer n'importe quel noeud addnbit générique
addnbit_%.ec: q1.lus
	ln -s /dev/stdin $(TMP_STDIN)
	cat <(echo "function addnbit_instance = addnbit<<$*>>") $< | lv6 -ec -node addnbit_instance -o $@ $(TMP_STDIN)
	-rm $(TMP_STDIN)

entierv6.lus:
	@echo [Makefile] Le fichier entiersv6.lus est requis dans le répertoire courant
	@exit 1

addnbit_decimal.ec: q1.lus entierv6.lus
	lv6 -ec -node addnbit_decimal16 -o $@ $^

andnbits_4.ec: q1.lus
	lv6 -ec -node andnbits_4 -o $@ $<

flags_addnbits.ec: flags.lus entierv6.lus
	lv6 -ec -node addnbits_4 -o $@ $^

flags_subnbits.ec: flags.lus entierv6.lus
	lv6 -ec -node subnbits_4 -o $@ $^