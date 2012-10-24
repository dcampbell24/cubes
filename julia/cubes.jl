# Copyright 2012 David Campbell.
# Use of this source code is governed by a MIT-style
# license that can be found in the LICENSE file.
const X = 1
const Y = 2
const Z = 3

const PLANES = Array(Any, 3)
PLANES[1] = [
    0 1 2 0 1 2 0 1 2
    0 0 0 1 1 1 2 2 2
    0 0 0 0 0 0 0 0 0
]
PLANES[2] = [
    0 1 2 0 1 2 0 1 2
    0 0 0 1 1 1 2 2 2
    1 1 1 1 1 1 1 1 1
]
PLANES[3] = [
    0 1 2 0 1 2 0 1 2
    0 0 0 1 1 1 2 2 2
    2 2 2 2 2 2 2 2 2
]

const MINOTAUR = Array(Any, 6)
# 2
MINOTAUR[1] = [
    0 1 1 1
    1 1 0 0
    0 0 0 1
]
# 3
MINOTAUR[2] = [
    0 1 1 1
    1 1 1 0
    0 0 1 1
]
# 5
MINOTAUR[3] = [
    0 1 2 1
    0 0 0 1
    0 0 0 0
]
# 4
MINOTAUR[4] = [
    0 0 0 0 1
    0 1 1 2 1
    1 1 0 0 0
]
# 6
MINOTAUR[5] = [
    0 1 1 1 1
    2 0 1 1 2
    0 1 1 0 0
]
# 1
MINOTAUR[6] = [
    0 0 0 1 0
    0 1 2 0 1
    0 0 0 0 1
]

# All of the rotation matricies.
const MROTS = Array(Int, (3, 3, 3, 3))

# 0, 90, 180, and 270 degrees
const SIN = [0, 1, 0, -1]
const COS = [1, 0, -1, 0]

function init()
    mrot  = Array(Int, (3, 3, 3))
    for i in 1:3
        θ = i + 1
        mrot[:, :, 1] = [      1      0        0
                               0 COS[θ]  -SIN[θ]
                               0 SIN[θ]   COS[θ]]

        mrot[:, :, 2] = [ COS[θ]      0   SIN[θ]
                               0      1        0
                         -SIN[θ]      0   COS[θ]]

        mrot[:, :, 3] = [ COS[θ]  -SIN[θ]      0
                          SIN[θ]   COS[θ]      0
                               0       0       1]
        MROTS[:, :, :, i] = mrot
    end
end

function rotate(piece, axis, theta)
    if theta == 0
        return piece
    end
    MROTS[:, :, axis, theta] * piece
end

function push_to_zero(piece)
    v = [min(piece[i, :]) for i in 1:size(piece, 1)]
    p = copy(piece)
    for i in 1:size(piece, 2)
        p[:, i] -= v
    end
    p
end

function col_sort!(M)
    sM = sort([M[:, i] for i in 1:size(M, 2)])
    for i in 1:size(M, 2)
        M[:, i] = sM[i]
    end
    M
end

canonical(piece) = col_sort!(push_to_zero(piece))

has_piece(A, p) = any((a) -> a == p, A)

function all_rots(piece)
    p1 = copy(piece)
    rots = {}
    for i in 0:3
        for j in 0:3
            for k in 0:3
                c = canonical(p1)
                if !has_piece(rots, c)
                    push(rots, c)
                end
                p1 = rotate(p1, X, 1)
            end
            p1 = rotate(p1, Y, 1)
        end
        p1 = rotate(p1, Z, 1)
    end
    rots
end

function is_legal(cube, piece)
    for i in 1:size(piece, 2)
        pt = piece[:, i]
        if cube[pt[X]+1, pt[Y]+1, pt[Z]+1] != 0
            return false
        end
    end
    true
end

function all_puts(cube, piece)
    allputs = {}
    for rot in all_rots(piece)
        maxv = [max(rot[i, :]) for i in 1:size(rot, 1)]
        for x = maxv[X]:2
            px = copy(rot)
            px[X, :] += x - maxv[X]
            for y = maxv[Y]:2
                py = copy(px)
                py[Y, :] += y - maxv[Y]
                for z = maxv[Z]:2
                    pz = copy(py)
                    pz[Z, :] += z - maxv[Z]
                    if is_legal(cube, pz)
                        push(allputs, pz)
                    end
                end
            end
        end
    end
    allputs
end

init()

function place(cube, piece, id)
    for i in 1:size(piece, 2)
        pt = piece[:, i]
        cube[pt[X]+1, pt[Y]+1, pt[Z]+1] = id
    end
end

function dense_to_sparse(cube)
    coords = findn(cube)
    dims = length(coords)
    pts = length(coords[1])
    sparse = Array(Int, dims, pts)
    for d in 1:dims
        for p in 1:pts
            sparse[d, p] = coords[d][p]
        end
    end
    sparse
end


function flip(bit)
    if bit == 0
        return 1
    end
    0
end

function island_vols(cube)
    cube1 = copy(cube)
    xmax, ymax, zmax = size(cube1)
    s = dense_to_sparse(map(flip, cube1))
    holes = Dict()
    for i in 1:size(s, 2)
        holes[s[:, i]] = s[:, i]
    end
    vols = Int[]
    while !isempty(holes)
        volume = 0
        Q = {values(holes)[1]}
        while !isempty(Q)
            n = pop(Q)
            if cube1[n[X], n[Y], n[Z]] == 0
                del(holes, n)
                volume += 1
                cube1[n[X], n[Y], n[Z]] = 1
                if n[X] < xmax
                    push(Q, [n[X]+1, n[Y], n[Z]])
                end
                if n[X] > 1
                    push(Q, [n[X]-1, n[Y], n[Z]])
                end
                if n[Y] < ymax
                    push(Q, [n[X], n[Y]+1, n[Z]])
                end
                if n[Y] > 1
                    push(Q, [n[X], n[Y]-1, n[Z]])
                end
                if n[Z] < zmax
                    push(Q, [n[X], n[Y], n[Z]+1])
                end
                if n[Z] > 1
                    push(Q, [n[X], n[Y], n[Z]-1])
                end
            end
        end
        push(vols, volume)
    end
    vols
end

function search(ss, cube, sols)
    if length(ss) == 0
        push(sols, cube)
        return
    end
    if min(island_vols(cube)) < size(ss[1], 2)
        return
    end
    puts = {}
    cputs = {}
    for v in all_puts(cube, ss[end])
        c1 = copy(cube)
        place(c1, v, length(ss))
        sparse = dense_to_sparse(c1)
        allRots = all_rots(sparse)
        if all([w != rot for w in cputs, rot in allRots])
            push(puts, c1)
            push(cputs, canonical(sparse))
        end
    end
    for p in puts
        search(ss[1:end-1], p, sols)
    end
end
