! Copyright 2012 David Campbell.
! Use of this source code is governed by a MIT-style
! license that can be found in the LICENSE file.

module list_module
    type piece
        integer, allocatable :: s(:, :)
    end type

    type list
        integer              :: length
        integer, allocatable :: s(:, :, :)    ! sparse
        integer, allocatable :: d(:, :, :, :) ! dense
    end type
end module

program cubes
    use iso_fortran_env
    use list_module
    implicit none

    integer, parameter       :: isin(0:3) = (/ 0, 1,  0, -1 /), &
                                icos(0:3) = (/ 1, 0, -1,  0 /)
    integer                  :: mrot(3, 3, 3, 4), cube(3, 3, 3)
    type(piece), allocatable :: ps(:)
    type(list)               :: sols

    cube = 0
    call init_mrot(mrot)
    call read_pieces(ps)
    call print_pieces(ps)
    sols%length = 0
    allocate(sols%d(3, 3, 3, 128))

    call search(ps, size(ps), cube, sols)
    print *, "Solutions: ", sols%length
    call print_cubes(sols)

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
        forall (i = 1:size(p1, 2))
            p2(p1(1, i), p1(2, i), p1(3, i)) = 1
        end forall
    end function

    function contains_p (ps, p) result(b)
        type(list), intent(in) :: ps
        integer, intent(in)    :: p(:, :)
        logical                :: b
        integer                :: i, j

        loop: do i = 1, ps%length
            do j = 1, size(p, 2)
                if (ps%d(p(1, j), p(2, j), p(3, j), i) == 0) then
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
        allocate(ps%s(size(p, 1), size(p, 2), 4**3))
        allocate(ps%d(3, 3, 3, 4**3))

        p1 = p
        do i = 0, 3
            do j = 0, 3
                do k = 0, 3
                    p1 = push_to_one(p1)
                    d = sparse_to_dense(p1)
                    if (.not.contains_p(ps, p1)) then
                        ps%length = ps%length + 1
                        ps%s(:, :, ps%length) = p1
                        ps%d(:, :, :, ps%length) = d
                    end if
                    p1 = rotate(p1, 1, 1)
                end do
                p1 = rotate(p1, 2, 1)
            end do
            p1 = rotate(p1, 3, 1)
        end do
    end function

    subroutine place(c1, p, id, c2, ok)
        integer, intent(in)  :: c1(3, 3, 3), p(:, :), id
        integer, intent(out) :: c2(3, 3, 3)
        logical, intent(out) :: ok
        integer              :: i, x, y, z

        c2 = c1
        do i = 1, size(p, 2)
            x = p(1, i)
            y = p(2, i)
            z = p(3, i)
            if (c1(x, y, z) /= 0) then
                ok = .false.
                return
            end if
            c2(x, y, z) = id
        end do
        ok = .true.
    end subroutine

    function all_puts(cube, ps, id) result(cubes)
        integer, intent(in)                   :: cube(3, 3, 3), id
        type(list), intent(in)                :: ps
        type(list)                            :: cubes
        integer                               :: i, x, y, z, rmax(3)
        integer, dimension(3, size(ps%s, 2))  :: p, px, py, pz
        integer                               :: c1(3, 3, 3)
        logical                               :: ok

        cubes%length = 0
        !allocate(cubes%s(size(piece, 1), size(piece, 2), 3**3)) !FIXME
        allocate(cubes%d(3, 3, 3, ps%length * 3**3))

        do i = 1, ps%length
            p = ps%s(:, :, i)
            rmax = maxval(p, 2)
            do x = 0, 3 - rmax(1)
                px = p
                px(1, :) = px(1, :) + x
                do y = 0, 3 - rmax(2)
                    py = px
                    py(2, :) = py(2, :) + y
                    do z = 0, 3 - rmax(3)
                        pz = py
                        pz(3, :) = pz(3, :) + z
                        call place(cube, pz, id, c1, ok)
                        if (ok) then
                            cubes%length = cubes%length + 1
                            !cubes%s(:, :, cubes%length) = pz
                            cubes%d(:, :, :, cubes%length) = c1
                        end if
                    end do
                end do
            end do
        end do
    end function

    recursive subroutine search(ps, n, cube, sols)
        type(piece), intent(in)   :: ps(:)
        integer, intent(in)       :: n, cube(3, 3, 3)
        type(list), intent(inout) :: sols
        type(list)                :: rots, puts
        integer                   :: i

        if (n == 0) then
            sols%length = sols%length + 1
            sols%d(:, :, :, sols%length) = cube
            return
        end if

        ! TODO Remove searchs with tiny islands.
        ! Skip all rotations for the first piece (they are redundant).
        if (n == size(ps)) then
            rots%length = 1
            allocate(rots%s(size(ps(n)%s, 1), size(ps(n)%s, 2), 1), &
                     rots%d(0, 0, 0, 0))
            rots%s(:, :, 1) = ps(n)%s
            puts = all_puts(cube, rots, n)
        else
            rots = all_rots(ps(n)%s)
            puts = all_puts(cube, rots, n)
        end if
        ! TODO Remove puts that are rotations of each other.
        do i = 1, puts%length
            call search(ps, n - 1, puts%d(:, :, :, i), sols)
        end do
    end subroutine

    subroutine read_pieces(ps)
        type(piece), allocatable, intent(inout) :: ps(:)
        character(len=20)                       :: name_
        integer                                 :: n, w, i, j

        read *, name_
        write (*, "(a, a)") "Reading in ", name_

        read *, n
        write (*, "(a, i3)") "pieces count: ", n
        allocate(ps(n))

        do j = 1, n
            read *, w
            write (*, "(a, i2, a, i2)") "piece ", j, ", volume ", w
            allocate(ps(j)%s(3, w))
            do i = 1, 3
                read *, ps(j)%s(i, :)
            end do
        end do
    end subroutine

    subroutine print_cubes(cs)
        type(list), intent(in) :: cs
        integer                :: ubounds(3)
        integer                :: ii, i, k
        character(len=20)      :: f_str

        do ii = 1, cs%length
            do k = 1, 3
                do i = 1, 3
                    write (*, "(3i2)") cs%d(i, :, k, ii)
                end do
                print *
            end do
            print *, "********************************"
        end do
    end subroutine

    subroutine print_pieces(ps)
        type(piece), intent(in) :: ps(:)
        integer                 :: i, j
        character(len=20)       :: f_str

        do i = 1, size(ps)
            write (f_str, "(a, i3, a)") "(", size(ps(i)%s, 2), "i2)"
            do j = 1, 3
                write (*, f_str) ps(i)%s(j, :)
            end do
            print *
        end do
    end subroutine
end program
