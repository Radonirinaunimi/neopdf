program check_fapi
    use neopdf
    use iso_c_binding

    implicit none

    type (neopdf_pdf) :: pdf
    character(len=256) :: pdf_name
    integer :: member
    real(c_double) :: x, q2, result
    integer :: id

    pdf_name = "NNPDF40_nnlo_as_01180"
    member = 0
    x = 0.1
    q2 = 10.0
    id = 21

    pdf = neopdf_pdf_load(pdf_name // c_null_char, member)
    result = neopdf_pdf_xfxq2(pdf, id, x, q2)
    print *, "xfxq2 result: ", result
    call neopdf_pdf_free(pdf)

end program check_fapi
