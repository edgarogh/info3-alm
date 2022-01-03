.PHONY: luciole clean

pgcd.ec: pgcd.lus *.lus
	lv6 -ec -n pgcdfin -o $@ $<

luciole: pgcd.ec
	luciole -comp $<

clean:
	rm *.ec
