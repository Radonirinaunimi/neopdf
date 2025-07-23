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
