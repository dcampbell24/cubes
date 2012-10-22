! Copyright 2012 David Campbell.
! Use of this source code is governed by a MIT-style
! license that can be found in the LICENSE file.

module list_module
    type list
        integer              :: length
        integer, allocatable :: val(:, :, :)
        integer, allocatable :: d(:, :, :, :) ! dense representation of vals.
    end type
end module

program cubes
    use iso_fortran_env
    use list_module
    implicit none

    integer, parameter   :: isin(0:3) = (/ 0, 1,  0, -1 /), &
                            icos(0:3) = (/ 1, 0, -1,  0 /)
    integer              :: mrot(3, 3, 3, 4)
    integer, allocatable :: vecs(:, :, :)
    type(list)           :: rots
    integer              :: ii

    call init_mrot(mrot)
    call read_pieces(vecs)
    call print_pieces(vecs)

    rots = all_rots(vecs(:, :, 1))
    print *, rots%length
    do ii = 1, rots%length
        call print_pieces(rots%d(:, :, :, ii))
    end do
contains
    subroutine init_mrot(mrot)
        integer, intent(inout) :: mrot(3, 3, 3, 4)
        integer                :: theta ! In 90 degree increments

        forall (theta = 1:3)
            mrot(:, :, :, theta) =                                &
                reshape((/ 1,           0,             0,         &
                           0, icos(theta),  -isin(theta),         &
                           0, isin(theta),   icos(theta),         &

                           icos(theta),      0,   isin(theta),    &
                                     0,      1,             0,    &
                          -isin(theta),      0,   icos(theta),    &

                           icos(theta),  -isin(theta),      0,    &
                           isin(theta),   icos(theta),      0,    &
                                     0,             0,      1 /), &
                        (/ 3, 3, 3 /))
        end forall
    end subroutine

    ! Do we need to make a copy?
    function rotate(p1, axis, theta) result(p2)
        integer, intent(in) :: p1(:, :), axis, theta
        integer             :: p2(size(p1, 1), size(p1, 2))

        if (theta == 0) then
            p2 = p1
        else
            p2 = matmul(mrot(:, :, axis, theta), p1)
        end if
    end function

    function push_to_one(p1) result(p2)
        integer, intent(in) :: p1(:, :)
        integer             :: p2(size(p1, 1), size(p1, 2))
        integer             :: i, col_mins(3)

        col_mins = minval(p1, 2)
        forall (i = 1:size(p1, 2))
            p2(:, i) = p1(:, i) - col_mins + 1
        end forall
    end function

    function sparse_to_dense(p1) result(p2)
        integer, intent(in) :: p1(:, :)
        integer             :: i
        integer             :: p2(3, 3, 3)

        p2 = 0
        do i = 1, size(p1, 2)
            p2(p1(1, i), p1(2, i), p1(3, i)) = 1
        end do
    end function

    function contains_p (a, p) result(b)
        integer, intent(in) :: a(:, :, :, :)
        integer, intent(in) :: p(:, :)
        logical             :: b
        integer             :: i, j

        loop: do i = 1, size(a, 4)
            do j = 1, size(p, 2)
                if (a(p(1, j), p(2, j), p(3, j), i) == 0) then
                    cycle loop
                end if
            end do
            b = .true.
            return
        end do loop
        b = .false.
    end function

    ! Non-parallel version.
    function all_rots(p) result(ps)
        integer, intent(in) :: p(:, :)
        integer             :: p1(size(p, 1), size(p, 2))
        integer             :: d(3, 3, 3)
        type(list)          :: ps
        integer             :: i, j, k

        ps%length = 0
        allocate(ps%val(size(p, 1), size(p, 2), 4**3))
        allocate(ps%d(3, 3, 3, 4**3))

        p1 = p
        do i = 0, 3
            do j = 0, 3
                do k = 0, 3
                    ! do some tests then maybe add it
                    p1 = push_to_one(p1)
                    d = sparse_to_dense(p1) !FIXME
                    if (.not.contains_p(ps%d, p1)) then
                        ps%length = ps%length + 1
                        ps%val(:, :, ps%length) = p1
                        ps%d(:, :, :, ps%length) = d
                    end if
                    p1 = rotate(p1, 1, 1)
                end do
                p1 = rotate(p1, 2, 1)
            end do
            p1 = rotate(p1, 3, 1)
        end do
    end function

    subroutine read_pieces(vecs)
        integer, allocatable, intent(inout) :: vecs(:, :, :)
        character(len=20) :: name_
        integer :: dims(3), i, j

        read *, name_
        write (*, "(a, a)") "Reading in ", name_

        read *, dims
        write (*, "(a, 3i3)") "shape: ", dims

        allocate(vecs(dims(1), dims(2), dims(3)))
        do j = 1, dims(3)
            do i = 1, dims(1)
                read *, vecs(i ,:, j)
            end do
        end do
    end subroutine

    subroutine print_pieces(a)
        integer, intent(in) :: a(:, :, :)
        integer             :: ubounds(3)
        integer             :: i, k
        character(len=20)   :: f_str

        write (f_str, "(a, i3, a)") "(", size(a, 2), "i2)"
        ubounds = ubound(a)
        do k = 1, ubounds(3)
            do i = 1, ubounds(1)
                write (*, f_str) a(i, :, k)
            end do
            print *
        end do
    end subroutine
end program
