module neopdf
    use iso_c_binding
    use iso_fortran_env

    implicit none

    integer, parameter, private :: dp = kind(0.0d0)

    type neopdf_pdf
        type (c_ptr) :: ptr = c_null_ptr
    end type

    type, bind(c) :: neopdf_pdf_members
        type (c_ptr) :: pdfs = c_null_ptr
        integer (c_size_t) :: size
    end type

    enum, bind(c) ! :: neopdf_subgrid_params
        enumerator :: neopdf_subgrid_params_nucleons
        enumerator :: neopdf_subgrid_params_alphas
        enumerator :: neopdf_subgrid_params_kt
        enumerator :: neopdf_subgrid_params_momentum
        enumerator :: neopdf_subgrid_params_scale

        enumerator :: neopdf_subgrid_params
    end enum

    interface
        function strlen(s) bind(c, name="strlen")
            use iso_c_binding

            implicit none

            type (c_ptr), value :: s
            integer (c_size_t)  :: strlen
        end function strlen

        type (c_ptr) function c_neopdf_pdf_load(pdf_name, member) bind(c, name="neopdf_pdf_load")
            use iso_c_binding
            character (c_char) :: pdf_name(*)
            integer (c_size_t), value :: member
        end function

        subroutine c_neopdf_pdf_free(pdf) bind(c, name="neopdf_pdf_free")
            use iso_c_binding
            type (c_ptr), value :: pdf
        end subroutine

        function c_neopdf_pdf_xfxq2(pdf, id, x, q2) bind(c, name="neopdf_pdf_xfxq2")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int), value :: id
            real (c_double), value :: x, q2
            real (c_double) :: c_neopdf_pdf_xfxq2
        end function

        function c_neopdf_pdf_alphas_q2(pdf, q2) bind(c, name="neopdf_pdf_alphas_q2")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double), value :: q2
            real (c_double) :: c_neopdf_pdf_alphas_q2
        end function

        function c_neopdf_pdf_x_min(pdf) bind(c, name="neopdf_pdf_x_min")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: c_neopdf_pdf_x_min
        end function

        function c_neopdf_pdf_x_max(pdf) bind(c, name="neopdf_pdf_x_max")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: c_neopdf_pdf_x_max
        end function

        function c_neopdf_pdf_q2_min(pdf) bind(c, name="neopdf_pdf_q2_min")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: c_neopdf_pdf_q2_min
        end function

        function c_neopdf_pdf_q2_max(pdf) bind(c, name="neopdf_pdf_q2_max")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: c_neopdf_pdf_q2_max
        end function

        function c_neopdf_pdf_num_pids(pdf) bind(c, name="neopdf_pdf_num_pids")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_size_t) :: c_neopdf_pdf_num_pids
        end function

        subroutine c_neopdf_pdf_pids(pdf, pids, num_pids) bind(c, name="neopdf_pdf_pids")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int) :: pids(*)
            integer (c_size_t), value :: num_pids
        end subroutine

        function c_neopdf_pdf_num_subgrids(pdf) bind(c, name="neopdf_pdf_num_subgrids")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_size_t) :: c_neopdf_pdf_num_subgrids
        end function

        subroutine c_neopdf_pdf_param_range(pdf, param, param_range) bind(c, name="neopdf_pdf_param_range")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int), value :: param
            real (c_double) :: param_range(*)
        end subroutine

        subroutine c_neopdf_pdf_subgrids_shape_for_param(pdf, subgrid_shape, num_subgrid, subgrid_param) bind(c, name="neopdf_pdf_subgrids_shape_for_param")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_size_t) :: subgrid_shape(*)
            integer (c_size_t), value :: num_subgrid
            integer (c_int), value :: subgrid_param
        end subroutine

        subroutine c_neopdf_pdf_subgrids_for_param(pdf, subgrid, subgrid_param, num_subgrid, subgrid_shape, subgrid_index) bind(c, name="neopdf_pdf_subgrids_for_param")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: subgrid(*)
            integer (c_int), value :: subgrid_param
            integer (c_size_t), value :: num_subgrid
            integer (c_size_t) :: subgrid_shape(*)
            integer (c_size_t), value :: subgrid_index
        end subroutine

        type (neopdf_pdf_members) function c_neopdf_pdf_load_all(pdf_name) bind(c, name="neopdf_pdf_load_all")
            use iso_c_binding
            import :: neopdf_pdf_members
            character (c_char) :: pdf_name(*)
        end function

        subroutine c_neopdf_pdf_array_free(array) bind(c, name="neopdf_pdf_array_free")
            use iso_c_binding
            import :: neopdf_pdf_members
            type (neopdf_pdf_members), value :: array
        end subroutine

        function c_neopdf_pdf_xfxq2_nd(pdf, id, params, num_params) bind(c, name="neopdf_pdf_xfxq2_nd")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int), value :: id
            real (c_double) :: params(*)
            integer (c_size_t), value :: num_params
            real (c_double) :: c_neopdf_pdf_xfxq2_nd
        end function
    end interface

