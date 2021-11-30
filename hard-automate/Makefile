.PHONY: clean

automate.ec: automate.lus
	lv6 -ec -n automate -o $@ $<

transition.ec: automate.lus
	lv6 -ec -n transition -o $@ $<

clean:
	rm *.ec
