.PHONY: test

alu_4.ec: alu.lus
	lv6 -ec -node alu_4 -o $@ $^

# requiert https://github.com/edgarogh/info3-alm/tree/main/lustre-simple-test
test: alu_4.ec alu.tests
	lustre-simple-test $^
