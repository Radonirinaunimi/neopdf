CXX = c++ -std=c++11
CXXFLAGS = -O3 -Wall -Wextra -Werror
NEOPDF_DEPS != pkg-config --cflags --libs neopdf_capi
LHAPDF_DEPS != pkg-config --cflags --libs lhapdf

PROGRAMS = check-capi check-writer

all: $(PROGRAMS)

test-examples: $(PROGRAMS)
	set -e && for i in $(PROGRAMS); do ./$${i} > output; diff -u $${i}.output output; done; rm output

check-capi: check-capi.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

check-writer: check-writer.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

.PHONY: clean

clean:
	rm -f $(PROGRAMS) *.neopdf
