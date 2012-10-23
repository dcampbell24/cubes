! Copyright 2012 David Campbell.
! Use of this source code is governed by a MIT-style
! license that can be found in the LICENSE file.

module list_module
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

    integer, parameter   :: isin(0:3) = (/ 0, 1,  0, -1 /), &
                            icos(0:3) = (/ 1, 0, -1,  0 /)
    integer              :: mrot(3, 3, 3, 4), cube(3, 3, 3)
    integer, allocatable :: vecs(:, :, :)
    type(list)           :: rots, sols, test
    integer              :: ii

    cube = 0
    call init_mrot(mrot)
    call read_pieces(vecs)
    call print_pieces(vecs)
    sols%length = 0
    allocate(sols%d(3, 3, 3, 128))

    call search(vecs, size(vecs, 3), cube, sols)
    print *, sols%length
    do ii = 1, sols%length
        call print_pieces(sols%d(:, :, :, ii))
        print *, "********************************"
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

    function all_puts(cube, piece, id) result(cubes)
        integer, intent(in)                   :: cube(3, 3, 3), piece(:, :), id
        type(list)                            :: cubes, rots
        integer                               :: i, x, y, z, rmax(3)
        integer, dimension(3, size(piece, 2)) :: rot, px, py, pz
        integer                               :: c1(3, 3, 3)
        logical                               :: ok

        cubes%length = 0
        !allocate(cubes%s(size(piece, 1), size(piece, 2), 3**3))
        allocate(cubes%d(3, 3, 3, 3**3))

        rots = all_rots(piece)
        do i = 1, rots%length
            rot = rots%s(:, :, i)
            rmax = maxval(rot, 2)
            do x = 0, 3 - rmax(1)
                px = rot
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
        integer, intent(in)       :: ps(:, :, :), n, cube(3, 3, 3)
        type(list), intent(inout) :: sols
        type(list)                :: puts
        integer                   :: i

        if (n == 0) then
            sols%length = sols%length + 1
            sols%d(:, :, :, sols%length) = cube
            return
        end if

        ! TODO Remove searchs with tiny islands.
        puts = all_puts(cube, ps(:, :, n), n)
        ! TODO Remove puts that are rotations of each other.
        do i = 1, puts%length
            call search(ps, n - 1, puts%d(:, :, :, i), sols)
        end do
    end subroutine

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
