.PHONY: luciole clean

machinecafe.ec: machinecafe.lus memoire.lus
	lv6 -ec -n machinecafe -o $@ $<

luciole: machinecafe.ec
	luciole $<

clean:
	rm *.ec
