package require Tk

set pname ""  ;# Name of the puzzle
set ps [list] ;# List of all the pieces

wm title . "Cube Puzzle Maker"

bind all <Escape> {exit}

grid [ttk::frame .c -padding 10] -row 0 -column 0 -sticky nwes

# Create a grid of checkboxes for adding a piece.
for {set i 1} {$i < 4} {incr i} {
	for {set j 1} {$j < 4} {incr j} {
		for {set k 1} {$k < 4} {incr k} {
			grid [ttk::checkbutton .c.v$i$j$k -variable v$i$j$k] \
				-row [expr {$i + 4*($k - 1)}]                    \
				-column $j                                       \
				-padx 5 -pady 5
			set v$i$j$k 0
		}
	}
}

grid [ttk::label .c.lblname -text "Puzzle Name"]                 -row 1 -column 4
grid [ttk::entry .c.name -textvariable pname]                    -row 2 -column 4
grid [ttk::button .c.addp -text "Add Piece" -command addp]       -row 3 -column 4
grid [ttk::button .c.saveps -text "Save Pieces" -command saveps] -row 4 -column 4
grid [ttk::button .c.quit -text "Quit" -command {exit}]          -row 8 -column 4

proc addp {} {
	set xs [list]
	set ys [list]
	set zs [list]
	for {set i 1} {$i < 4} {incr i} {
		for {set j 1} {$j < 4} {incr j} {
			for {set k 1} {$k < 4} {incr k} {
				if {[set ::v$i$j$k]} {
					lappend xs $i
					lappend ys $j
					lappend zs $k
				}
			}
		}
	}
	lappend ::ps "[llength $xs]\n$xs\n$ys\n$zs\n"
}

proc saveps {} {
	set out [open "[string tolower $::pname].txt" w 0644]
	puts $out $::pname
	puts $out "[llength $::ps]\n"
	foreach p [lsort -dictionary $::ps] {
		puts $out $p
	}
	flush $out
	close $out
}
