.PHONY: test clean

automate.ec: automate.lus
	lv6 -ec -n automate -o $@ $<

transition.ec: automate.lus
	lv6 -ec -n transition -o $@ $<

test: automate.lus automate.tests
	lustre-simple-test $^

clean:
	rm *.ec
