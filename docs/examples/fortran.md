# Fortran API Example

The following examples illustrate how to use the Fortran interface to the
`NeoPDF` library and to evaluate distributions.

## Prerequisites

Build and install the C API as described in the [installation guide](../installation.md).
Then, copy the `neopdf_fapi/neopdf.f90` in the working directory and include it in the
compilation. The following is an example of Makefile:

```make title="Makefile"
FC = gfortran
FFLAGS = -Wall -Wextra -O0 -g -ffree-line-length-none
NEOPDF_LIBS != pkg-config neopdf_capi --libs
LHAPDF_LIBS != pkg-config lhapdf --libs

PROGRAMS = check-fapi

all: $(PROGRAMS)

test-examples: $(PROGRAMS)
	set -e && for i in $(PROGRAMS); do ./$${i} > output; diff -u $${i}.output output; done; rm output

neopdf.o: neopdf.f90
	$(FC) $(FFLAGS) -c $< -o $@

check-fapi: check-fapi.f90 neopdf.o
	$(FC) $(FFLAGS) $< neopdf.o $(LHAPDF_LIBS) $(NEOPDF_LIBS) -o $@

.PHONY: clean

clean:
	rm -f *.o *.mod $(PROGRAMS)
```

## Example 1: Loading and Evaluating PDFs

This example demonstrates the use of the `NeoPDF` Fortran API to load a single PDF
member, evaluate parton distributions for a range of $x$ values, and compare the
results to LHAPDF.

```fortran
program check_fapi
    use neopdf
    use iso_c_binding

    implicit none

    type (neopdf_pdf)  :: pdf
    character(len=256) :: pdf_name
    integer            :: id, ix, member
    double precision   :: x, q2, g_neo, g_lha
    double precision xfs(-6:6)

    pdf_name = "NNPDF40_nnlo_as_01180"
    member = 0
    q2 = 100.0
    id = 21

    call setlhaparm("SILENT")
    call initpdfsetbyname(pdf_name)
    pdf = neopdf_pdf_load(trim(pdf_name), member)

    write(*,'(A)') repeat('-', 38)
    write(*,'(A10, 2X, A12, 2X, A12)') 'x', 'LHAPDF', 'NeOPDF'
    write(*,'(A)') repeat('-', 38)

    do ix=1,10
        x = (ix - 0.5d0) / 10.0d0

        call evolvepdf(x, sqrt(q2), xfs)
        g_lha = xfs(0)
        g_neo = neopdf_pdf_xfxq2(pdf, id, x, q2)

        ! check the agreement (absolute value)
        if (abs(g_lha - g_neo) > 1e-16) then
            write(*, *) "LHAPDF and NeoPDF differs"
            stop 1
        end if

        write(*,'(ES10.3, 2X, ES12.5, 2X, ES12.5)') x, g_lha, g_neo
    enddo

    write(*,'(A)') repeat('-', 38)

    call neopdf_pdf_free(pdf)

end program check_fapi
```

The above code would print the following results:

```
--------------------------------------
         x        LHAPDF        NeOPDF
--------------------------------------
 5.000E-02   2.35826E+00   2.35826E+00
 1.500E-01   6.21341E-01   6.21341E-01
 2.500E-01   2.34180E-01   2.34180E-01
 3.500E-01   1.00192E-01   1.00192E-01
 4.500E-01   3.86353E-02   3.86353E-02
 5.500E-01   1.14658E-02   1.14658E-02
 6.500E-01   2.41062E-03   2.41062E-03
 7.500E-01   3.15737E-04   3.15737E-04
 8.500E-01   1.94082E-05   1.94082E-05
 9.500E-01   1.35325E-07   1.35325E-07
--------------------------------------
```