contains
    function c_f_string(c_str) result(f_str)
        use :: iso_c_binding

        type(c_ptr), intent(in) :: c_str
        character(kind=c_char), dimension(:), pointer :: arr_f_ptr => null()
        character(len=:, kind=c_char), allocatable :: f_str
        integer(kind=c_size_t) :: i, length

        length = strlen(c_str)
        call c_f_pointer(c_str, arr_f_ptr, [length])

        if (.not.associated(arr_f_ptr)) then
            f_str = "NULL"
            return
        end if

        allocate(character(len=length)::f_str)

        do i = 1, length
            f_str(i:i) = arr_f_ptr(i)
        end do
    end function

    type (neopdf_pdf) function neopdf_pdf_load(pdf_name, member)
        implicit none

        character (*), intent(in) :: pdf_name
        integer, intent(in) :: member

        neopdf_pdf_load = neopdf_pdf(c_neopdf_pdf_load(pdf_name // c_null_char, int(member, c_size_t)))
    end function

    subroutine neopdf_pdf_free(pdf)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf

        call c_neopdf_pdf_free(pdf%ptr)
    end subroutine

    function neopdf_pdf_xfxq2(pdf, id, x, q2) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer, intent(in) :: id
        real (dp), intent(in) :: x, q2
        real (dp) :: res

        res = c_neopdf_pdf_xfxq2(pdf%ptr, id, x, q2)
    end function

    function neopdf_pdf_alphas_q2(pdf, q2) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp), intent(in) :: q2
        real (dp) :: res

        res = c_neopdf_pdf_alphas_q2(pdf%ptr, q2)
    end function

    function neopdf_pdf_x_min(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp) :: res

        res = c_neopdf_pdf_x_min(pdf%ptr)
    end function

    function neopdf_pdf_x_max(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp) :: res

        res = c_neopdf_pdf_x_max(pdf%ptr)
    end function

    function neopdf_pdf_q2_min(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp) :: res

        res = c_neopdf_pdf_q2_min(pdf%ptr)
    end function

    function neopdf_pdf_q2_max(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp) :: res

        res = c_neopdf_pdf_q2_max(pdf%ptr)
    end function

    function neopdf_pdf_num_pids(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer :: res

        res = int(c_neopdf_pdf_num_pids(pdf%ptr))
    end function

    subroutine neopdf_pdf_pids(pdf, pids)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer, intent(out) :: pids(:)

        call c_neopdf_pdf_pids(pdf%ptr, pids, int(size(pids), c_size_t))
    end subroutine

    function neopdf_pdf_num_subgrids(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer :: res

        res = int(c_neopdf_pdf_num_subgrids(pdf%ptr))
    end function

    subroutine neopdf_pdf_param_range(pdf, param, param_range)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer, intent(in) :: param
        real (dp), intent(out) :: param_range(2)

        call c_neopdf_pdf_param_range(pdf%ptr, param, param_range)
    end subroutine

    subroutine neopdf_pdf_subgrids_shape_for_param(pdf, subgrid_shape, subgrid_param)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer(c_size_t), intent(out) :: subgrid_shape(:)
        integer, intent(in) :: subgrid_param

        call c_neopdf_pdf_subgrids_shape_for_param(pdf%ptr, subgrid_shape, int(size(subgrid_shape), c_size_t), subgrid_param)
    end subroutine

    subroutine neopdf_pdf_subgrids_for_param(pdf, subgrid, subgrid_param, subgrid_shape, subgrid_index)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp), intent(out) :: subgrid(*)
        integer, intent(in) :: subgrid_param
        integer(c_size_t), intent(in) :: subgrid_shape(:)
        integer, intent(in) :: subgrid_index

        call c_neopdf_pdf_subgrids_for_param(pdf%ptr, subgrid, subgrid_param, int(size(subgrid_shape), c_size_t), subgrid_shape, int(subgrid_index, c_size_t))
    end subroutine

    type (neopdf_pdf_members) function neopdf_pdf_load_all(pdf_name)
        implicit none

        character (*), intent(in) :: pdf_name

        neopdf_pdf_load_all = c_neopdf_pdf_load_all(pdf_name // c_null_char)
    end function

    subroutine neopdf_pdf_array_free(array)
        implicit none

        type (neopdf_pdf_members), intent(in) :: array

        call c_neopdf_pdf_array_free(array)
    end subroutine

    function neopdf_pdf_xfxq2_nd(pdf, id, params) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer, intent(in) :: id
        real (dp), intent(in) :: params(:)
        real (dp) :: res

        res = c_neopdf_pdf_xfxq2_nd(pdf%ptr, id, params, int(size(params), c_size_t))
    end function

end module
