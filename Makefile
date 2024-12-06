test-results:
	cargo test -- -Z unstable-options --format json --report-time 2> test.log | tee results.json
	cat test.log
	cat results.json | cargo2junit > results.xml
