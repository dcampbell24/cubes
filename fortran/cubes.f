! Copyright 2012 David Campbell.
! Use of this source code is governed by a MIT-style
! license that can be found in the LICENSE file.

module list_module
    type piece
        integer              :: cnt
        integer, allocatable :: s(:, :)
    end type

    type puts_t
        integer, allocatable :: d(:, :, :, :)
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

    type puzzle
        character(len=20)        :: name_
        type(piece), allocatable :: ps(:)
    end type

    ! Command line arguments
    character(len=80)         :: cmd
    logical                   :: elim_islands = .true.
    logical                   :: one_sol      = .true.
    logical                   :: script_mode  = .false.

    integer, parameter        :: isin(0:3) = (/ 0, 1,  0, -1 /),    &
                                 icos(0:3) = (/ 1, 0, -1,  0 /)
    integer, parameter        :: eye(3, 3) = reshape((/ 1, 0, 0,    &
                                                        0, 1, 0,    &
                                                        0, 0, 1 /), &
                                                     (/ 3, 3 /))
    integer                   :: mrot(3, 3, 3, 4), cube(3, 3, 3), calls
    type(puts_t), allocatable :: puts_cache(:)
    type(list)                :: sols
    real                      :: t0, t1
    type(puzzle)              :: puzz

    ! Parse command line.
    call get_command(cmd)
    elim_islands = (index(cmd, '--no-elim-islands') == 0)
    one_sol      = (index(cmd, '--many-sols') == 0)
    script_mode  = (index(cmd, '--script-mode') /= 0)

    call cpu_time(t0)
    call init_mrot(mrot)
    cube = 0
    call read_puzzle(puzz)
    if (.not.script_mode) call print_puzzle(puzz)
    sols%length = 0
    allocate(sols%d(3, 3, 3, 16))

    calls = 1
    call search(puzz%ps, size(puzz%ps), 1, cube, sols)
    call finish()

