.PHONY: luciole clean

POpgcdfin.ec: pgcdPO.lus
	lv6 -ec -n POpgcdfin -o $@ $<

pgcdfin.ec: pgcd.lus
	lv6 -ec -n pgcdfin -o $@ $<

clean:
	rm *.ec

luciole: pgcdfin.ec
	luciole -comp $<