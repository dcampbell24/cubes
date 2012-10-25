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

    logical, parameter       :: elim_islands = .true.
    integer, parameter       :: isin(0:3) = (/ 0, 1,  0, -1 /), &
                                icos(0:3) = (/ 1, 0, -1,  0 /)
    integer, parameter       :: eye(3, 3) = reshape((/ 1, 0, 0,       &
                                                       0, 1, 0,       &
                                                       0, 0, 1 /),    &
                                                    (/ 3, 3 /))
    integer                  :: mrot(3, 3, 3, 4), cube(3, 3, 3)
    type(list), allocatable  :: rots_cache(:)
    type(piece), allocatable :: ps(:)
    type(list)               :: sols

    call init_mrot(mrot)
    cube = 0
    call read_pieces(ps)
    sols%length = 0
    allocate(sols%d(3, 3, 3, 128))

    call search(ps, size(ps), cube, sols)

    write (*, "(a, i6)") "Solutions: ", sols%length
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

    pure function rotate(p1, axis, theta) result(p2)
        integer, intent(in) :: p1(:, :), axis, theta
        integer             :: p2(size(p1, 1), size(p1, 2))

        if (theta == 0) then
            p2 = p1
        else
            p2 = matmul(mrot(:, :, axis, theta), p1)
        end if
    end function

    pure function push_to_one(p1) result(p2)
        integer, intent(in) :: p1(:, :)
        integer             :: p2(size(p1, 1), size(p1, 2))
        integer             :: i, col_mins(3)

        col_mins = minval(p1, 2)
        forall (i = 1:size(p1, 2))
            p2(:, i) = p1(:, i) - col_mins + 1
        end forall
    end function

    pure function sparse_to_dense(p1) result(p2)
        integer, intent(in) :: p1(:, :)
        integer             :: i
        integer             :: p2(3, 3, 3)

        p2 = 0
        forall (i = 1:size(p1, 2))
            p2(p1(1, i), p1(2, i), p1(3, i)) = 1
        end forall
    end function

    pure function contains_p (ps, p) result(b)
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
    pure function all_rots(p) result(ps)
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
        integer, intent(in)                  :: cube(3, 3, 3), id
        type(list), intent(in)               :: ps
        type(list)                           :: cubes
        integer                              :: i, x, y, z, rmax(3), c1(3, 3, 3)
        integer, dimension(3, size(ps%s, 2)) :: p, px, py, pz
        logical                              :: ok

        cubes%length = 0
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
                            cubes%d(:, :, :, cubes%length) = c1
                        end if
                    end do
                end do
            end do
        end do
    end function

    ! Use flood fill to find the smallest island volume.
    function min_island_vol(cube) result(min_vol)
        integer, intent(in)  :: cube(3, 3, 3)
        integer              :: i, j, k, n, d, vol, min_vol
        integer              :: holes(0:4, 0:4, 0:4), q(3, 27), p(3), t(3)

        holes = -1 ! Create a border around holes.
        holes(1:3, 1:3, 1:3) = cube
        min_vol = huge(1)
        do i = 1, 3
        do j = 1, 3
        do k = 1, 3
            if (holes(i, j, k) /= 0) then
                cycle
            end if
            ! Found a hole, see how large it is.
            holes(i, j, k) = 1
            vol = 0
            n = 1
            q(:, n) = (/ i, j, k /)
            do
                p = q(:, n)
                n = n - 1
                vol = vol + 1
                do d = 1, 3
                    t = p + eye(:, d)
                    if (holes(t(1), t(2), t(3)) == 0) then
                        holes(t(1), t(2), t(3)) = 1
                        n = n + 1
                        q(:, n) = t
                    end if
                    t = p - eye(:, d)
                    if (holes(t(1), t(2), t(3)) == 0) then
                        holes(t(1), t(2), t(3)) = 1
                        n = n + 1
                        q(:, n) = t
                    end if
                end do
                if (n == 0) exit
            end do
            if (vol < min_vol) min_vol = vol
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

        if (elim_islands) then
            if (min_island_vol(cube) < size(ps(1)%s, 2)) return
        end if

        ! This is the first piece.
        if (n == size(ps)) then
            ! Cache all of the other pieces' rotations.
            allocate(rots_cache(n - 1))
            forall (i = 1 : n - 1)
                rots_cache(i) = all_rots(ps(i)%s)
            end forall

            ! Skip all of this piece's rotations (they are redundant).
            rots%length = 1
            allocate(rots%s(size(ps(n)%s, 1), size(ps(n)%s, 2), 1), &
                     rots%d(0, 0, 0, 0))
            rots%s(:, :, 1) = push_to_one(ps(n)%s)
            puts = all_puts(cube, rots, n)
        else
            puts = all_puts(cube, rots_cache(n), n)
        end if
        do i = 1, puts%length
            call search(ps, n - 1, puts%d(:, :, :, i), sols)
        end do
    end subroutine

    subroutine print_piece(p)
        integer, intent(in) :: p(:, :)
        integer             :: i
        character(len=20)   :: f_str

        write (f_str, "(a, i3, a)") "(", size(p, 2), "i2)"
        do i = 1, 3
            write (*, f_str) p(i, :)
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
            call print_piece(ps(j)%s)
            print *
        end do
    end subroutine

    subroutine print_cube(c)
        integer, intent(in) :: c(:, :, :)
        integer             :: i, j

        do j = 1, 3
            do i = 1, 3
                write (*, "(3i2)") c(i, 1:3, j)
            end do
            print *
        end do
    end subroutine

    subroutine print_cubes(cs)
        type(list), intent(in) :: cs
        integer                :: i

        do i = 1, cs%length
            print *
            call print_cube(cs%d(:, :, :, i))
            print *, "********************************"
        end do
    end subroutine
end program