contains
    subroutine init_mrot(mrot)
        integer, intent(inout) :: mrot(3, 3, 3, 4)
        integer                :: theta ! In 90 degree increments

        do theta = 1, 3
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
        end do
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
        do i = 1, size(p1, 2)
            p2(:, i) = p1(:, i) - col_mins + 1
        end do
    end function

    pure function sparse_to_dense(p1, id) result(p2)
        integer, intent(in) :: p1(:, :), id
        integer             :: i
        integer             :: p2(3, 3, 3)

        p2 = 0
        do i = 1, size(p1, 2)
            p2(p1(1, i), p1(2, i), p1(3, i)) = id
        end do
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
                    d = sparse_to_dense(p1, 1)
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

    pure function all_puts(ps, id) result(cubes)
        integer, intent(in)                  :: id
        type(list), intent(in)               :: ps
        integer, allocatable                 :: cubes(:, :, :, :)
        integer                              :: i, n, x, y, z, rmax(3)
        integer, dimension(3, size(ps%s, 2)) :: p, px, py, pz

        p = ps%s(:, :, 1)
        rmax = maxval(p, 2)
        allocate(cubes(3, 3, 3, ps%length * (4 - rmax(1)) * (4 - rmax(2)) * (4 - rmax(3))))
        n = 0

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
                        n = n + 1
                        pz = py
                        pz(3, :) = pz(3, :) + z
                        cubes(:, :, :, n) = sparse_to_dense(pz, id)
                    end do
                end do
            end do
        end do
    end function

    function fast_puts(cube, ps, id) result(cubes)
        integer, intent(in)                  :: cube(3, 3, 3), ps(:, :, :, :), id
        type(list)                           :: cubes
        integer                              :: i

        cubes%length = 0
        allocate(cubes%d(3, 3, 3, size(ps, 4)))

        do i = 1, size(ps, 4)
            if (all(ps(:, :, :, i) == 0 .or. cube == 0)) then
                cubes%length = cubes%length + 1
                cubes%d(:, :, :, cubes%length) = cube
                where (ps(:, :, :, i) /= 0) cubes%d(:, :, :, cubes%length) = id
            end if
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

    recursive subroutine search(ps, n, k, cube, sols)
        type(piece), intent(in)   :: ps(:)
        integer, intent(in)       :: n, k, cube(3, 3, 3)
        type(list), intent(inout) :: sols
        type(list)                :: rots, puts
        integer                   :: i, id

        if (n == 0) then
            sols%length = sols%length + 1
            if (sols%length <= size(sols%d, 4)) then
                sols%d(:, :, :, sols%length) = cube
            end if
            if (one_sol) then
                call finish()
                stop
            end if
            return
        end if

        if (elim_islands) then
            if (min_island_vol(cube) < size(ps(1)%s, 2)) return
        end if

        ! This is the first piece.
        !! Potentially doing a little wasted work with the first piece to simplify the code.
        id = n*10 + k
        if (n == size(ps) .and. k == 1) then
            ! Cache all of the pieces' potential placements including
            ! rotations (this can used to estimate the branching factors).
            allocate(puts_cache(n))
            do i = 1, n
                puts_cache(i)%d = all_puts(all_rots(ps(i)%s), i)
            end do

            ! Skip all of this piece's rotations (they are redundant).
            rots%length = 1
            allocate(rots%s(size(ps(n)%s, 1), size(ps(n)%s, 2), 1), &
                     rots%d(0, 0, 0, 0))
            rots%s(:, :, 1) = push_to_one(ps(n)%s)

            puts%d = all_puts(rots, id)
            puts%length = size(puts%d, 4)
        else
            puts = fast_puts(cube, puts_cache(n)%d, id)
        end if

        do i = 1, puts%length
            calls = calls + 1
            if (k < ps(n)%cnt) then
                call search(ps, n, k + 1, puts%d(:, :, :, i), sols)
            else
                call search(ps, n - 1, 1, puts%d(:, :, :, i), sols)
            end if
        end do
    end subroutine

    subroutine read_puzzle(puzz)
        type(puzzle), intent(out) :: puzz
        integer                   :: n, w, i, j

        read *, puzz%name_
        read *, n
        allocate(puzz%ps(n))

        do j = 1, n
            read *, w, puzz%ps(j)%cnt
            allocate(puzz%ps(j)%s(3, w))
            do i = 1, 3
                read *, puzz%ps(j)%s(i, :)
            end do
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

    subroutine print_puzzle(puzz)
        type(puzzle), intent(in) :: puzz
        type(piece)              :: p
        integer                  :: i

        write (*, "(a, a)")  "puzzle name: ", puzz%name_
        write (*, "(a, i3)") "unique pieces: ", size(puzz%ps)
        print *
        do i = 1, size(puzz%ps)
            p = puzz%ps(i)
            write (*, "(a, i2, a, i2, a, i2)") "piece ", i, ", volume ", size(p%s, 2), ", count ", p%cnt
            call print_piece(p%s)
            print *
        end do
        write (*, "(a, i3)") "total pieces: ", sum(puzz%ps%cnt)
        print *
    end subroutine

    subroutine print_cube(c)
        integer, intent(in) :: c(:, :, :)
        integer             :: i, j

        do j = 1, 3
            do i = 1, 3
                write (*, "(3i3)") c(i, 1:3, j)
            end do
            print *
        end do
    end subroutine

    subroutine print_cubes(cs)
        type(list), intent(in) :: cs
        integer                :: i

        do i = 1, min(cs%length, size(cs%d, 4))
            print *
            call print_cube(cs%d(:, :, :, i))
            print *, "********************************"
        end do
    end subroutine

    subroutine finish()
        call cpu_time(t1)
        print '("Solutions:       ", i12)', sols%length
        print '("Calls of search: ", i12)', calls
        print '("Search time(s):  ", f12.3)', t1 - t0
        call print_cubes(sols)
    end subroutine
end program
