program check_lazy_fapi
    use neopdf
    use iso_c_binding

    implicit none

    type (neopdf_pdf)  :: pdf
    type (neopdf_lazy_iterator) :: iter
    character(len=256) :: pdf_name
    integer            :: id, member
    double precision   :: x, q2, g_neo

    pdf_name = "NNPDF40_nnlo_as_01180.neopdf.lz4"
    q2 = 100.0
    id = 21
    x = 0.1

    iter = neopdf_pdf_load_lazy(trim(pdf_name))

    write(*,'(A)') repeat('-', 26)
    write(*,'(A10, 2X, A12)') 'Member', 'NeOPDF'
    write(*,'(A)') repeat('-', 26)

    do member=0,10
        pdf = neopdf_lazy_iterator_next(iter)
        if (.not. c_associated(pdf%ptr)) exit

        g_neo = neopdf_pdf_xfxq2(pdf, id, x, q2)

        write(*,'(I10, 2X, ES12.5)') member, g_neo

        call neopdf_pdf_free(pdf)
    enddo

    write(*,'(A)') repeat('-', 26)

    call neopdf_lazy_iterator_free(iter)

end program check_lazy_fapi
