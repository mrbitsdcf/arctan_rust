program atan_mrbits

    implicit none                  
    real (kind=8) :: PI = 3.14159265358979323846264338327950288419716939937510
    real (kind=8) :: x, atan_true, y, arc
    real (kind=8), external :: taylor_arctan
    integer :: n

    n = 20
    x = 0.009284533333
    arc = 0.0
    atan_true = atan(x) * 180 / PI
    y = taylor_arctan(x, n, arc)
    print *, "x             = ", x
    print *, "atan_true     = ", atan_true
    print *, "taylor_arctan = ", y
    print *, "error         = ", y - atan_true

end program atan_mrbits

function taylor_arctan(x,n,arc)
    implicit none

    real (kind=8) :: PI = 3.14159265358979323846264338327950288419716939937510
    real (kind=8) ::x, arc
    integer::n, i
    real (kind=8) ::num, nm2
    real (kind=8) ::taylor_arctan

    arc = 0.0

    do i=1,n,4
        num = i
        nm2 = num+2
        arc = arc+((x**num)/(num)) - (x**(nm2)/(nm2))
    enddo

    arc = arc * 180 / PI

    taylor_arctan = arc
end function taylor_arctan
